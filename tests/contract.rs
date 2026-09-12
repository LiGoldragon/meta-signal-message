use meta_signal_message::{
    ByteViewable, ConfigurationRejectionReason, OperationKind, Query, RequestUnimplemented,
    Response, Restorable, Signal, Signalizable, UnimplementedReason,
};
use signal_message::{
    ComponentMessageIngress, ComponentName, InternalComponentInstanceOrigin,
    MessageDaemonConfiguration, OwnerIdentity,
};

fn configuration() -> MessageDaemonConfiguration {
    MessageDaemonConfiguration {
        message_socket_path: "/run/message/message.sock".into(),
        message_socket_mode: 0o660,
        supervision_socket_path: "/run/message/supervision.sock".into(),
        supervision_socket_mode: 0o600,
        router_socket_path: "/run/router/router.sock".into(),
        component_ingresses: vec![ComponentMessageIngress {
            internal_component_instance_origin: InternalComponentInstanceOrigin {
                component_name: ComponentName::Terminal,
                component_instance_name: "operator".into(),
            },
            ingress_socket_path: "/run/message/ingress/terminal-operator.sock".into(),
            socket_mode: 0o600,
        }],
        owner_identity: OwnerIdentity::UnixUser(1000),
    }
}

#[test]
fn query_and_response_round_trip_through_received_bytes() {
    let query = Query::Configure(configuration());
    let received =
        Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
    assert_eq!(received.restore().expect("query restores"), query);

    let response = Response::OperationUnimplemented(RequestUnimplemented {
        unimplemented_operation_kind: OperationKind::Configure,
        reason: UnimplementedReason::DependencyNotReady,
    });
    let received = Signal::<Response>::from(
        response
            .signalize()
            .expect("response archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("response restores"), response);

    let applied = Response::ConfigurationApplied(7);
    let received = Signal::<Response>::from(
        applied
            .signalize()
            .expect("applied archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("applied restores"), applied);

    let refused =
        Response::ConfigurationRefused(ConfigurationRejectionReason::MalformedConfiguration);
    let received = Signal::<Response>::from(
        refused
            .signalize()
            .expect("refused archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("refused restores"), refused);
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
    assert!(
        Signal::<Response>::from(vec![0xff, 0, 1])
            .restore()
            .is_err()
    );
}

#[cfg(feature = "datom")]
mod datom {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn budget() -> Budget {
        Budget {
            remaining: 8192,
            reader: ReaderBudget { remaining: 8192 },
            depth: 0,
            maximum_depth: 256,
        }
    }

    macro_rules! render {
        ($value:expr) => {
            $value.clone().datomize(vec![]).protosize().textualize()
        };
    }

    #[test]
    fn query_round_trips_as_datom_text() {
        let query = Query::Configure(configuration());
        let restored = Potential::<Query>::from(render!(query))
            .actualize(&mut budget())
            .expect("Datom restores the query");
        assert_eq!(restored, query);
    }

    /// Every canonical line is actualized into the contract root its head
    /// belongs to, and re-rendered through the codec to the identical text.
    /// A canonical file nothing reads hides a wrong wire shape; this reads it.
    #[test]
    fn every_canonical_line_actualizes_and_re_renders() {
        let source = include_str!("../examples/canonical.datom");
        let lines: Vec<&str> = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .collect();
        assert_eq!(lines.len(), 7, "canonical file line count");

        let mut queries = 0usize;
        let mut responses = 0usize;
        for line in lines {
            let text = line.to_owned();
            if let Ok(query) = Potential::<Query>::from(text.clone()).actualize(&mut budget()) {
                assert_eq!(
                    render!(query),
                    line,
                    "query re-renders to its canonical line"
                );
                queries += 1;
                continue;
            }
            let response = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| {
                    panic!("canonical line is neither Query nor Response: {line}\n{error:?}")
                });
            assert_eq!(
                render!(response),
                line,
                "response re-renders to its canonical line"
            );
            responses += 1;
        }
        assert_eq!(queries, 1, "every request head is exercised");
        assert_eq!(responses, 6, "every reply head is exercised");
    }
}

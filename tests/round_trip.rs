//! Round-trip witnesses for the schema-derived message meta contract.

use meta_signal_message::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Frame, FrameBody,
    Input, OperationKind, Output, RequestUnimplemented, UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_message::{
    MessageDaemonConfiguration, MessageDaemonConfigurationParts, OwnerIdentity, SocketMode,
    UnixUserIdentifier, WirePath,
};

struct MessageConfigurationFixture;

impl MessageConfigurationFixture {
    fn exchange() -> ExchangeIdentifier {
        ExchangeIdentifier::new(
            SessionEpoch::new(1),
            ExchangeLane::Connector,
            LaneSequence::first(),
        )
    }

    fn path(value: &str) -> WirePath {
        WirePath::new(value.to_owned())
    }

    fn configuration() -> MessageDaemonConfiguration {
        MessageDaemonConfiguration::from(MessageDaemonConfigurationParts {
            message_socket_path: Self::path("/run/persona/X/message.sock"),
            message_socket_mode: SocketMode::new(0o660),
            supervision_socket_path: Self::path("/run/persona/X/message-supervision.sock"),
            supervision_socket_mode: SocketMode::new(0o600),
            router_socket_path: Self::path("/run/persona/X/router.sock"),
            component_ingresses: Vec::new(),
            owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
        })
    }

    fn assert_request_round_trips(request: Input) {
        let frame = Frame::new(FrameBody::Request {
            exchange: Self::exchange(),
            request: request.clone().into_request(),
        });
        let bytes = frame.encode_length_prefixed().expect("encode request");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
        match decoded.into_body() {
            FrameBody::Request {
                request: decoded_request,
                ..
            } => assert_eq!(decoded_request.payloads().head(), &request),
            other => panic!("expected request frame, got {other:?}"),
        }
    }

    fn assert_reply_round_trips(reply: Output) {
        let frame = Frame::new(FrameBody::Reply {
            exchange: Self::exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
        });
        let bytes = frame.encode_length_prefixed().expect("encode reply");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
        match decoded.into_body() {
            FrameBody::Reply {
                reply: decoded_reply,
                ..
            } => match decoded_reply {
                Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                    SubReply::Ok(payload) => assert_eq!(payload, reply),
                    other => panic!("expected accepted reply payload, got {other:?}"),
                },
                Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
            },
            other => panic!("expected reply frame, got {other:?}"),
        }
    }

    #[cfg(feature = "nota-text")]
    fn assert_nota_round_trips<Value>(value: &Value)
    where
        Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
    {
        let text = value.to_nota();
        let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
        assert_eq!(&recovered, value);
    }
}

#[test]
fn configure_request_carries_the_signal_message_configuration_type() {
    let request = Input::Configure(MessageConfigurationFixture::configuration());
    assert_eq!(request.kind(), OperationKind::Configure);
    MessageConfigurationFixture::assert_request_round_trips(request.clone());
    #[cfg(feature = "nota-text")]
    MessageConfigurationFixture::assert_nota_round_trips(&request);
}

#[test]
fn reply_variants_round_trip() {
    let replies = [
        Output::configured(ConfigurationGeneration::new(7)),
        Output::ConfigurationRejected(ConfigurationRejected::new(
            ConfigurationRejectionReason::ManagerAuthorityRequired,
        )),
        Output::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Configure,
            reason: UnimplementedReason::DependencyNotReady,
        }),
    ];
    for reply in replies {
        MessageConfigurationFixture::assert_reply_round_trips(reply.clone());
        #[cfg(feature = "nota-text")]
        MessageConfigurationFixture::assert_nota_round_trips(&reply);
    }
}

#[test]
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

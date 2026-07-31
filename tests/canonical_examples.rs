//! Canonical DOTOS examples round-trip witness.

use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_message::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Generation,
    Input, MessageDaemonConfiguration, OperationKind, Output, Reason, RejectionReason,
    RequestUnimplemented, UnimplementedOperationKind, UnimplementedReason,
};
use signal_message::{
    MessageDaemonConfigurationParts, OwnerIdentity, SocketMode, UnixUserIdentifier, WirePath,
};

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

struct CanonicalFixture;

impl CanonicalFixture {
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

    fn round_trip<Value>(value: Value)
    where
        Value: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
    {
        let text = value.to_dotos();
        let decoded = DotosSource::new(&text).parse::<Value>().expect("decode");
        assert_eq!(decoded, value, "decode for {text}");
        assert!(
            CANONICAL.contains(&text),
            "examples/canonical.dotos missing line: {text}",
        );
    }
}

#[test]
fn canonical_input_examples_round_trip() {
    CanonicalFixture::round_trip(Input::Configure(CanonicalFixture::configuration()));
}

#[test]
fn canonical_output_examples_round_trip() {
    CanonicalFixture::round_trip(Output::ConfigurationApplied(
        Generation::new(ConfigurationGeneration::new(7)).into(),
    ));
    CanonicalFixture::round_trip(Output::ConfigurationRefused(ConfigurationRejected::new(
        RejectionReason::new(ConfigurationRejectionReason::ManagerAuthorityRequired),
    )));
    CanonicalFixture::round_trip(Output::OperationUnimplemented(RequestUnimplemented {
        unimplemented_operation_kind: UnimplementedOperationKind::new(OperationKind::Configure),
        reason: Reason::new(UnimplementedReason::DependencyNotReady),
    }));
}

//! Every privileged record kind as a concrete datom, and each request and
//! reply through the rkyv archive.

use message_library::Configuration;
use meta_signal_message::{ConfigureRefusal, ETHOS, Query, RedeliverRequest, Response};

fn configuration() -> Configuration {
    Configuration {
        ordinary: "/run/user/1001/message/message.sock".into(),
        meta: "/run/user/1001/message/message-owner.sock".into(),
        flow: "/run/user/1001/flow/flow.sock".into(),
    }
}

#[test]
fn contract_identity_includes_shared_configuration() {
    assert!(ETHOS.starts_with(message_library::LIBRARY_ETHOS));
}

#[test]
fn requests_and_replies_survive_the_archive() {
    let configure = Query::Configure(configuration());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configure).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).unwrap(),
        configure
    );
    let query = Query::Redeliver(RedeliverRequest {
        message_id: "m-81b0e4".into(),
        flow_id: "7d41e0".into(),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).unwrap(),
        query
    );
    for response in [
        Response::Configured,
        Response::RestartRequired,
        Response::Refused(ConfigureRefusal::StoreRefused),
    ] {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).unwrap(),
            response
        );
    }
}

#[cfg(feature = "datom")]
mod datom {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_message::{Query, Response};
    use protos::{Compactable, Protosizable, ReaderBudget};

    fn budget() -> Budget {
        Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn every_request_kind_has_a_concrete_datom() {
        for text in [
            "Configure.{ /run/user/1001/message/message.sock /run/user/1001/message/message-owner.sock /run/user/1001/flow/flow.sock }",
            "Send.{ [ 7d41e0 ] HardAbrupt Text.«Stop the ouranos build now.» }",
            "Redeliver.{ m-81b0e4 7d41e0 }",
        ] {
            let query = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        }
    }

    #[test]
    fn every_reply_kind_has_a_concrete_datom() {
        for text in [
            "Configured",
            "RestartRequired",
            "Refused.StoreRefused",
            "Submitted.{ m-81b0e4 [ { 7d41e0 Observed Transported } ] }",
            "SendRejected.EmptyRecipients",
            "Redelivered.{ 7d41e0 Observed Presented }",
            "RedeliverRejected.NotUncertain",
            "MetaRefused.PeerUnknown",
            "MetaRefused.PeerNotAuthorized.{ da88cf Field Medium gpt-5.5 }",
        ] {
            let response = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(response.datomize(vec![]).protosize().compact(), text);
        }
    }
}

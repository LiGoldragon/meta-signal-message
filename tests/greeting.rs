use meta_signal_message::{ETHOS, Query};
use signal::{Contracted, Handshake, HandshakeReceipt, HandshakeRejection};

#[test]
fn a_peer_built_from_this_source_is_greeted() {
    let digest = Query::contract_digest();
    assert_eq!(digest, Handshake::of_source(ETHOS).contract_digest);
    assert_eq!(
        Query::receipt(&Handshake::of_source(ETHOS)),
        HandshakeReceipt::Greeted(digest)
    );
}

#[test]
fn a_peer_built_from_another_source_is_refused_with_this_digest() {
    let older = format!("{ETHOS}\n");
    assert_eq!(
        Query::receipt(&Handshake::of_source(&older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}

#[test]
fn the_ordinary_contract_is_refused_on_the_meta_socket() {
    assert_eq!(
        Query::receipt(&<signal_message::Query as Contracted>::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}

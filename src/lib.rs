//! Privileged Message Nexus configuration Signal contract.

extern crate signal_message as message_library;

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv Signal frame is one shared type across the estate.
/// Declaring a second copy here would fork the wire, so it is imported.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

mod contract_source {
    include!(concat!(env!("OUT_DIR"), "/contract_source.rs"));
}

pub use contract_source::ETHOS;
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

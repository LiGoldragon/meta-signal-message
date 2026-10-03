//! Privileged Message Nexus Signal contract: configuration, the owner's
//! Send, and Redeliver out of Uncertain.
//!
//! The Send request and its replies are the ordinary contract's own types,
//! imported by identity, so the owner's Send and a flow's Send are one shape.

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv Signal frame is one shared type across the estate.
/// Declaring a second copy here would fork the wire, so it is imported.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

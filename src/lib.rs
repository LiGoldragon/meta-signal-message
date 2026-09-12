//! Meta Message Signal contract — the privileged configuration channel of the
//! Message daemon.
//!
//! The ordinary contract owns `MessageDaemonConfiguration`; this contract
//! imports it by identity rather than redeclaring it, so the manager and the
//! daemon agree on one shape.

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv Signal frame is one shared type across the estate.
/// Declaring a second copy here would fork the wire, so it is imported.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

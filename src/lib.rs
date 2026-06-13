//! Schema-derived meta signal contract for privileged `message` daemon
//! configuration.
//!
//! Ordinary message submission and inbox traffic lives in `signal-message`.
//! This crate carries the meta plane: the authenticated `Configure` operation
//! that applies `message`'s typed daemon configuration (the ingress socket
//! mode and engine-owner origin policy the stamp-and-forward daemon binds).

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl ConfigurationGeneration {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl Input {
    pub fn kind(&self) -> OperationKind {
        match self {
            Self::Configure(_) => OperationKind::Configure,
        }
    }
}

pub type Operation = Input;
pub type MetaMessageOperation = Input;
pub type MetaMessageReply = Output;

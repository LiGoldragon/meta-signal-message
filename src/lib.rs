//! Owner Message configuration Interface.
//!
//! The producer-owned Message configuration Type is imported by identity.
//! The local Type projection is strict; owner request/reply behavior remains
//! handwritten until Logos owns that operational slice.

pub mod bootstrap_manifest;
pub mod schema;

pub const META_MESSAGE_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const META_MESSAGE_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

pub use schema::lib::*;

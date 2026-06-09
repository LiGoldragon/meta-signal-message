//! Meta signal contract — privileged `message` daemon configuration.
//!
//! Ordinary message submission and inbox traffic lives in `signal-message`.
//! This crate carries the meta plane: the authenticated `Configure` operation
//! that applies `message`'s typed daemon configuration (the ingress socket
//! mode and engine-owner origin policy the stamp-and-forward daemon binds).
//!
//! The basic meta operation of every component is daemon configuration — the
//! `MessageDaemonConfiguration` the Persona manager encodes is itself the
//! binary startup message, and later reconfiguration arrives over this meta
//! plane as the same typed record, never as flags.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_message::MessageDaemonConfiguration;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct ConfigurationGeneration(u64);

impl ConfigurationGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl NotaDecode for ConfigurationGeneration {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

impl NotaEncode for ConfigurationGeneration {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    UnsupportedConfiguration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaMessage {
        operation Configure(MessageDaemonConfiguration),
    }
    reply MetaMessageReply {
        Configured(Configured),
        ConfigurationRejected(ConfigurationRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
}

impl From<MessageDaemonConfiguration> for Operation {
    fn from(payload: MessageDaemonConfiguration) -> Self {
        Self::Configure(payload)
    }
}

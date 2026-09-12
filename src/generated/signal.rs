#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ConfigurationRejected = RejectionReason;
#[rustfmt::skip]
pub type RejectionReason = ConfigurationRejectionReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    UnsupportedConfiguration,
    MalformedConfiguration,
}
#[rustfmt::skip]
pub type Configured = Generation;
#[rustfmt::skip]
pub type Generation = ConfigurationGeneration;
#[rustfmt::skip]
pub type ConfigurationGeneration = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RequestUnimplemented {
    pub unimplemented_operation_kind: UnimplementedOperationKind,
    pub reason: Reason,
}
#[rustfmt::skip]
pub type UnimplementedOperationKind = OperationKind;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OperationKind {
    Configure,
}
#[rustfmt::skip]
pub type Reason = UnimplementedReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Configure(signal_message::MessageDaemonConfiguration),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    ConfigurationRefused(ConfigurationRejected),
    ConfigurationApplied(Configured),
    OperationUnimplemented(RequestUnimplemented),
}

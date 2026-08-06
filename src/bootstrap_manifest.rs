//! Explicit producer-owned bootstrap authority state for the owner Message Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    229, 122, 127, 139, 166, 123, 79, 69, 243, 59, 234, 182, 103, 248, 144, 166, 164, 16, 205, 124,
    120, 232, 79, 106, 188, 34, 114, 157, 70, 78, 169, 105,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 39506;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 13490;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 14197, 0xd99de049842377df);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 1987, 0xcd82a65e62cdd412);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 46221, 0x9004f3604886cd47);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 600, 0xb56682132ade699a);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 1596, 0x14e8404794366d6d);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 30245, 0x467b854e8670db91);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 52442, 0x9a6b653e2d09e6fa);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 58551, 0xab0206941ebabbb3);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 1155, 0xd84ea61f20322d95);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 47563, 0x7d73dea87f8280a5);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 46232, 0x54feb9b99f89b463);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 22000, 0xd89a553bd4720d40);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 42074, 0xcfd6df74480c4dfd);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 45350, 0x9c0f1391e8968cd2);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 15469, 0x2254fe32c18d3781);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 43412, 0x62405b78a2927971);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    36508, 18451, 41072, 21915, 7819, 7510, 19574, 28311, 15301, 14880,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "ConfigurationGeneration", 43303, 0xc0a9470661cbdd48),
    DeclarationSeat::new(None, "Generation", 2404, 0x01e0f4af625c0df7),
    DeclarationSeat::new(None, "Configured", 45336, 0x1815f592d30acc71),
    DeclarationSeat::new(
        None,
        "ConfigurationRejectionReason",
        35050,
        0xc2635221e60d91c2,
    ),
    DeclarationSeat::new(
        Some(35050),
        "ManagerAuthorityRequired",
        51140,
        0x77c0dc6d48739781,
    ),
    DeclarationSeat::new(
        Some(35050),
        "MalformedConfiguration",
        26331,
        0xf7ad16042c321260,
    ),
    DeclarationSeat::new(
        Some(35050),
        "UnsupportedConfiguration",
        34590,
        0xca4e2d7ff55e5096,
    ),
    DeclarationSeat::new(None, "RejectionReason", 34671, 0x8ce4e63423646662),
    DeclarationSeat::new(None, "ConfigurationRejected", 24988, 0x0b1e54dc7ea212ef),
    DeclarationSeat::new(None, "OperationKind", 41418, 0x06bf5f53068b444f),
    DeclarationSeat::new(Some(41418), "Configure", 60008, 0x486c49aff14c0080),
    DeclarationSeat::new(None, "UnimplementedReason", 4538, 0x1c03c70d08ebfb53),
    DeclarationSeat::new(Some(4538), "NotBuiltYet", 575, 0x0f448bb20ddeb9c8),
    DeclarationSeat::new(Some(4538), "DependencyNotReady", 38277, 0x6252be863195c1ac),
    DeclarationSeat::new(
        None,
        "UnimplementedOperationKind",
        29834,
        0xb54dfba69e49d5c1,
    ),
    DeclarationSeat::new(None, "Reason", 712, 0x696fcbdc7970d8b2),
    DeclarationSeat::new(None, "RequestUnimplemented", 17963, 0x28e05212de7e470d),
    DeclarationSeat::new(None, "OwnerRequest", 54715, 0x4230262a502309aa),
    DeclarationSeat::new(Some(54715), "Configure", 35679, 0x5204962e708addb2),
    DeclarationSeat::new(None, "OwnerReply", 42301, 0xafd9de76a6bf05af),
    DeclarationSeat::new(
        Some(42301),
        "ConfigurationApplied",
        24615,
        0x4ef5723cb91aed0e,
    ),
    DeclarationSeat::new(
        Some(42301),
        "ConfigurationRefused",
        56356,
        0x0cf5969358a321d4,
    ),
    DeclarationSeat::new(
        Some(42301),
        "OperationUnimplemented",
        54808,
        0xa1b3427ff0f4b6b5,
    ),
];

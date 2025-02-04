//! Zbb extension Instruction

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zbb Extension.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ZbbOpcode {
    /// TODO: Add a description of the instruction here.
    RORIW,

    /// TODO: Add a description of the instruction here.
    RORI,

    /// TODO: Add a description of the instruction here.
    ROLW,

    /// TODO: Add a description of the instruction here.
    RORW,

    /// TODO: Add a description of the instruction here.
    ANDN,

    /// TODO: Add a description of the instruction here.
    ORN,

    /// TODO: Add a description of the instruction here.
    XNOR,

    /// TODO: Add a description of the instruction here.
    MAX,

    /// TODO: Add a description of the instruction here.
    MAXU,

    /// TODO: Add a description of the instruction here.
    MIN,

    /// TODO: Add a description of the instruction here.
    MINU,

    /// TODO: Add a description of the instruction here.
    ROL,

    /// TODO: Add a description of the instruction here.
    ROR,

    /// TODO: Add a description of the instruction here.
    SEXTB,

    /// TODO: Add a description of the instruction here.
    SEXTH,

    /// TODO: Add a description of the instruction here.
    ZEXTH,

    /// TODO: Add a description of the instruction here.
    REV8,

    /// TODO: Add a description of the instruction here.
    ORCB,

    /// TODO: Add a description of the instruction here.
    CPOP,

    /// TODO: Add a description of the instruction here.
    CPOPW,

    /// TODO: Add a description of the instruction here.
    CLZ,

    /// TODO: Add a description of the instruction here.
    CLZW,

    /// TODO: Add a description of the instruction here.
    CTZ,

    /// TODO: Add a description of the instruction here.
    CTZW,
}

impl Display for ZbbOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZbbOpcode::RORIW => write!(f, "roriw"),
            ZbbOpcode::RORI => write!(f, "rori"),
            ZbbOpcode::ROLW => write!(f, "rolw"),
            ZbbOpcode::RORW => write!(f, "rorw"),
            ZbbOpcode::ANDN => write!(f, "andn"),
            ZbbOpcode::ORN => write!(f, "orn"),
            ZbbOpcode::XNOR => write!(f, "xnor"),
            ZbbOpcode::MAX => write!(f, "max"),
            ZbbOpcode::MAXU => write!(f, "maxu"),
            ZbbOpcode::MIN => write!(f, "min"),
            ZbbOpcode::MINU => write!(f, "minu"),
            ZbbOpcode::ROL => write!(f, "rol"),
            ZbbOpcode::ROR => write!(f, "ror"),
            ZbbOpcode::SEXTB => write!(f, "sextb"),
            ZbbOpcode::SEXTH => write!(f, "sexth"),
            ZbbOpcode::ZEXTH => write!(f, "zexth"),
            ZbbOpcode::REV8 => write!(f, "rev8"),
            ZbbOpcode::ORCB => write!(f, "orcb"),
            ZbbOpcode::CPOP => write!(f, "cpop"),
            ZbbOpcode::CPOPW => write!(f, "cpopw"),
            ZbbOpcode::CLZ => write!(f, "clz"),
            ZbbOpcode::CLZW => write!(f, "clzw"),
            ZbbOpcode::CTZ => write!(f, "ctz"),
            ZbbOpcode::CTZW => write!(f, "ctzw"),
        }
    }
}

impl Opcode for ZbbOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZbbOpcode::RORIW => InstFormat::RFormat,
            ZbbOpcode::RORI => InstFormat::RFormat,
            ZbbOpcode::ROLW => InstFormat::RFormat,
            ZbbOpcode::RORW => InstFormat::RFormat,
            ZbbOpcode::ANDN => InstFormat::RFormat,
            ZbbOpcode::ORN => InstFormat::RFormat,
            ZbbOpcode::XNOR => InstFormat::RFormat,
            ZbbOpcode::MAX => InstFormat::RFormat,
            ZbbOpcode::MAXU => InstFormat::RFormat,
            ZbbOpcode::MIN => InstFormat::RFormat,
            ZbbOpcode::MINU => InstFormat::RFormat,
            ZbbOpcode::ROL => InstFormat::RFormat,
            ZbbOpcode::ROR => InstFormat::RFormat,
            ZbbOpcode::SEXTB => InstFormat::RShamtFormat,
            ZbbOpcode::SEXTH => InstFormat::RShamtFormat,
            ZbbOpcode::ZEXTH => InstFormat::RShamtFormat,
            ZbbOpcode::REV8 => InstFormat::RShamtFormat,
            ZbbOpcode::ORCB => InstFormat::RShamtFormat,
            ZbbOpcode::CPOP => InstFormat::RShamtFormat,
            ZbbOpcode::CPOPW => InstFormat::RShamtFormat,
            ZbbOpcode::CLZ => InstFormat::RShamtFormat,
            ZbbOpcode::CLZW => InstFormat::RShamtFormat,
            ZbbOpcode::CTZ => InstFormat::RShamtFormat,
            ZbbOpcode::CTZW => InstFormat::RShamtFormat,
        }
    }
}

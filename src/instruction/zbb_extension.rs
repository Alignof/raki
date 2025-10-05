//! Zbb extension Instruction

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zbb Extension.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ZbbOpcode {
    /// Rotate Right Word by Immediate
    RORIW,

    /// Rotate Right (Immediate)
    RORI,

    /// Rotate Left Word (Register)
    ROLW,

    /// Rotate Right Word (Register)
    RORW,

    /// AND with inverted operand
    ANDN,

    /// OR with inverted operand
    ORN,

    /// Exclusive NOR
    XNOR,

    /// Maximum
    MAX,

    /// Unsigned maximum
    MAXU,

    /// Minimum
    MIN,

    /// Unsigned minimum
    MINU,

    /// Rotate Left (Register)
    ROL,

    /// Rotate Right
    ROR,

    /// Sign-extend byte
    SEXTB,

    /// Sign-extend halfword
    SEXTH,

    /// Zero-extend halfword
    ZEXTH,

    /// Byte-reverse register
    REV8,

    /// Bitwise OR-Combine, byte granule
    ORCB,

    /// Count set bits
    CPOP,

    /// Count set bits in word
    CPOPW,

    /// Count leading zero bits
    CLZ,

    /// Count leading zero bits in word
    CLZW,

    /// Count leading zero bits
    CTZ,

    /// Count leading zero bits in word
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
            ZbbOpcode::RORIW
            | ZbbOpcode::RORI
            | ZbbOpcode::ROLW
            | ZbbOpcode::RORW
            | ZbbOpcode::ANDN
            | ZbbOpcode::ORN
            | ZbbOpcode::XNOR
            | ZbbOpcode::MAX
            | ZbbOpcode::MAXU
            | ZbbOpcode::MIN
            | ZbbOpcode::MINU
            | ZbbOpcode::ROL
            | ZbbOpcode::ROR => InstFormat::RFormat,
            ZbbOpcode::SEXTB
            | ZbbOpcode::SEXTH
            | ZbbOpcode::ZEXTH
            | ZbbOpcode::REV8
            | ZbbOpcode::ORCB
            | ZbbOpcode::CPOP
            | ZbbOpcode::CPOPW
            | ZbbOpcode::CLZ
            | ZbbOpcode::CLZW
            | ZbbOpcode::CTZ
            | ZbbOpcode::CTZW => InstFormat::RShamtFormat,
        }
    }
}

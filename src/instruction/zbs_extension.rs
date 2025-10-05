//! Zbs extension Instruction

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zbs Extension.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ZbsOpcode {
    /// Single-Bit Clear (Immediate)
    BCLRI,

    /// Single-Bit Extract (Immediate)
    BEXTI,

    /// Single-Bit Invert (Immediate)
    BINVI,

    /// Single-Bit Set (Immediate)
    BSETI,

    /// Single-Bit Clear (Register)
    BCLR,

    /// Single-Bit Extract (Register)
    BEXT,

    /// Single-Bit Invert (Register)
    BINV,

    /// Single-Bit Set (Register)
    BSET,
}

impl Display for ZbsOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZbsOpcode::BCLRI => write!(f, "bclri"),
            ZbsOpcode::BEXTI => write!(f, "bexti"),
            ZbsOpcode::BINVI => write!(f, "binvi"),
            ZbsOpcode::BSETI => write!(f, "bseti"),
            ZbsOpcode::BCLR => write!(f, "bclr"),
            ZbsOpcode::BEXT => write!(f, "bext"),
            ZbsOpcode::BINV => write!(f, "binv"),
            ZbsOpcode::BSET => write!(f, "bset"),
        }
    }
}

impl Opcode for ZbsOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZbsOpcode::BCLRI => InstFormat::RShamtFormat,
            ZbsOpcode::BEXTI => InstFormat::RShamtFormat,
            ZbsOpcode::BINVI => InstFormat::RShamtFormat,
            ZbsOpcode::BSETI => InstFormat::RShamtFormat,
            ZbsOpcode::BCLR => InstFormat::RFormat,
            ZbsOpcode::BEXT => InstFormat::RFormat,
            ZbsOpcode::BINV => InstFormat::RFormat,
            ZbsOpcode::BSET => InstFormat::RFormat,
        }
    }
}

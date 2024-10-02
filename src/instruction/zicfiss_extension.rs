//! Zicfiss extension Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zicntr Extension.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ZicfissOpcode {
    /// Shadow stack push.
    SSPUSH,
    /// Shadow stack push (compressed).
    C_SSPUSH,
    /// Shadow stack pop and check.
    SSPOPCHK,
    /// Shadow stack pop and check (compressed).
    C_SSPOPCHK,
    /// Shadow stack read pointer.
    SSRDP,
    /// Shadow stack AMO swap (word).
    SSAMOSWAP_W,
    /// Shadow stack AMO swap (double word).
    SSAMOSWAP_D,
}

impl Display for ZicfissOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZicfissOpcode::SSPUSH => write!(f, "sspush"),
            ZicfissOpcode::C_SSPUSH => write!(f, "c.sspush"),
            ZicfissOpcode::SSPOPCHK => write!(f, "sspopchk"),
            ZicfissOpcode::C_SSPOPCHK => write!(f, "c.sspopchk"),
            ZicfissOpcode::SSRDP => write!(f, "ssrdp"),
            ZicfissOpcode::SSAMOSWAP_W => write!(f, "ssamoswap.w"),
            ZicfissOpcode::SSAMOSWAP_D => write!(f, "ssamoswap.d"),
        }
    }
}

impl Opcode for ZicfissOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZicfissOpcode::SSPUSH => InstFormat::OnlyRs1,
            ZicfissOpcode::SSPOPCHK => InstFormat::OnlyRs2,
            ZicfissOpcode::SSRDP => InstFormat::OnlyRd,
            ZicfissOpcode::SSAMOSWAP_W | ZicfissOpcode::SSAMOSWAP_D => InstFormat::AFormat,
            ZicfissOpcode::C_SSPUSH | ZicfissOpcode::C_SSPOPCHK => InstFormat::NoOperand,
        }
    }
}

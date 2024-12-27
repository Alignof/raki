//! Zicboz extension Instruction.
//!
//! The RISC-V Instruction Set Manual Volume I p.108 19.7.4 cbo.zero

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zicboz Extension.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ZicbozOpcode {
    CBO_ZERO,
}

impl Display for ZicbozOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZicbozOpcode::CBO_ZERO => write!(f, "cbo.zero"),
        }
    }
}

impl Opcode for ZicbozOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZicbozOpcode::CBO_ZERO => InstFormat::OnlyRs1,
        }
    }
}

//! Zicsr extension Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zifencei Extension.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ZifenceiOpcode {
    FENCE,
}

impl Display for ZifenceiOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZifenceiOpcode::FENCE => write!(f, "fence.i"),
        }
    }
}

impl Opcode for ZifenceiOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZifenceiOpcode::FENCE => InstFormat::NoOperand,
        }
    }
}

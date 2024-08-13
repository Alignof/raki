//! Zicsr extension Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zicsr Extension.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ZicsrOpcode {
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
}

impl Display for ZicsrOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZicsrOpcode::CSRRW => write!(f, "csrrw"),
            ZicsrOpcode::CSRRS => write!(f, "csrrs"),
            ZicsrOpcode::CSRRC => write!(f, "csrrc"),
            ZicsrOpcode::CSRRWI => write!(f, "csrrwi"),
            ZicsrOpcode::CSRRSI => write!(f, "csrrsi"),
            ZicsrOpcode::CSRRCI => write!(f, "csrrci"),
        }
    }
}

impl Opcode for ZicsrOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZicsrOpcode::CSRRW | ZicsrOpcode::CSRRS | ZicsrOpcode::CSRRC => InstFormat::CSRformat,
            ZicsrOpcode::CSRRWI | ZicsrOpcode::CSRRSI | ZicsrOpcode::CSRRCI => {
                InstFormat::CSRuiformat
            }
        }
    }
}

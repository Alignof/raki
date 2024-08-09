//! Zicsr extension Instruction.

use core::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
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

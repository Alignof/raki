//! Privileged Instruction.

use core::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
pub enum PrivOpcode {
    SRET,
    MRET,
    WFI,
    SFENCE_VMA,
}

impl Display for PrivOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PrivOpcode::SRET => write!(f, "sret"),
            PrivOpcode::MRET => write!(f, "mret"),
            PrivOpcode::WFI => write!(f, "wfi"),
            PrivOpcode::SFENCE_VMA => write!(f, "sfence.vma"),
        }
    }
}

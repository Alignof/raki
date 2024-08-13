//! Zicntr extension Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Zicntr Extension.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ZicntrOpcode {
    // For Rv32, these instructions indicate lower 32 bits.
    // For Rv64, these instructions do not exist.
    RDCYCLE_H,
    RDTIME_H,
    RDINSTRET_H,

    // For Rv32, these instructions indicate upper 32 bits.
    // For Rv64, these instructions can access the full 64-bit CSRs directly.
    RDCYCLE,
    RDTIME,
    RDINSTRET,
}

impl Display for ZicntrOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZicntrOpcode::RDCYCLE_H => write!(f, "rdcycleh"),
            ZicntrOpcode::RDTIME_H => write!(f, "rdtimeh"),
            ZicntrOpcode::RDINSTRET_H => write!(f, "rdinstreth"),
            ZicntrOpcode::RDCYCLE => write!(f, "rdcycle"),
            ZicntrOpcode::RDTIME => write!(f, "rdtime"),
            ZicntrOpcode::RDINSTRET => write!(f, "rdinstret"),
        }
    }
}

impl Opcode for ZicntrOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZicntrOpcode::RDCYCLE_H
            | ZicntrOpcode::RDTIME_H
            | ZicntrOpcode::RDINSTRET_H
            | ZicntrOpcode::RDCYCLE
            | ZicntrOpcode::RDTIME
            | ZicntrOpcode::RDINSTRET => InstFormat::CSRcntrformat,
        }
    }
}

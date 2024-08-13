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

//! M extension Instruction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in M Extension.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum MOpcode {
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,

    //-- rv64 only --
    MULW,
    DIVW,
    DIVUW,
    REMW,
    REMUW,
}

impl Display for MOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MOpcode::MUL => write!(f, "mul"),
            MOpcode::MULH => write!(f, "mulh"),
            MOpcode::MULHSU => write!(f, "mulhsu,"),
            MOpcode::MULHU => write!(f, "mulhu"),
            MOpcode::DIV => write!(f, "div"),
            MOpcode::DIVU => write!(f, "divu"),
            MOpcode::REM => write!(f, "rem"),
            MOpcode::REMU => write!(f, "remu"),
            MOpcode::MULW => write!(f, "mulw"),
            MOpcode::DIVW => write!(f, "divw"),
            MOpcode::DIVUW => write!(f, "divuw"),
            MOpcode::REMW => write!(f, "remw"),
            MOpcode::REMUW => write!(f, "remuw"),
        }
    }
}

impl Opcode for MOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            MOpcode::MUL
            | MOpcode::MULH
            | MOpcode::MULHSU
            | MOpcode::MULHU
            | MOpcode::DIV
            | MOpcode::DIVU
            | MOpcode::REM
            | MOpcode::REMU
            | MOpcode::MULW
            | MOpcode::DIVW
            | MOpcode::DIVUW
            | MOpcode::REMW
            | MOpcode::REMUW => InstFormat::Mformat,
        }
    }
}

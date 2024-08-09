//! M extension Instruction.

use core::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
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

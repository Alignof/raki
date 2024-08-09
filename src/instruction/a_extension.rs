//! A extension Insturction.

use core::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
pub enum AOpcode {
    LR_W,
    SC_W,
    AMOSWAP_W,
    AMOADD_W,
    AMOXOR_W,
    AMOAND_W,
    AMOOR_W,
    AMOMIN_W,
    AMOMAX_W,
    AMOMINU_W,
    AMOMAXU_W,

    //-- rv64 only --
    LR_D,
    SC_D,
    AMOSWAP_D,
    AMOADD_D,
    AMOXOR_D,
    AMOAND_D,
    AMOOR_D,
    AMOMIN_D,
    AMOMAX_D,
    AMOMINU_D,
    AMOMAXU_D,
}

impl Display for AOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AOpcode::LR_W => write!(f, "lr.w"),
            AOpcode::SC_W => write!(f, "sc.w"),
            AOpcode::AMOSWAP_W => write!(f, "amoswap.w"),
            AOpcode::AMOADD_W => write!(f, "amoadd.w"),
            AOpcode::AMOXOR_W => write!(f, "amoxor.w"),
            AOpcode::AMOAND_W => write!(f, "amoand.w"),
            AOpcode::AMOOR_W => write!(f, "amoor.w"),
            AOpcode::AMOMIN_W => write!(f, "amomin.w"),
            AOpcode::AMOMAX_W => write!(f, "amomax.w"),
            AOpcode::AMOMINU_W => write!(f, "amominu.w"),
            AOpcode::AMOMAXU_W => write!(f, "amomaxu.w"),
            AOpcode::LR_D => write!(f, "lr.d"),
            AOpcode::SC_D => write!(f, "sc.d"),
            AOpcode::AMOSWAP_D => write!(f, "amoswap.d"),
            AOpcode::AMOADD_D => write!(f, "amoadd.d"),
            AOpcode::AMOXOR_D => write!(f, "amoxor.d"),
            AOpcode::AMOAND_D => write!(f, "amoand.d"),
            AOpcode::AMOOR_D => write!(f, "amoor.d"),
            AOpcode::AMOMIN_D => write!(f, "amomin.d"),
            AOpcode::AMOMAX_D => write!(f, "amomax.d"),
            AOpcode::AMOMINU_D => write!(f, "amominu.d"),
            AOpcode::AMOMAXU_D => write!(f, "amomaxu.d"),
        }
    }
}

//! C extension Insturction.

use core::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
pub enum COpcode {
    ADDI4SPN,
    LW,
    SW,
    NOP,
    ADDI,
    JAL,
    LI,
    ADDI16SP,
    LUI,
    SRLI,
    SRAI,
    ANDI,
    SUB,
    XOR,
    OR,
    AND,
    J,
    BEQZ,
    BNEZ,
    SLLI,
    LWSP,
    JR,
    MV,
    EBREAK,
    JALR,
    ADD,
    SWSP,

    //-- rv64 only --
    LD,
    SD,
    ADDIW,
    SUBW,
    ADDW,
    LDSP,
    SDSP,
}

impl Display for COpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            COpcode::ADDI4SPN => write!(f, "C.addi4spn"),
            COpcode::LW => write!(f, "C.lw"),
            COpcode::SW => write!(f, "C.sw"),
            COpcode::NOP => write!(f, "C.nop"),
            COpcode::ADDI => write!(f, "C.addi"),
            COpcode::JAL => write!(f, "C.jal"),
            COpcode::LI => write!(f, "C.li"),
            COpcode::ADDI16SP => write!(f, "C.addi16sp"),
            COpcode::LUI => write!(f, "C.lui"),
            COpcode::SRLI => write!(f, "C.srli"),
            COpcode::SRAI => write!(f, "C.srai"),
            COpcode::ANDI => write!(f, "C.andi"),
            COpcode::SUB => write!(f, "C.sub"),
            COpcode::XOR => write!(f, "C.xor"),
            COpcode::OR => write!(f, "C.or"),
            COpcode::AND => write!(f, "C.and"),
            COpcode::J => write!(f, "C.j"),
            COpcode::BEQZ => write!(f, "C.beqz"),
            COpcode::BNEZ => write!(f, "C.bnez"),
            COpcode::SLLI => write!(f, "C.slli"),
            COpcode::LWSP => write!(f, "C.lwsp"),
            COpcode::JR => write!(f, "C.jr"),
            COpcode::MV => write!(f, "C.mv"),
            COpcode::EBREAK => write!(f, "C.ebreak"),
            COpcode::JALR => write!(f, "C.jalr"),
            COpcode::ADD => write!(f, "C.add"),
            COpcode::SWSP => write!(f, "C.swsp"),
            COpcode::LD => write!(f, "C.ld"),
            COpcode::SD => write!(f, "C.sd"),
            COpcode::ADDIW => write!(f, "C.addiw"),
            COpcode::SUBW => write!(f, "C.subw"),
            COpcode::ADDW => write!(f, "C.addw"),
            COpcode::LDSP => write!(f, "C.ldsp"),
            COpcode::SDSP => write!(f, "C.sdsp"),
        }
    }
}

//! Base I Instructions.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in Base-I.
#[allow(non_camel_case_types, clippy::module_name_repetitions)]
#[derive(Debug, PartialEq)]
pub enum BaseIOpcode {
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    ECALL,
    EBREAK,

    //-- rv64 only --
    LWU,
    LD,
    SD,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}

impl Display for BaseIOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BaseIOpcode::LUI => write!(f, "lui"),
            BaseIOpcode::AUIPC => write!(f, "auipc"),
            BaseIOpcode::JAL => write!(f, "jal"),
            BaseIOpcode::JALR => write!(f, "jalr"),
            BaseIOpcode::BEQ => write!(f, "beq"),
            BaseIOpcode::BNE => write!(f, "bne"),
            BaseIOpcode::BLT => write!(f, "blt"),
            BaseIOpcode::BGE => write!(f, "bge"),
            BaseIOpcode::BLTU => write!(f, "bltu"),
            BaseIOpcode::BGEU => write!(f, "bgeu"),
            BaseIOpcode::LB => write!(f, "lb"),
            BaseIOpcode::LH => write!(f, "lh"),
            BaseIOpcode::LW => write!(f, "lw"),
            BaseIOpcode::LBU => write!(f, "lbu"),
            BaseIOpcode::LHU => write!(f, "lhu"),
            BaseIOpcode::SB => write!(f, "sb"),
            BaseIOpcode::SH => write!(f, "sh"),
            BaseIOpcode::SW => write!(f, "sw"),
            BaseIOpcode::ADDI => write!(f, "addi"),
            BaseIOpcode::SLTI => write!(f, "slti"),
            BaseIOpcode::SLTIU => write!(f, "sltiu"),
            BaseIOpcode::XORI => write!(f, "xori"),
            BaseIOpcode::ORI => write!(f, "ori"),
            BaseIOpcode::ANDI => write!(f, "andi"),
            BaseIOpcode::SLLI => write!(f, "slli"),
            BaseIOpcode::SRLI => write!(f, "srli"),
            BaseIOpcode::SRAI => write!(f, "srai"),
            BaseIOpcode::ADD => write!(f, "add"),
            BaseIOpcode::SUB => write!(f, "sub"),
            BaseIOpcode::SLL => write!(f, "sll"),
            BaseIOpcode::SLT => write!(f, "slt"),
            BaseIOpcode::SLTU => write!(f, "sltu"),
            BaseIOpcode::XOR => write!(f, "xor"),
            BaseIOpcode::SRL => write!(f, "srl"),
            BaseIOpcode::SRA => write!(f, "sra"),
            BaseIOpcode::OR => write!(f, "or"),
            BaseIOpcode::AND => write!(f, "and"),
            BaseIOpcode::FENCE => write!(f, "fence"),
            BaseIOpcode::ECALL => write!(f, "ecall"),
            BaseIOpcode::EBREAK => write!(f, "ebreak"),
            BaseIOpcode::LWU => write!(f, "lwu"),
            BaseIOpcode::LD => write!(f, "ld"),
            BaseIOpcode::SD => write!(f, "sd"),
            BaseIOpcode::ADDIW => write!(f, "addiw"),
            BaseIOpcode::SLLIW => write!(f, "slliw"),
            BaseIOpcode::SRLIW => write!(f, "srliw"),
            BaseIOpcode::SRAIW => write!(f, "sraiw"),
            BaseIOpcode::ADDW => write!(f, "addw"),
            BaseIOpcode::SUBW => write!(f, "subw"),
            BaseIOpcode::SLLW => write!(f, "sllw"),
            BaseIOpcode::SRLW => write!(f, "srlw"),
            BaseIOpcode::SRAW => write!(f, "sraw"),
        }
    }
}

impl Opcode for BaseIOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            BaseIOpcode::BEQ
            | BaseIOpcode::BNE
            | BaseIOpcode::BLT
            | BaseIOpcode::BGE
            | BaseIOpcode::BLTU
            | BaseIOpcode::BGEU => InstFormat::Bformat,
            BaseIOpcode::JALR
            | BaseIOpcode::LB
            | BaseIOpcode::LH
            | BaseIOpcode::LW
            | BaseIOpcode::LBU
            | BaseIOpcode::LHU
            | BaseIOpcode::ADDI
            | BaseIOpcode::SLTI
            | BaseIOpcode::SLTIU
            | BaseIOpcode::XORI
            | BaseIOpcode::ORI
            | BaseIOpcode::ANDI
            | BaseIOpcode::LWU
            | BaseIOpcode::LD
            | BaseIOpcode::ADDIW => InstFormat::Iformat,
            BaseIOpcode::SLLI
            | BaseIOpcode::SRLI
            | BaseIOpcode::SRAI
            | BaseIOpcode::SLLIW
            | BaseIOpcode::SRLIW
            | BaseIOpcode::SRAIW => InstFormat::R_SHAMTformat,
            BaseIOpcode::ADD
            | BaseIOpcode::SUB
            | BaseIOpcode::SLL
            | BaseIOpcode::SLT
            | BaseIOpcode::SLTU
            | BaseIOpcode::XOR
            | BaseIOpcode::SRL
            | BaseIOpcode::SRA
            | BaseIOpcode::OR
            | BaseIOpcode::AND
            | BaseIOpcode::ADDW
            | BaseIOpcode::SUBW
            | BaseIOpcode::SLLW
            | BaseIOpcode::SRLW
            | BaseIOpcode::SRAW => InstFormat::Rformat,
            BaseIOpcode::SB | BaseIOpcode::SH | BaseIOpcode::SW | BaseIOpcode::SD => {
                InstFormat::Sformat
            }
            BaseIOpcode::JAL => InstFormat::Jformat,
            BaseIOpcode::LUI | BaseIOpcode::AUIPC => InstFormat::Uformat,
            BaseIOpcode::ECALL | BaseIOpcode::FENCE | BaseIOpcode::EBREAK => {
                InstFormat::Uncategorized
            }
        }
    }
}

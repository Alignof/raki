use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::base_i::BaseIOpcode;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<BaseIOpcode, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct5: u8 = inst.slice(24, 20) as u8;
    let funct6: u8 = inst.slice(31, 26) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match opmap {
        0b011_0111 => Ok(BaseIOpcode::LUI),
        0b001_0111 => Ok(BaseIOpcode::AUIPC),
        0b110_1111 => Ok(BaseIOpcode::JAL),
        0b110_0111 => Ok(BaseIOpcode::JALR),
        0b110_0011 => match funct3 {
            0b000 => Ok(BaseIOpcode::BEQ),
            0b001 => Ok(BaseIOpcode::BNE),
            0b100 => Ok(BaseIOpcode::BLT),
            0b101 => Ok(BaseIOpcode::BGE),
            0b110 => Ok(BaseIOpcode::BLTU),
            0b111 => Ok(BaseIOpcode::BGEU),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b000_0011 => match funct3 {
            0b000 => Ok(BaseIOpcode::LB),
            0b001 => Ok(BaseIOpcode::LH),
            0b010 => Ok(BaseIOpcode::LW),
            0b011 => only_rv64(BaseIOpcode::LD, isa),
            0b100 => Ok(BaseIOpcode::LBU),
            0b101 => Ok(BaseIOpcode::LHU),
            0b110 => only_rv64(BaseIOpcode::LWU, isa),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b010_0011 => match funct3 {
            0b000 => Ok(BaseIOpcode::SB),
            0b001 => Ok(BaseIOpcode::SH),
            0b010 => Ok(BaseIOpcode::SW),
            0b011 => only_rv64(BaseIOpcode::SD, isa),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b001_0011 => match funct3 {
            0b000 => Ok(BaseIOpcode::ADDI),
            0b001 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(BaseIOpcode::SLLI),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(BaseIOpcode::SLLI),
                    _ => Err(DecodingError::InvalidFunct6),
                },
            },
            0b010 => Ok(BaseIOpcode::SLTI),
            0b011 => Ok(BaseIOpcode::SLTIU),
            0b100 => Ok(BaseIOpcode::XORI),
            0b101 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(BaseIOpcode::SRLI),
                    0b010_0000 => Ok(BaseIOpcode::SRAI),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(BaseIOpcode::SRLI),
                    0b01_0000 => Ok(BaseIOpcode::SRAI),
                    _ => Err(DecodingError::InvalidFunct6),
                },
            },
            0b110 => Ok(BaseIOpcode::ORI),
            0b111 => Ok(BaseIOpcode::ANDI),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b011_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => Ok(BaseIOpcode::ADD),
                0b010_0000 => Ok(BaseIOpcode::SUB),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b001 => Ok(BaseIOpcode::SLL),
            0b010 => Ok(BaseIOpcode::SLT),
            0b011 => Ok(BaseIOpcode::SLTU),
            0b100 => Ok(BaseIOpcode::XOR),
            0b101 => match funct7 {
                0b000_0000 => Ok(BaseIOpcode::SRL),
                0b010_0000 => Ok(BaseIOpcode::SRA),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b110 => Ok(BaseIOpcode::OR),
            0b111 => Ok(BaseIOpcode::AND),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b000_1111 => Ok(BaseIOpcode::FENCE),
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => match funct5 {
                    0b00000 => Ok(BaseIOpcode::ECALL),
                    0b00001 => Ok(BaseIOpcode::EBREAK),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b001_1011 => match funct3 {
            0b000 => only_rv64(BaseIOpcode::ADDIW, isa),
            0b001 => only_rv64(BaseIOpcode::SLLIW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(BaseIOpcode::SRLIW, isa),
                0b010_0000 => only_rv64(BaseIOpcode::SRAIW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b011_1011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => only_rv64(BaseIOpcode::ADDW, isa),
                0b010_0000 => only_rv64(BaseIOpcode::SUBW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b001 => only_rv64(BaseIOpcode::SLLW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(BaseIOpcode::SRLW, isa),
                0b010_0000 => only_rv64(BaseIOpcode::SRAW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &BaseIOpcode) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        BaseIOpcode::LUI
        | BaseIOpcode::AUIPC
        | BaseIOpcode::JAL
        | BaseIOpcode::JALR
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
        | BaseIOpcode::SLLI
        | BaseIOpcode::SRLI
        | BaseIOpcode::SRAI
        | BaseIOpcode::ADD
        | BaseIOpcode::SUB
        | BaseIOpcode::SLL
        | BaseIOpcode::SLT
        | BaseIOpcode::SLTU
        | BaseIOpcode::XOR
        | BaseIOpcode::SRL
        | BaseIOpcode::SRA
        | BaseIOpcode::OR
        | BaseIOpcode::AND
        | BaseIOpcode::LWU
        | BaseIOpcode::LD
        | BaseIOpcode::ADDIW
        | BaseIOpcode::SLLIW
        | BaseIOpcode::SRLIW
        | BaseIOpcode::SRAIW
        | BaseIOpcode::ADDW
        | BaseIOpcode::SUBW
        | BaseIOpcode::SLLW
        | BaseIOpcode::SRLW
        | BaseIOpcode::SRAW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &BaseIOpcode) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        BaseIOpcode::JALR
        | BaseIOpcode::BEQ
        | BaseIOpcode::BNE
        | BaseIOpcode::BLT
        | BaseIOpcode::BGE
        | BaseIOpcode::BLTU
        | BaseIOpcode::BGEU
        | BaseIOpcode::LB
        | BaseIOpcode::LH
        | BaseIOpcode::LW
        | BaseIOpcode::LBU
        | BaseIOpcode::LHU
        | BaseIOpcode::SB
        | BaseIOpcode::SH
        | BaseIOpcode::SW
        | BaseIOpcode::ADDI
        | BaseIOpcode::SLTI
        | BaseIOpcode::SLTIU
        | BaseIOpcode::XORI
        | BaseIOpcode::ORI
        | BaseIOpcode::ANDI
        | BaseIOpcode::SLLI
        | BaseIOpcode::SRLI
        | BaseIOpcode::SRAI
        | BaseIOpcode::ADD
        | BaseIOpcode::SUB
        | BaseIOpcode::SLL
        | BaseIOpcode::SLT
        | BaseIOpcode::SLTU
        | BaseIOpcode::XOR
        | BaseIOpcode::SRL
        | BaseIOpcode::SRA
        | BaseIOpcode::OR
        | BaseIOpcode::AND
        | BaseIOpcode::LWU
        | BaseIOpcode::LD
        | BaseIOpcode::SD
        | BaseIOpcode::ADDIW
        | BaseIOpcode::SLLIW
        | BaseIOpcode::SRLIW
        | BaseIOpcode::SRAIW
        | BaseIOpcode::ADDW
        | BaseIOpcode::SUBW
        | BaseIOpcode::SLLW
        | BaseIOpcode::SRLW
        | BaseIOpcode::SRAW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &BaseIOpcode) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        BaseIOpcode::BEQ
        | BaseIOpcode::BNE
        | BaseIOpcode::BLT
        | BaseIOpcode::BGE
        | BaseIOpcode::BLTU
        | BaseIOpcode::BGEU
        | BaseIOpcode::SB
        | BaseIOpcode::SH
        | BaseIOpcode::SW
        | BaseIOpcode::ADD
        | BaseIOpcode::SUB
        | BaseIOpcode::SLL
        | BaseIOpcode::SLT
        | BaseIOpcode::SLTU
        | BaseIOpcode::XOR
        | BaseIOpcode::SRL
        | BaseIOpcode::SRA
        | BaseIOpcode::OR
        | BaseIOpcode::AND
        | BaseIOpcode::SD
        | BaseIOpcode::ADDW
        | BaseIOpcode::SUBW
        | BaseIOpcode::SLLW
        | BaseIOpcode::SRLW
        | BaseIOpcode::SRAW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(inst: u32, opkind: &BaseIOpcode, isa: Isa) -> Result<Option<i32>, DecodingError> {
    let U_type = || (inst.slice(31, 12) << 12) as i32;
    let I_type = || {
        let imm32 = inst.slice(31, 20) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let S_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 0])
            | inst.slice(31, 25).set(&[11, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let B_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 11])
            | inst.slice(31, 25).set(&[12, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 13)
    };
    let J_type = || {
        let imm32 = inst.slice(31, 12).set(&[
            20, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 19, 18, 17, 16, 15, 14, 13, 12,
        ]) as i32;
        inst.to_signed_nbit(imm32, 21)
    };
    let shamt5 = || inst.slice(24, 20) as i32;
    let shamt6 = || inst.slice(25, 20) as i32;

    match opkind {
        // u-type
        BaseIOpcode::LUI | BaseIOpcode::AUIPC => Ok(Some(U_type())),
        // j-type
        BaseIOpcode::JAL => Ok(Some(J_type())),
        // b-type
        BaseIOpcode::BEQ
        | BaseIOpcode::BNE
        | BaseIOpcode::BLT
        | BaseIOpcode::BGE
        | BaseIOpcode::BLTU
        | BaseIOpcode::BGEU => Ok(Some(B_type())),
        // i-type
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
        | BaseIOpcode::ADDIW
        | BaseIOpcode::LD => Ok(Some(I_type())),
        // s-type
        BaseIOpcode::SD | BaseIOpcode::SB | BaseIOpcode::SH | BaseIOpcode::SW => Ok(Some(S_type())),
        BaseIOpcode::SRAI | BaseIOpcode::SLLI | BaseIOpcode::SRLI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        BaseIOpcode::SLLIW | BaseIOpcode::SRLIW | BaseIOpcode::SRAIW => Ok(Some(shamt5())),
        _ => Ok(None),
    }
}

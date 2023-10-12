use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::OpcodeKind;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct5: u8 = inst.slice(24, 20) as u8;
    let funct6: u8 = inst.slice(31, 26) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match opmap {
        0b011_0111 => Ok(OpcodeKind::LUI),
        0b001_0111 => Ok(OpcodeKind::AUIPC),
        0b110_1111 => Ok(OpcodeKind::JAL),
        0b110_0111 => Ok(OpcodeKind::JALR),
        0b110_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::BEQ),
            0b001 => Ok(OpcodeKind::BNE),
            0b100 => Ok(OpcodeKind::BLT),
            0b101 => Ok(OpcodeKind::BGE),
            0b110 => Ok(OpcodeKind::BLTU),
            0b111 => Ok(OpcodeKind::BGEU),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b000_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::LB),
            0b001 => Ok(OpcodeKind::LH),
            0b010 => Ok(OpcodeKind::LW),
            0b011 => only_rv64(OpcodeKind::LD, isa),
            0b100 => Ok(OpcodeKind::LBU),
            0b101 => Ok(OpcodeKind::LHU),
            0b110 => only_rv64(OpcodeKind::LWU, isa),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b010_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::SB),
            0b001 => Ok(OpcodeKind::SH),
            0b010 => Ok(OpcodeKind::SW),
            0b011 => only_rv64(OpcodeKind::SD, isa),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b001_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::ADDI),
            0b001 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(OpcodeKind::SLLI),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(OpcodeKind::SLLI),
                    _ => Err(DecodingError::InvalidFunct6),
                },
            },
            0b010 => Ok(OpcodeKind::SLTI),
            0b011 => Ok(OpcodeKind::SLTIU),
            0b100 => Ok(OpcodeKind::XORI),
            0b101 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(OpcodeKind::SRLI),
                    0b010_0000 => Ok(OpcodeKind::SRAI),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(OpcodeKind::SRLI),
                    0b01_0000 => Ok(OpcodeKind::SRAI),
                    _ => Err(DecodingError::InvalidFunct6),
                },
            },
            0b110 => Ok(OpcodeKind::ORI),
            0b111 => Ok(OpcodeKind::ANDI),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b011_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => Ok(OpcodeKind::ADD),
                0b010_0000 => Ok(OpcodeKind::SUB),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b001 => Ok(OpcodeKind::SLL),
            0b010 => Ok(OpcodeKind::SLT),
            0b011 => Ok(OpcodeKind::SLTU),
            0b100 => Ok(OpcodeKind::XOR),
            0b101 => match funct7 {
                0b000_0000 => Ok(OpcodeKind::SRL),
                0b010_0000 => Ok(OpcodeKind::SRA),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b110 => Ok(OpcodeKind::OR),
            0b111 => Ok(OpcodeKind::AND),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b000_1111 => Ok(OpcodeKind::FENCE),
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => match funct5 {
                    0b00000 => Ok(OpcodeKind::ECALL),
                    0b00001 => Ok(OpcodeKind::EBREAK),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b001_1011 => match funct3 {
            0b000 => only_rv64(OpcodeKind::ADDIW, isa),
            0b001 => only_rv64(OpcodeKind::SLLIW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::SRLIW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SRAIW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b011_1011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::ADDW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SUBW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b001 => only_rv64(OpcodeKind::SLLW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::SRLW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SRAW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        OpcodeKind::LUI
        | OpcodeKind::AUIPC
        | OpcodeKind::JAL
        | OpcodeKind::JALR
        | OpcodeKind::LB
        | OpcodeKind::LH
        | OpcodeKind::LW
        | OpcodeKind::LBU
        | OpcodeKind::LHU
        | OpcodeKind::ADDI
        | OpcodeKind::SLTI
        | OpcodeKind::SLTIU
        | OpcodeKind::XORI
        | OpcodeKind::ORI
        | OpcodeKind::ANDI
        | OpcodeKind::SLLI
        | OpcodeKind::SRLI
        | OpcodeKind::SRAI
        | OpcodeKind::ADD
        | OpcodeKind::SUB
        | OpcodeKind::SLL
        | OpcodeKind::SLT
        | OpcodeKind::SLTU
        | OpcodeKind::XOR
        | OpcodeKind::SRL
        | OpcodeKind::SRA
        | OpcodeKind::OR
        | OpcodeKind::AND
        | OpcodeKind::LWU
        | OpcodeKind::LD
        | OpcodeKind::ADDIW
        | OpcodeKind::SLLIW
        | OpcodeKind::SRLIW
        | OpcodeKind::SRAIW
        | OpcodeKind::ADDW
        | OpcodeKind::SUBW
        | OpcodeKind::SLLW
        | OpcodeKind::SRLW
        | OpcodeKind::SRAW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpcodeKind::JALR
        | OpcodeKind::BEQ
        | OpcodeKind::BNE
        | OpcodeKind::BLT
        | OpcodeKind::BGE
        | OpcodeKind::BLTU
        | OpcodeKind::BGEU
        | OpcodeKind::LB
        | OpcodeKind::LH
        | OpcodeKind::LW
        | OpcodeKind::LBU
        | OpcodeKind::LHU
        | OpcodeKind::SB
        | OpcodeKind::SH
        | OpcodeKind::SW
        | OpcodeKind::ADDI
        | OpcodeKind::SLTI
        | OpcodeKind::SLTIU
        | OpcodeKind::XORI
        | OpcodeKind::ORI
        | OpcodeKind::ANDI
        | OpcodeKind::SLLI
        | OpcodeKind::SRLI
        | OpcodeKind::SRAI
        | OpcodeKind::ADD
        | OpcodeKind::SUB
        | OpcodeKind::SLL
        | OpcodeKind::SLT
        | OpcodeKind::SLTU
        | OpcodeKind::XOR
        | OpcodeKind::SRL
        | OpcodeKind::SRA
        | OpcodeKind::OR
        | OpcodeKind::AND
        | OpcodeKind::LWU
        | OpcodeKind::LD
        | OpcodeKind::SD
        | OpcodeKind::ADDIW
        | OpcodeKind::SLLIW
        | OpcodeKind::SRLIW
        | OpcodeKind::SRAIW
        | OpcodeKind::ADDW
        | OpcodeKind::SUBW
        | OpcodeKind::SLLW
        | OpcodeKind::SRLW
        | OpcodeKind::SRAW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        OpcodeKind::BEQ
        | OpcodeKind::BNE
        | OpcodeKind::BLT
        | OpcodeKind::BGE
        | OpcodeKind::BLTU
        | OpcodeKind::BGEU
        | OpcodeKind::SB
        | OpcodeKind::SH
        | OpcodeKind::SW
        | OpcodeKind::ADD
        | OpcodeKind::SUB
        | OpcodeKind::SLL
        | OpcodeKind::SLT
        | OpcodeKind::SLTU
        | OpcodeKind::XOR
        | OpcodeKind::SRL
        | OpcodeKind::SRA
        | OpcodeKind::OR
        | OpcodeKind::AND
        | OpcodeKind::SD
        | OpcodeKind::ADDW
        | OpcodeKind::SUBW
        | OpcodeKind::SLLW
        | OpcodeKind::SRLW
        | OpcodeKind::SRAW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(inst: u32, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError> {
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
        OpcodeKind::LUI | OpcodeKind::AUIPC => Ok(Some(U_type())),
        // j-type
        OpcodeKind::JAL => Ok(Some(J_type())),
        // b-type
        OpcodeKind::BEQ
        | OpcodeKind::BNE
        | OpcodeKind::BLT
        | OpcodeKind::BGE
        | OpcodeKind::BLTU
        | OpcodeKind::BGEU => Ok(Some(B_type())),
        // i-type
        OpcodeKind::JALR
        | OpcodeKind::LB
        | OpcodeKind::LH
        | OpcodeKind::LW
        | OpcodeKind::LBU
        | OpcodeKind::LHU
        | OpcodeKind::ADDI
        | OpcodeKind::SLTI
        | OpcodeKind::SLTIU
        | OpcodeKind::XORI
        | OpcodeKind::ORI
        | OpcodeKind::ANDI
        | OpcodeKind::LWU
        | OpcodeKind::ADDIW
        | OpcodeKind::LD => Ok(Some(I_type())),
        // s-type
        OpcodeKind::SD | OpcodeKind::SB | OpcodeKind::SH | OpcodeKind::SW => Ok(Some(S_type())),
        OpcodeKind::SRAI | OpcodeKind::SLLI | OpcodeKind::SRLI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        OpcodeKind::SLLIW | OpcodeKind::SRLIW | OpcodeKind::SRAIW => Ok(Some(shamt5())),
        _ => Ok(None),
    }
}

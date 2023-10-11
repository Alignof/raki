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
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b000_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::LB),
            0b001 => Ok(OpcodeKind::LH),
            0b010 => Ok(OpcodeKind::LW),
            0b011 => only_rv64(OpcodeKind::LD, isa),
            0b100 => Ok(OpcodeKind::LBU),
            0b101 => Ok(OpcodeKind::LHU),
            0b110 => only_rv64(OpcodeKind::LWU, isa),
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b010_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::SB),
            0b001 => Ok(OpcodeKind::SH),
            0b010 => Ok(OpcodeKind::SW),
            0b011 => only_rv64(OpcodeKind::SD, isa),
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b001_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::ADDI),
            0b001 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(OpcodeKind::SLLI),
                    _ => Err(DecodingError::IllegalFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(OpcodeKind::SLLI),
                    _ => Err(DecodingError::IllegalFunct6),
                },
            },
            0b010 => Ok(OpcodeKind::SLTI),
            0b011 => Ok(OpcodeKind::SLTIU),
            0b100 => Ok(OpcodeKind::XORI),
            0b101 => match isa {
                Isa::Rv32 => match funct7 {
                    0b000_0000 => Ok(OpcodeKind::SRLI),
                    0b010_0000 => Ok(OpcodeKind::SRAI),
                    _ => Err(DecodingError::IllegalFunct7),
                },
                Isa::Rv64 => match funct6 {
                    0b00_0000 => Ok(OpcodeKind::SRLI),
                    0b01_0000 => Ok(OpcodeKind::SRAI),
                    _ => Err(DecodingError::IllegalFunct6),
                },
            },
            0b110 => Ok(OpcodeKind::ORI),
            0b111 => Ok(OpcodeKind::ANDI),
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b011_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => Ok(OpcodeKind::ADD),
                0b010_0000 => Ok(OpcodeKind::SUB),
                _ => Err(DecodingError::IllegalFunct7),
            },
            0b001 => Ok(OpcodeKind::SLL),
            0b010 => Ok(OpcodeKind::SLT),
            0b011 => Ok(OpcodeKind::SLTU),
            0b100 => Ok(OpcodeKind::XOR),
            0b101 => match funct7 {
                0b000_0000 => Ok(OpcodeKind::SRL),
                0b010_0000 => Ok(OpcodeKind::SRA),
                _ => Err(DecodingError::IllegalFunct7),
            },
            0b110 => Ok(OpcodeKind::OR),
            0b111 => Ok(OpcodeKind::AND),
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b000_1111 => Ok(OpcodeKind::FENCE),
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => match funct5 {
                    0b00000 => Ok(OpcodeKind::ECALL),
                    0b00001 => Ok(OpcodeKind::EBREAK),
                    _ => Err(DecodingError::IllegalFunct5),
                },
                _ => Err(DecodingError::IllegalFunct7),
            },
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b001_1011 => match funct3 {
            0b000 => only_rv64(OpcodeKind::ADDIW, isa),
            0b001 => only_rv64(OpcodeKind::SLLIW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::SRLIW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SRAIW, isa),
                _ => Err(DecodingError::IllegalFunct7),
            },
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b011_1011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::ADDW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SUBW, isa),
                _ => Err(DecodingError::IllegalFunct7),
            },
            0b001 => only_rv64(OpcodeKind::SLLW, isa),
            0b101 => match funct7 {
                0b000_0000 => only_rv64(OpcodeKind::SRLW, isa),
                0b010_0000 => only_rv64(OpcodeKind::SRAW, isa),
                _ => Err(DecodingError::IllegalFunct7),
            },
            _ => Err(DecodingError::IllegalFunct3),
        },
        _ => Err(DecodingError::IllegalOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        OpcodeKind::LUI => Ok(Some(rd)),
        OpcodeKind::AUIPC => Ok(Some(rd)),
        OpcodeKind::JAL => Ok(Some(rd)),
        OpcodeKind::JALR => Ok(Some(rd)),
        OpcodeKind::LB => Ok(Some(rd)),
        OpcodeKind::LH => Ok(Some(rd)),
        OpcodeKind::LW => Ok(Some(rd)),
        OpcodeKind::LBU => Ok(Some(rd)),
        OpcodeKind::LHU => Ok(Some(rd)),
        OpcodeKind::ADDI => Ok(Some(rd)),
        OpcodeKind::SLTI => Ok(Some(rd)),
        OpcodeKind::SLTIU => Ok(Some(rd)),
        OpcodeKind::XORI => Ok(Some(rd)),
        OpcodeKind::ORI => Ok(Some(rd)),
        OpcodeKind::ANDI => Ok(Some(rd)),
        OpcodeKind::SLLI => Ok(Some(rd)),
        OpcodeKind::SRLI => Ok(Some(rd)),
        OpcodeKind::SRAI => Ok(Some(rd)),
        OpcodeKind::ADD => Ok(Some(rd)),
        OpcodeKind::SUB => Ok(Some(rd)),
        OpcodeKind::SLL => Ok(Some(rd)),
        OpcodeKind::SLT => Ok(Some(rd)),
        OpcodeKind::SLTU => Ok(Some(rd)),
        OpcodeKind::XOR => Ok(Some(rd)),
        OpcodeKind::SRL => Ok(Some(rd)),
        OpcodeKind::SRA => Ok(Some(rd)),
        OpcodeKind::OR => Ok(Some(rd)),
        OpcodeKind::AND => Ok(Some(rd)),
        OpcodeKind::LWU => Ok(Some(rd)),
        OpcodeKind::LD => Ok(Some(rd)),
        OpcodeKind::ADDIW => Ok(Some(rd)),
        OpcodeKind::SLLIW => Ok(Some(rd)),
        OpcodeKind::SRLIW => Ok(Some(rd)),
        OpcodeKind::SRAIW => Ok(Some(rd)),
        OpcodeKind::ADDW => Ok(Some(rd)),
        OpcodeKind::SUBW => Ok(Some(rd)),
        OpcodeKind::SLLW => Ok(Some(rd)),
        OpcodeKind::SRLW => Ok(Some(rd)),
        OpcodeKind::SRAW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpcodeKind::JALR => Ok(Some(rs1)),
        OpcodeKind::BEQ => Ok(Some(rs1)),
        OpcodeKind::BNE => Ok(Some(rs1)),
        OpcodeKind::BLT => Ok(Some(rs1)),
        OpcodeKind::BGE => Ok(Some(rs1)),
        OpcodeKind::BLTU => Ok(Some(rs1)),
        OpcodeKind::BGEU => Ok(Some(rs1)),
        OpcodeKind::LB => Ok(Some(rs1)),
        OpcodeKind::LH => Ok(Some(rs1)),
        OpcodeKind::LW => Ok(Some(rs1)),
        OpcodeKind::LBU => Ok(Some(rs1)),
        OpcodeKind::LHU => Ok(Some(rs1)),
        OpcodeKind::SB => Ok(Some(rs1)),
        OpcodeKind::SH => Ok(Some(rs1)),
        OpcodeKind::SW => Ok(Some(rs1)),
        OpcodeKind::ADDI => Ok(Some(rs1)),
        OpcodeKind::SLTI => Ok(Some(rs1)),
        OpcodeKind::SLTIU => Ok(Some(rs1)),
        OpcodeKind::XORI => Ok(Some(rs1)),
        OpcodeKind::ORI => Ok(Some(rs1)),
        OpcodeKind::ANDI => Ok(Some(rs1)),
        OpcodeKind::SLLI => Ok(Some(rs1)),
        OpcodeKind::SRLI => Ok(Some(rs1)),
        OpcodeKind::SRAI => Ok(Some(rs1)),
        OpcodeKind::ADD => Ok(Some(rs1)),
        OpcodeKind::SUB => Ok(Some(rs1)),
        OpcodeKind::SLL => Ok(Some(rs1)),
        OpcodeKind::SLT => Ok(Some(rs1)),
        OpcodeKind::SLTU => Ok(Some(rs1)),
        OpcodeKind::XOR => Ok(Some(rs1)),
        OpcodeKind::SRL => Ok(Some(rs1)),
        OpcodeKind::SRA => Ok(Some(rs1)),
        OpcodeKind::OR => Ok(Some(rs1)),
        OpcodeKind::AND => Ok(Some(rs1)),
        OpcodeKind::LWU => Ok(Some(rs1)),
        OpcodeKind::LD => Ok(Some(rs1)),
        OpcodeKind::SD => Ok(Some(rs1)),
        OpcodeKind::ADDIW => Ok(Some(rs1)),
        OpcodeKind::SLLIW => Ok(Some(rs1)),
        OpcodeKind::SRLIW => Ok(Some(rs1)),
        OpcodeKind::SRAIW => Ok(Some(rs1)),
        OpcodeKind::ADDW => Ok(Some(rs1)),
        OpcodeKind::SUBW => Ok(Some(rs1)),
        OpcodeKind::SLLW => Ok(Some(rs1)),
        OpcodeKind::SRLW => Ok(Some(rs1)),
        OpcodeKind::SRAW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        OpcodeKind::BEQ => Ok(Some(rs2)),
        OpcodeKind::BNE => Ok(Some(rs2)),
        OpcodeKind::BLT => Ok(Some(rs2)),
        OpcodeKind::BGE => Ok(Some(rs2)),
        OpcodeKind::BLTU => Ok(Some(rs2)),
        OpcodeKind::BGEU => Ok(Some(rs2)),
        OpcodeKind::SB => Ok(Some(rs2)),
        OpcodeKind::SH => Ok(Some(rs2)),
        OpcodeKind::SW => Ok(Some(rs2)),
        OpcodeKind::ADD => Ok(Some(rs2)),
        OpcodeKind::SUB => Ok(Some(rs2)),
        OpcodeKind::SLL => Ok(Some(rs2)),
        OpcodeKind::SLT => Ok(Some(rs2)),
        OpcodeKind::SLTU => Ok(Some(rs2)),
        OpcodeKind::XOR => Ok(Some(rs2)),
        OpcodeKind::SRL => Ok(Some(rs2)),
        OpcodeKind::SRA => Ok(Some(rs2)),
        OpcodeKind::OR => Ok(Some(rs2)),
        OpcodeKind::AND => Ok(Some(rs2)),
        OpcodeKind::SD => Ok(Some(rs2)),
        OpcodeKind::ADDW => Ok(Some(rs2)),
        OpcodeKind::SUBW => Ok(Some(rs2)),
        OpcodeKind::SLLW => Ok(Some(rs2)),
        OpcodeKind::SRLW => Ok(Some(rs2)),
        OpcodeKind::SRAW => Ok(Some(rs2)),
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
        OpcodeKind::LUI => Ok(Some(U_type())),
        OpcodeKind::AUIPC => Ok(Some(U_type())),
        OpcodeKind::JAL => Ok(Some(J_type())),
        OpcodeKind::JALR => Ok(Some(I_type())),
        OpcodeKind::BEQ => Ok(Some(B_type())),
        OpcodeKind::BNE => Ok(Some(B_type())),
        OpcodeKind::BLT => Ok(Some(B_type())),
        OpcodeKind::BGE => Ok(Some(B_type())),
        OpcodeKind::BLTU => Ok(Some(B_type())),
        OpcodeKind::BGEU => Ok(Some(B_type())),
        OpcodeKind::LB => Ok(Some(I_type())),
        OpcodeKind::LH => Ok(Some(I_type())),
        OpcodeKind::LW => Ok(Some(I_type())),
        OpcodeKind::LBU => Ok(Some(I_type())),
        OpcodeKind::LHU => Ok(Some(I_type())),
        OpcodeKind::SB => Ok(Some(S_type())),
        OpcodeKind::SH => Ok(Some(S_type())),
        OpcodeKind::SW => Ok(Some(S_type())),
        OpcodeKind::ADDI => Ok(Some(I_type())),
        OpcodeKind::SLTI => Ok(Some(I_type())),
        OpcodeKind::SLTIU => Ok(Some(I_type())),
        OpcodeKind::XORI => Ok(Some(I_type())),
        OpcodeKind::ORI => Ok(Some(I_type())),
        OpcodeKind::ANDI => Ok(Some(I_type())),
        OpcodeKind::SLLI | OpcodeKind::SRLI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        OpcodeKind::SRAI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        OpcodeKind::SLLIW => Ok(Some(shamt5())),
        OpcodeKind::SRLIW => Ok(Some(shamt5())),
        OpcodeKind::SRAIW => Ok(Some(shamt5())),
        OpcodeKind::LWU => Ok(Some(I_type())),
        OpcodeKind::LD => Ok(Some(I_type())),
        OpcodeKind::SD => Ok(Some(S_type())),
        OpcodeKind::ADDIW => Ok(Some(I_type())),
        _ => Ok(None),
    }
}

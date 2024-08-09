use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::m_extension::MOpcode;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<MOpcode, DecodingError> {
    let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
    let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();

    match opmap {
        0b011_0011 => match funct3 {
            0b000 => Ok(MOpcode::MUL),
            0b001 => Ok(MOpcode::MULH),
            0b010 => Ok(MOpcode::MULHSU),
            0b011 => Ok(MOpcode::MULHU),
            0b100 => Ok(MOpcode::DIV),
            0b101 => Ok(MOpcode::DIVU),
            0b110 => Ok(MOpcode::REM),
            0b111 => Ok(MOpcode::REMU),
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b011_1011 => match funct3 {
            0b000 => only_rv64(MOpcode::MULW, isa),
            0b100 => only_rv64(MOpcode::DIVW, isa),
            0b101 => only_rv64(MOpcode::DIVUW, isa),
            0b110 => only_rv64(MOpcode::REMW, isa),
            0b111 => only_rv64(MOpcode::REMUW, isa),
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &MOpcode) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
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
        | MOpcode::REMUW => Ok(Some(rd)),
    }
}

pub fn parse_rs1(inst: u32, opkind: &MOpcode) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
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
        | MOpcode::REMUW => Ok(Some(rs1)),
    }
}

pub fn parse_rs2(inst: u32, opkind: &MOpcode) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
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
        | MOpcode::REMUW => Ok(Some(rs2)),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &MOpcode) -> Result<Option<i32>, DecodingError> {
    Ok(None)
}

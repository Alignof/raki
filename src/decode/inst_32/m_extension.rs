use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::OpcodeKind;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b011_0011 => match funct3 {
            0b000 => Ok(OpcodeKind::MUL),
            0b001 => Ok(OpcodeKind::MULH),
            0b010 => Ok(OpcodeKind::MULHSU),
            0b011 => Ok(OpcodeKind::MULHU),
            0b100 => Ok(OpcodeKind::DIV),
            0b101 => Ok(OpcodeKind::DIVU),
            0b110 => Ok(OpcodeKind::REM),
            0b111 => Ok(OpcodeKind::REMU),
            _ => Err(DecodingError::IllegalFunct3),
        },
        0b011_1011 => match funct3 {
            0b000 => only_rv64(OpcodeKind::MULW, isa),
            0b100 => only_rv64(OpcodeKind::DIVW, isa),
            0b101 => only_rv64(OpcodeKind::DIVUW, isa),
            0b110 => only_rv64(OpcodeKind::REMW, isa),
            0b111 => only_rv64(OpcodeKind::REMUW, isa),
            _ => Err(DecodingError::IllegalFunct3),
        },
        _ => Err(DecodingError::IllegalOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpcodeKind::MUL
        | OpcodeKind::MULH
        | OpcodeKind::MULHSU
        | OpcodeKind::MULHU
        | OpcodeKind::DIV
        | OpcodeKind::DIVU
        | OpcodeKind::REM
        | OpcodeKind::REMU
        | OpcodeKind::MULW
        | OpcodeKind::DIVW
        | OpcodeKind::DIVUW
        | OpcodeKind::REMW
        | OpcodeKind::REMUW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpcodeKind::MUL
        | OpcodeKind::MULH
        | OpcodeKind::MULHSU
        | OpcodeKind::MULHU
        | OpcodeKind::DIV
        | OpcodeKind::DIVU
        | OpcodeKind::REM
        | OpcodeKind::REMU
        | OpcodeKind::MULW
        | OpcodeKind::DIVW
        | OpcodeKind::DIVUW
        | OpcodeKind::REMW
        | OpcodeKind::REMUW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpcodeKind::MUL
        | OpcodeKind::MULH
        | OpcodeKind::MULHSU
        | OpcodeKind::MULHU
        | OpcodeKind::DIV
        | OpcodeKind::DIVU
        | OpcodeKind::REM
        | OpcodeKind::REMU
        | OpcodeKind::MULW
        | OpcodeKind::DIVW
        | OpcodeKind::DIVUW
        | OpcodeKind::REMW
        | OpcodeKind::REMUW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &OpcodeKind) -> Result<Option<i32>, DecodingError> {
    Ok(None)
}

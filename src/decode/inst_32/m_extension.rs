use super::super::{only_rv64, DecodeUtil};
use crate::instruction::OpcodeKind;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<OpcodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            format!("opcode decoding failed in m extension, {inst:b}"),
        ))
    };

    match opmap {
        0b0110011 => match funct3 {
            0b000 => Ok(OpcodeKind::MUL),
            0b001 => Ok(OpcodeKind::MULH),
            0b010 => Ok(OpcodeKind::MULHSU),
            0b011 => Ok(OpcodeKind::MULHU),
            0b100 => Ok(OpcodeKind::DIV),
            0b101 => Ok(OpcodeKind::DIVU),
            0b110 => Ok(OpcodeKind::REM),
            0b111 => Ok(OpcodeKind::REMU),
            _ => illegal_inst_exception(),
        },
        0b0111011 => match funct3 {
            0b000 => only_rv64(OpcodeKind::MULW, isa),
            0b100 => only_rv64(OpcodeKind::DIVW, isa),
            0b101 => only_rv64(OpcodeKind::DIVUW, isa),
            0b110 => only_rv64(OpcodeKind::REMW, isa),
            0b111 => only_rv64(OpcodeKind::REMUW, isa),
            _ => illegal_inst_exception(),
        },
        _ => illegal_inst_exception(),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpcodeKind::MUL => Ok(Some(rd)),
        OpcodeKind::MULH => Ok(Some(rd)),
        OpcodeKind::MULHSU => Ok(Some(rd)),
        OpcodeKind::MULHU => Ok(Some(rd)),
        OpcodeKind::DIV => Ok(Some(rd)),
        OpcodeKind::DIVU => Ok(Some(rd)),
        OpcodeKind::REM => Ok(Some(rd)),
        OpcodeKind::REMU => Ok(Some(rd)),
        OpcodeKind::MULW => Ok(Some(rd)),
        OpcodeKind::DIVW => Ok(Some(rd)),
        OpcodeKind::DIVUW => Ok(Some(rd)),
        OpcodeKind::REMW => Ok(Some(rd)),
        OpcodeKind::REMUW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpcodeKind::MUL => Ok(Some(rs1)),
        OpcodeKind::MULH => Ok(Some(rs1)),
        OpcodeKind::MULHSU => Ok(Some(rs1)),
        OpcodeKind::MULHU => Ok(Some(rs1)),
        OpcodeKind::DIV => Ok(Some(rs1)),
        OpcodeKind::DIVU => Ok(Some(rs1)),
        OpcodeKind::REM => Ok(Some(rs1)),
        OpcodeKind::REMU => Ok(Some(rs1)),
        OpcodeKind::MULW => Ok(Some(rs1)),
        OpcodeKind::DIVW => Ok(Some(rs1)),
        OpcodeKind::DIVUW => Ok(Some(rs1)),
        OpcodeKind::REMW => Ok(Some(rs1)),
        OpcodeKind::REMUW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpcodeKind::MUL => Ok(Some(rs2)),
        OpcodeKind::MULH => Ok(Some(rs2)),
        OpcodeKind::MULHSU => Ok(Some(rs2)),
        OpcodeKind::MULHU => Ok(Some(rs2)),
        OpcodeKind::DIV => Ok(Some(rs2)),
        OpcodeKind::DIVU => Ok(Some(rs2)),
        OpcodeKind::REM => Ok(Some(rs2)),
        OpcodeKind::REMU => Ok(Some(rs2)),
        OpcodeKind::MULW => Ok(Some(rs2)),
        OpcodeKind::DIVW => Ok(Some(rs2)),
        OpcodeKind::DIVUW => Ok(Some(rs2)),
        OpcodeKind::REMW => Ok(Some(rs2)),
        OpcodeKind::REMUW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &OpcodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    Ok(None)
}

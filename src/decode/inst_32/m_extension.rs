use super::super::{only_rv64, DecodeUtil};
use crate::instruction::OpecodeKind;
use crate::Isa;

pub fn parse_opecode(inst: u32, isa: Isa) -> Result<OpecodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            format!("opecode decoding failed in m extension, {inst:b}"),
        ))
    };

    match opmap {
        0b0110011 => match funct3 {
            0b000 => Ok(OpecodeKind::MUL),
            0b001 => Ok(OpecodeKind::MULH),
            0b010 => Ok(OpecodeKind::MULHSU),
            0b011 => Ok(OpecodeKind::MULHU),
            0b100 => Ok(OpecodeKind::DIV),
            0b101 => Ok(OpecodeKind::DIVU),
            0b110 => Ok(OpecodeKind::REM),
            0b111 => Ok(OpecodeKind::REMU),
            _ => illegal_inst_exception(),
        },
        0b0111011 => match funct3 {
            0b000 => only_rv64(OpecodeKind::MULW, isa),
            0b100 => only_rv64(OpecodeKind::DIVW, isa),
            0b101 => only_rv64(OpecodeKind::DIVUW, isa),
            0b110 => only_rv64(OpecodeKind::REMW, isa),
            0b111 => only_rv64(OpecodeKind::REMUW, isa),
            _ => illegal_inst_exception(),
        },
        _ => illegal_inst_exception(),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::MUL => Ok(Some(rd)),
        OpecodeKind::MULH => Ok(Some(rd)),
        OpecodeKind::MULHSU => Ok(Some(rd)),
        OpecodeKind::MULHU => Ok(Some(rd)),
        OpecodeKind::DIV => Ok(Some(rd)),
        OpecodeKind::DIVU => Ok(Some(rd)),
        OpecodeKind::REM => Ok(Some(rd)),
        OpecodeKind::REMU => Ok(Some(rd)),
        OpecodeKind::MULW => Ok(Some(rd)),
        OpecodeKind::DIVW => Ok(Some(rd)),
        OpecodeKind::DIVUW => Ok(Some(rd)),
        OpecodeKind::REMW => Ok(Some(rd)),
        OpecodeKind::REMUW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::MUL => Ok(Some(rs1)),
        OpecodeKind::MULH => Ok(Some(rs1)),
        OpecodeKind::MULHSU => Ok(Some(rs1)),
        OpecodeKind::MULHU => Ok(Some(rs1)),
        OpecodeKind::DIV => Ok(Some(rs1)),
        OpecodeKind::DIVU => Ok(Some(rs1)),
        OpecodeKind::REM => Ok(Some(rs1)),
        OpecodeKind::REMU => Ok(Some(rs1)),
        OpecodeKind::MULW => Ok(Some(rs1)),
        OpecodeKind::DIVW => Ok(Some(rs1)),
        OpecodeKind::DIVUW => Ok(Some(rs1)),
        OpecodeKind::REMW => Ok(Some(rs1)),
        OpecodeKind::REMUW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::MUL => Ok(Some(rs2)),
        OpecodeKind::MULH => Ok(Some(rs2)),
        OpecodeKind::MULHSU => Ok(Some(rs2)),
        OpecodeKind::MULHU => Ok(Some(rs2)),
        OpecodeKind::DIV => Ok(Some(rs2)),
        OpecodeKind::DIVU => Ok(Some(rs2)),
        OpecodeKind::REM => Ok(Some(rs2)),
        OpecodeKind::REMU => Ok(Some(rs2)),
        OpecodeKind::MULW => Ok(Some(rs2)),
        OpecodeKind::DIVW => Ok(Some(rs2)),
        OpecodeKind::DIVUW => Ok(Some(rs2)),
        OpecodeKind::REMW => Ok(Some(rs2)),
        OpecodeKind::REMUW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    Ok(None)
}

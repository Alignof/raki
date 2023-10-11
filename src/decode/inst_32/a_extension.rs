use super::super::{only_rv64, DecodeUtil};
use crate::instruction::OpecodeKind;
use crate::Isa;

pub fn parse_opecode(inst: u32, isa: Isa) -> Result<OpecodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 27) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            format!("opecode decoding failed in a extension, {inst:b}"),
        ))
    };

    match opmap {
        0b0101111 => match funct3 {
            0b010 => match funct7 {
                0b00010 => Ok(OpecodeKind::LR_W),
                0b00011 => Ok(OpecodeKind::SC_W),
                0b00001 => Ok(OpecodeKind::AMOSWAP_W),
                0b00000 => Ok(OpecodeKind::AMOADD_W),
                0b00100 => Ok(OpecodeKind::AMOXOR_W),
                0b01100 => Ok(OpecodeKind::AMOAND_W),
                0b01000 => Ok(OpecodeKind::AMOOR_W),
                0b10000 => Ok(OpecodeKind::AMOMIN_W),
                0b10100 => Ok(OpecodeKind::AMOMAX_W),
                0b11000 => Ok(OpecodeKind::AMOMINU_W),
                0b11100 => Ok(OpecodeKind::AMOMAXU_W),
                _ => illegal_inst_exception(),
            },
            0b011 => match funct7 {
                0b00010 => only_rv64(OpecodeKind::LR_D, isa),
                0b00011 => only_rv64(OpecodeKind::SC_D, isa),
                0b00001 => only_rv64(OpecodeKind::AMOSWAP_D, isa),
                0b00000 => only_rv64(OpecodeKind::AMOADD_D, isa),
                0b00100 => only_rv64(OpecodeKind::AMOXOR_D, isa),
                0b01100 => only_rv64(OpecodeKind::AMOAND_D, isa),
                0b01000 => only_rv64(OpecodeKind::AMOOR_D, isa),
                0b10000 => only_rv64(OpecodeKind::AMOMIN_D, isa),
                0b10100 => only_rv64(OpecodeKind::AMOMAX_D, isa),
                0b11000 => only_rv64(OpecodeKind::AMOMINU_D, isa),
                0b11100 => only_rv64(OpecodeKind::AMOMAXU_D, isa),
                _ => illegal_inst_exception(),
            },
            _ => illegal_inst_exception(),
        },
        _ => illegal_inst_exception(),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::LR_W => Ok(Some(rd)),
        OpecodeKind::SC_W => Ok(Some(rd)),
        OpecodeKind::AMOSWAP_W => Ok(Some(rd)),
        OpecodeKind::AMOADD_W => Ok(Some(rd)),
        OpecodeKind::AMOXOR_W => Ok(Some(rd)),
        OpecodeKind::AMOAND_W => Ok(Some(rd)),
        OpecodeKind::AMOOR_W => Ok(Some(rd)),
        OpecodeKind::AMOMIN_W => Ok(Some(rd)),
        OpecodeKind::AMOMAX_W => Ok(Some(rd)),
        OpecodeKind::AMOMINU_W => Ok(Some(rd)),
        OpecodeKind::AMOMAXU_W => Ok(Some(rd)),
        OpecodeKind::LR_D => Ok(Some(rd)),
        OpecodeKind::SC_D => Ok(Some(rd)),
        OpecodeKind::AMOSWAP_D => Ok(Some(rd)),
        OpecodeKind::AMOADD_D => Ok(Some(rd)),
        OpecodeKind::AMOXOR_D => Ok(Some(rd)),
        OpecodeKind::AMOAND_D => Ok(Some(rd)),
        OpecodeKind::AMOOR_D => Ok(Some(rd)),
        OpecodeKind::AMOMIN_D => Ok(Some(rd)),
        OpecodeKind::AMOMAX_D => Ok(Some(rd)),
        OpecodeKind::AMOMINU_D => Ok(Some(rd)),
        OpecodeKind::AMOMAXU_D => Ok(Some(rd)),
        _ => panic!("rd decoding failed in A extension"),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::LR_W => Ok(Some(rs1)),
        OpecodeKind::SC_W => Ok(Some(rs1)),
        OpecodeKind::AMOSWAP_W => Ok(Some(rs1)),
        OpecodeKind::AMOADD_W => Ok(Some(rs1)),
        OpecodeKind::AMOXOR_W => Ok(Some(rs1)),
        OpecodeKind::AMOAND_W => Ok(Some(rs1)),
        OpecodeKind::AMOOR_W => Ok(Some(rs1)),
        OpecodeKind::AMOMIN_W => Ok(Some(rs1)),
        OpecodeKind::AMOMAX_W => Ok(Some(rs1)),
        OpecodeKind::AMOMINU_W => Ok(Some(rs1)),
        OpecodeKind::AMOMAXU_W => Ok(Some(rs1)),
        OpecodeKind::LR_D => Ok(Some(rs1)),
        OpecodeKind::SC_D => Ok(Some(rs1)),
        OpecodeKind::AMOSWAP_D => Ok(Some(rs1)),
        OpecodeKind::AMOADD_D => Ok(Some(rs1)),
        OpecodeKind::AMOXOR_D => Ok(Some(rs1)),
        OpecodeKind::AMOAND_D => Ok(Some(rs1)),
        OpecodeKind::AMOOR_D => Ok(Some(rs1)),
        OpecodeKind::AMOMIN_D => Ok(Some(rs1)),
        OpecodeKind::AMOMAX_D => Ok(Some(rs1)),
        OpecodeKind::AMOMINU_D => Ok(Some(rs1)),
        OpecodeKind::AMOMAXU_D => Ok(Some(rs1)),
        _ => panic!("rs1 decoding failed in A extension"),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::SC_W => Ok(Some(rs2)),
        OpecodeKind::AMOSWAP_W => Ok(Some(rs2)),
        OpecodeKind::AMOADD_W => Ok(Some(rs2)),
        OpecodeKind::AMOXOR_W => Ok(Some(rs2)),
        OpecodeKind::AMOAND_W => Ok(Some(rs2)),
        OpecodeKind::AMOOR_W => Ok(Some(rs2)),
        OpecodeKind::AMOMIN_W => Ok(Some(rs2)),
        OpecodeKind::AMOMAX_W => Ok(Some(rs2)),
        OpecodeKind::AMOMINU_W => Ok(Some(rs2)),
        OpecodeKind::AMOMAXU_W => Ok(Some(rs2)),
        OpecodeKind::SC_D => Ok(Some(rs2)),
        OpecodeKind::AMOSWAP_D => Ok(Some(rs2)),
        OpecodeKind::AMOADD_D => Ok(Some(rs2)),
        OpecodeKind::AMOXOR_D => Ok(Some(rs2)),
        OpecodeKind::AMOAND_D => Ok(Some(rs2)),
        OpecodeKind::AMOOR_D => Ok(Some(rs2)),
        OpecodeKind::AMOMIN_D => Ok(Some(rs2)),
        OpecodeKind::AMOMAX_D => Ok(Some(rs2)),
        OpecodeKind::AMOMINU_D => Ok(Some(rs2)),
        OpecodeKind::AMOMAXU_D => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(inst: u32, opkind: &OpecodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    let aq_and_rl = || inst.slice(26, 25) as i32;

    match opkind {
        OpecodeKind::LR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::SC_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOSWAP_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOADD_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOXOR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOAND_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOOR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMIN_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMAX_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMINU_W => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMAXU_W => Ok(Some(aq_and_rl())),
        OpecodeKind::LR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::SC_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOSWAP_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOADD_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOXOR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOAND_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOOR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMIN_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMAX_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMINU_D => Ok(Some(aq_and_rl())),
        OpecodeKind::AMOMAXU_D => Ok(Some(aq_and_rl())),
        _ => panic!("imm decoding failed in A extension"),
    }
}

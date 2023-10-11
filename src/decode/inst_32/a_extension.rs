use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::OpcodeKind;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 27) as u8;

    match opmap {
        0b010_1111 => match funct3 {
            0b010 => match funct7 {
                0b00010 => Ok(OpcodeKind::LR_W),
                0b00011 => Ok(OpcodeKind::SC_W),
                0b00001 => Ok(OpcodeKind::AMOSWAP_W),
                0b00000 => Ok(OpcodeKind::AMOADD_W),
                0b00100 => Ok(OpcodeKind::AMOXOR_W),
                0b01100 => Ok(OpcodeKind::AMOAND_W),
                0b01000 => Ok(OpcodeKind::AMOOR_W),
                0b10000 => Ok(OpcodeKind::AMOMIN_W),
                0b10100 => Ok(OpcodeKind::AMOMAX_W),
                0b11000 => Ok(OpcodeKind::AMOMINU_W),
                0b11100 => Ok(OpcodeKind::AMOMAXU_W),
                _ => Err(DecodingError::IllegalFunct7),
            },
            0b011 => match funct7 {
                0b00010 => only_rv64(OpcodeKind::LR_D, isa),
                0b00011 => only_rv64(OpcodeKind::SC_D, isa),
                0b00001 => only_rv64(OpcodeKind::AMOSWAP_D, isa),
                0b00000 => only_rv64(OpcodeKind::AMOADD_D, isa),
                0b00100 => only_rv64(OpcodeKind::AMOXOR_D, isa),
                0b01100 => only_rv64(OpcodeKind::AMOAND_D, isa),
                0b01000 => only_rv64(OpcodeKind::AMOOR_D, isa),
                0b10000 => only_rv64(OpcodeKind::AMOMIN_D, isa),
                0b10100 => only_rv64(OpcodeKind::AMOMAX_D, isa),
                0b11000 => only_rv64(OpcodeKind::AMOMINU_D, isa),
                0b11100 => only_rv64(OpcodeKind::AMOMAXU_D, isa),
                _ => Err(DecodingError::IllegalFunct7),
            },
            _ => Err(DecodingError::IllegalFunct3),
        },
        _ => Err(DecodingError::IllegalOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpcodeKind::LR_W => Ok(Some(rd)),
        OpcodeKind::SC_W => Ok(Some(rd)),
        OpcodeKind::AMOSWAP_W => Ok(Some(rd)),
        OpcodeKind::AMOADD_W => Ok(Some(rd)),
        OpcodeKind::AMOXOR_W => Ok(Some(rd)),
        OpcodeKind::AMOAND_W => Ok(Some(rd)),
        OpcodeKind::AMOOR_W => Ok(Some(rd)),
        OpcodeKind::AMOMIN_W => Ok(Some(rd)),
        OpcodeKind::AMOMAX_W => Ok(Some(rd)),
        OpcodeKind::AMOMINU_W => Ok(Some(rd)),
        OpcodeKind::AMOMAXU_W => Ok(Some(rd)),
        OpcodeKind::LR_D => Ok(Some(rd)),
        OpcodeKind::SC_D => Ok(Some(rd)),
        OpcodeKind::AMOSWAP_D => Ok(Some(rd)),
        OpcodeKind::AMOADD_D => Ok(Some(rd)),
        OpcodeKind::AMOXOR_D => Ok(Some(rd)),
        OpcodeKind::AMOAND_D => Ok(Some(rd)),
        OpcodeKind::AMOOR_D => Ok(Some(rd)),
        OpcodeKind::AMOMIN_D => Ok(Some(rd)),
        OpcodeKind::AMOMAX_D => Ok(Some(rd)),
        OpcodeKind::AMOMINU_D => Ok(Some(rd)),
        OpcodeKind::AMOMAXU_D => Ok(Some(rd)),
        _ => panic!("rd decoding failed in A extension"),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpcodeKind::LR_W => Ok(Some(rs1)),
        OpcodeKind::SC_W => Ok(Some(rs1)),
        OpcodeKind::AMOSWAP_W => Ok(Some(rs1)),
        OpcodeKind::AMOADD_W => Ok(Some(rs1)),
        OpcodeKind::AMOXOR_W => Ok(Some(rs1)),
        OpcodeKind::AMOAND_W => Ok(Some(rs1)),
        OpcodeKind::AMOOR_W => Ok(Some(rs1)),
        OpcodeKind::AMOMIN_W => Ok(Some(rs1)),
        OpcodeKind::AMOMAX_W => Ok(Some(rs1)),
        OpcodeKind::AMOMINU_W => Ok(Some(rs1)),
        OpcodeKind::AMOMAXU_W => Ok(Some(rs1)),
        OpcodeKind::LR_D => Ok(Some(rs1)),
        OpcodeKind::SC_D => Ok(Some(rs1)),
        OpcodeKind::AMOSWAP_D => Ok(Some(rs1)),
        OpcodeKind::AMOADD_D => Ok(Some(rs1)),
        OpcodeKind::AMOXOR_D => Ok(Some(rs1)),
        OpcodeKind::AMOAND_D => Ok(Some(rs1)),
        OpcodeKind::AMOOR_D => Ok(Some(rs1)),
        OpcodeKind::AMOMIN_D => Ok(Some(rs1)),
        OpcodeKind::AMOMAX_D => Ok(Some(rs1)),
        OpcodeKind::AMOMINU_D => Ok(Some(rs1)),
        OpcodeKind::AMOMAXU_D => Ok(Some(rs1)),
        _ => panic!("rs1 decoding failed in A extension"),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpcodeKind::SC_W => Ok(Some(rs2)),
        OpcodeKind::AMOSWAP_W => Ok(Some(rs2)),
        OpcodeKind::AMOADD_W => Ok(Some(rs2)),
        OpcodeKind::AMOXOR_W => Ok(Some(rs2)),
        OpcodeKind::AMOAND_W => Ok(Some(rs2)),
        OpcodeKind::AMOOR_W => Ok(Some(rs2)),
        OpcodeKind::AMOMIN_W => Ok(Some(rs2)),
        OpcodeKind::AMOMAX_W => Ok(Some(rs2)),
        OpcodeKind::AMOMINU_W => Ok(Some(rs2)),
        OpcodeKind::AMOMAXU_W => Ok(Some(rs2)),
        OpcodeKind::SC_D => Ok(Some(rs2)),
        OpcodeKind::AMOSWAP_D => Ok(Some(rs2)),
        OpcodeKind::AMOADD_D => Ok(Some(rs2)),
        OpcodeKind::AMOXOR_D => Ok(Some(rs2)),
        OpcodeKind::AMOAND_D => Ok(Some(rs2)),
        OpcodeKind::AMOOR_D => Ok(Some(rs2)),
        OpcodeKind::AMOMIN_D => Ok(Some(rs2)),
        OpcodeKind::AMOMAX_D => Ok(Some(rs2)),
        OpcodeKind::AMOMINU_D => Ok(Some(rs2)),
        OpcodeKind::AMOMAXU_D => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(inst: u32, opkind: &OpcodeKind) -> Result<Option<i32>, DecodingError> {
    let aq_and_rl = || inst.slice(26, 25) as i32;

    match opkind {
        OpcodeKind::LR_W => Ok(Some(aq_and_rl())),
        OpcodeKind::SC_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOSWAP_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOADD_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOXOR_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOAND_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOOR_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMIN_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMAX_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMINU_W => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMAXU_W => Ok(Some(aq_and_rl())),
        OpcodeKind::LR_D => Ok(Some(aq_and_rl())),
        OpcodeKind::SC_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOSWAP_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOADD_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOXOR_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOAND_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOOR_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMIN_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMAX_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMINU_D => Ok(Some(aq_and_rl())),
        OpcodeKind::AMOMAXU_D => Ok(Some(aq_and_rl())),
        _ => panic!("imm decoding failed in A extension"),
    }
}

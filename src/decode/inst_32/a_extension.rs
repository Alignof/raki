use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::a_extension::AOpcode;
use crate::Isa;

pub fn parse_opcode(inst: u32, isa: Isa) -> Result<AOpcode, DecodingError> {
    let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
    let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
    let funct7: u8 = u8::try_from(inst.slice(31, 27)).unwrap();

    match opmap {
        0b010_1111 => match funct3 {
            0b010 => match funct7 {
                0b00010 => Ok(AOpcode::LR_W),
                0b00011 => Ok(AOpcode::SC_W),
                0b00001 => Ok(AOpcode::AMOSWAP_W),
                0b00000 => Ok(AOpcode::AMOADD_W),
                0b00100 => Ok(AOpcode::AMOXOR_W),
                0b01100 => Ok(AOpcode::AMOAND_W),
                0b01000 => Ok(AOpcode::AMOOR_W),
                0b10000 => Ok(AOpcode::AMOMIN_W),
                0b10100 => Ok(AOpcode::AMOMAX_W),
                0b11000 => Ok(AOpcode::AMOMINU_W),
                0b11100 => Ok(AOpcode::AMOMAXU_W),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b011 => match funct7 {
                0b00010 => only_rv64(AOpcode::LR_D, isa),
                0b00011 => only_rv64(AOpcode::SC_D, isa),
                0b00001 => only_rv64(AOpcode::AMOSWAP_D, isa),
                0b00000 => only_rv64(AOpcode::AMOADD_D, isa),
                0b00100 => only_rv64(AOpcode::AMOXOR_D, isa),
                0b01100 => only_rv64(AOpcode::AMOAND_D, isa),
                0b01000 => only_rv64(AOpcode::AMOOR_D, isa),
                0b10000 => only_rv64(AOpcode::AMOMIN_D, isa),
                0b10100 => only_rv64(AOpcode::AMOMAX_D, isa),
                0b11000 => only_rv64(AOpcode::AMOMINU_D, isa),
                0b11100 => only_rv64(AOpcode::AMOMAXU_D, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rd(inst: u32, opkind: &AOpcode) -> Option<usize> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        AOpcode::LR_W
        | AOpcode::SC_W
        | AOpcode::AMOSWAP_W
        | AOpcode::AMOADD_W
        | AOpcode::AMOXOR_W
        | AOpcode::AMOAND_W
        | AOpcode::AMOOR_W
        | AOpcode::AMOMIN_W
        | AOpcode::AMOMAX_W
        | AOpcode::AMOMINU_W
        | AOpcode::AMOMAXU_W
        | AOpcode::LR_D
        | AOpcode::SC_D
        | AOpcode::AMOSWAP_D
        | AOpcode::AMOADD_D
        | AOpcode::AMOXOR_D
        | AOpcode::AMOAND_D
        | AOpcode::AMOOR_D
        | AOpcode::AMOMIN_D
        | AOpcode::AMOMAX_D
        | AOpcode::AMOMINU_D
        | AOpcode::AMOMAXU_D => Some(rd),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs1(inst: u32, opkind: &AOpcode) -> Option<usize> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        AOpcode::LR_W
        | AOpcode::SC_W
        | AOpcode::AMOSWAP_W
        | AOpcode::AMOADD_W
        | AOpcode::AMOXOR_W
        | AOpcode::AMOAND_W
        | AOpcode::AMOOR_W
        | AOpcode::AMOMIN_W
        | AOpcode::AMOMAX_W
        | AOpcode::AMOMINU_W
        | AOpcode::AMOMAXU_W
        | AOpcode::LR_D
        | AOpcode::SC_D
        | AOpcode::AMOSWAP_D
        | AOpcode::AMOADD_D
        | AOpcode::AMOXOR_D
        | AOpcode::AMOAND_D
        | AOpcode::AMOOR_D
        | AOpcode::AMOMIN_D
        | AOpcode::AMOMAX_D
        | AOpcode::AMOMINU_D
        | AOpcode::AMOMAXU_D => Some(rs1),
    }
}

pub fn parse_rs2(inst: u32, opkind: &AOpcode) -> Option<usize> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        AOpcode::SC_W
        | AOpcode::AMOSWAP_W
        | AOpcode::AMOADD_W
        | AOpcode::AMOXOR_W
        | AOpcode::AMOAND_W
        | AOpcode::AMOOR_W
        | AOpcode::AMOMIN_W
        | AOpcode::AMOMAX_W
        | AOpcode::AMOMINU_W
        | AOpcode::AMOMAXU_W
        | AOpcode::SC_D
        | AOpcode::AMOSWAP_D
        | AOpcode::AMOADD_D
        | AOpcode::AMOXOR_D
        | AOpcode::AMOAND_D
        | AOpcode::AMOOR_D
        | AOpcode::AMOMIN_D
        | AOpcode::AMOMAX_D
        | AOpcode::AMOMINU_D
        | AOpcode::AMOMAXU_D => Some(rs2),
        _ => None,
    }
}

#[allow(non_snake_case)]
#[allow(clippy::unnecessary_wraps, clippy::cast_possible_wrap)]
pub fn parse_imm(inst: u32, opkind: &AOpcode) -> Option<i32> {
    let aq_and_rl = || inst.slice(26, 25) as i32;

    match opkind {
        AOpcode::LR_W
        | AOpcode::SC_W
        | AOpcode::AMOSWAP_W
        | AOpcode::AMOADD_W
        | AOpcode::AMOXOR_W
        | AOpcode::AMOAND_W
        | AOpcode::AMOOR_W
        | AOpcode::AMOMIN_W
        | AOpcode::AMOMAX_W
        | AOpcode::AMOMINU_W
        | AOpcode::AMOMAXU_W
        | AOpcode::LR_D
        | AOpcode::SC_D
        | AOpcode::AMOSWAP_D
        | AOpcode::AMOADD_D
        | AOpcode::AMOXOR_D
        | AOpcode::AMOAND_D
        | AOpcode::AMOOR_D
        | AOpcode::AMOMIN_D
        | AOpcode::AMOMAX_D
        | AOpcode::AMOMINU_D
        | AOpcode::AMOMAXU_D => Some(aq_and_rl()),
    }
}

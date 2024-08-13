use super::super::{DecodeUtil, DecodingError};
use crate::instruction::zicntr_extension::ZicntrOpcode;

pub fn parse_opcode(inst: u32) -> Result<ZicntrOpcode, DecodingError> {
    let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
    let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
    let csr_num: u16 = u16::try_from(inst.slice(31, 20)).unwrap();

    match opmap {
        0b111_0011 => match funct3 {
            0b010 => match csr_num {
                0xc00 => Ok(ZicntrOpcode::RDCYCLE),
                0xc01 => Ok(ZicntrOpcode::RDTIME),
                0xc02 => Ok(ZicntrOpcode::RDINSTRET),
                0xc80 => Ok(ZicntrOpcode::RDCYCLE_H),
                0xc81 => Ok(ZicntrOpcode::RDTIME_H),
                0xc82 => Ok(ZicntrOpcode::RDINSTRET_H),
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rd(inst: u32, opkind: &ZicntrOpcode) -> Option<usize> {
    let rd: usize = inst.slice(11, 7) as usize;
    match opkind {
        ZicntrOpcode::RDCYCLE
        | ZicntrOpcode::RDTIME
        | ZicntrOpcode::RDINSTRET
        | ZicntrOpcode::RDCYCLE_H
        | ZicntrOpcode::RDTIME_H
        | ZicntrOpcode::RDINSTRET_H => Some(rd),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs1(_inst: u32, opkind: &ZicntrOpcode) -> Option<usize> {
    match opkind {
        ZicntrOpcode::RDCYCLE
        | ZicntrOpcode::RDTIME
        | ZicntrOpcode::RDINSTRET
        | ZicntrOpcode::RDCYCLE_H
        | ZicntrOpcode::RDTIME_H
        | ZicntrOpcode::RDINSTRET_H => None,
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs2(_inst: u32, opkind: &ZicntrOpcode) -> Option<usize> {
    match opkind {
        ZicntrOpcode::RDCYCLE
        | ZicntrOpcode::RDTIME
        | ZicntrOpcode::RDINSTRET
        | ZicntrOpcode::RDCYCLE_H
        | ZicntrOpcode::RDTIME_H
        | ZicntrOpcode::RDINSTRET_H => None,
    }
}

#[allow(clippy::cast_possible_wrap, clippy::unnecessary_wraps)]
pub fn parse_imm(_inst: u32, opkind: &ZicntrOpcode) -> Option<i32> {
    match opkind {
        ZicntrOpcode::RDCYCLE
        | ZicntrOpcode::RDTIME
        | ZicntrOpcode::RDINSTRET
        | ZicntrOpcode::RDCYCLE_H
        | ZicntrOpcode::RDTIME_H
        | ZicntrOpcode::RDINSTRET_H => None,
    }
}

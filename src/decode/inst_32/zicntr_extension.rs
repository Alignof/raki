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

#[cfg(test)]
#[allow(unused_variables)]
mod test_zicntr {
    #[test]
    #[allow(overflowing_literals)]
    fn zicntr_decode_test() {
        use super::*;
        use crate::{Decode, Isa, OpcodeKind};

        let test_32 = |inst_32: u32,
                       op: OpcodeKind,
                       rd: Option<usize>,
                       rs1: Option<usize>,
                       rs2: Option<usize>,
                       imm: Option<i32>| {
            let op_32 = inst_32.parse_opcode(Isa::Rv64).unwrap();
            assert!(matches!(&op_32, op));
            assert_eq!(inst_32.parse_rd(&op_32).unwrap(), rd);
            assert_eq!(inst_32.parse_rs1(&op_32).unwrap(), rs1);
            assert_eq!(inst_32.parse_rs2(&op_32).unwrap(), rs2);
            assert_eq!(inst_32.parse_imm(&op_32, Isa::Rv64).unwrap(), imm);
        };

        test_32(
            0b1100_0000_0001_0000_0010_0111_1111_0011,
            OpcodeKind::Zicntr(ZicntrOpcode::RDTIME),
            Some(15),
            None,
            None,
            None,
        )
    }
}

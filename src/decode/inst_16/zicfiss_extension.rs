use super::super::DecodingError;
use crate::instruction::zicfiss_extension::ZicfissOpcode;
use crate::Isa;

pub fn parse_opcode(inst: u16, _isa: Isa) -> Result<ZicfissOpcode, DecodingError> {
    match inst {
        0b0110_0000_1000_0001 => Ok(ZicfissOpcode::C_SSPUSH),
        0b0110_0010_1000_0001 => Ok(ZicfissOpcode::C_SSPOPCHK),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rd(_inst: u16, opkind: &ZicfissOpcode) -> Option<usize> {
    match opkind {
        ZicfissOpcode::C_SSPUSH => Some(1),
        ZicfissOpcode::C_SSPOPCHK => Some(5),
        _ => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs1(_inst: u16, _opkind: &ZicfissOpcode) -> Option<usize> {
    None
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs2(_inst: u16, _opkind: &ZicfissOpcode) -> Option<usize> {
    None
}

#[allow(clippy::cast_possible_wrap, clippy::unnecessary_wraps)]
pub fn parse_imm(_inst: u16, _opkind: &ZicfissOpcode) -> Option<i32> {
    None
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_zicfiss {
    #[test]
    #[allow(overflowing_literals)]
    fn zicfiss_16() {
        use super::*;
        use crate::{Decode, Isa, OpcodeKind};
        let test_16 = |inst_16: u16,
                       op: OpcodeKind,
                       rd: Option<usize>,
                       rs1: Option<usize>,
                       rs2: Option<usize>,
                       imm: Option<i32>| {
            let op_16 = inst_16.parse_opcode(Isa::Rv64).unwrap();
            assert!(matches!(&op_16, op));
            assert_eq!(inst_16.parse_rd(&op_16).unwrap(), rd);
            assert_eq!(inst_16.parse_rs1(&op_16).unwrap(), rs1);
            assert_eq!(inst_16.parse_rs2(&op_16).unwrap(), rs2);
            assert_eq!(inst_16.parse_imm(&op_16, Isa::Rv64).unwrap(), imm);
        };

        test_16(
            0x6081,
            OpcodeKind::Zicfiss(ZicfissOpcode::C_SSPUSH),
            Some(1),
            None,
            None,
            None,
        );

        test_16(
            0x6281,
            OpcodeKind::Zicfiss(ZicfissOpcode::C_SSPOPCHK),
            Some(5),
            None,
            None,
            None,
        );
    }
}

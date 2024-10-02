use super::super::{DecodeUtil, DecodingError};
use crate::instruction::zicfiss_extension::ZicfissOpcode;

pub fn parse_opcode(inst: u32) -> Result<ZicfissOpcode, DecodingError> {
    let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
    let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
    let rs1: u8 = u8::try_from(inst.slice(19, 15)).unwrap();
    let funct7_rs2: u16 = u16::try_from(inst.slice(31, 20)).unwrap();

    match opmap {
        0b111_0011 => match funct3 {
            0b100 => match funct7_rs2 {
                0b1100_1110_0001 | 0b1100_1110_0101 => Ok(ZicfissOpcode::SSPUSH),
                0b1100_1101_1100 => match rs1 {
                    0b0_0000 => Ok(ZicfissOpcode::SSRDP),
                    _ => Ok(ZicfissOpcode::SSPOPCHK),
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        0b010_1111 => match funct3 {
            0b010 => Ok(ZicfissOpcode::SSAMOSWAP_W),
            0b011 => Ok(ZicfissOpcode::SSAMOSWAP_D),
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rd(inst: u32, opkind: &ZicfissOpcode) -> Option<usize> {
    let rd: usize = inst.slice(11, 7) as usize;
    match opkind {
        ZicfissOpcode::SSPUSH | ZicfissOpcode::SSPOPCHK => Some(0),
        ZicfissOpcode::SSRDP | ZicfissOpcode::SSAMOSWAP_W | ZicfissOpcode::SSAMOSWAP_D => Some(rd),
        ZicfissOpcode::C_SSPUSH | ZicfissOpcode::C_SSPOPCHK => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs1(inst: u32, opkind: &ZicfissOpcode) -> Option<usize> {
    let rs1: usize = inst.slice(19, 15) as usize;
    match opkind {
        ZicfissOpcode::SSPUSH => Some(0),
        ZicfissOpcode::SSPOPCHK | ZicfissOpcode::SSAMOSWAP_W | ZicfissOpcode::SSAMOSWAP_D => {
            Some(rs1)
        }
        ZicfissOpcode::SSRDP => None,
        ZicfissOpcode::C_SSPUSH | ZicfissOpcode::C_SSPOPCHK => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_rs2(inst: u32, opkind: &ZicfissOpcode) -> Option<usize> {
    let rs2: usize = inst.slice(24, 20) as usize;
    match opkind {
        ZicfissOpcode::SSPUSH | ZicfissOpcode::SSAMOSWAP_W | ZicfissOpcode::SSAMOSWAP_D => {
            Some(rs2)
        }
        ZicfissOpcode::SSPOPCHK | ZicfissOpcode::SSRDP => None,
        ZicfissOpcode::C_SSPUSH | ZicfissOpcode::C_SSPOPCHK => unreachable!(),
    }
}

#[allow(clippy::cast_possible_wrap, clippy::unnecessary_wraps)]
pub fn parse_imm(_inst: u32, opkind: &ZicfissOpcode) -> Option<i32> {
    match opkind {
        ZicfissOpcode::SSPUSH
        | ZicfissOpcode::SSPOPCHK
        | ZicfissOpcode::SSRDP
        | ZicfissOpcode::SSAMOSWAP_W
        | ZicfissOpcode::SSAMOSWAP_D => None,
        ZicfissOpcode::C_SSPUSH | ZicfissOpcode::C_SSPOPCHK => unreachable!(),
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_zicfiss {
    #[test]
    #[allow(overflowing_literals)]
    fn zicfiss_32bit_decode_test() {
        use super::*;
        use crate::{Decode, Isa, OpcodeKind};

        let test_32 = |inst_32: u32,
                       expected_op: OpcodeKind,
                       expected_rd: Option<usize>,
                       expected_rs1: Option<usize>,
                       expected_rs2: Option<usize>,
                       expected_imm: Option<i32>| {
            let op_32 = inst_32.parse_opcode(Isa::Rv64).unwrap();
            assert_eq!(op_32, expected_op);
            assert_eq!(inst_32.parse_rd(&op_32).unwrap(), expected_rd);
            assert_eq!(inst_32.parse_rs1(&op_32).unwrap(), expected_rs1);
            assert_eq!(inst_32.parse_rs2(&op_32).unwrap(), expected_rs2);
            assert_eq!(inst_32.parse_imm(&op_32, Isa::Rv64).unwrap(), expected_imm);
        };

        test_32(
            0b1100_1110_0101_0000_0100_0000_0111_0011,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSPUSH),
            Some(0),
            Some(0),
            Some(5),
            None,
        );

        test_32(
            0b1100_1110_0001_0000_0100_0000_0111_0011,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSPUSH),
            Some(0),
            Some(0),
            Some(1),
            None,
        );

        test_32(
            0b1100_1101_1100_0000_1100_0000_0111_0011,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSPOPCHK),
            Some(0),
            Some(1),
            None,
            None,
        );

        test_32(
            0b1100_1101_1100_0010_1100_0000_0111_0011,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSPOPCHK),
            Some(0),
            Some(5),
            None,
            None,
        );

        test_32(
            0b0100_1000_1100_0010_1010_0001_1010_1111,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSAMOSWAP_W),
            Some(3),
            Some(5),
            Some(12),
            None,
        );

        test_32(
            0b0100_1000_1100_0111_0011_0001_1010_1111,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSAMOSWAP_D),
            Some(3),
            Some(14),
            Some(12),
            None,
        );

        test_32(
            0xcdc0c073,
            OpcodeKind::Zicfiss(ZicfissOpcode::SSPOPCHK),
            Some(0),
            Some(1),
            None,
            None,
        );
    }
}

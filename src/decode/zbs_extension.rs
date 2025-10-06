//! Zbs extension decoder

pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::zbs_extension::ZbsOpcode;

    pub fn parse_opcode(inst: u32) -> Result<ZbsOpcode, DecodingError> {
        let op_6_0: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let op_14_12: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let op_31_25: u8 = u8::try_from(inst.slice(31, 25)).unwrap();
        let op_31_26: u8 = u8::try_from(inst.slice(31, 26)).unwrap();
        match op_6_0 {
            0b1_0011 => match op_14_12 {
                0b1 => match op_31_26 {
                    0b1010 => Ok(ZbsOpcode::BSETI),
                    0b1_0010 => Ok(ZbsOpcode::BCLRI),
                    0b1_1010 => Ok(ZbsOpcode::BINVI),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b101 => Ok(ZbsOpcode::BEXTI),
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b11_0011 => match op_14_12 {
                0b1 => match op_31_25 {
                    0b1_0100 => Ok(ZbsOpcode::BSET),
                    0b10_0100 => Ok(ZbsOpcode::BCLR),
                    0b11_0100 => Ok(ZbsOpcode::BINV),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b101 => Ok(ZbsOpcode::BEXT),
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// Parsing Zbs instruction's rd
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, opkind: &ZbsOpcode) -> Option<usize> {
        let rd_11_7: usize = inst.slice(11, 7) as usize;
        match opkind {
            ZbsOpcode::BCLRI
            | ZbsOpcode::BEXTI
            | ZbsOpcode::BINVI
            | ZbsOpcode::BSETI
            | ZbsOpcode::BCLR
            | ZbsOpcode::BEXT
            | ZbsOpcode::BINV
            | ZbsOpcode::BSET => Some(rd_11_7),
        }
    }

    /// Parsing Zbs instruction's rs1
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, opkind: &ZbsOpcode) -> Option<usize> {
        let rs1_19_15: usize = inst.slice(19, 15) as usize;
        match opkind {
            ZbsOpcode::BCLRI
            | ZbsOpcode::BEXTI
            | ZbsOpcode::BINVI
            | ZbsOpcode::BSETI
            | ZbsOpcode::BCLR
            | ZbsOpcode::BEXT
            | ZbsOpcode::BINV
            | ZbsOpcode::BSET => Some(rs1_19_15),
        }
    }

    /// Parsing Zbs instruction's rs2
    pub fn parse_rs2(inst: u32, opkind: &ZbsOpcode) -> Option<usize> {
        let rs2_24_20: usize = inst.slice(24, 20) as usize;
        match opkind {
            ZbsOpcode::BCLRI | ZbsOpcode::BEXTI | ZbsOpcode::BINVI | ZbsOpcode::BSETI => None,
            ZbsOpcode::BCLR | ZbsOpcode::BEXT | ZbsOpcode::BINV | ZbsOpcode::BSET => {
                Some(rs2_24_20)
            }
        }
    }

    /// Parsing Zbs instruction's imm
    #[allow(clippy::cast_possible_wrap)]
    pub fn parse_imm(inst: u32, opkind: &ZbsOpcode) -> Option<i32> {
        let imm_25_20: i32 = inst.slice(25, 20) as i32;
        match opkind {
            ZbsOpcode::BCLRI | ZbsOpcode::BEXTI | ZbsOpcode::BINVI | ZbsOpcode::BSETI => {
                Some(imm_25_20)
            }
            ZbsOpcode::BCLR | ZbsOpcode::BEXT | ZbsOpcode::BINV | ZbsOpcode::BSET => None,
        }
    }
}
#[cfg(test)]
#[allow(unused_variables)]
mod test_zbs {
    #[test]
    #[allow(overflowing_literals)]
    fn zbs_32bit_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::zbs_extension::ZbsOpcode;
        use crate::OpcodeKind;

        test_32_in_rv64(
            0b100_1010_1000_0011_1001_0100_1001_0011,
            OpcodeKind::Zbs(ZbsOpcode::BCLRI),
            Some(9),
            Some(7),
            None,
            Some(40),
        );
        test_32_in_rv64(
            0b100_1010_0111_1100_0101_1111_1001_0011,
            OpcodeKind::Zbs(ZbsOpcode::BEXTI),
            Some(31),
            Some(24),
            None,
            Some(39),
        );
        test_32_in_rv64(
            0b110_1000_0001_1111_0001_0111_0001_0011,
            OpcodeKind::Zbs(ZbsOpcode::BINVI),
            Some(14),
            Some(30),
            None,
            Some(1),
        );
        test_32_in_rv64(
            0b10_1011_0111_0101_1001_1100_0001_0011,
            OpcodeKind::Zbs(ZbsOpcode::BSETI),
            Some(24),
            Some(11),
            None,
            Some(55),
        );
        test_32_in_rv64(
            0b100_1000_0001_1011_1001_1100_1011_0011,
            OpcodeKind::Zbs(ZbsOpcode::BCLR),
            Some(25),
            Some(23),
            Some(1),
            None,
        );
        test_32_in_rv64(
            0b100_1001_1011_0001_0101_1101_0011_0011,
            OpcodeKind::Zbs(ZbsOpcode::BEXT),
            Some(26),
            Some(2),
            Some(27),
            None,
        );
        test_32_in_rv64(
            0b110_1001_1000_1010_1001_1111_1011_0011,
            OpcodeKind::Zbs(ZbsOpcode::BINV),
            Some(31),
            Some(21),
            Some(24),
            None,
        );
        test_32_in_rv64(
            0b10_1000_1101_0101_0001_0010_0011_0011,
            OpcodeKind::Zbs(ZbsOpcode::BSET),
            Some(4),
            Some(10),
            Some(13),
            None,
        );
    }
}

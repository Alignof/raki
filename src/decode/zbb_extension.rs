//! Zbb extension decoder

pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::zbb_extension::ZbbOpcode;

    pub fn parse_opcode(inst: u32) -> Result<ZbbOpcode, DecodingError> {
        let op_6_0: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let op_14_12: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let op_31_20: u16 = u16::try_from(inst.slice(31, 20)).unwrap();
        let op_24_20: u8 = u8::try_from(inst.slice(24, 20)).unwrap();
        let op_31_25: u8 = u8::try_from(inst.slice(31, 25)).unwrap();
        match op_6_0 {
            0b111011 => match op_14_12 {
                0b1 => Ok(ZbbOpcode::ROLW),
                0b100 => Ok(ZbbOpcode::ZEXTH),
                0b101 => Ok(ZbbOpcode::RORW),
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b11011 => match op_14_12 {
                0b101 => Ok(ZbbOpcode::RORIW),
                0b1 => match op_31_20 {
                    0b11000000000 => Ok(ZbbOpcode::CLZW),
                    0b11000000001 => Ok(ZbbOpcode::CTZW),
                    0b11000000010 => Ok(ZbbOpcode::CPOPW),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b10011 => match op_14_12 {
                0b101 => match op_31_20 {
                    0b1010000111 => Ok(ZbbOpcode::ORCB),
                    0b11010111000 => Ok(ZbbOpcode::REV8),
                    _ => Ok(ZbbOpcode::RORI),
                },
                0b1 => match op_31_20 {
                    0b11000000000 => Ok(ZbbOpcode::CLZ),
                    0b11000000001 => Ok(ZbbOpcode::CTZ),
                    0b11000000010 => Ok(ZbbOpcode::CPOP),
                    _ => match op_24_20 {
                        0b100 => Ok(ZbbOpcode::SEXTB),
                        0b101 => Ok(ZbbOpcode::SEXTH),
                        _ => Err(DecodingError::InvalidOpcode),
                    },
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b110011 => match op_14_12 {
                0b1 => Ok(ZbbOpcode::ROL),
                0b100 => match op_31_25 {
                    0b00101 => Ok(ZbbOpcode::MIN),
                    0b100000 => Ok(ZbbOpcode::XNOR),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b101 => match op_31_25 {
                    0b00101 => Ok(ZbbOpcode::MINU),
                    0b110000 => Ok(ZbbOpcode::ROR),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b110 => match op_31_25 {
                    0b00101 => Ok(ZbbOpcode::MAX),
                    0b100000 => Ok(ZbbOpcode::ORN),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b111 => match op_31_25 {
                    0b00101 => Ok(ZbbOpcode::MAXU),
                    0b100000 => Ok(ZbbOpcode::ANDN),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// Parsing Zbb instruction's rd
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rd_11_7: usize = inst.slice(11, 7) as usize;
        match opkind {
            ZbbOpcode::RORIW
            | ZbbOpcode::RORI
            | ZbbOpcode::ROLW
            | ZbbOpcode::RORW
            | ZbbOpcode::ANDN
            | ZbbOpcode::ORN
            | ZbbOpcode::XNOR
            | ZbbOpcode::MAX
            | ZbbOpcode::MAXU
            | ZbbOpcode::MIN
            | ZbbOpcode::MINU
            | ZbbOpcode::ROL
            | ZbbOpcode::ROR
            | ZbbOpcode::SEXTB
            | ZbbOpcode::SEXTH
            | ZbbOpcode::ZEXTH
            | ZbbOpcode::REV8
            | ZbbOpcode::ORCB
            | ZbbOpcode::CPOP
            | ZbbOpcode::CPOPW
            | ZbbOpcode::CLZ
            | ZbbOpcode::CLZW
            | ZbbOpcode::CTZ
            | ZbbOpcode::CTZW => Some(rd_11_7),
        }
    }

    /// Parsing Zbb instruction's rs1
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rs1_19_15: usize = inst.slice(19, 15) as usize;
        match opkind {
            ZbbOpcode::RORIW
            | ZbbOpcode::RORI
            | ZbbOpcode::ROLW
            | ZbbOpcode::RORW
            | ZbbOpcode::ANDN
            | ZbbOpcode::ORN
            | ZbbOpcode::XNOR
            | ZbbOpcode::MAX
            | ZbbOpcode::MAXU
            | ZbbOpcode::MIN
            | ZbbOpcode::MINU
            | ZbbOpcode::ROL
            | ZbbOpcode::ROR
            | ZbbOpcode::SEXTB
            | ZbbOpcode::SEXTH
            | ZbbOpcode::ZEXTH
            | ZbbOpcode::REV8
            | ZbbOpcode::ORCB
            | ZbbOpcode::CPOP
            | ZbbOpcode::CPOPW
            | ZbbOpcode::CLZ
            | ZbbOpcode::CLZW
            | ZbbOpcode::CTZ
            | ZbbOpcode::CTZW => Some(rs1_19_15),
        }
    }

    /// Parsing Zbb instruction's rs2
    pub fn parse_rs2(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rs2_24_20: usize = inst.slice(24, 20) as usize;
        match opkind {
            ZbbOpcode::ROLW
            | ZbbOpcode::RORW
            | ZbbOpcode::ANDN
            | ZbbOpcode::ORN
            | ZbbOpcode::XNOR
            | ZbbOpcode::MAX
            | ZbbOpcode::MAXU
            | ZbbOpcode::MIN
            | ZbbOpcode::MINU
            | ZbbOpcode::ROL
            | ZbbOpcode::ROR => Some(rs2_24_20),
            ZbbOpcode::RORIW
            | ZbbOpcode::RORI
            | ZbbOpcode::SEXTB
            | ZbbOpcode::SEXTH
            | ZbbOpcode::ZEXTH
            | ZbbOpcode::REV8
            | ZbbOpcode::ORCB
            | ZbbOpcode::CPOP
            | ZbbOpcode::CPOPW
            | ZbbOpcode::CLZ
            | ZbbOpcode::CLZW
            | ZbbOpcode::CTZ
            | ZbbOpcode::CTZW => None,
        }
    }

    /// Parsing Zbb instruction's imm
    #[allow(clippy::cast_possible_wrap)]
    pub fn parse_imm(inst: u32, opkind: &ZbbOpcode) -> Option<i32> {
        let imm_24_20: i32 = inst.slice(24, 20) as i32;
        let imm_25_20: i32 = inst.slice(25, 20) as i32;
        match opkind {
            ZbbOpcode::RORIW => Some(imm_24_20),
            ZbbOpcode::RORI => Some(imm_25_20),
            ZbbOpcode::ROLW
            | ZbbOpcode::RORW
            | ZbbOpcode::ANDN
            | ZbbOpcode::ORN
            | ZbbOpcode::XNOR
            | ZbbOpcode::MAX
            | ZbbOpcode::MAXU
            | ZbbOpcode::MIN
            | ZbbOpcode::MINU
            | ZbbOpcode::ROL
            | ZbbOpcode::ROR
            | ZbbOpcode::SEXTB
            | ZbbOpcode::SEXTH
            | ZbbOpcode::ZEXTH
            | ZbbOpcode::REV8
            | ZbbOpcode::ORCB
            | ZbbOpcode::CPOP
            | ZbbOpcode::CPOPW
            | ZbbOpcode::CLZ
            | ZbbOpcode::CLZW
            | ZbbOpcode::CTZ
            | ZbbOpcode::CTZW => None,
        }
    }
}
#[cfg(test)]
#[allow(unused_variables)]
mod test_zbb {
    #[test]
    #[allow(overflowing_literals)]
    fn zbb_32bit_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::zbb_extension::ZbbOpcode;
        use crate::OpcodeKind;

        test_32_in_rv64(
            0b1100001101100001101100010011011,
            OpcodeKind::Zbb(ZbbOpcode::RORIW),
            Some(17),
            Some(1),
            None,
            Some(27),
        );
        test_32_in_rv64(
            0b1100001000011111101111100010011,
            OpcodeKind::Zbb(ZbbOpcode::RORI),
            Some(30),
            Some(31),
            None,
            Some(16),
        );
        test_32_in_rv64(
            0b1100000000110000001110100111011,
            OpcodeKind::Zbb(ZbbOpcode::ROLW),
            Some(26),
            Some(16),
            Some(1),
            None,
        );
        test_32_in_rv64(
            0b1100001000000011101101010111011,
            OpcodeKind::Zbb(ZbbOpcode::RORW),
            Some(21),
            Some(3),
            Some(16),
            None,
        );
        test_32_in_rv64(
            0b1000000010101110111010100110011,
            OpcodeKind::Zbb(ZbbOpcode::ANDN),
            Some(10),
            Some(14),
            Some(5),
            None,
        );
        test_32_in_rv64(
            0b1000000001001100110111100110011,
            OpcodeKind::Zbb(ZbbOpcode::ORN),
            Some(30),
            Some(12),
            Some(2),
            None,
        );
        test_32_in_rv64(
            0b1000001011001111100111100110011,
            OpcodeKind::Zbb(ZbbOpcode::XNOR),
            Some(30),
            Some(15),
            Some(22),
            None,
        );
        test_32_in_rv64(
            0b001011111010100110101110110011,
            OpcodeKind::Zbb(ZbbOpcode::MAX),
            Some(23),
            Some(20),
            Some(30),
            None,
        );
        test_32_in_rv64(
            0b001010001111111111001000110011,
            OpcodeKind::Zbb(ZbbOpcode::MAXU),
            Some(4),
            Some(31),
            Some(3),
            None,
        );
        test_32_in_rv64(
            0b001010011111011100011110110011,
            OpcodeKind::Zbb(ZbbOpcode::MIN),
            Some(15),
            Some(27),
            Some(7),
            None,
        );
        test_32_in_rv64(
            0b001011101101100101010100110011,
            OpcodeKind::Zbb(ZbbOpcode::MINU),
            Some(10),
            Some(12),
            Some(27),
            None,
        );
        test_32_in_rv64(
            0b1100000001100010001011100110011,
            OpcodeKind::Zbb(ZbbOpcode::ROL),
            Some(14),
            Some(2),
            Some(3),
            None,
        );
        test_32_in_rv64(
            0b1100001100011100101010000110011,
            OpcodeKind::Zbb(ZbbOpcode::ROR),
            Some(8),
            Some(28),
            Some(24),
            None,
        );
        test_32_in_rv64(
            0b1100000010000110001001110010011,
            OpcodeKind::Zbb(ZbbOpcode::SEXTB),
            Some(7),
            Some(6),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000010101110001001100010011,
            OpcodeKind::Zbb(ZbbOpcode::SEXTH),
            Some(6),
            Some(14),
            None,
            None,
        );
        test_32_in_rv64(
            0b001000000010000100001010111011,
            OpcodeKind::Zbb(ZbbOpcode::ZEXTH),
            Some(5),
            Some(16),
            None,
            None,
        );
        test_32_in_rv64(
            0b1101011100011010101010010010011,
            OpcodeKind::Zbb(ZbbOpcode::REV8),
            Some(9),
            Some(26),
            None,
            None,
        );
        test_32_in_rv64(
            0b101000011111111101110010010011,
            OpcodeKind::Zbb(ZbbOpcode::ORCB),
            Some(25),
            Some(31),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000001010111001000100010011,
            OpcodeKind::Zbb(ZbbOpcode::CPOP),
            Some(2),
            Some(23),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000001000000001110000011011,
            OpcodeKind::Zbb(ZbbOpcode::CPOPW),
            Some(24),
            Some(0),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000000001101001011010010011,
            OpcodeKind::Zbb(ZbbOpcode::CLZ),
            Some(13),
            Some(13),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000000001001001001100011011,
            OpcodeKind::Zbb(ZbbOpcode::CLZW),
            Some(6),
            Some(9),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000000111101001001110010011,
            OpcodeKind::Zbb(ZbbOpcode::CTZ),
            Some(7),
            Some(29),
            None,
            None,
        );
        test_32_in_rv64(
            0b1100000000110101001011100011011,
            OpcodeKind::Zbb(ZbbOpcode::CTZW),
            Some(14),
            Some(21),
            None,
            None,
        );
    }
}

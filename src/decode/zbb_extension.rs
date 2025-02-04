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
        let op_31_26: u8 = u8::try_from(inst.slice(31, 26)).unwrap();
        match op_6_0 {
            0b111011 => match op_14_12 {
                0b1 => Ok(ZbbOpcode::ROLW),
                0b101 => Ok(ZbbOpcode::RORW),
                0b100 => Ok(ZbbOpcode::ZEXTH),
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b11011 => match op_14_12 {
                0b101 => Ok(ZbbOpcode::RORIW),
                0b1 => match op_31_20 {
                    0b11000000001 => Ok(ZbbOpcode::CTZW),
                    0b11000000010 => Ok(ZbbOpcode::CPOPW),
                    0b11000000000 => Ok(ZbbOpcode::CLZW),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            0b10011 => match op_14_12 {
                0b101 => match op_31_20 {
                    0b11010011000 => Ok(ZbbOpcode::REV8),
                    0b1010000111 => Ok(ZbbOpcode::ORCB),
                    _ => Ok(ZbbOpcode::RORI),
                    0b11010111000 => Ok(ZbbOpcode::REV8),
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
                0b110 => match op_31_25 {
                    0b100000 => Ok(ZbbOpcode::ORN),
                    0b0101 => Ok(ZbbOpcode::MAX),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b101 => match op_31_25 {
                    0b0101 => Ok(ZbbOpcode::MINU),
                    0b110000 => Ok(ZbbOpcode::ROR),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b111 => match op_31_25 {
                    0b100000 => Ok(ZbbOpcode::ANDN),
                    0b0101 => Ok(ZbbOpcode::MAXU),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b100 => match op_24_20 {
                    0b00 => Ok(ZbbOpcode::ZEXTH),
                    _ => match op_31_25 {
                        0b0101 => Ok(ZbbOpcode::MIN),
                        0b100000 => Ok(ZbbOpcode::XNOR),
                        _ => Err(DecodingError::InvalidOpcode),
                    },
                },
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// Parsing Zbb instruction's rd
    pub fn parse_rd(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rd_11_7: usize = inst.slice(11, 7) as usize;
        match opkind {
            ZbbOpcode::RORIW => Some(rd_11_7),
            ZbbOpcode::RORI => Some(rd_11_7),
            ZbbOpcode::ROLW => Some(rd_11_7),
            ZbbOpcode::RORW => Some(rd_11_7),
            ZbbOpcode::ANDN => Some(rd_11_7),
            ZbbOpcode::ORN => Some(rd_11_7),
            ZbbOpcode::XNOR => Some(rd_11_7),
            ZbbOpcode::MAX => Some(rd_11_7),
            ZbbOpcode::MAXU => Some(rd_11_7),
            ZbbOpcode::MIN => Some(rd_11_7),
            ZbbOpcode::MINU => Some(rd_11_7),
            ZbbOpcode::ROL => Some(rd_11_7),
            ZbbOpcode::ROR => Some(rd_11_7),
            ZbbOpcode::SEXTB => Some(rd_11_7),
            ZbbOpcode::SEXTH => Some(rd_11_7),
            ZbbOpcode::ZEXTH => Some(rd_11_7),
            ZbbOpcode::ZEXTH => Some(rd_11_7),
            ZbbOpcode::REV8 => Some(rd_11_7),
            ZbbOpcode::REV8 => Some(rd_11_7),
            ZbbOpcode::ORCB => Some(rd_11_7),
            ZbbOpcode::CPOP => Some(rd_11_7),
            ZbbOpcode::CPOPW => Some(rd_11_7),
            ZbbOpcode::CLZ => Some(rd_11_7),
            ZbbOpcode::CLZW => Some(rd_11_7),
            ZbbOpcode::CTZ => Some(rd_11_7),
            ZbbOpcode::CTZW => Some(rd_11_7),
        }
    }

    /// Parsing Zbb instruction's rs1
    pub fn parse_rs1(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rs1_19_15: usize = inst.slice(19, 15) as usize;
        match opkind {
            ZbbOpcode::RORIW => Some(rs1_19_15),
            ZbbOpcode::RORI => Some(rs1_19_15),
            ZbbOpcode::ROLW => Some(rs1_19_15),
            ZbbOpcode::RORW => Some(rs1_19_15),
            ZbbOpcode::ANDN => Some(rs1_19_15),
            ZbbOpcode::ORN => Some(rs1_19_15),
            ZbbOpcode::XNOR => Some(rs1_19_15),
            ZbbOpcode::MAX => Some(rs1_19_15),
            ZbbOpcode::MAXU => Some(rs1_19_15),
            ZbbOpcode::MIN => Some(rs1_19_15),
            ZbbOpcode::MINU => Some(rs1_19_15),
            ZbbOpcode::ROL => Some(rs1_19_15),
            ZbbOpcode::ROR => Some(rs1_19_15),
            ZbbOpcode::SEXTB => Some(rs1_19_15),
            ZbbOpcode::SEXTH => Some(rs1_19_15),
            ZbbOpcode::ZEXTH => Some(rs1_19_15),
            ZbbOpcode::ZEXTH => Some(rs1_19_15),
            ZbbOpcode::REV8 => Some(rs1_19_15),
            ZbbOpcode::REV8 => Some(rs1_19_15),
            ZbbOpcode::ORCB => Some(rs1_19_15),
            ZbbOpcode::CPOP => Some(rs1_19_15),
            ZbbOpcode::CPOPW => Some(rs1_19_15),
            ZbbOpcode::CLZ => Some(rs1_19_15),
            ZbbOpcode::CLZW => Some(rs1_19_15),
            ZbbOpcode::CTZ => Some(rs1_19_15),
            ZbbOpcode::CTZW => Some(rs1_19_15),
        }
    }

    /// Parsing Zbb instruction's rs2
    pub fn parse_rs2(inst: u32, opkind: &ZbbOpcode) -> Option<usize> {
        let rs2_24_20: usize = inst.slice(24, 20) as usize;
        match opkind {
            ZbbOpcode::RORIW => None,
            ZbbOpcode::RORI => None,
            ZbbOpcode::ROLW => Some(rs2_24_20),
            ZbbOpcode::RORW => Some(rs2_24_20),
            ZbbOpcode::ANDN => Some(rs2_24_20),
            ZbbOpcode::ORN => Some(rs2_24_20),
            ZbbOpcode::XNOR => Some(rs2_24_20),
            ZbbOpcode::MAX => Some(rs2_24_20),
            ZbbOpcode::MAXU => Some(rs2_24_20),
            ZbbOpcode::MIN => Some(rs2_24_20),
            ZbbOpcode::MINU => Some(rs2_24_20),
            ZbbOpcode::ROL => Some(rs2_24_20),
            ZbbOpcode::ROR => Some(rs2_24_20),
            ZbbOpcode::SEXTB => None,
            ZbbOpcode::SEXTH => None,
            ZbbOpcode::ZEXTH => None,
            ZbbOpcode::ZEXTH => None,
            ZbbOpcode::REV8 => None,
            ZbbOpcode::REV8 => None,
            ZbbOpcode::ORCB => None,
            ZbbOpcode::CPOP => None,
            ZbbOpcode::CPOPW => None,
            ZbbOpcode::CLZ => None,
            ZbbOpcode::CLZW => None,
            ZbbOpcode::CTZ => None,
            ZbbOpcode::CTZW => None,
        }
    }

    /// Parsing Zbb instruction's imm
    pub fn parse_imm(inst: u32, opkind: &ZbbOpcode) -> Option<i32> {
        match opkind {
            ZbbOpcode::RORIW => None,
            ZbbOpcode::RORI => None,
            ZbbOpcode::ROLW => None,
            ZbbOpcode::RORW => None,
            ZbbOpcode::ANDN => None,
            ZbbOpcode::ORN => None,
            ZbbOpcode::XNOR => None,
            ZbbOpcode::MAX => None,
            ZbbOpcode::MAXU => None,
            ZbbOpcode::MIN => None,
            ZbbOpcode::MINU => None,
            ZbbOpcode::ROL => None,
            ZbbOpcode::ROR => None,
            ZbbOpcode::SEXTB => None,
            ZbbOpcode::SEXTH => None,
            ZbbOpcode::ZEXTH => None,
            ZbbOpcode::ZEXTH => None,
            ZbbOpcode::REV8 => None,
            ZbbOpcode::REV8 => None,
            ZbbOpcode::ORCB => None,
            ZbbOpcode::CPOP => None,
            ZbbOpcode::CPOPW => None,
            ZbbOpcode::CLZ => None,
            ZbbOpcode::CLZW => None,
            ZbbOpcode::CTZ => None,
            ZbbOpcode::CTZW => None,
        }
    }
}
#[cfg(test)]
#[allow(unused_variables)]
mod test_zbb {
    #[test]
    #[allow(overflowing_literals)]
    fn zbb_32bit_decode_test() {
        use crate::instruction::zbb_extension::ZbbOpcode;
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
            0b000001101100010101101010110000,
            OpcodeKind::Zbb(ZbbOpcode::RORIW),
            Some(0),
            Some(6),
            None,
            Some(10),
        );
        test_32(
            0b000001001110110111100111011000,
            OpcodeKind::Zbb(ZbbOpcode::RORI),
            Some(10),
            Some(14),
            None,
            Some(14),
        );
        test_32(
            0b000011101111110101100110110000,
            OpcodeKind::Zbb(ZbbOpcode::ROLW),
            Some(15),
            Some(6),
            Some(6),
            None,
        );
        test_32(
            0b000011101100110111000011110000,
            OpcodeKind::Zbb(ZbbOpcode::RORW),
            Some(2),
            Some(12),
            Some(3),
            None,
        );
        test_32(
            0b000011001111011111000111100000,
            OpcodeKind::Zbb(ZbbOpcode::ANDN),
            Some(12),
            Some(12),
            Some(7),
            None,
        );
        test_32(
            0b000011001100011010010000100000,
            OpcodeKind::Zbb(ZbbOpcode::ORN),
            Some(1),
            Some(9),
            Some(0),
            None,
        );
        test_32(
            0b000011001111110010111110100000,
            OpcodeKind::Zbb(ZbbOpcode::XNOR),
            Some(15),
            Some(11),
            Some(14),
            None,
        );
        test_32(
            0b000011001111011000111011000101,
            OpcodeKind::Zbb(ZbbOpcode::MAX),
            Some(13),
            Some(3),
            Some(11),
            None,
        );
        test_32(
            0b000011001101011101001000000101,
            OpcodeKind::Zbb(ZbbOpcode::MAXU),
            Some(4),
            Some(4),
            Some(8),
            None,
        );
        test_32(
            0b000011001100010000100101000101,
            OpcodeKind::Zbb(ZbbOpcode::MIN),
            Some(0),
            Some(2),
            Some(5),
            None,
        );
        test_32(
            0b000011001110110101101100000101,
            OpcodeKind::Zbb(ZbbOpcode::MINU),
            Some(11),
            Some(6),
            Some(12),
            None,
        );
        test_32(
            0b000011001111000100110111110000,
            OpcodeKind::Zbb(ZbbOpcode::ROL),
            Some(12),
            Some(3),
            Some(7),
            None,
        );
        test_32(
            0b000011001101010110100010110000,
            OpcodeKind::Zbb(ZbbOpcode::ROR),
            Some(5),
            Some(10),
            Some(2),
            None,
        );
        test_32(
            0b000001001110010101100100110000,
            OpcodeKind::Zbb(ZbbOpcode::SEXTB),
            Some(9),
            Some(6),
            None,
            None,
        );
        test_32(
            0b000001001100000101100101110000,
            OpcodeKind::Zbb(ZbbOpcode::SEXTH),
            Some(0),
            Some(6),
            None,
            None,
        );
        test_32(
            0b000011001100110010100000000100,
            OpcodeKind::Zbb(ZbbOpcode::ZEXTH),
            Some(2),
            Some(10),
            None,
            None,
        );
        test_32(
            0b000011101111010011010000000100,
            OpcodeKind::Zbb(ZbbOpcode::ZEXTH),
            Some(13),
            Some(13),
            None,
            None,
        );
        test_32(
            0b000010011001101001011010011000,
            OpcodeKind::Zbb(ZbbOpcode::REV8),
            Some(2),
            Some(2),
            None,
            None,
        );
        test_32(
            0b000010011111101011111010111000,
            OpcodeKind::Zbb(ZbbOpcode::REV8),
            Some(15),
            Some(7),
            None,
            None,
        );
        test_32(
            0b000010011001101001001010000111,
            OpcodeKind::Zbb(ZbbOpcode::ORCB),
            Some(2),
            Some(2),
            None,
            None,
        );
        test_32(
            0b000010011110101010111000000010,
            OpcodeKind::Zbb(ZbbOpcode::CPOP),
            Some(13),
            Some(5),
            None,
            None,
        );
        test_32(
            0b000011011100101010111000000010,
            OpcodeKind::Zbb(ZbbOpcode::CPOPW),
            Some(9),
            Some(5),
            None,
            None,
        );
        test_32(
            0b000010011010001010011000000000,
            OpcodeKind::Zbb(ZbbOpcode::CLZ),
            Some(4),
            Some(4),
            None,
            None,
        );
        test_32(
            0b000011011010001100011000000000,
            OpcodeKind::Zbb(ZbbOpcode::CLZW),
            Some(4),
            Some(8),
            None,
            None,
        );
        test_32(
            0b000010011010101100111000000001,
            OpcodeKind::Zbb(ZbbOpcode::CTZ),
            Some(5),
            Some(9),
            None,
            None,
        );
        test_32(
            0b000011011010101101111000000001,
            OpcodeKind::Zbb(ZbbOpcode::CTZW),
            Some(5),
            Some(11),
            None,
            None,
        );
    }
}

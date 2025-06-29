pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::zicboz_extension::ZicbozOpcode;

    pub fn parse_opcode(inst: u32) -> Result<ZicbozOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let field_11_7: u8 = u8::try_from(inst.slice(11, 7)).unwrap();
        let cbo_num: u16 = u16::try_from(inst.slice(31, 20)).unwrap();

        match opmap {
            0b000_1111 => match funct3 {
                0b010 => match cbo_num {
                    0b100 => match field_11_7 {
                        0x0 => Ok(ZicbozOpcode::CBO_ZERO),
                        _ => Err(DecodingError::InvalidOpcode),
                    },
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => Err(DecodingError::InvalidFunct3),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(_inst: u32, _opkind: &ZicbozOpcode) -> Option<usize> {
        None
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, opkind: &ZicbozOpcode) -> Option<usize> {
        let rs1: u8 = u8::try_from(inst.slice(19, 15)).unwrap();
        match opkind {
            ZicbozOpcode::CBO_ZERO => Some(rs1.into()),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs2(_inst: u32, _opkind: &ZicbozOpcode) -> Option<usize> {
        None
    }

    #[allow(clippy::cast_possible_wrap, clippy::unnecessary_wraps)]
    pub fn parse_imm(_inst: u32, _opkind: &ZicbozOpcode) -> Option<i32> {
        None
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_zicboz {
    #[test]
    #[allow(overflowing_literals)]
    fn zicboz_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::zicboz_extension::ZicbozOpcode;
        use crate::OpcodeKind;

        test_32_in_rv64(
            0b0000_0000_0100_1010_1010_0000_0000_1111,
            OpcodeKind::Zicboz(ZicbozOpcode::CBO_ZERO),
            None,
            Some(21),
            None,
            None,
        );

        test_32_in_rv64(
            0b0000_0000_0100_1000_0010_0000_0000_1111,
            OpcodeKind::Zicboz(ZicbozOpcode::CBO_ZERO),
            None,
            Some(16),
            None,
            None,
        );
    }
}

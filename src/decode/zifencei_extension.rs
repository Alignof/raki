pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::zifencei_extension::ZifenceiOpcode;

    pub fn parse_opcode(inst: u32) -> Result<ZifenceiOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();

        match opmap {
            0b000_1111 => Ok(ZifenceiOpcode::FENCE),
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, opkind: &ZifenceiOpcode) -> Option<usize> {
        let rd: usize = inst.slice(11, 7) as usize;

        match opkind {
            ZifenceiOpcode::FENCE => Some(rd),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, opkind: &ZifenceiOpcode) -> Option<usize> {
        let rs1: usize = inst.slice(19, 15) as usize;

        match opkind {
            ZifenceiOpcode::FENCE => Some(rs1),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs2(_inst: u32, opkind: &ZifenceiOpcode) -> Option<usize> {
        match opkind {
            ZifenceiOpcode::FENCE => None,
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::unnecessary_wraps)]
    pub fn parse_imm(inst: u32, opkind: &ZifenceiOpcode) -> Option<i32> {
        let fm_pred_succ: u32 = inst.slice(31, 20);
        match opkind {
            ZifenceiOpcode::FENCE => Some(fm_pred_succ as i32),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_zifenci {
    #[test]
    #[allow(overflowing_literals)]
    fn zifenci_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::zifencei_extension::ZifenceiOpcode;
        use crate::OpcodeKind;

        test_32_in_rv64(
            0b0000_0011_0011_0000_0000_0000_0000_1111,
            OpcodeKind::Zifencei(ZifenceiOpcode::FENCE),
            Some(0),
            Some(0),
            None,
            Some(0b0011_0011),
        );
    }
}

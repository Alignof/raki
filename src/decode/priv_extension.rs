pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::priv_extension::PrivOpcode;

    pub fn parse_opcode(inst: u32) -> Result<PrivOpcode, DecodingError> {
        let _opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let _funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let funct7: u8 = u8::try_from(inst.slice(31, 25)).unwrap();

        match inst {
            0b0001_0000_0010_0000_0000_0000_0111_0011 => Ok(PrivOpcode::SRET),
            0b0011_0000_0010_0000_0000_0000_0111_0011 => Ok(PrivOpcode::MRET),
            0b0001_0000_0101_0000_0000_0000_0111_0011 => Ok(PrivOpcode::WFI),
            _ => match funct7 {
                0b000_1001 => Ok(PrivOpcode::SFENCE_VMA),
                _ => Err(DecodingError::InvalidFunct7),
            },
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(_inst: u32, _opkind: &PrivOpcode) -> Option<usize> {
        None
    }

    pub fn parse_rs1(inst: u32, opkind: &PrivOpcode) -> Option<usize> {
        let rs1: usize = inst.slice(19, 15) as usize;

        match opkind {
            PrivOpcode::SFENCE_VMA => Some(rs1),
            _ => None,
        }
    }

    pub fn parse_rs2(inst: u32, opkind: &PrivOpcode) -> Option<usize> {
        let rs2: usize = inst.slice(24, 20) as usize;

        match opkind {
            PrivOpcode::SFENCE_VMA => Some(rs2),
            _ => None,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_imm(_inst: u32, _opkind: &PrivOpcode) -> Option<i32> {
        None
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_priv {
    #[test]
    #[allow(overflowing_literals)]
    fn priv_decode_test() {
        use crate::instruction::priv_extension::PrivOpcode;
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
            0x10200073,
            OpcodeKind::Priv(PrivOpcode::SRET),
            None,
            None,
            None,
            None,
        );
        test_32(
            0x10500073,
            OpcodeKind::Priv(PrivOpcode::WFI),
            None,
            None,
            None,
            None,
        );
    }
}

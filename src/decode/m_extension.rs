pub mod bit_32 {
    use super::super::{only_rv64, DecodeUtil, DecodingError};
    use crate::instruction::m_extension::MOpcode;
    use crate::Isa;

    pub fn parse_opcode(inst: u32, isa: Isa) -> Result<MOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();

        match opmap {
            0b011_0011 => match funct3 {
                0b000 => Ok(MOpcode::MUL),
                0b001 => Ok(MOpcode::MULH),
                0b010 => Ok(MOpcode::MULHSU),
                0b011 => Ok(MOpcode::MULHU),
                0b100 => Ok(MOpcode::DIV),
                0b101 => Ok(MOpcode::DIVU),
                0b110 => Ok(MOpcode::REM),
                0b111 => Ok(MOpcode::REMU),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b011_1011 => match funct3 {
                0b000 => only_rv64(MOpcode::MULW, isa),
                0b100 => only_rv64(MOpcode::DIVW, isa),
                0b101 => only_rv64(MOpcode::DIVUW, isa),
                0b110 => only_rv64(MOpcode::REMW, isa),
                0b111 => only_rv64(MOpcode::REMUW, isa),
                _ => Err(DecodingError::InvalidFunct3),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, opkind: &MOpcode) -> Option<usize> {
        let rd: usize = inst.slice(11, 7) as usize;

        match opkind {
            MOpcode::MUL
            | MOpcode::MULH
            | MOpcode::MULHSU
            | MOpcode::MULHU
            | MOpcode::DIV
            | MOpcode::DIVU
            | MOpcode::REM
            | MOpcode::REMU
            | MOpcode::MULW
            | MOpcode::DIVW
            | MOpcode::DIVUW
            | MOpcode::REMW
            | MOpcode::REMUW => Some(rd),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, opkind: &MOpcode) -> Option<usize> {
        let rs1: usize = inst.slice(19, 15) as usize;

        match opkind {
            MOpcode::MUL
            | MOpcode::MULH
            | MOpcode::MULHSU
            | MOpcode::MULHU
            | MOpcode::DIV
            | MOpcode::DIVU
            | MOpcode::REM
            | MOpcode::REMU
            | MOpcode::MULW
            | MOpcode::DIVW
            | MOpcode::DIVUW
            | MOpcode::REMW
            | MOpcode::REMUW => Some(rs1),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs2(inst: u32, opkind: &MOpcode) -> Option<usize> {
        let rs2: usize = inst.slice(24, 20) as usize;

        match opkind {
            MOpcode::MUL
            | MOpcode::MULH
            | MOpcode::MULHSU
            | MOpcode::MULHU
            | MOpcode::DIV
            | MOpcode::DIVU
            | MOpcode::REM
            | MOpcode::REMU
            | MOpcode::MULW
            | MOpcode::DIVW
            | MOpcode::DIVUW
            | MOpcode::REMW
            | MOpcode::REMUW => Some(rs2),
        }
    }

    #[allow(non_snake_case)]
    pub fn parse_imm(_inst: u32, _opkind: &MOpcode) -> Option<i32> {
        None
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_m {
    #[test]
    #[allow(overflowing_literals)]
    fn m_decode_test() {
        use crate::instruction::m_extension::MOpcode;
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
            0x02d706b3,
            OpcodeKind::M(MOpcode::MUL),
            Some(13),
            Some(14),
            Some(13),
            None,
        );
        test_32(
            0x0289_7933,
            OpcodeKind::M(MOpcode::REMU),
            Some(18),
            Some(18),
            Some(8),
            None,
        );
        test_32(
            0x0289_5933,
            OpcodeKind::M(MOpcode::DIVU),
            Some(18),
            Some(18),
            Some(8),
            None,
        );
    }
}

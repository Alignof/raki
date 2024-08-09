#[allow(non_snake_case)]
mod c_extension;

use super::{Decode, DecodeUtil, DecodingError};
use crate::instruction::{Extensions, InstFormat, Instruction, OpcodeKind};
use crate::Isa;

impl Decode for u16 {
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError> {
        if *self == 0 {
            return Err(DecodingError::IllegalInstruction);
        }

        let new_opc = self.parse_opcode(isa)?;
        let new_rd: Option<usize> = self.parse_rd(&new_opc)?;
        let new_rs1: Option<usize> = self.parse_rs1(&new_opc)?;
        let new_rs2: Option<usize> = self.parse_rs2(&new_opc)?;
        let new_imm: Option<i32> = self.parse_imm(&new_opc, isa)?;
        let new_fmt: InstFormat = new_opc.get_format();

        Ok(Instruction {
            opc: new_opc,
            rd: new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
            inst_format: new_fmt,
        })
    }

    fn parse_extension(self) -> Result<Extensions, DecodingError> {
        Ok(Extensions::C)
    }

    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError> {
        let extension = self.parse_extension();

        match extension {
            Ok(Extensions::C) => Ok(OpcodeKind::C(c_extension::parse_opcode(self, isa)?)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => c_extension::parse_rd(self, opc),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => c_extension::parse_rs1(self, opc),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => c_extension::parse_rs2(self, opc),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, _isa: Isa) -> Result<Option<i32>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => c_extension::parse_imm(self, opc),
            _ => Err(DecodingError::Not16BitInst),
        }
    }
}

impl DecodeUtil for u16 {
    fn slice(self, end: u32, start: u32) -> Self {
        (self >> start) & (2_u16.pow(end - start + 1) - 1)
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((u32::from(self) >> i) & 0x1) << m;
        }

        inst
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod decode_16 {
    #[test]
    fn decoding_16bit_test() {
        use super::*;
        use crate::instruction::c_extension::COpcode;
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
            0b0000_0000_0000_0001,
            OpcodeKind::C(COpcode::NOP),
            None,
            None,
            None,
            Some(0),
        );
        test_16(
            0b0110_0011_1000_0001,
            OpcodeKind::C(COpcode::LUI),
            Some(7),
            None,
            None,
            Some(0),
        );
        test_16(
            0b1000_0010_1100_0001,
            OpcodeKind::C(COpcode::SRAI),
            Some(13),
            Some(13),
            None,
            Some(16),
        );
        test_16(
            0x4521,
            OpcodeKind::C(COpcode::LI),
            Some(10),
            None,
            None,
            Some(8),
        );
        test_16(
            0xb5e5,
            OpcodeKind::C(COpcode::J),
            None,
            None,
            None,
            Some(-280),
        );
        test_16(
            0x6105,
            OpcodeKind::C(COpcode::ADDI),
            Some(2),
            Some(2),
            None,
            Some(32),
        );
        test_16(
            0x8082,
            OpcodeKind::C(COpcode::JR),
            None,
            Some(1),
            Some(0),
            None,
        );
        test_16(
            0xe29d,
            OpcodeKind::C(COpcode::BNEZ),
            None,
            Some(13),
            None,
            Some(38),
        );
        test_16(
            0xc05c,
            OpcodeKind::C(COpcode::SW),
            None,
            Some(8),
            Some(15),
            Some(4),
        );
        test_16(
            0x9002,
            OpcodeKind::C(COpcode::EBREAK),
            None,
            None,
            None,
            None,
        );
        test_16(
            0x880a,
            OpcodeKind::C(COpcode::MV),
            Some(16),
            None,
            Some(2),
            None,
        );
        test_16(
            0x8585,
            OpcodeKind::C(COpcode::SRAI),
            Some(11),
            Some(11),
            None,
            Some(1),
        );
    }
}

mod a_extension;
mod base_i;
mod m_extension;
mod priv_extension;
mod zicsr_extension;

use super::{Decode, DecodeUtil, DecodingError};
use crate::instruction::{Extensions, InstFormat, Instruction, OpcodeKind};
use crate::Isa;

#[allow(non_snake_case)]
impl Decode for u32 {
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError> {
        let new_opc: OpcodeKind = self.parse_opcode(isa)?;
        let new_rd: Option<usize> = self.parse_rd(&new_opc)?;
        let new_rs1: Option<usize> = self.parse_rs1(&new_opc)?;
        let new_rs2: Option<usize> = self.parse_rs2(&new_opc)?;
        let new_imm: Option<i32> = self.parse_imm(&new_opc, isa)?;
        let new_ext: Extensions = new_opc.get_extension();
        let new_fmt: InstFormat = new_opc.get_format();

        Ok(Instruction {
            opc: new_opc,
            rd: new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
            extension: new_ext,
            inst_format: new_fmt,
        })
    }

    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError> {
        match self.extension() {
            Ok(Extensions::BaseI) => base_i::parse_opcode(self, isa),
            Ok(Extensions::M) => m_extension::parse_opcode(self, isa),
            Ok(Extensions::A) => a_extension::parse_opcode(self, isa),
            Ok(Extensions::Zicsr) => zicsr_extension::parse_opcode(self),
            Ok(Extensions::Priv) => priv_extension::parse_opcode(self),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Ok(Extensions::BaseI) => base_i::parse_rd(self, opkind),
            Ok(Extensions::M) => m_extension::parse_rd(self, opkind),
            Ok(Extensions::A) => a_extension::parse_rd(self, opkind),
            Ok(Extensions::Zicsr) => zicsr_extension::parse_rd(self, opkind),
            Ok(Extensions::Priv) => priv_extension::parse_rd(self, opkind),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Ok(Extensions::BaseI) => base_i::parse_rs1(self, opkind),
            Ok(Extensions::M) => m_extension::parse_rs1(self, opkind),
            Ok(Extensions::A) => a_extension::parse_rs1(self, opkind),
            Ok(Extensions::Zicsr) => zicsr_extension::parse_rs1(self, opkind),
            Ok(Extensions::Priv) => priv_extension::parse_rs1(self, opkind),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Ok(Extensions::BaseI) => base_i::parse_rs2(self, opkind),
            Ok(Extensions::M) => m_extension::parse_rs2(self, opkind),
            Ok(Extensions::A) => a_extension::parse_rs2(self, opkind),
            Ok(Extensions::Zicsr) => zicsr_extension::parse_rs2(self, opkind),
            Ok(Extensions::Priv) => priv_extension::parse_rs2(self, opkind),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError> {
        match self.extension() {
            Ok(Extensions::BaseI) => base_i::parse_imm(self, opkind, isa),
            Ok(Extensions::M) => m_extension::parse_imm(self, opkind),
            Ok(Extensions::A) => a_extension::parse_imm(self, opkind),
            Ok(Extensions::Zicsr) => zicsr_extension::parse_imm(self, opkind),
            Ok(Extensions::Priv) => priv_extension::parse_imm(self, opkind),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }
}

impl DecodeUtil for u32 {
    fn slice(self, end: u32, start: u32) -> Self {
        (self >> start) & (2_u32.pow(end - start + 1) - 1)
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((self >> i) & 0x1) << m;
        }

        inst
    }

    fn extension(self) -> Result<Extensions, DecodingError> {
        let opmap: u8 = self.slice(6, 0) as u8;
        let funct3: u8 = self.slice(14, 12) as u8;
        let funct7: u8 = self.slice(31, 25) as u8;

        match opmap {
            0b010_1111 => Ok(Extensions::A),
            0b011_0011 => match funct7 {
                0b000_0001 => Ok(Extensions::M),
                _ => Ok(Extensions::BaseI),
            },
            0b011_1011 => match funct7 {
                0b000_0000 | 0b010_0000 => Ok(Extensions::BaseI),
                0b000_0001 => Ok(Extensions::M),
                _ => Err(DecodingError::UnknownExtension),
            },
            0b111_0011 => match funct3 {
                0b000 => match funct7 {
                    0b000_0000 => Ok(Extensions::BaseI),
                    _ => Ok(Extensions::Priv),
                },
                _ => Ok(Extensions::Zicsr),
            },
            _ => Ok(Extensions::BaseI),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod decode_32 {
    #[test]
    #[allow(overflowing_literals)]
    fn decoding_32bit_inst_test() {
        use super::*;
        use OpcodeKind::*;
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
            0b1000_0000_0000_0000_0000_0000_1011_0111,
            LUI,
            Some(1),
            None,
            None,
            Some(0x8000_0000),
        );
        test_32(
            0b0000_0000_0000_0000_0000_0010_1001_0111,
            AUIPC,
            Some(5),
            None,
            None,
            Some(0),
        );
        test_32(
            0b1111_1111_1001_1111_1111_0000_0110_1111,
            JAL,
            Some(0),
            None,
            None,
            Some(-8),
        );
        test_32(
            0b1111_1110_0010_0000_1000_1110_1010_0011,
            SB,
            None,
            Some(1),
            Some(2),
            Some(-3),
        );
        test_32(
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            ADDI,
            Some(5),
            Some(5),
            None,
            Some(-276),
        );
        test_32(
            0b0000_0000_0000_0000_0000_0000_0111_0011,
            ECALL,
            None,
            None,
            None,
            None,
        );
        test_32(
            0b0000_0000_0000_0101_0100_1100_0110_0011,
            BLT,
            None,
            Some(10),
            Some(0),
            Some(24),
        );
        test_32(0x0010_0513, ADDI, Some(10), Some(0), None, Some(1));
        test_32(0x04d7_27af, AMOADD_W, Some(15), Some(14), Some(13), Some(2));
        test_32(0x4170_04b3, SUB, Some(9), Some(0), Some(23), None);
        test_32(0x3307_3983, LD, Some(19), Some(14), None, Some(816));
        test_32(0x10ec_eb63, BLTU, None, Some(25), Some(14), Some(278));
        test_32(0x31e1_60ef, JAL, Some(1), None, None, Some(90910));
        test_32(0x0019_4913, XORI, Some(18), Some(18), None, Some(1));
        test_32(0x00a9_3933, SLTU, Some(18), Some(18), Some(10), None);
        test_32(0x0289_7933, REMU, Some(18), Some(18), Some(8), None);
        test_32(0x0289_5933, DIVU, Some(18), Some(18), Some(8), None);
    }
}

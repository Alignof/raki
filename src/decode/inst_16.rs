#[allow(non_snake_case)]
mod c_extension;

use super::{Decode, DecodeUtil, DecodingError};
use crate::instruction::{Extensions, InstFormat, Instruction, OpcodeKind};
use crate::Isa;

impl Decode for u16 {
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError> {
        let new_opc = self.parse_opcode(isa)?;
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
            Extensions::C => c_extension::parse_opcode(self, isa),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Extensions::C => c_extension::parse_rd(self, opkind),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Extensions::C => c_extension::parse_rs1(self, opkind),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match self.extension() {
            Extensions::C => c_extension::parse_rs2(self, opkind),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, _isa: Isa) -> Result<Option<i32>, DecodingError> {
        match self.extension() {
            Extensions::C => c_extension::parse_imm(self, opkind),
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
            inst |= ((self as u32 >> i) & 0x1) << m;
        }

        inst
    }

    fn extension(self) -> Extensions {
        Extensions::C
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod decode_16 {
    use super::*;

    #[test]
    fn parsing_compressed_opcode_test() {
        use OpcodeKind::*;
        let test_16 = |inst_16: u16, _op: OpcodeKind, _rd: Option<u8>| {
            let op_16 = inst_16.parse_opcode(Isa::Rv64).unwrap();
            assert!(matches!(&op_16, _op));
            assert!(matches!(inst_16.parse_rd(&op_16), _rd));
        };

        test_16(0b0000000000000001, C_NOP, None);
        test_16(0b0000000010000001, C_ADDI, Some(0));
        test_16(0b0110000100000001, C_ADDI16SP, None);
        test_16(0b0110001110000001, C_LUI, None);
        test_16(0b1000001011000001, C_SRAI, Some(0));
        test_16(0b1000010011000001, C_ANDI, None);
    }
}

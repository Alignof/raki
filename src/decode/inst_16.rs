#[allow(non_snake_case)]
mod c_extension;
mod zicfiss_extension;

use super::{Decode, DecodeUtil, DecodingError};
use crate::instruction::{InstFormat, Instruction, OpcodeKind};
use crate::{Extensions, Isa};

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

    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError> {
        let extension = self.parse_extension();

        match extension {
            Ok(Extensions::C) => Ok(OpcodeKind::C(c_extension::parse_opcode(self, isa)?)),
            Ok(Extensions::Zicfiss) => Ok(OpcodeKind::Zicfiss(zicfiss_extension::parse_opcode(
                self, isa,
            )?)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => Ok(c_extension::parse_rd(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::parse_rd(self, opc)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => Ok(c_extension::parse_rs1(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::parse_rs1(self, opc)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => Ok(c_extension::parse_rs2(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::parse_rs2(self, opc)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, _isa: Isa) -> Result<Option<i32>, DecodingError> {
        match opkind {
            OpcodeKind::C(opc) => Ok(c_extension::parse_imm(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::parse_imm(self, opc)),
            _ => Err(DecodingError::Not16BitInst),
        }
    }
}

impl DecodeUtil for u16 {
    fn slice(self, end: u32, start: u32) -> Self {
        assert!(end >= start);
        (self >> start) & (2_u16.pow(end - start + 1) - 1)
    }

    fn parse_extension(self) -> Result<Extensions, DecodingError> {
        match self {
            0b0110_0000_1000_0001 | 0b0110_0010_1000_0001 => Ok(Extensions::Zicfiss),
            _ => Ok(Extensions::C),
        }
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((u32::from(self) >> i) & 0x1) << m;
        }

        inst
    }
}

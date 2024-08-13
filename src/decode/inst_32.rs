mod a_extension;
mod base_i;
mod m_extension;
mod priv_extension;
mod zicntr_extension;
mod zicsr_extension;
mod zifencei_extension;

use super::{Decode, DecodeUtil, DecodingError};
use crate::instruction::{InstFormat, Instruction, OpcodeKind};
use crate::{Extensions, Isa};

#[allow(non_snake_case)]
impl Decode for u32 {
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError> {
        let new_opc: OpcodeKind = self.parse_opcode(isa)?;
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
            Ok(Extensions::BaseI) => Ok(OpcodeKind::BaseI(base_i::parse_opcode(self, isa)?)),
            Ok(Extensions::M) => Ok(OpcodeKind::M(m_extension::parse_opcode(self, isa)?)),
            Ok(Extensions::A) => Ok(OpcodeKind::A(a_extension::parse_opcode(self, isa)?)),
            Ok(Extensions::Zifencei) => Ok(OpcodeKind::Zifencei(zifencei_extension::parse_opcode(
                self,
            )?)),
            Ok(Extensions::Zicsr) => Ok(OpcodeKind::Zicsr(zicsr_extension::parse_opcode(self)?)),
            Ok(Extensions::Zicntr) => Ok(OpcodeKind::Zicntr(zicntr_extension::parse_opcode(self)?)),
            Ok(Extensions::Priv) => Ok(OpcodeKind::Priv(priv_extension::parse_opcode(self)?)),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::parse_rd(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::parse_rd(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::parse_rd(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::parse_rd(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::parse_rd(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::parse_rd(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::parse_rd(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::parse_rs1(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::parse_rs1(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::parse_rs1(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::parse_rs1(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::parse_rs1(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::parse_rs1(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::parse_rs1(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::parse_rs2(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::parse_rs2(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::parse_rs2(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::parse_rs2(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::parse_rs2(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::parse_rs2(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::parse_rs2(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::parse_imm(self, opc, isa)),
            OpcodeKind::M(opc) => Ok(m_extension::parse_imm(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::parse_imm(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::parse_imm(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::parse_imm(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::parse_imm(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::parse_imm(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }
}

impl DecodeUtil for u32 {
    fn slice(self, end: u32, start: u32) -> Self {
        assert!(end >= start);
        (self >> start) & (2_u32.pow(end - start + 1) - 1)
    }

    fn parse_extension(self) -> Result<Extensions, DecodingError> {
        let opmap: u8 = u8::try_from(self.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(self.slice(14, 12)).unwrap();
        let funct7: u8 = u8::try_from(self.slice(31, 25)).unwrap();
        let csr: u16 = u16::try_from(self.slice(31, 20)).unwrap();

        match opmap {
            0b000_1111 => Ok(Extensions::Zifencei),
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
                0b010 => match csr {
                    // csrrs rd, (cycle|time|instret), zero
                    0xc00..=0xc02 | 0xc80..=0xc82 => Ok(Extensions::Zicntr),
                    _ => Ok(Extensions::Zicsr),
                },
                _ => Ok(Extensions::Zicsr),
            },
            _ => Ok(Extensions::BaseI),
        }
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((self >> i) & 0x1) << m;
        }

        inst
    }
}

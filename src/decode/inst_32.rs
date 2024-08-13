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

#[cfg(test)]
#[allow(unused_variables)]
mod decode_32 {
    #[test]
    #[allow(overflowing_literals)]
    fn decoding_32bit_inst_test() {
        use super::*;
        use crate::{AOpcode, BaseIOpcode, MOpcode, ZifenceiOpcode};

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
            OpcodeKind::BaseI(BaseIOpcode::LUI),
            Some(1),
            None,
            None,
            Some(0x8000_0000),
        );
        test_32(
            0b0000_0000_0000_0000_0000_0010_1001_0111,
            OpcodeKind::BaseI(BaseIOpcode::AUIPC),
            Some(5),
            None,
            None,
            Some(0),
        );
        test_32(
            0b1111_1111_1001_1111_1111_0000_0110_1111,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
            Some(0),
            None,
            None,
            Some(-8),
        );
        test_32(
            0b1111_1110_0010_0000_1000_1110_1010_0011,
            OpcodeKind::BaseI(BaseIOpcode::SB),
            None,
            Some(1),
            Some(2),
            Some(-3),
        );
        test_32(
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            OpcodeKind::BaseI(BaseIOpcode::ADDI),
            Some(5),
            Some(5),
            None,
            Some(-276),
        );
        test_32(
            0b0000_0000_0000_0000_0000_0000_0111_0011,
            OpcodeKind::BaseI(BaseIOpcode::ECALL),
            None,
            None,
            None,
            None,
        );
        test_32(
            0b0000_0000_0000_0101_0100_1100_0110_0011,
            OpcodeKind::BaseI(BaseIOpcode::BLT),
            None,
            Some(10),
            Some(0),
            Some(24),
        );
        test_32(
            0x0010_0513,
            OpcodeKind::BaseI(BaseIOpcode::ADDI),
            Some(10),
            Some(0),
            None,
            Some(1),
        );
        test_32(
            0x04d7_27af,
            OpcodeKind::A(AOpcode::AMOADD_W),
            Some(15),
            Some(14),
            Some(13),
            Some(2),
        );
        test_32(
            0x4170_04b3,
            OpcodeKind::BaseI(BaseIOpcode::SUB),
            Some(9),
            Some(0),
            Some(23),
            None,
        );
        test_32(
            0x3307_3983,
            OpcodeKind::BaseI(BaseIOpcode::LD),
            Some(19),
            Some(14),
            None,
            Some(816),
        );
        test_32(
            0x10ec_eb63,
            OpcodeKind::BaseI(BaseIOpcode::BLTU),
            None,
            Some(25),
            Some(14),
            Some(278),
        );
        test_32(
            0x31e1_60ef,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
            Some(1),
            None,
            None,
            Some(90910),
        );
        test_32(
            0x0019_4913,
            OpcodeKind::BaseI(BaseIOpcode::XORI),
            Some(18),
            Some(18),
            None,
            Some(1),
        );
        test_32(
            0x00a9_3933,
            OpcodeKind::BaseI(BaseIOpcode::SLTU),
            Some(18),
            Some(18),
            Some(10),
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
        test_32(
            0b0000_0011_0011_0000_0000_0000_0000_1111,
            OpcodeKind::Zifencei(ZifenceiOpcode::FENCE),
            Some(0),
            Some(0),
            None,
            Some(0x033),
        )
    }
}

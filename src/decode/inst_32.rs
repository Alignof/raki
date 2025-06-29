use super::{
    a_extension, base_i, m_extension, priv_extension, zicboz_extension, zicfiss_extension,
    zicntr_extension, zicsr_extension, zifencei_extension,
};
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
            is_compressed: false,
        })
    }

    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError> {
        let extension = self.parse_extension();

        match extension {
            Ok(Extensions::BaseI) => {
                Ok(OpcodeKind::BaseI(base_i::bit_32::parse_opcode(self, isa)?))
            }
            Ok(Extensions::M) => Ok(OpcodeKind::M(m_extension::bit_32::parse_opcode(self, isa)?)),
            Ok(Extensions::A) => Ok(OpcodeKind::A(a_extension::bit_32::parse_opcode(self, isa)?)),
            Ok(Extensions::Zifencei) => Ok(OpcodeKind::Zifencei(
                zifencei_extension::bit_32::parse_opcode(self)?,
            )),
            Ok(Extensions::Zicsr) => Ok(OpcodeKind::Zicsr(zicsr_extension::bit_32::parse_opcode(
                self,
            )?)),
            Ok(Extensions::Zicfiss) => Ok(OpcodeKind::Zicfiss(
                zicfiss_extension::bit_32::parse_opcode(self)?,
            )),
            Ok(Extensions::Zicntr) => Ok(OpcodeKind::Zicntr(
                zicntr_extension::bit_32::parse_opcode(self)?,
            )),
            Ok(Extensions::Zicboz) => Ok(OpcodeKind::Zicboz(
                zicboz_extension::bit_32::parse_opcode(self)?,
            )),
            Ok(Extensions::Priv) => Ok(OpcodeKind::Priv(priv_extension::bit_32::parse_opcode(
                self,
            )?)),
            Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
            Err(decoding_err) => Err(decoding_err),
        }
    }

    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::bit_32::parse_rd(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Zicboz(opc) => Ok(zicboz_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::bit_32::parse_rd(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::bit_32::parse_rs1(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Zicboz(opc) => Ok(zicboz_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::bit_32::parse_rs1(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::bit_32::parse_rs2(self, opc)),
            OpcodeKind::M(opc) => Ok(m_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Zicboz(opc) => Ok(zicboz_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::bit_32::parse_rs2(self, opc)),
            OpcodeKind::C(_) => Err(DecodingError::Not32BitInst),
        }
    }

    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError> {
        match opkind {
            OpcodeKind::BaseI(opc) => Ok(base_i::bit_32::parse_imm(self, opc, isa)),
            OpcodeKind::M(opc) => Ok(m_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::A(opc) => Ok(a_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Zifencei(opc) => Ok(zifencei_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Zicsr(opc) => Ok(zicsr_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Zicfiss(opc) => Ok(zicfiss_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Zicntr(opc) => Ok(zicntr_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Zicboz(opc) => Ok(zicboz_extension::bit_32::parse_imm(self, opc)),
            OpcodeKind::Priv(opc) => Ok(priv_extension::bit_32::parse_imm(self, opc)),
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
        let funct5: u8 = u8::try_from(self.slice(31, 27)).unwrap();
        let funct7: u8 = u8::try_from(self.slice(31, 25)).unwrap();
        let csr: u16 = u16::try_from(self.slice(31, 20)).unwrap();

        match opmap {
            0b000_1111 => match funct3 {
                0b000 => Ok(Extensions::Zifencei),
                0b010 => Ok(Extensions::Zicboz),
                _ => Err(DecodingError::UnknownExtension),
            },
            0b010_1111 => match funct5 {
                0b00000 | 0b00001 | 0b00010 | 0b00011 | 0b00100 | 0b01000 | 0b01100 | 0b10000
                | 0b10100 | 0b11000 | 0b11100 => Ok(Extensions::A),
                0b01001 => Ok(Extensions::Zicfiss),
                _ => Err(DecodingError::UnknownExtension),
            },
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
                0b100 => Ok(Extensions::Zicfiss),
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
pub fn test_32(
    isa: Isa,
    location: &std::panic::Location,
    inst_32: u32,
    op: OpcodeKind,
    rd: Option<usize>,
    rs1: Option<usize>,
    rs2: Option<usize>,
    imm: Option<i32>,
) {
    let op_32 = inst_32.parse_opcode(isa).unwrap_or_else(|e| {
        panic!(
            "{e:?}: failed to decode {inst_32:032b} ({}: line {})",
            location.file(),
            location.line()
        );
    });
    assert_eq!(
        op_32,
        op,
        "Opecode does not match: {op_32} != {op} ({}: line {})",
        location.file(),
        location.line()
    );
    assert_eq!(
        inst_32.parse_rd(&op_32).unwrap_or_else(|e| {
            panic!(
                "{e:?}: failed to parse rd in {op}({inst_32:032b}) ({}: line {})",
                location.file(),
                location.line()
            );
        }),
        rd,
        "rd does not match: {:x?} != {rd:x?} ({}: line {})",
        inst_32.parse_rd(&op_32),
        location.file(),
        location.line()
    );
    assert_eq!(
        inst_32.parse_rs1(&op_32).unwrap_or_else(|e| {
            panic!(
                "{e:?}: failed to parse rs1 in {op}({inst_32:032b}) ({}: line {})",
                location.file(),
                location.line()
            );
        }),
        rs1,
        "rs1 does not match: {:x?} != {rs1:x?} ({}: line {})",
        inst_32.parse_rs1(&op_32),
        location.file(),
        location.line()
    );
    assert_eq!(
        inst_32.parse_rs2(&op_32).unwrap_or_else(|e| {
            panic!(
                "{e:?}: failed to parse rs2 in {op}({inst_32:032b}) ({}: line {})",
                location.file(),
                location.line()
            );
        }),
        rs2,
        "rs2 does not match: {:x?} != {rs2:x?} ({}: line {})",
        inst_32.parse_rs2(&op_32),
        location.file(),
        location.line()
    );
    assert_eq!(
        inst_32.parse_imm(&op_32, isa).unwrap_or_else(|e| {
            panic!(
                "{e:?}: failed to parse imm in {op}({inst_32:032b}) ({}: line {})",
                location.file(),
                location.line()
            );
        }),
        imm,
        "imm does not match: {:x?} != {imm:x?} ({}: line {})",
        inst_32.parse_imm(&op_32, isa),
        location.file(),
        location.line()
    );
}

#[cfg(test)]
#[track_caller]
#[allow(dead_code)]
pub fn test_32_in_rv32(
    inst_32: u32,
    op: OpcodeKind,
    rd: Option<usize>,
    rs1: Option<usize>,
    rs2: Option<usize>,
    imm: Option<i32>,
) {
    let location = std::panic::Location::caller();
    test_32(Isa::Rv32, location, inst_32, op, rd, rs1, rs2, imm);
}

#[cfg(test)]
#[track_caller]
pub fn test_32_in_rv64(
    inst_32: u32,
    op: OpcodeKind,
    rd: Option<usize>,
    rs1: Option<usize>,
    rs2: Option<usize>,
    imm: Option<i32>,
) {
    let location = std::panic::Location::caller();
    test_32(Isa::Rv64, location, inst_32, op, rd, rs1, rs2, imm);
}

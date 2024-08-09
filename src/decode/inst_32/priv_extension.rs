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

pub fn parse_rd(_inst: u32, _opkind: &PrivOpcode) -> Result<Option<usize>, DecodingError> {
    Ok(None)
}

pub fn parse_rs1(inst: u32, opkind: &PrivOpcode) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        PrivOpcode::SFENCE_VMA => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &PrivOpcode) -> Result<Option<usize>, DecodingError> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        PrivOpcode::SFENCE_VMA => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &PrivOpcode) -> Result<Option<i32>, DecodingError> {
    Ok(None)
}

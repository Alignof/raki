use super::super::{DecodeUtil, DecodingError};
use crate::instruction::OpcodeKind;

pub fn parse_opcode(inst: u32) -> Result<OpcodeKind, DecodingError> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b111_0011 => match funct3 {
            0b001 => Ok(OpcodeKind::CSRRW),
            0b010 => Ok(OpcodeKind::CSRRS),
            0b011 => Ok(OpcodeKind::CSRRC),
            0b101 => Ok(OpcodeKind::CSRRWI),
            0b110 => Ok(OpcodeKind::CSRRSI),
            0b111 => Ok(OpcodeKind::CSRRCI),
            _ => Err(DecodingError::IllegalFunct3),
        },
        _ => Err(DecodingError::IllegalOpcode),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpcodeKind::CSRRW
        | OpcodeKind::CSRRS
        | OpcodeKind::CSRRC
        | OpcodeKind::CSRRWI
        | OpcodeKind::CSRRSI
        | OpcodeKind::CSRRCI => Ok(Some(rd)),
        _ => panic!("This instruction does not have rd"),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpcodeKind::CSRRW
        | OpcodeKind::CSRRS
        | OpcodeKind::CSRRC
        | OpcodeKind::CSRRWI
        | OpcodeKind::CSRRSI
        | OpcodeKind::CSRRCI => Ok(Some(rs1)),
        _ => panic!("This instruction does not have rs1"),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    let csr: usize = inst.slice(31, 20) as usize;

    match opkind {
        OpcodeKind::CSRRW
        | OpcodeKind::CSRRS
        | OpcodeKind::CSRRC
        | OpcodeKind::CSRRWI
        | OpcodeKind::CSRRSI
        | OpcodeKind::CSRRCI => Ok(Some(csr)),
        _ => panic!("This instruction does not have rs2"),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpcodeKind) -> Result<Option<i32>, DecodingError> {
    Ok(None)
}

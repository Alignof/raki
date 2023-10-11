use super::super::DecodeUtil;
use crate::instruction::OpcodeKind;

pub fn parse_opcode(inst: u32) -> Result<OpcodeKind, (Option<u64>, String)> {
    let _opmap: u8 = inst.slice(6, 0) as u8;
    let _funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match inst {
        0b00010000001000000000000001110011 => Ok(OpcodeKind::SRET),
        0b00110000001000000000000001110011 => Ok(OpcodeKind::MRET),
        0b00010000010100000000000001110011 => Ok(OpcodeKind::WFI),
        _ => match funct7 {
            0b0001001 => Ok(OpcodeKind::SFENCE_VMA),
            _ => Err((
                Some(u64::from(inst)),
                format!("opcode decoding failed in priv extension, {inst:b}"),
            )),
        },
    }
}

pub fn parse_rd(_inst: u32, _opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    Ok(None)
}

pub fn parse_rs1(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpcodeKind::SFENCE_VMA => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpcodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpcodeKind::SFENCE_VMA => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpcodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    Ok(None)
}

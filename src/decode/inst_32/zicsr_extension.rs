use super::super::DecodeUtil;
use crate::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b1110011 => match funct3 {
            0b001 => Ok(OpecodeKind::CSRRW),
            0b010 => Ok(OpecodeKind::CSRRS),
            0b011 => Ok(OpecodeKind::CSRRC),
            0b101 => Ok(OpecodeKind::CSRRWI),
            0b110 => Ok(OpecodeKind::CSRRSI),
            0b111 => Ok(OpecodeKind::CSRRCI),
            _ => Err((
                Some(u64::from(inst)),
                format!("opecode decoding failed in zicsr extension, {inst:b}"),
            )),
        },
        _ => Err((
            Some(u64::from(inst)),
            format!("opecode decoding failed in zicsr extension, {inst:b}"),
        )),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::CSRRW => Ok(Some(rd)),
        OpecodeKind::CSRRS => Ok(Some(rd)),
        OpecodeKind::CSRRC => Ok(Some(rd)),
        OpecodeKind::CSRRWI => Ok(Some(rd)),
        OpecodeKind::CSRRSI => Ok(Some(rd)),
        OpecodeKind::CSRRCI => Ok(Some(rd)),
        _ => panic!("rd not found in csr instruction"),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::CSRRW => Ok(Some(rs1)),
        OpecodeKind::CSRRS => Ok(Some(rs1)),
        OpecodeKind::CSRRC => Ok(Some(rs1)),
        OpecodeKind::CSRRWI => Ok(Some(rs1)),
        OpecodeKind::CSRRSI => Ok(Some(rs1)),
        OpecodeKind::CSRRCI => Ok(Some(rs1)),
        _ => panic!("rs1 not found in csr instruction"),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let csr: usize = inst.slice(31, 20) as usize;

    match opkind {
        OpecodeKind::CSRRW => Ok(Some(csr)),
        OpecodeKind::CSRRS => Ok(Some(csr)),
        OpecodeKind::CSRRC => Ok(Some(csr)),
        OpecodeKind::CSRRWI => Ok(Some(csr)),
        OpecodeKind::CSRRSI => Ok(Some(csr)),
        OpecodeKind::CSRRCI => Ok(Some(csr)),
        _ => panic!("rs2 not found in csr instruction"),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    Ok(None)
}

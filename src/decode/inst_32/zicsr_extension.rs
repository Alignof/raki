use super::super::DecodeUtil;
use crate::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b1110011 => match funct3 {
            0b001 => Ok(OpecodeKind::OP_CSRRW),
            0b010 => Ok(OpecodeKind::OP_CSRRS),
            0b011 => Ok(OpecodeKind::OP_CSRRC),
            0b101 => Ok(OpecodeKind::OP_CSRRWI),
            0b110 => Ok(OpecodeKind::OP_CSRRSI),
            0b111 => Ok(OpecodeKind::OP_CSRRCI),
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
        OpecodeKind::OP_CSRRW => Ok(Some(rd)),
        OpecodeKind::OP_CSRRS => Ok(Some(rd)),
        OpecodeKind::OP_CSRRC => Ok(Some(rd)),
        OpecodeKind::OP_CSRRWI => Ok(Some(rd)),
        OpecodeKind::OP_CSRRSI => Ok(Some(rd)),
        OpecodeKind::OP_CSRRCI => Ok(Some(rd)),
        _ => panic!("rd not found in csr instruction"),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_CSRRW => Ok(Some(rs1)),
        OpecodeKind::OP_CSRRS => Ok(Some(rs1)),
        OpecodeKind::OP_CSRRC => Ok(Some(rs1)),
        OpecodeKind::OP_CSRRWI => Ok(Some(rs1)),
        OpecodeKind::OP_CSRRSI => Ok(Some(rs1)),
        OpecodeKind::OP_CSRRCI => Ok(Some(rs1)),
        _ => panic!("rs1 not found in csr instruction"),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let csr: usize = inst.slice(31, 20) as usize;

    match opkind {
        OpecodeKind::OP_CSRRW => Ok(Some(csr)),
        OpecodeKind::OP_CSRRS => Ok(Some(csr)),
        OpecodeKind::OP_CSRRC => Ok(Some(csr)),
        OpecodeKind::OP_CSRRWI => Ok(Some(csr)),
        OpecodeKind::OP_CSRRSI => Ok(Some(csr)),
        OpecodeKind::OP_CSRRCI => Ok(Some(csr)),
        _ => panic!("rs2 not found in csr instruction"),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<i32>, (Option<u64>, String)> {
    Ok(None)
}

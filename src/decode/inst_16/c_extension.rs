use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::OpcodeKind;
use crate::Isa;

fn quadrant0(_inst: u16, opmap: u8, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    match opmap {
        0b000 => Ok(OpcodeKind::C_ADDI4SPN),
        0b010 => Ok(OpcodeKind::C_LW),
        0b011 => only_rv64(OpcodeKind::C_LD, isa),
        0b110 => Ok(OpcodeKind::C_SW),
        0b111 => only_rv64(OpcodeKind::C_SD, isa),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

fn quadrant1(inst: u16, opmap: u8, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let sr_flag: u8 = inst.slice(11, 10) as u8;
    let lo_flag: u8 = inst.slice(6, 5) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;
    let bit_12: u8 = inst.slice(12, 12) as u8;

    match opmap {
        0b000 => match mi_flag {
            0b00000 => Ok(OpcodeKind::C_NOP),
            _ => Ok(OpcodeKind::C_ADDI),
        },
        0b001 => match isa {
            Isa::Rv32 => Ok(OpcodeKind::C_JAL),
            Isa::Rv64 => Ok(OpcodeKind::C_ADDIW),
        },
        0b010 => Ok(OpcodeKind::C_LI),
        0b011 => match mi_flag {
            0b00010 => Ok(OpcodeKind::C_ADDI16SP),
            _ => Ok(OpcodeKind::C_LUI),
        },
        0b100 => match sr_flag {
            0b00 => Ok(OpcodeKind::C_SRLI),
            0b01 => Ok(OpcodeKind::C_SRAI),
            0b10 => Ok(OpcodeKind::C_ANDI),
            0b11 => match bit_12 {
                0b0 => match lo_flag {
                    0b00 => Ok(OpcodeKind::C_SUB),
                    0b01 => Ok(OpcodeKind::C_XOR),
                    0b10 => Ok(OpcodeKind::C_OR),
                    0b11 => Ok(OpcodeKind::C_AND),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b1 => match lo_flag {
                    0b00 => only_rv64(OpcodeKind::C_SUBW, isa),
                    0b01 => only_rv64(OpcodeKind::C_ADDW, isa),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => unreachable!(),
            },
            _ => Err(DecodingError::InvalidOpcode),
        },
        0b101 => Ok(OpcodeKind::C_J),
        0b110 => Ok(OpcodeKind::C_BEQZ),
        0b111 => Ok(OpcodeKind::C_BNEZ),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

fn quadrant2(inst: u16, opmap: u8, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let lo_flag: u8 = inst.slice(6, 2) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;
    let hi_flag: u8 = inst.slice(12, 12) as u8;

    match opmap {
        0b000 => Ok(OpcodeKind::C_SLLI),
        0b010 => Ok(OpcodeKind::C_LWSP),
        0b011 => only_rv64(OpcodeKind::C_LDSP, isa),
        0b100 => match hi_flag {
            0b0 => match lo_flag {
                0b0 => Ok(OpcodeKind::C_JR),
                _ => Ok(OpcodeKind::C_MV),
            },
            0b1 => match mi_flag {
                0b0 => Ok(OpcodeKind::C_EBREAK),
                _ => match lo_flag {
                    0b0 => Ok(OpcodeKind::C_JALR),
                    _ => Ok(OpcodeKind::C_ADD),
                },
            },
            _ => Err(DecodingError::InvalidOpcode),
        },
        0b110 => Ok(OpcodeKind::C_SWSP),
        0b111 => only_rv64(OpcodeKind::C_SDSP, isa),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_opcode(inst: u16, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    let opmap: u8 = inst.slice(15, 13) as u8;
    let quadrant: u8 = inst.slice(1, 0) as u8;

    if inst == 0b0000_0000_0000_0000 {
        return Err(DecodingError::InvalidOpcode);
    }

    match quadrant {
        0b00 => quadrant0(inst, opmap, isa),
        0b01 => quadrant1(inst, opmap, isa),
        0b10 => quadrant2(inst, opmap, isa),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_rd(inst: u16, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rd: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rd: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_wide_rd: usize = inst.slice(11, 7) as usize;
    let q2_rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        OpcodeKind::C_ADDI4SPN | OpcodeKind::C_LW | OpcodeKind::C_LD => Ok(Some(q0_rd)),
        // Quadrant 1
        OpcodeKind::C_SRLI
        | OpcodeKind::C_SRAI
        | OpcodeKind::C_ANDI
        | OpcodeKind::C_SUB
        | OpcodeKind::C_XOR
        | OpcodeKind::C_OR
        | OpcodeKind::C_AND
        | OpcodeKind::C_ADDW
        | OpcodeKind::C_SUBW => Ok(Some(q1_rd)),
        OpcodeKind::C_LI | OpcodeKind::C_LUI | OpcodeKind::C_ADDI | OpcodeKind::C_ADDIW => {
            Ok(Some(q1_wide_rd))
        }
        // Quadrant 2
        OpcodeKind::C_SLLI
        | OpcodeKind::C_LWSP
        | OpcodeKind::C_LDSP
        | OpcodeKind::C_MV
        | OpcodeKind::C_JALR
        | OpcodeKind::C_ADD => Ok(Some(q2_rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u16, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_addi_rs1: usize = inst.slice(11, 7) as usize;
    let q2_rs1: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        OpcodeKind::C_LW | OpcodeKind::C_LD | OpcodeKind::C_SW | OpcodeKind::C_SD => {
            Ok(Some(q0_rs1))
        }
        // Quadrant 1
        OpcodeKind::C_ADDI | OpcodeKind::C_ADDIW | OpcodeKind::C_ADDI16SP => Ok(Some(q1_addi_rs1)),
        OpcodeKind::C_SRLI
        | OpcodeKind::C_SRAI
        | OpcodeKind::C_ANDI
        | OpcodeKind::C_SUB
        | OpcodeKind::C_XOR
        | OpcodeKind::C_OR
        | OpcodeKind::C_AND
        | OpcodeKind::C_BEQZ
        | OpcodeKind::C_BNEZ
        | OpcodeKind::C_SUBW
        | OpcodeKind::C_ADDW => Ok(Some(q1_rs1)),
        // Quadrant 2
        OpcodeKind::C_SLLI | OpcodeKind::C_JR | OpcodeKind::C_JALR | OpcodeKind::C_ADD => {
            Ok(Some(q2_rs1))
        }
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u16, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q2_rs2: usize = inst.slice(6, 2) as usize;

    match opkind {
        // Quadrant 0
        OpcodeKind::C_SW | OpcodeKind::C_SD => Ok(Some(q0_rs2)),
        // Quadrant 1
        OpcodeKind::C_SUB
        | OpcodeKind::C_XOR
        | OpcodeKind::C_OR
        | OpcodeKind::C_AND
        | OpcodeKind::C_SUBW
        | OpcodeKind::C_ADDW => Ok(Some(q1_rs2)),
        // Quadrant 2
        OpcodeKind::C_MV | OpcodeKind::C_ADD | OpcodeKind::C_SWSP | OpcodeKind::C_SDSP => {
            Ok(Some(q2_rs2))
        }
        _ => Ok(None),
    }
}

pub fn parse_imm(inst: u16, opkind: &OpcodeKind) -> Result<Option<i32>, DecodingError> {
    let q0_uimm = || (inst.slice(12, 10).set(&[5, 4, 3]) | inst.slice(6, 5).set(&[2, 6])) as i32;
    let q0_uimm_64 = || (inst.slice(12, 10).set(&[5, 4, 3]) | inst.slice(6, 5).set(&[7, 6])) as i32;
    let q0_nzuimm = || inst.slice(12, 5).set(&[5, 4, 9, 8, 7, 6, 2, 3]) as i32;
    let q1_nzuimm =
        || (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
    let q1_nzimm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
        inst.to_signed_nbit(imm16, 6)
    };
    let q1_imm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
        inst.to_signed_nbit(imm16, 6)
    };
    let q1_j_imm = || {
        let imm16 = inst.slice(12, 2).set(&[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]) as i32;
        inst.to_signed_nbit(imm16, 12)
    };
    let q1_b_imm = || {
        let imm16 =
            (inst.slice(6, 2).set(&[7, 6, 2, 1, 5]) | inst.slice(12, 10).set(&[8, 4, 3])) as i32;
        inst.to_signed_nbit(imm16, 9)
    };
    let q1_16sp_nzimm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 6, 8, 7, 5]) | inst.slice(12, 12).set(&[9])) as i32;
        inst.to_signed_nbit(imm16, 10)
    };
    let q1_lui_imm = || {
        let imm16 =
            (inst.slice(6, 2).set(&[16, 15, 14, 13, 12]) | inst.slice(12, 12).set(&[17])) as i32;
        inst.to_signed_nbit(imm16, 18)
    };
    let q2_imm = || (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
    let q2_lwsp_imm =
        || (inst.slice(6, 2).set(&[4, 3, 2, 7, 6]) | inst.slice(12, 12).set(&[5])) as i32;
    let q2_ldsp_imm =
        || (inst.slice(6, 2).set(&[4, 3, 8, 7, 6]) | inst.slice(12, 12).set(&[5])) as i32;
    let q2_swsp_imm = || inst.slice(12, 7).set(&[5, 4, 3, 2, 7, 6]) as i32;
    let q2_sdsp_imm = || inst.slice(12, 7).set(&[5, 4, 3, 8, 7, 6]) as i32;

    match opkind {
        // Quadrant0
        OpcodeKind::C_ADDI4SPN => Ok(Some(q0_nzuimm())),
        OpcodeKind::C_LW | OpcodeKind::C_SW => Ok(Some(q0_uimm())),
        OpcodeKind::C_LD | OpcodeKind::C_SD => Ok(Some(q0_uimm_64())),
        // Quadrant1
        OpcodeKind::C_ADDIW | OpcodeKind::C_LI | OpcodeKind::C_ANDI => Ok(Some(q1_imm())),
        OpcodeKind::C_NOP | OpcodeKind::C_ADDI => Ok(Some(q1_nzimm())),
        OpcodeKind::C_SRLI | OpcodeKind::C_SRAI => Ok(Some(q1_nzuimm())),
        OpcodeKind::C_JAL | OpcodeKind::C_J => Ok(Some(q1_j_imm())),
        OpcodeKind::C_BEQZ | OpcodeKind::C_BNEZ => Ok(Some(q1_b_imm())),
        OpcodeKind::C_LUI => Ok(Some(q1_lui_imm())),
        OpcodeKind::C_ADDI16SP => Ok(Some(q1_16sp_nzimm())),
        // Quadrant2
        OpcodeKind::C_SLLI => Ok(Some(q2_imm())),
        OpcodeKind::C_LWSP => Ok(Some(q2_lwsp_imm())),
        OpcodeKind::C_LDSP => Ok(Some(q2_ldsp_imm())),
        OpcodeKind::C_SWSP => Ok(Some(q2_swsp_imm())),
        OpcodeKind::C_SDSP => Ok(Some(q2_sdsp_imm())),
        _ => Ok(None),
    }
}

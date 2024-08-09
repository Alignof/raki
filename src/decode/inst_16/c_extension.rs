use super::super::{only_rv64, DecodeUtil, DecodingError};
use crate::instruction::c_extension::COpcode;
use crate::Isa;

fn quadrant0(_inst: u16, opmap: u8, isa: Isa) -> Result<COpcode, DecodingError> {
    match opmap {
        0b000 => Ok(COpcode::ADDI4SPN),
        0b010 => Ok(COpcode::LW),
        0b011 => only_rv64(COpcode::LD, isa),
        0b110 => Ok(COpcode::SW),
        0b111 => only_rv64(COpcode::SD, isa),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

fn quadrant1(inst: u16, opmap: u8, isa: Isa) -> Result<COpcode, DecodingError> {
    let sr_flag: u8 = inst.slice(11, 10) as u8;
    let lo_flag: u8 = inst.slice(6, 5) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;
    let bit_12: u8 = inst.slice(12, 12) as u8;

    match opmap {
        0b000 => match mi_flag {
            0b00000 => Ok(COpcode::NOP),
            _ => Ok(COpcode::ADDI),
        },
        0b001 => match isa {
            Isa::Rv32 => Ok(COpcode::JAL),
            Isa::Rv64 => Ok(COpcode::ADDIW),
        },
        0b010 => Ok(COpcode::LI),
        0b011 => match mi_flag {
            0b00010 => Ok(COpcode::ADDI16SP),
            _ => Ok(COpcode::LUI),
        },
        0b100 => match sr_flag {
            0b00 => Ok(COpcode::SRLI),
            0b01 => Ok(COpcode::SRAI),
            0b10 => Ok(COpcode::ANDI),
            0b11 => match bit_12 {
                0b0 => match lo_flag {
                    0b00 => Ok(COpcode::SUB),
                    0b01 => Ok(COpcode::XOR),
                    0b10 => Ok(COpcode::OR),
                    0b11 => Ok(COpcode::AND),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                0b1 => match lo_flag {
                    0b00 => only_rv64(COpcode::SUBW, isa),
                    0b01 => only_rv64(COpcode::ADDW, isa),
                    _ => Err(DecodingError::InvalidOpcode),
                },
                _ => unreachable!(),
            },
            _ => Err(DecodingError::InvalidOpcode),
        },
        0b101 => Ok(COpcode::J),
        0b110 => Ok(COpcode::BEQZ),
        0b111 => Ok(COpcode::BNEZ),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

fn quadrant2(inst: u16, opmap: u8, isa: Isa) -> Result<COpcode, DecodingError> {
    let lo_flag: u8 = inst.slice(6, 2) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;
    let hi_flag: u8 = inst.slice(12, 12) as u8;

    match opmap {
        0b000 => Ok(COpcode::SLLI),
        0b010 => Ok(COpcode::LWSP),
        0b011 => only_rv64(COpcode::LDSP, isa),
        0b100 => match hi_flag {
            0b0 => match lo_flag {
                0b0 => Ok(COpcode::JR),
                _ => Ok(COpcode::MV),
            },
            0b1 => match mi_flag {
                0b0 => Ok(COpcode::EBREAK),
                _ => match lo_flag {
                    0b0 => Ok(COpcode::JALR),
                    _ => Ok(COpcode::ADD),
                },
            },
            _ => Err(DecodingError::InvalidOpcode),
        },
        0b110 => Ok(COpcode::SWSP),
        0b111 => only_rv64(COpcode::SDSP, isa),
        _ => Err(DecodingError::InvalidOpcode),
    }
}

pub fn parse_opcode(inst: u16, isa: Isa) -> Result<COpcode, DecodingError> {
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

pub fn parse_rd(inst: u16, opkind: &COpcode) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rd: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rd: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_wide_rd: usize = inst.slice(11, 7) as usize;
    let q2_rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        COpcode::ADDI4SPN | COpcode::LW | COpcode::LD => Ok(Some(q0_rd)),
        // Quadrant 1
        COpcode::SRLI
        | COpcode::SRAI
        | COpcode::ANDI
        | COpcode::SUB
        | COpcode::XOR
        | COpcode::OR
        | COpcode::AND
        | COpcode::ADDW
        | COpcode::SUBW => Ok(Some(q1_rd)),
        COpcode::LI | COpcode::LUI | COpcode::ADDI | COpcode::ADDIW | COpcode::ADDI16SP => {
            Ok(Some(q1_wide_rd))
        }
        // Quadrant 2
        COpcode::SLLI | COpcode::LWSP | COpcode::LDSP | COpcode::MV | COpcode::ADD => {
            Ok(Some(q2_rd))
        }
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u16, opkind: &COpcode) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_addi_rs1: usize = inst.slice(11, 7) as usize;
    let q2_rs1: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        COpcode::LW | COpcode::LD | COpcode::SW | COpcode::SD => Ok(Some(q0_rs1)),
        // Quadrant 1
        COpcode::ADDI | COpcode::ADDIW | COpcode::ADDI16SP => Ok(Some(q1_addi_rs1)),
        COpcode::SRLI
        | COpcode::SRAI
        | COpcode::ANDI
        | COpcode::SUB
        | COpcode::XOR
        | COpcode::OR
        | COpcode::AND
        | COpcode::BEQZ
        | COpcode::BNEZ
        | COpcode::SUBW
        | COpcode::ADDW => Ok(Some(q1_rs1)),
        // Quadrant 2
        COpcode::SLLI | COpcode::JR | COpcode::JALR | COpcode::ADD => Ok(Some(q2_rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u16, opkind: &COpcode) -> Result<Option<usize>, DecodingError> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q2_rs2: usize = inst.slice(6, 2) as usize;

    match opkind {
        // Quadrant 0
        COpcode::SW | COpcode::SD => Ok(Some(q0_rs2)),
        // Quadrant 1
        COpcode::SUB
        | COpcode::XOR
        | COpcode::OR
        | COpcode::AND
        | COpcode::SUBW
        | COpcode::ADDW => Ok(Some(q1_rs2)),
        // Quadrant 2
        COpcode::JR
        | COpcode::JALR
        | COpcode::MV
        | COpcode::ADD
        | COpcode::SWSP
        | COpcode::SDSP => Ok(Some(q2_rs2)),
        _ => Ok(None),
    }
}

pub fn parse_imm(inst: u16, opkind: &COpcode) -> Result<Option<i32>, DecodingError> {
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
        COpcode::ADDI4SPN => Ok(Some(q0_nzuimm())),
        COpcode::LW | COpcode::SW => Ok(Some(q0_uimm())),
        COpcode::LD | COpcode::SD => Ok(Some(q0_uimm_64())),
        // Quadrant1
        COpcode::ADDIW | COpcode::LI | COpcode::ANDI => Ok(Some(q1_imm())),
        COpcode::NOP | COpcode::ADDI => Ok(Some(q1_nzimm())),
        COpcode::SRLI | COpcode::SRAI => Ok(Some(q1_nzuimm())),
        COpcode::JAL | COpcode::J => Ok(Some(q1_j_imm())),
        COpcode::BEQZ | COpcode::BNEZ => Ok(Some(q1_b_imm())),
        COpcode::LUI => Ok(Some(q1_lui_imm())),
        COpcode::ADDI16SP => Ok(Some(q1_16sp_nzimm())),
        // Quadrant2
        COpcode::SLLI => Ok(Some(q2_imm())),
        COpcode::LWSP => Ok(Some(q2_lwsp_imm())),
        COpcode::LDSP => Ok(Some(q2_ldsp_imm())),
        COpcode::SWSP => Ok(Some(q2_swsp_imm())),
        COpcode::SDSP => Ok(Some(q2_sdsp_imm())),
        _ => Ok(None),
    }
}

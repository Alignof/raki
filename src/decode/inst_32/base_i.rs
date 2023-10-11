use super::super::{only_rv64, DecodeUtil};
use crate::instruction::OpecodeKind;
use crate::Isa;

pub fn parse_opecode(inst: u32, isa: Isa) -> Result<OpecodeKind, (Option<u64>, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct5: u8 = inst.slice(24, 20) as u8;
    let funct6: u8 = inst.slice(31, 26) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            //format!("opecode decoding failed in base, {inst:b}"),
            String::new(),
        ))
    };

    match opmap {
        0b0110111 => Ok(OpecodeKind::LUI),
        0b0010111 => Ok(OpecodeKind::AUIPC),
        0b1101111 => Ok(OpecodeKind::JAL),
        0b1100111 => Ok(OpecodeKind::JALR),
        0b1100011 => match funct3 {
            0b000 => Ok(OpecodeKind::BEQ),
            0b001 => Ok(OpecodeKind::BNE),
            0b100 => Ok(OpecodeKind::BLT),
            0b101 => Ok(OpecodeKind::BGE),
            0b110 => Ok(OpecodeKind::BLTU),
            0b111 => Ok(OpecodeKind::BGEU),
            _ => illegal_inst_exception(),
        },
        0b0000011 => match funct3 {
            0b000 => Ok(OpecodeKind::LB),
            0b001 => Ok(OpecodeKind::LH),
            0b010 => Ok(OpecodeKind::LW),
            0b011 => only_rv64(OpecodeKind::LD, isa),
            0b100 => Ok(OpecodeKind::LBU),
            0b101 => Ok(OpecodeKind::LHU),
            0b110 => only_rv64(OpecodeKind::LWU, isa),
            _ => illegal_inst_exception(),
        },
        0b0100011 => match funct3 {
            0b000 => Ok(OpecodeKind::SB),
            0b001 => Ok(OpecodeKind::SH),
            0b010 => Ok(OpecodeKind::SW),
            0b011 => only_rv64(OpecodeKind::SD, isa),
            _ => illegal_inst_exception(),
        },
        0b0010011 => match funct3 {
            0b000 => Ok(OpecodeKind::ADDI),
            0b001 => match isa {
                Isa::Rv32 => match funct7 {
                    0b0000000 => Ok(OpecodeKind::SLLI),
                    _ => illegal_inst_exception(),
                },
                Isa::Rv64 => match funct6 {
                    0b000000 => Ok(OpecodeKind::SLLI),
                    _ => illegal_inst_exception(),
                },
            },
            0b010 => Ok(OpecodeKind::SLTI),
            0b011 => Ok(OpecodeKind::SLTIU),
            0b100 => Ok(OpecodeKind::XORI),
            0b101 => match isa {
                Isa::Rv32 => match funct7 {
                    0b0000000 => Ok(OpecodeKind::SRLI),
                    0b0100000 => Ok(OpecodeKind::SRAI),
                    _ => illegal_inst_exception(),
                },
                Isa::Rv64 => match funct6 {
                    0b000000 => Ok(OpecodeKind::SRLI),
                    0b010000 => Ok(OpecodeKind::SRAI),
                    _ => illegal_inst_exception(),
                },
            },
            0b110 => Ok(OpecodeKind::ORI),
            0b111 => Ok(OpecodeKind::ANDI),
            _ => illegal_inst_exception(),
        },
        0b0110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => Ok(OpecodeKind::ADD),
                0b0100000 => Ok(OpecodeKind::SUB),
                _ => illegal_inst_exception(),
            },
            0b001 => Ok(OpecodeKind::SLL),
            0b010 => Ok(OpecodeKind::SLT),
            0b011 => Ok(OpecodeKind::SLTU),
            0b100 => Ok(OpecodeKind::XOR),
            0b101 => match funct7 {
                0b0000000 => Ok(OpecodeKind::SRL),
                0b0100000 => Ok(OpecodeKind::SRA),
                _ => illegal_inst_exception(),
            },
            0b110 => Ok(OpecodeKind::OR),
            0b111 => Ok(OpecodeKind::AND),
            _ => illegal_inst_exception(),
        },
        0b0001111 => Ok(OpecodeKind::FENCE),
        0b1110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => match funct5 {
                    0b00000 => Ok(OpecodeKind::ECALL),
                    0b00001 => Ok(OpecodeKind::EBREAK),
                    _ => illegal_inst_exception(),
                },
                _ => illegal_inst_exception(),
            },
            _ => illegal_inst_exception(),
        },
        0b0011011 => match funct3 {
            0b000 => only_rv64(OpecodeKind::ADDIW, isa),
            0b001 => only_rv64(OpecodeKind::SLLIW, isa),
            0b101 => match funct7 {
                0b0000000 => only_rv64(OpecodeKind::SRLIW, isa),
                0b0100000 => only_rv64(OpecodeKind::SRAIW, isa),
                _ => illegal_inst_exception(),
            },
            _ => illegal_inst_exception(),
        },
        0b0111011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => only_rv64(OpecodeKind::ADDW, isa),
                0b0100000 => only_rv64(OpecodeKind::SUBW, isa),
                _ => illegal_inst_exception(),
            },
            0b001 => only_rv64(OpecodeKind::SLLW, isa),
            0b101 => match funct7 {
                0b0000000 => only_rv64(OpecodeKind::SRLW, isa),
                0b0100000 => only_rv64(OpecodeKind::SRAW, isa),
                _ => illegal_inst_exception(),
            },
            _ => illegal_inst_exception(),
        },
        _ => illegal_inst_exception(),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        OpecodeKind::LUI => Ok(Some(rd)),
        OpecodeKind::AUIPC => Ok(Some(rd)),
        OpecodeKind::JAL => Ok(Some(rd)),
        OpecodeKind::JALR => Ok(Some(rd)),
        OpecodeKind::LB => Ok(Some(rd)),
        OpecodeKind::LH => Ok(Some(rd)),
        OpecodeKind::LW => Ok(Some(rd)),
        OpecodeKind::LBU => Ok(Some(rd)),
        OpecodeKind::LHU => Ok(Some(rd)),
        OpecodeKind::ADDI => Ok(Some(rd)),
        OpecodeKind::SLTI => Ok(Some(rd)),
        OpecodeKind::SLTIU => Ok(Some(rd)),
        OpecodeKind::XORI => Ok(Some(rd)),
        OpecodeKind::ORI => Ok(Some(rd)),
        OpecodeKind::ANDI => Ok(Some(rd)),
        OpecodeKind::SLLI => Ok(Some(rd)),
        OpecodeKind::SRLI => Ok(Some(rd)),
        OpecodeKind::SRAI => Ok(Some(rd)),
        OpecodeKind::ADD => Ok(Some(rd)),
        OpecodeKind::SUB => Ok(Some(rd)),
        OpecodeKind::SLL => Ok(Some(rd)),
        OpecodeKind::SLT => Ok(Some(rd)),
        OpecodeKind::SLTU => Ok(Some(rd)),
        OpecodeKind::XOR => Ok(Some(rd)),
        OpecodeKind::SRL => Ok(Some(rd)),
        OpecodeKind::SRA => Ok(Some(rd)),
        OpecodeKind::OR => Ok(Some(rd)),
        OpecodeKind::AND => Ok(Some(rd)),
        OpecodeKind::LWU => Ok(Some(rd)),
        OpecodeKind::LD => Ok(Some(rd)),
        OpecodeKind::ADDIW => Ok(Some(rd)),
        OpecodeKind::SLLIW => Ok(Some(rd)),
        OpecodeKind::SRLIW => Ok(Some(rd)),
        OpecodeKind::SRAIW => Ok(Some(rd)),
        OpecodeKind::ADDW => Ok(Some(rd)),
        OpecodeKind::SUBW => Ok(Some(rd)),
        OpecodeKind::SLLW => Ok(Some(rd)),
        OpecodeKind::SRLW => Ok(Some(rd)),
        OpecodeKind::SRAW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::JALR => Ok(Some(rs1)),
        OpecodeKind::BEQ => Ok(Some(rs1)),
        OpecodeKind::BNE => Ok(Some(rs1)),
        OpecodeKind::BLT => Ok(Some(rs1)),
        OpecodeKind::BGE => Ok(Some(rs1)),
        OpecodeKind::BLTU => Ok(Some(rs1)),
        OpecodeKind::BGEU => Ok(Some(rs1)),
        OpecodeKind::LB => Ok(Some(rs1)),
        OpecodeKind::LH => Ok(Some(rs1)),
        OpecodeKind::LW => Ok(Some(rs1)),
        OpecodeKind::LBU => Ok(Some(rs1)),
        OpecodeKind::LHU => Ok(Some(rs1)),
        OpecodeKind::SB => Ok(Some(rs1)),
        OpecodeKind::SH => Ok(Some(rs1)),
        OpecodeKind::SW => Ok(Some(rs1)),
        OpecodeKind::ADDI => Ok(Some(rs1)),
        OpecodeKind::SLTI => Ok(Some(rs1)),
        OpecodeKind::SLTIU => Ok(Some(rs1)),
        OpecodeKind::XORI => Ok(Some(rs1)),
        OpecodeKind::ORI => Ok(Some(rs1)),
        OpecodeKind::ANDI => Ok(Some(rs1)),
        OpecodeKind::SLLI => Ok(Some(rs1)),
        OpecodeKind::SRLI => Ok(Some(rs1)),
        OpecodeKind::SRAI => Ok(Some(rs1)),
        OpecodeKind::ADD => Ok(Some(rs1)),
        OpecodeKind::SUB => Ok(Some(rs1)),
        OpecodeKind::SLL => Ok(Some(rs1)),
        OpecodeKind::SLT => Ok(Some(rs1)),
        OpecodeKind::SLTU => Ok(Some(rs1)),
        OpecodeKind::XOR => Ok(Some(rs1)),
        OpecodeKind::SRL => Ok(Some(rs1)),
        OpecodeKind::SRA => Ok(Some(rs1)),
        OpecodeKind::OR => Ok(Some(rs1)),
        OpecodeKind::AND => Ok(Some(rs1)),
        OpecodeKind::LWU => Ok(Some(rs1)),
        OpecodeKind::LD => Ok(Some(rs1)),
        OpecodeKind::SD => Ok(Some(rs1)),
        OpecodeKind::ADDIW => Ok(Some(rs1)),
        OpecodeKind::SLLIW => Ok(Some(rs1)),
        OpecodeKind::SRLIW => Ok(Some(rs1)),
        OpecodeKind::SRAIW => Ok(Some(rs1)),
        OpecodeKind::ADDW => Ok(Some(rs1)),
        OpecodeKind::SUBW => Ok(Some(rs1)),
        OpecodeKind::SLLW => Ok(Some(rs1)),
        OpecodeKind::SRLW => Ok(Some(rs1)),
        OpecodeKind::SRAW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::BEQ => Ok(Some(rs2)),
        OpecodeKind::BNE => Ok(Some(rs2)),
        OpecodeKind::BLT => Ok(Some(rs2)),
        OpecodeKind::BGE => Ok(Some(rs2)),
        OpecodeKind::BLTU => Ok(Some(rs2)),
        OpecodeKind::BGEU => Ok(Some(rs2)),
        OpecodeKind::SB => Ok(Some(rs2)),
        OpecodeKind::SH => Ok(Some(rs2)),
        OpecodeKind::SW => Ok(Some(rs2)),
        OpecodeKind::ADD => Ok(Some(rs2)),
        OpecodeKind::SUB => Ok(Some(rs2)),
        OpecodeKind::SLL => Ok(Some(rs2)),
        OpecodeKind::SLT => Ok(Some(rs2)),
        OpecodeKind::SLTU => Ok(Some(rs2)),
        OpecodeKind::XOR => Ok(Some(rs2)),
        OpecodeKind::SRL => Ok(Some(rs2)),
        OpecodeKind::SRA => Ok(Some(rs2)),
        OpecodeKind::OR => Ok(Some(rs2)),
        OpecodeKind::AND => Ok(Some(rs2)),
        OpecodeKind::SD => Ok(Some(rs2)),
        OpecodeKind::ADDW => Ok(Some(rs2)),
        OpecodeKind::SUBW => Ok(Some(rs2)),
        OpecodeKind::SLLW => Ok(Some(rs2)),
        OpecodeKind::SRLW => Ok(Some(rs2)),
        OpecodeKind::SRAW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(
    inst: u32,
    opkind: &OpecodeKind,
    isa: Isa,
) -> Result<Option<i32>, (Option<u64>, String)> {
    let U_type = || (inst.slice(31, 12) << 12) as i32;
    let I_type = || {
        let imm32 = inst.slice(31, 20) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let S_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 0])
            | inst.slice(31, 25).set(&[11, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let B_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 11])
            | inst.slice(31, 25).set(&[12, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 13)
    };
    let J_type = || {
        let imm32 = inst.slice(31, 12).set(&[
            20, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 19, 18, 17, 16, 15, 14, 13, 12,
        ]) as i32;
        inst.to_signed_nbit(imm32, 21)
    };
    let shamt5 = || inst.slice(24, 20) as i32;
    let shamt6 = || inst.slice(25, 20) as i32;

    match opkind {
        OpecodeKind::LUI => Ok(Some(U_type())),
        OpecodeKind::AUIPC => Ok(Some(U_type())),
        OpecodeKind::JAL => Ok(Some(J_type())),
        OpecodeKind::JALR => Ok(Some(I_type())),
        OpecodeKind::BEQ => Ok(Some(B_type())),
        OpecodeKind::BNE => Ok(Some(B_type())),
        OpecodeKind::BLT => Ok(Some(B_type())),
        OpecodeKind::BGE => Ok(Some(B_type())),
        OpecodeKind::BLTU => Ok(Some(B_type())),
        OpecodeKind::BGEU => Ok(Some(B_type())),
        OpecodeKind::LB => Ok(Some(I_type())),
        OpecodeKind::LH => Ok(Some(I_type())),
        OpecodeKind::LW => Ok(Some(I_type())),
        OpecodeKind::LBU => Ok(Some(I_type())),
        OpecodeKind::LHU => Ok(Some(I_type())),
        OpecodeKind::SB => Ok(Some(S_type())),
        OpecodeKind::SH => Ok(Some(S_type())),
        OpecodeKind::SW => Ok(Some(S_type())),
        OpecodeKind::ADDI => Ok(Some(I_type())),
        OpecodeKind::SLTI => Ok(Some(I_type())),
        OpecodeKind::SLTIU => Ok(Some(I_type())),
        OpecodeKind::XORI => Ok(Some(I_type())),
        OpecodeKind::ORI => Ok(Some(I_type())),
        OpecodeKind::ANDI => Ok(Some(I_type())),
        OpecodeKind::SLLI | OpecodeKind::SRLI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        OpecodeKind::SRAI => match isa {
            Isa::Rv32 => Ok(Some(shamt5())), // shamt
            Isa::Rv64 => Ok(Some(shamt6())),
        },
        OpecodeKind::SLLIW => Ok(Some(shamt5())),
        OpecodeKind::SRLIW => Ok(Some(shamt5())),
        OpecodeKind::SRAIW => Ok(Some(shamt5())),
        OpecodeKind::LWU => Ok(Some(I_type())),
        OpecodeKind::LD => Ok(Some(I_type())),
        OpecodeKind::SD => Ok(Some(S_type())),
        OpecodeKind::ADDIW => Ok(Some(I_type())),
        _ => Ok(None),
    }
}

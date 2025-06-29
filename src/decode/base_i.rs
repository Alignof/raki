pub mod bit_32 {
    use super::super::{only_rv64, DecodeUtil, DecodingError};
    use crate::instruction::base_i::BaseIOpcode;
    use crate::Isa;

    #[allow(clippy::too_many_lines)]
    pub fn parse_opcode(inst: u32, isa: Isa) -> Result<BaseIOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let funct5: u8 = u8::try_from(inst.slice(24, 20)).unwrap();
        let funct6: u8 = u8::try_from(inst.slice(31, 26)).unwrap();
        let funct7: u8 = u8::try_from(inst.slice(31, 25)).unwrap();

        match opmap {
            0b011_0111 => Ok(BaseIOpcode::LUI),
            0b001_0111 => Ok(BaseIOpcode::AUIPC),
            0b110_1111 => Ok(BaseIOpcode::JAL),
            0b110_0111 => Ok(BaseIOpcode::JALR),
            0b110_0011 => match funct3 {
                0b000 => Ok(BaseIOpcode::BEQ),
                0b001 => Ok(BaseIOpcode::BNE),
                0b100 => Ok(BaseIOpcode::BLT),
                0b101 => Ok(BaseIOpcode::BGE),
                0b110 => Ok(BaseIOpcode::BLTU),
                0b111 => Ok(BaseIOpcode::BGEU),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b000_0011 => match funct3 {
                0b000 => Ok(BaseIOpcode::LB),
                0b001 => Ok(BaseIOpcode::LH),
                0b010 => Ok(BaseIOpcode::LW),
                0b011 => only_rv64(BaseIOpcode::LD, isa),
                0b100 => Ok(BaseIOpcode::LBU),
                0b101 => Ok(BaseIOpcode::LHU),
                0b110 => only_rv64(BaseIOpcode::LWU, isa),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b010_0011 => match funct3 {
                0b000 => Ok(BaseIOpcode::SB),
                0b001 => Ok(BaseIOpcode::SH),
                0b010 => Ok(BaseIOpcode::SW),
                0b011 => only_rv64(BaseIOpcode::SD, isa),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b001_0011 => match funct3 {
                0b000 => Ok(BaseIOpcode::ADDI),
                0b001 => match isa {
                    Isa::Rv32 => match funct7 {
                        0b000_0000 => Ok(BaseIOpcode::SLLI),
                        _ => Err(DecodingError::InvalidFunct7),
                    },
                    Isa::Rv64 => match funct6 {
                        0b00_0000 => Ok(BaseIOpcode::SLLI),
                        _ => Err(DecodingError::InvalidFunct6),
                    },
                },
                0b010 => Ok(BaseIOpcode::SLTI),
                0b011 => Ok(BaseIOpcode::SLTIU),
                0b100 => Ok(BaseIOpcode::XORI),
                0b101 => match isa {
                    Isa::Rv32 => match funct7 {
                        0b000_0000 => Ok(BaseIOpcode::SRLI),
                        0b010_0000 => Ok(BaseIOpcode::SRAI),
                        _ => Err(DecodingError::InvalidFunct7),
                    },
                    Isa::Rv64 => match funct6 {
                        0b00_0000 => Ok(BaseIOpcode::SRLI),
                        0b01_0000 => Ok(BaseIOpcode::SRAI),
                        _ => Err(DecodingError::InvalidFunct6),
                    },
                },
                0b110 => Ok(BaseIOpcode::ORI),
                0b111 => Ok(BaseIOpcode::ANDI),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b011_0011 => match funct3 {
                0b000 => match funct7 {
                    0b000_0000 => Ok(BaseIOpcode::ADD),
                    0b010_0000 => Ok(BaseIOpcode::SUB),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                0b001 => Ok(BaseIOpcode::SLL),
                0b010 => Ok(BaseIOpcode::SLT),
                0b011 => Ok(BaseIOpcode::SLTU),
                0b100 => Ok(BaseIOpcode::XOR),
                0b101 => match funct7 {
                    0b000_0000 => Ok(BaseIOpcode::SRL),
                    0b010_0000 => Ok(BaseIOpcode::SRA),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                0b110 => Ok(BaseIOpcode::OR),
                0b111 => Ok(BaseIOpcode::AND),
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b111_0011 => match funct3 {
                0b000 => match funct7 {
                    0b000_0000 => match funct5 {
                        0b00000 => Ok(BaseIOpcode::ECALL),
                        0b00001 => Ok(BaseIOpcode::EBREAK),
                        _ => Err(DecodingError::InvalidFunct5),
                    },
                    _ => Err(DecodingError::InvalidFunct7),
                },
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b001_1011 => match funct3 {
                0b000 => only_rv64(BaseIOpcode::ADDIW, isa),
                0b001 => only_rv64(BaseIOpcode::SLLIW, isa),
                0b101 => match funct7 {
                    0b000_0000 => only_rv64(BaseIOpcode::SRLIW, isa),
                    0b010_0000 => only_rv64(BaseIOpcode::SRAIW, isa),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                _ => Err(DecodingError::InvalidFunct3),
            },
            0b011_1011 => match funct3 {
                0b000 => match funct7 {
                    0b000_0000 => only_rv64(BaseIOpcode::ADDW, isa),
                    0b010_0000 => only_rv64(BaseIOpcode::SUBW, isa),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                0b001 => only_rv64(BaseIOpcode::SLLW, isa),
                0b101 => match funct7 {
                    0b000_0000 => only_rv64(BaseIOpcode::SRLW, isa),
                    0b010_0000 => only_rv64(BaseIOpcode::SRAW, isa),
                    _ => Err(DecodingError::InvalidFunct7),
                },
                _ => Err(DecodingError::InvalidFunct3),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    pub fn parse_rd(inst: u32, opkind: &BaseIOpcode) -> Option<usize> {
        let rd: usize = inst.slice(11, 7) as usize;

        // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
        match opkind {
            BaseIOpcode::LUI
            | BaseIOpcode::AUIPC
            | BaseIOpcode::JAL
            | BaseIOpcode::JALR
            | BaseIOpcode::LB
            | BaseIOpcode::LH
            | BaseIOpcode::LW
            | BaseIOpcode::LBU
            | BaseIOpcode::LHU
            | BaseIOpcode::ADDI
            | BaseIOpcode::SLTI
            | BaseIOpcode::SLTIU
            | BaseIOpcode::XORI
            | BaseIOpcode::ORI
            | BaseIOpcode::ANDI
            | BaseIOpcode::SLLI
            | BaseIOpcode::SRLI
            | BaseIOpcode::SRAI
            | BaseIOpcode::ADD
            | BaseIOpcode::SUB
            | BaseIOpcode::SLL
            | BaseIOpcode::SLT
            | BaseIOpcode::SLTU
            | BaseIOpcode::XOR
            | BaseIOpcode::SRL
            | BaseIOpcode::SRA
            | BaseIOpcode::OR
            | BaseIOpcode::AND
            | BaseIOpcode::LWU
            | BaseIOpcode::LD
            | BaseIOpcode::ADDIW
            | BaseIOpcode::SLLIW
            | BaseIOpcode::SRLIW
            | BaseIOpcode::SRAIW
            | BaseIOpcode::ADDW
            | BaseIOpcode::SUBW
            | BaseIOpcode::SLLW
            | BaseIOpcode::SRLW
            | BaseIOpcode::SRAW => Some(rd),
            _ => None,
        }
    }

    pub fn parse_rs1(inst: u32, opkind: &BaseIOpcode) -> Option<usize> {
        let rs1: usize = inst.slice(19, 15) as usize;

        // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
        match opkind {
            BaseIOpcode::JALR
            | BaseIOpcode::BEQ
            | BaseIOpcode::BNE
            | BaseIOpcode::BLT
            | BaseIOpcode::BGE
            | BaseIOpcode::BLTU
            | BaseIOpcode::BGEU
            | BaseIOpcode::LB
            | BaseIOpcode::LH
            | BaseIOpcode::LW
            | BaseIOpcode::LBU
            | BaseIOpcode::LHU
            | BaseIOpcode::SB
            | BaseIOpcode::SH
            | BaseIOpcode::SW
            | BaseIOpcode::ADDI
            | BaseIOpcode::SLTI
            | BaseIOpcode::SLTIU
            | BaseIOpcode::XORI
            | BaseIOpcode::ORI
            | BaseIOpcode::ANDI
            | BaseIOpcode::SLLI
            | BaseIOpcode::SRLI
            | BaseIOpcode::SRAI
            | BaseIOpcode::ADD
            | BaseIOpcode::SUB
            | BaseIOpcode::SLL
            | BaseIOpcode::SLT
            | BaseIOpcode::SLTU
            | BaseIOpcode::XOR
            | BaseIOpcode::SRL
            | BaseIOpcode::SRA
            | BaseIOpcode::OR
            | BaseIOpcode::AND
            | BaseIOpcode::LWU
            | BaseIOpcode::LD
            | BaseIOpcode::SD
            | BaseIOpcode::ADDIW
            | BaseIOpcode::SLLIW
            | BaseIOpcode::SRLIW
            | BaseIOpcode::SRAIW
            | BaseIOpcode::ADDW
            | BaseIOpcode::SUBW
            | BaseIOpcode::SLLW
            | BaseIOpcode::SRLW
            | BaseIOpcode::SRAW => Some(rs1),
            _ => None,
        }
    }

    pub fn parse_rs2(inst: u32, opkind: &BaseIOpcode) -> Option<usize> {
        let rs2: usize = inst.slice(24, 20) as usize;

        // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
        // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
        // FENCE, ECALL, EBREAK
        match opkind {
            BaseIOpcode::BEQ
            | BaseIOpcode::BNE
            | BaseIOpcode::BLT
            | BaseIOpcode::BGE
            | BaseIOpcode::BLTU
            | BaseIOpcode::BGEU
            | BaseIOpcode::SB
            | BaseIOpcode::SH
            | BaseIOpcode::SW
            | BaseIOpcode::ADD
            | BaseIOpcode::SUB
            | BaseIOpcode::SLL
            | BaseIOpcode::SLT
            | BaseIOpcode::SLTU
            | BaseIOpcode::XOR
            | BaseIOpcode::SRL
            | BaseIOpcode::SRA
            | BaseIOpcode::OR
            | BaseIOpcode::AND
            | BaseIOpcode::SD
            | BaseIOpcode::ADDW
            | BaseIOpcode::SUBW
            | BaseIOpcode::SLLW
            | BaseIOpcode::SRLW
            | BaseIOpcode::SRAW => Some(rs2),
            _ => None,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[allow(non_snake_case)]
    pub fn parse_imm(inst: u32, opkind: &BaseIOpcode, isa: Isa) -> Option<i32> {
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
        let shamt5 = || inst.slice(24, 20); // shamt = SHift AMounT
        let shamt6 = || inst.slice(25, 20);

        match opkind {
            // u-type
            BaseIOpcode::LUI | BaseIOpcode::AUIPC => Some(U_type()),
            // j-type
            BaseIOpcode::JAL => Some(J_type()),
            // b-type
            BaseIOpcode::BEQ
            | BaseIOpcode::BNE
            | BaseIOpcode::BLT
            | BaseIOpcode::BGE
            | BaseIOpcode::BLTU
            | BaseIOpcode::BGEU => Some(B_type()),
            // i-type
            BaseIOpcode::JALR
            | BaseIOpcode::LB
            | BaseIOpcode::LH
            | BaseIOpcode::LW
            | BaseIOpcode::LBU
            | BaseIOpcode::LHU
            | BaseIOpcode::ADDI
            | BaseIOpcode::SLTI
            | BaseIOpcode::SLTIU
            | BaseIOpcode::XORI
            | BaseIOpcode::ORI
            | BaseIOpcode::ANDI
            | BaseIOpcode::LWU
            | BaseIOpcode::ADDIW
            | BaseIOpcode::LD => Some(I_type()),
            // s-type
            BaseIOpcode::SD | BaseIOpcode::SB | BaseIOpcode::SH | BaseIOpcode::SW => Some(S_type()),
            BaseIOpcode::SRAI | BaseIOpcode::SLLI | BaseIOpcode::SRLI => match isa {
                Isa::Rv32 => Some(shamt5() as i32), // shamt
                Isa::Rv64 => Some(shamt6() as i32),
            },
            BaseIOpcode::SLLIW | BaseIOpcode::SRLIW | BaseIOpcode::SRAIW => Some(shamt5() as i32),
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_basei {
    #[test]
    #[allow(overflowing_literals)]
    fn basei_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::base_i::BaseIOpcode;
        use crate::OpcodeKind;

        test_32_in_rv64(
            0b1000_0000_0000_0000_0000_0000_1011_0111,
            OpcodeKind::BaseI(BaseIOpcode::LUI),
            Some(1),
            None,
            None,
            Some(0x8000_0000),
        );
        test_32_in_rv64(
            0b0000_0000_0000_0000_0000_0010_1001_0111,
            OpcodeKind::BaseI(BaseIOpcode::AUIPC),
            Some(5),
            None,
            None,
            Some(0),
        );
        test_32_in_rv64(
            0b1111_1111_1001_1111_1111_0000_0110_1111,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
            Some(0),
            None,
            None,
            Some(-8),
        );
        test_32_in_rv64(
            0b1111_1110_0010_0000_1000_1110_1010_0011,
            OpcodeKind::BaseI(BaseIOpcode::SB),
            None,
            Some(1),
            Some(2),
            Some(-3),
        );
        test_32_in_rv64(
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            OpcodeKind::BaseI(BaseIOpcode::ADDI),
            Some(5),
            Some(5),
            None,
            Some(-276),
        );
        test_32_in_rv64(
            0b0000_0000_0000_0000_0000_0000_0111_0011,
            OpcodeKind::BaseI(BaseIOpcode::ECALL),
            None,
            None,
            None,
            None,
        );
        test_32_in_rv64(
            0b0000_0000_0000_0101_0100_1100_0110_0011,
            OpcodeKind::BaseI(BaseIOpcode::BLT),
            None,
            Some(10),
            Some(0),
            Some(24),
        );
        test_32_in_rv64(
            0x0010_0513,
            OpcodeKind::BaseI(BaseIOpcode::ADDI),
            Some(10),
            Some(0),
            None,
            Some(1),
        );
        test_32_in_rv64(
            0x4170_04b3,
            OpcodeKind::BaseI(BaseIOpcode::SUB),
            Some(9),
            Some(0),
            Some(23),
            None,
        );
        test_32_in_rv64(
            0x3307_3983,
            OpcodeKind::BaseI(BaseIOpcode::LD),
            Some(19),
            Some(14),
            None,
            Some(816),
        );
        test_32_in_rv64(
            0x10ec_eb63,
            OpcodeKind::BaseI(BaseIOpcode::BLTU),
            None,
            Some(25),
            Some(14),
            Some(278),
        );
        test_32_in_rv64(
            0x31e1_60ef,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
            Some(1),
            None,
            None,
            Some(90910),
        );
        test_32_in_rv64(
            0x0019_4913,
            OpcodeKind::BaseI(BaseIOpcode::XORI),
            Some(18),
            Some(18),
            None,
            Some(1),
        );
        test_32_in_rv64(
            0x00a9_3933,
            OpcodeKind::BaseI(BaseIOpcode::SLTU),
            Some(18),
            Some(18),
            Some(10),
            None,
        );
    }
}

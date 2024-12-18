pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::zicsr_extension::ZicsrOpcode;

    pub fn parse_opcode(inst: u32) -> Result<ZicsrOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();

        match opmap {
            0b111_0011 => match funct3 {
                0b001 => Ok(ZicsrOpcode::CSRRW),
                0b010 => Ok(ZicsrOpcode::CSRRS),
                0b011 => Ok(ZicsrOpcode::CSRRC),
                0b101 => Ok(ZicsrOpcode::CSRRWI),
                0b110 => Ok(ZicsrOpcode::CSRRSI),
                0b111 => Ok(ZicsrOpcode::CSRRCI),
                _ => Err(DecodingError::InvalidFunct3),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, opkind: &ZicsrOpcode) -> Option<usize> {
        let rd: usize = inst.slice(11, 7) as usize;

        match opkind {
            ZicsrOpcode::CSRRW
            | ZicsrOpcode::CSRRS
            | ZicsrOpcode::CSRRC
            | ZicsrOpcode::CSRRWI
            | ZicsrOpcode::CSRRSI
            | ZicsrOpcode::CSRRCI => Some(rd),
        }
    }

    pub fn parse_rs1(inst: u32, opkind: &ZicsrOpcode) -> Option<usize> {
        let rs1: usize = inst.slice(19, 15) as usize;

        // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
        match opkind {
            ZicsrOpcode::CSRRW | ZicsrOpcode::CSRRS | ZicsrOpcode::CSRRC => Some(rs1),
            _ => None,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs2(inst: u32, opkind: &ZicsrOpcode) -> Option<usize> {
        let csr: usize = inst.slice(31, 20) as usize;

        match opkind {
            ZicsrOpcode::CSRRW
            | ZicsrOpcode::CSRRS
            | ZicsrOpcode::CSRRC
            | ZicsrOpcode::CSRRWI
            | ZicsrOpcode::CSRRSI
            | ZicsrOpcode::CSRRCI => Some(csr),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn parse_imm(inst: u32, opkind: &ZicsrOpcode) -> Option<i32> {
        let uimm: u32 = inst.slice(19, 15);
        match opkind {
            ZicsrOpcode::CSRRWI | ZicsrOpcode::CSRRSI | ZicsrOpcode::CSRRCI => Some(uimm as i32),
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_zicsr {
    #[test]
    #[allow(overflowing_literals)]
    fn zicsr_decode_test() {
        use crate::instruction::zicsr_extension::ZicsrOpcode;
        use crate::{Decode, Isa, OpcodeKind};

        let test_32 = |inst_32: u32,
                       op: OpcodeKind,
                       rd: Option<usize>,
                       rs1: Option<usize>,
                       rs2: Option<usize>,
                       imm: Option<i32>| {
            let op_32 = inst_32.parse_opcode(Isa::Rv64).unwrap();
            assert!(matches!(&op_32, op));
            assert_eq!(inst_32.parse_rd(&op_32).unwrap(), rd);
            assert_eq!(inst_32.parse_rs1(&op_32).unwrap(), rs1);
            assert_eq!(inst_32.parse_rs2(&op_32).unwrap(), rs2);
            assert_eq!(inst_32.parse_imm(&op_32, Isa::Rv64).unwrap(), imm);
        };

        test_32(
            0b0001_0000_0000_1000_0011_0000_0111_0011,
            OpcodeKind::Zicsr(ZicsrOpcode::CSRRC),
            Some(0),     // rd
            Some(16),    // rs1
            Some(0x100), // csr
            None,
        );
        test_32(
            0b0001_0000_0000_1000_0010_0000_0111_0011,
            OpcodeKind::Zicsr(ZicsrOpcode::CSRRS),
            Some(0),     // rd
            Some(16),    // rs1
            Some(0x100), // csr
            None,
        );
        test_32(
            0b0001_0000_0000_0001_0110_0000_0111_0011,
            OpcodeKind::Zicsr(ZicsrOpcode::CSRRSI),
            Some(0),     // rd
            None,        // rs1
            Some(0x100), // csr
            Some(2),     // imm
        );
    }
}

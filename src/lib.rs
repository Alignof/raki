//! raki
//!
//! `raki` is a RISC-V instruction decoder written in Rust.
//!
//! - Both 32/64bit support.
//! - Support `rv32/64imac`, `Zicsr`, `Zifencei` extensions.
//! - Implement Display trait for formatting.
//!
//! # Usage
//! Call the `decode` as u16/u32 method.
//! ```
//! use raki::{BaseIOpcode, Decode, Instruction, Isa, OpcodeKind};
//!
//! fn example() {
//!     let inst_bytes: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
//!     let inst: Instruction = match inst_bytes.decode(Isa::Rv32) {
//!         Ok(inst) => inst,
//!         Err(e) => panic!("decoding failed due to {e:?}"),
//!     };
//!
//!     assert_eq!(inst.opc, OpcodeKind::BaseI(BaseIOpcode::ADDI));
//!     println!("{inst}");
//! }
//! // --output--
//! // addi t0, t0, -276
//! ```

mod decode;
mod instruction;

// re-export
pub use crate::decode::{Decode, DecodingError};
pub use crate::instruction::{
    a_extension::AOpcode, base_i::BaseIOpcode, c_extension::COpcode, m_extension::MOpcode,
    priv_extension::PrivOpcode, zicsr_extension::ZicsrOpcode, zifencei_extension::ZifenceiOpcode,
    InstFormat, Instruction, OpcodeKind,
};

/// Target isa.
#[derive(Copy, Clone)]
pub enum Isa {
    /// 32 bit architecture.
    Rv32,
    /// 64 bit architecture.
    Rv64,
}

/// RISC-V extensions
#[derive(Debug)]
enum Extensions {
    /// Base Integer Instruction Set
    BaseI,
    /// Integer Multiplication and Division
    M,
    /// Atomic Instructions
    A,
    /// Compressed Instructions
    C,
    /// Instruction-Fetch Fence
    Zifencei,
    /// Control and Status Register Instructions
    Zicsr,
    /// Base Counters and Timers
    Zicntr,
    /// Privileged Instructions
    Priv,
}

#[cfg(test)]
mod tests {
    #[test]
    fn display_32bit_test() {
        use crate::decode::Decode;
        use crate::instruction::Instruction;
        use crate::Isa;

        let instructions: [u32; 8] = [
            0b1110_1110_1100_0010_1000_0010_1001_0011,
            0b110_1001_1000_0001_1000_0111_1001_0011,
            0b1_1100_1110_0100_0010_0110_1010_1111,
            0b1111_0101_0000_0000_0000_0000_1111,
            0b1000_1111_0100_0010_0111_1010_1111,
            0b111_0011,
            0b110_0101_1111_0000_0011_0000_1110_1111,
            0b110_1001_0111,
        ];

        for inst in &instructions {
            let inst: Instruction = match inst.decode(Isa::Rv32) {
                Ok(inst) => inst,
                Err(e) => panic!("decoding failed due to {e:?}"),
            };

            println!("{inst}");
        }
    }

    #[test]
    fn display_16bit_test() {
        use crate::decode::Decode;
        use crate::instruction::Instruction;
        use crate::Isa;

        let instructions: [u16; 8] = [
            0b1110_0100_0010_0110,
            0b110_1100_0000_0100,
            0b1110_0101_0001_0001,
            0b1000_0101_0010_0110,
            0b1011_0111_1111_0101,
            0b11_0000,
            0b1000_0000_1000_0010,
            0b11_0111_1111_1101,
        ];

        for inst in &instructions {
            let inst: Instruction = match inst.decode(Isa::Rv64) {
                Ok(inst) => inst,
                Err(e) => panic!("decoding failed due to {e:?}"),
            };

            println!("{inst}");
        }
    }

    #[test]
    fn inst_eq_test() {
        use crate::decode::Decode;
        use crate::instruction::{base_i::BaseIOpcode, InstFormat, Instruction, OpcodeKind};
        use crate::Isa;

        assert_eq!(
            0b1111_1111_1001_1111_1111_0000_0110_1111_u32.decode(Isa::Rv64),
            Ok(Instruction {
                opc: OpcodeKind::BaseI(BaseIOpcode::JAL),
                rd: Some(0),
                rs1: None,
                rs2: None,
                imm: Some(-8),
                inst_format: InstFormat::Jformat,
            })
        );

        assert_eq!(
            0b1111_1111_1001_1111_1111_0000_0110_1111_u32
                .decode(Isa::Rv64)
                .unwrap()
                .opc,
            OpcodeKind::BaseI(BaseIOpcode::JAL),
        )
    }
}

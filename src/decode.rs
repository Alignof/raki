mod inst_16;
mod inst_32;

use crate::instruction::{Extensions, Instruction, OpcodeKind};
use crate::Isa;

fn only_rv64(opcode: OpcodeKind, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    match isa {
        Isa::Rv32 => Err(DecodingError::OnlyRv64Inst),
        Isa::Rv64 => Ok(opcode),
    }
}

/// Error kind
#[derive(Debug)]
pub enum DecodingError {
    /// 32bit instructions are expected, but it is compressed instruction.
    Not16BitInst,
    /// Compressed instructions are expected, but it is 32bit length.
    Not32BitInst,
    /// It has unexpected Funct3 value.
    IllegalFunct3,
    /// It has unexpected Funct5 value.
    IllegalFunct5,
    /// It has unexpected Funct6 value.
    IllegalFunct6,
    /// It has unexpected Funct7 value.
    IllegalFunct7,
    /// Has an opcode that cannot be decoded.
    IllegalOpcode,
    /// This instruction is only for Rv64 but appeared at Rv32.
    OnlyRv64Inst,
}

pub trait Decode {
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError>;
    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError>;
    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError>;
}

trait DecodeUtil {
    fn slice(self, end: u32, start: u32) -> Self;
    fn set(self, mask: &[u32]) -> u32;
    fn extension(self) -> Extensions;
    fn to_signed_nbit(&self, imm32: i32, bit_size: u32) -> i32 {
        let imm32 = imm32 & (2_i32.pow(bit_size) - 1);
        if imm32 >> (bit_size - 1) & 0x1 == 1 {
            imm32 - 2_i32.pow(bit_size)
        } else {
            imm32
        }
    }
}

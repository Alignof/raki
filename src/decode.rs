mod inst_16;
mod inst_32;

use crate::instruction::{Extensions, Instruction, OpecodeKind};
use crate::Isa;

pub fn only_rv64(opcode: OpecodeKind, isa: Isa) -> Result<OpecodeKind, (Option<u64>, String)> {
    match isa {
        Isa::Rv32 => Err((
            None,
            "This instruction is only available on rv64".to_string(),
        )),
        Isa::Rv64 => Ok(opcode),
    }
}

pub trait Decode {
    fn decode(&self, isa: Isa) -> Result<Instruction, (Option<u64>, String)>;
    fn parse_opecode(self, isa: Isa) -> Result<OpecodeKind, (Option<u64>, String)>;
    fn parse_rd(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)>;
    fn parse_rs1(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)>;
    fn parse_rs2(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u64>, String)>;
    fn parse_imm(
        self,
        opkind: &OpecodeKind,
        isa: Isa,
    ) -> Result<Option<i32>, (Option<u64>, String)>;
}

pub trait DecodeUtil {
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

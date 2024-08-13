//! Implementation of decoder.

mod inst_16;
mod inst_32;

use crate::instruction::{Instruction, Opcode, OpcodeKind};
use crate::{Extensions, Isa};

/// Return Err if given opcode is only available on Rv64.
fn only_rv64<T: Opcode>(opcode: T, isa: Isa) -> Result<T, DecodingError> {
    match isa {
        Isa::Rv32 => Err(DecodingError::OnlyRv64Inst),
        Isa::Rv64 => Ok(opcode),
    }
}

/// Cause of decoding error.
///
/// # Example
/// ```
/// use raki::Isa;
/// use raki::decode::{Decode, DecodingError};
/// use raki::instruction::Instruction;
///
/// // try to decode illegal instruction.
/// let illegal_inst: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
/// if let Err(error) = illegal_inst.decode(Isa::Rv64) {
///     assert!(matches!(error, DecodingError::InvalidOpcode));
/// }
///
/// // try to decode rv64 instruction on rv32 environment.
/// let rv64_inst: u32 = 0b100000000100010011010000100011;
/// if let Err(error) = rv64_inst.decode(Isa::Rv32) {
///     assert!(matches!(error, DecodingError::OnlyRv64Inst));
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum DecodingError {
    /// 32bit instructions are expected, but it is compressed instruction.
    Not16BitInst,
    /// Compressed instructions are expected, but it is 32bit length.
    Not32BitInst,
    /// It has unexpected Funct3 value.
    InvalidFunct3,
    /// It has unexpected Funct5 value.
    InvalidFunct5,
    /// It has unexpected Funct6 value.
    InvalidFunct6,
    /// It has unexpected Funct7 value.
    InvalidFunct7,
    /// Has an opcode that cannot be decoded.
    InvalidOpcode,
    /// This instruction is included in the unknown extension.
    UnknownExtension,
    /// Illegal instruction (e.g. all zero value instruction)
    IllegalInstruction,
    /// This instruction is only for Rv64 but appeared at Rv32.
    OnlyRv64Inst,
}

/// A trait to decode an instruction from u16/u32.
/// This trait provides public api.
///
/// # Usage
/// `decode` method is implemented for u16/u32.
/// thus, just call `decode` as method of u16/u32.
/// ```
/// use raki::Isa;
/// use raki::decode::Decode;
///
/// let inst: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
/// println!("{:?}", inst.decode(Isa::Rv64));
/// ```
pub trait Decode {
    /// Decode an instruction from u16/u32.
    ///
    /// # Errors
    /// It will throws an error if target bytes is invalid for decoding.
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError>;

    /// Parse opcode.
    ///
    /// # Errors
    /// It will throws an error if opcode is unknown.
    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError>;

    /// Parse destination register.
    ///
    /// # Errors
    /// It will throws an error if rd is invalid.
    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;

    /// Parse source register 1.
    ///
    /// # Errors
    /// It will throws an error if rs1 is invalid.
    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;

    /// Parse source register 2.
    ///
    /// # Errors
    /// It will throws an error if rs2 is invalid.
    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;

    /// Parse immediate.
    ///
    /// # Errors
    /// It will throws an error if immediate is invalid.
    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError>;
}

/// A trait to help decoding.
/// This trait provides private api.
trait DecodeUtil {
    /// Obtains bits in a specified range.
    /// The range is `[end, start]`.
    /// ```ignore
    /// use raki::decode::DecodeUtil;
    /// let bit = 0b0101_0101_1001;
    /// let sliced = bit.slice(5, 2);
    /// assert_eq!(sliced, 0b1_0110);
    /// ```
    /// # Arguments
    /// * `end` - end of range.
    /// * `start` - start of range.
    fn slice(self, end: u32, start: u32) -> Self;

    /// The values of the bits of Self are set to the array value positions in order from the highest to the lowest.
    /// ```ignore
    /// use raki::decode::DecodeUtil;
    /// let bit: u32 = 0b1010_1101;
    /// let sliced = bit.set(&[7, 5, 3, 2, 0, 6, 4, 1]);
    /// assert_eq!(sliced, 0b1111_1000);
    /// ```
    /// # Arguments
    /// * `mask` - It contain the bit order.
    fn set(self, mask: &[u32]) -> u32;

    /// Parse extension from a u16/u32 value.
    ///
    /// # Errors
    /// It will throws `UnknownExtension` if the extension is unsupported.
    fn parse_extension(self) -> Result<Extensions, DecodingError>;

    /// Convert i32 to a sign-extended any size number.
    /// # Arguments
    /// * `imm32` - The value to be converted.
    /// * `bit_size` - Bit width to be converted.
    fn to_signed_nbit(&self, imm32: i32, bit_size: u32) -> i32 {
        let imm32 = imm32 & (2_i32.pow(bit_size) - 1);
        if imm32 >> (bit_size - 1) & 0x1 == 1 {
            imm32 - 2_i32.pow(bit_size)
        } else {
            imm32
        }
    }
}

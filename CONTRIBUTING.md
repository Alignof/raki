# Contribution Guide

## What can I do?
Any contribution is welcome.
- Correction typo in document.
- Pull request for new extension supporting
- Point out the lack of API
- Correction of English expressions (If you are better at English than DeepL, here's your chance to be a contributor!)

## How to add a supporting extension?
Zicntr extension as an example.  
See: [https://github.com/Alignof/raki/pull/16](https://github.com/Alignof/raki/pull/16)

### Add the definition of an instruction
First, list the instructions that the extension has in an Enum.
```sh
any_editor_command src/instruction/zicntr_extension.rs
```

```rust
/// Insturctions in Zicntr Extension.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ZicntrOpcode {
    // For Rv32, these instructions indicate lower 32 bits.
    // For Rv64, these instructions do not exist.
    RDCYCLE_H,
    RDTIME_H,
    RDINSTRET_H,

    // For Rv32, these instructions indicate upper 32 bits.
    // For Rv64, these instructions can access the full 64-bit CSRs directly.
    RDCYCLE,
    RDTIME,
    RDINSTRET,
}
```

And implement traits `Display` and `Opcode`.
```rust
impl Display for ZicntrOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ZicntrOpcode::RDCYCLE_H => write!(f, "rdcycleh"),
            ZicntrOpcode::RDTIME_H => write!(f, "rdtimeh"),
            ZicntrOpcode::RDINSTRET_H => write!(f, "rdinstreth"),
            ZicntrOpcode::RDCYCLE => write!(f, "rdcycle"),
            ZicntrOpcode::RDTIME => write!(f, "rdtime"),
            ZicntrOpcode::RDINSTRET => write!(f, "rdinstret"),
        }
    }
}

impl Opcode for ZicntrOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            ZicntrOpcode::RDCYCLE_H
            | ZicntrOpcode::RDTIME_H
            | ZicntrOpcode::RDINSTRET_H
            | ZicntrOpcode::RDCYCLE
            | ZicntrOpcode::RDTIME
            | ZicntrOpcode::RDINSTRET => InstFormat::CSRcntrformat,
        }
    }
}
```

In this case, there are no appropriate exising format.  
Thus, I added new format to `InstFormat`.

### Implement decoding.
Since the instruction to be added is a 32-bit instruction (not compressed), add the file to the following directory.
```sh
any_editor_command src/decode/inst_32/zicntr_extension.rs
```

```rust
pub fn parse_opcode(inst: u32) -> Result<ZicntrOpcode, DecodingError> {
    let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
    let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
    let csr_num: u16 = u16::try_from(inst.slice(20, 31)).unwrap();

    match opmap {
        0b111_0011 => match funct3 {
            0b010 => match csr_num {
                0xc00 => Ok(ZicntrOpcode::RDCYCLE),
                0xc01 => Ok(ZicntrOpcode::RDTIME),
                0xc02 => Ok(ZicntrOpcode::RDINSTRET),
                0xc80 => Ok(ZicntrOpcode::RDCYCLE_H),
                0xc81 => Ok(ZicntrOpcode::RDTIME_H),
                0xc82 => Ok(ZicntrOpcode::RDINSTRET_H),
                _ => Err(DecodingError::InvalidOpcode),
            },
            _ => Err(DecodingError::InvalidFunct3),
        },
        _ => Err(DecodingError::InvalidOpcode),
    }
}
...
```

### Add extension definition
Add extension definition to lib.rs
```rust
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
```

and add to `OpcodeKind`.
```rust
/// Extension type and Instruction name.
#[derive(Debug, PartialEq)]
pub enum OpcodeKind {
    /// Base Integer Instruction Set
    BaseI(BaseIOpcode),
    /// Integer Multiplication and Division
    M(MOpcode),
    /// Atomic Instructions
    A(AOpcode),
    /// Compressed Instructions
    C(COpcode),
    /// Instruction-Fetch Fence,
    Zifencei(ZifenceiOpcode),
    /// Control and Status Register Instructions
    Zicsr(ZicsrOpcode),
    /// Base Counters and Timers
    Zicntr(ZicntrOpcode),
    /// Privileged Instructions
    Priv(PrivOpcode),
}
```

### Add match pattern to Decode trait functions
When you add the extension definitions, you'll see that error occur that don't cover patterns in the `Decode` trait.
```diff
 fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError> {
     let extension = self.parse_extension();
 
     match extension {
         Ok(Extensions::BaseI) => Ok(OpcodeKind::BaseI(base_i::parse_opcode(self, isa)?)),
         Ok(Extensions::M) => Ok(OpcodeKind::M(m_extension::parse_opcode(self, isa)?)),
         Ok(Extensions::A) => Ok(OpcodeKind::A(a_extension::parse_opcode(self, isa)?)),
         Ok(Extensions::Zifencei) => Ok(OpcodeKind::Zifencei(zifencei_extension::parse_opcode(
             self,
         )?)),
         Ok(Extensions::Zicsr) => Ok(OpcodeKind::Zicsr(zicsr_extension::parse_opcode(self)?)),
+        Ok(Extensions::Zicntr) => Ok(OpcodeKind::Zicntr(zicntr_extension::parse_opcode(self)?)),
         Ok(Extensions::Priv) => Ok(OpcodeKind::Priv(priv_extension::parse_opcode(self)?)),
         Ok(Extensions::C) => Err(DecodingError::Not32BitInst),
         Err(decoding_err) => Err(decoding_err),
     }
 }
 ...
```

### Implement extension parsing
The parse_extension is used to determine which extension the instruction belongs to.
```rust
fn parse_extension(self) -> Result<Extensions, DecodingError> {
    // ...
    let csr: u16 = u16::try_from(self.slice(31, 20)).unwrap();

    match opmap {
        // ...
        0b111_0011 => match funct3 {
            0b000 => match funct7 {
                0b000_0000 => Ok(Extensions::BaseI),
                _ => Ok(Extensions::Priv),
            },
            0b010 => match csr {
                // csrrs rd, (cycle|time|instret), zero
                0xc00..=0xc02 | 0xc80..=0xc82 => Ok(Extensions::Zicntr),
                _ => Ok(Extensions::Zicsr),
            },
            _ => Ok(Extensions::Zicsr),
        },
        _ => Ok(Extensions::BaseI),
    }
}
```

### Re-export
Re-export the definition in lib.rs.
```rust
// re-export definition
pub use crate::instruction::{
    a_extension::AOpcode, base_i::BaseIOpcode, c_extension::COpcode, m_extension::MOpcode,
    priv_extension::PrivOpcode, zicsr_extension::ZicsrOpcode, zifencei_extension::ZifenceiOpcode,
    InstFormat, Instruction, OpcodeKind,
};
```

### Add unit tests.
Add unit tests to decode/inst\_32/xxx_extension.rs.
```rust
#[cfg(test)]
#[allow(unused_variables)]
mod test_zicntr {
    #[test]
    #[allow(overflowing_literals)]
    fn zicntr_decode_test() {
        use super::*;
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
            0b1100_0000_0001_0000_0010_0111_1111_0011,
            OpcodeKind::Zicntr(ZicntrOpcode::RDTIME),
            Some(15),
            None,
            None,
            None,
        )
    }
}
```

### Address clippy warnings
```
$ cargo clippy # pedantic is enable by default.
```

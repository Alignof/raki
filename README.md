# raki
[![Rust](https://github.com/Alignof/raki/actions/workflows/rust.yml/badge.svg)](https://github.com/Alignof/raki/actions/workflows/rust.yml)  
RISC-V instruction decoder written in Rust.

- Both 32/64bit support.
- Support `rv32/64imac`, `Zicsr`, `Zifencei` extensions.
- Implement Display trait for formatting.

## Usage
Call the `decode` as u16/u32 method.
```rust
use raki::{BaseIOpcode, Decode, Instruction, Isa, OpcodeKind};

fn main() {
    let inst_bytes: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
    let inst: Instruction = match inst_bytes.decode(Isa::Rv32) {
        Ok(inst) => inst,
        Err(e) => panic!("decoding failed due to {e:?}"),
    };

    assert_eq!(inst.opc, OpcodeKind::BaseI(BaseIOpcode::ADDI));
    println!("{inst}");
}
// --output--
// addi t0, t0, -276
```

## Support
- [x] BaseI (RV32I, RV64I)
- [x] M
- [x] A
- [ ] D
- [ ] G
- [ ] Q
- [x] C
- [ ] B
- [ ] P
- [ ] V
- [ ] H
- [x] Zicsr
- [x] Zifencei
- [ ] Priv (Now only supports `mret`, `sret`, `wfi`, `sfence.vma`)

## License
This crate is licensed under MIT.  
See `LICENSE` for details.

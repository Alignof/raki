# raki
RISC-V instruction decoder written in Rust.

- Both 32/64bit support.
- Support `rv32/64imac`, `Zicsr`, `Zifencei` extensions.
- Implement Display trait for formatting.

## Usage
Call the `decode` as u16/u32 method.
```rust
use raki::Isa;
use raki::decode::Decode;
use raki::instruction::Instruction;

let inst: u32 = 0b1110_1110_1100_0010_1000_0010_1001_0011;
let inst: Instruction = match inst.decode(Isa::Rv32) {
    Ok(inst) => inst,
    Err(e) => panic!("decoding failed due to {e:?}"),
};
println!("{inst}");
// --output--
// addi t0, t0, -276
```

## License
This crate is licensed under MIT.
See `LICENSE` for details.

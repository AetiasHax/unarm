# unarm

Disassembler for the ARM instruction set, inspired by [ppc750cl](https://github.com/encounter/ppc750cl).

It only supports ARMv5TE, but other versions may be implemented in the future.

## Contents

- [ARM disassembler](#arm-disassembler)
  - [Performance](#performance)
  - [Usage](#usage)
- [Thumb disassembler](#thumb-disassembler)
  - [Performance](#performance-1)
  - [Usage](#usage-1)
  - [32-bit instructions](#32-bit-instructions)

## ARM disassembler

The [`/disasm/v5te/arm/`](/disasm/src/v5te/arm/) module can disassemble ARM instructions in the ARMv5TE instruction set.

- It is generated from [`/specs/v5te/arm.yaml`](/specs/v5te/arm.yaml) by the [`/generator/`](/generator/) module.
- It accepts all 2^32 possible instructions without returning errors.
- No promises that the output is 100% correct.
  - Some illegal instructions may not be parsed as illegal.
  - Some instructions may not stringify correctly.
  - (more, probably)

### Performance

Tested on all 2^32 ARM instructions using the [`/fuzz/`](/fuzz/) module on a single thread:

- Intel Core i7-8700: 140 million insn/s (~534 MB/s)

### Usage

```rust
use unarm::{args::*, v5te::arm::{Ins, Opcode}};

let ins = Ins::new(0xe5902268);
assert_eq!(ins.op, Opcode::Ldr);

let parsed = ins.parse();
assert_eq!(
    parsed.args[0],
    Argument::Reg(Reg { reg: Register::R2, deref: false, writeback: false })
);
assert_eq!(
    parsed.args[1],
    Argument::Reg(Reg { reg: Register::R0, deref: true, writeback: false })
);
assert!(matches!(
    parsed.args[2],
    Argument::OffsetImm(OffsetImm { value: 0x268, post_indexed: false })
));
assert_eq!(parsed.to_string(), "ldr r2, [r0, #0x268]");
```

## Thumb disassembler

The [`/disasm/v5te/thumb/`](/disasm/src/v5te/thumb/) module can disassemble Thumb instructions in the ARMv5TE instruction set.

- It is generated from [`specs/v5te/thumb.yaml`](/specs/v5te/thumb.yaml) by the [`/generator/`](/generator/) module.
- It accepts all 2^16 possible instructions without returning errors.
- The output should be more correct than ARM, but still no promises.

### Performance

Tested on all 2^16 ARM instructions using the [`/fuzz/`](/fuzz/) module on a single thread, averaged after 100,000 iterations:

- Intel Core i7-8700: 256 million insn/s (~488 MB/s)

### Usage

```rust
use unarm::{args::*, v5te::thumb::{Ins, Opcode}};

let ins = Ins::new(0x6081);
assert_eq!(ins.op, Opcode::StrI);

let parsed = ins.parse();
assert_eq!(
    parsed.args[0],
    Argument::Reg(Reg { reg: Register::R1, deref: false, writeback: false })
);
assert_eq!(
    parsed.args[1],
    Argument::Reg(Reg { reg: Register::R0, deref: true, writeback: false })
);
assert!(matches!(
    parsed.args[2],
    Argument::OffsetImm(OffsetImm { value: 0x8, post_indexed: false })
));
assert_eq!(parsed.to_string(), "str r1, [r0, #0x8]");
```

### 32-bit instructions

Thumb uses 16-bit instructions, trading a subset of ARM instructions for smaller code size. However, this leaves little room
for BL/BLX target offsets. This would mean that function calls further than ±1KB away would only be possible using
a register as target address, which could take 3 or 4 instructions in total.

To solve this, Thumb includes "32-bit" BL and BLX instructions with a maximum offset of ±4MB. In truth, these are just two
16-bit instructions strung together. For BL, we call them `Opcode::BlH` and `Opcode::Bl`. For BLX, we call them `Opcode::BlH`
and `Opcode::BlxI`. The first instruction is the same for BL and BLX.

To tell if an instruction needs to be combined, you can use `Ins::is_half_bl(&self)`, which simply checks if the opcode is
`Opcode::BlH`. To combine two instructions into a BL/BLX, use `ParsedIns::combine_bl(&self, second: &Self)`.

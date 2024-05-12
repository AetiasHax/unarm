# armv5te

Instruction decoder for the ARMv5TE instruction set, inspired by [ppc750cl](https://github.com/encounter/ppc750cl).

## Contents

- [ARMv5TE](#armv5te-1)
- [ARM disassembler](#arm-disassembler)
  - [Usage](#usage)
  - [Performance](#performance)

## ARM disassembler

The [`/disasm/`](/disasm/) module can disassemble ARM instructions in the ARMv5TE instruction set.

- It is generated from [`arm.yaml`](/arm.yaml) by the [`/generator/`](/generator/) module.
- It accepts all 2^32 possible instructions without returning errors.
- No promises that the output is 100% correct.
  - Some illegal instructions may not be parsed as illegal.
  - Some instructions may not stringify correctly.
  - (more, probably)

### Usage

```rust
use armv5te::arm::{Argument, Ins, Opcode, ParsedIns, Reg};

let ins = Ins::new(0xe5902268);
assert_eq!(ins.op, Opcode::Ldr);

let parsed = ParsedIns::parse(ins);
assert_eq!(parsed.args[0], Argument::Reg(Reg::R2));
assert_eq!(parsed.args[1], Argument::RegDeref(Reg::R0));
assert!(matches!(parsed.args[2], Argument::Offset((0x268, _))));
assert_eq!(parsed.to_string(), "ldr r2, [r0, #0x268]");
```

### Performance

Tested on all 2^32 ARM instrutions using the [`/fuzz/`](/fuzz/) module:

- Intel Core i7-8700
  - 12 threads: 11M insn/s (~44 MB/s) per thread
  - 6 threads: 16M insn/s (~64 MB/s) per thread
  - 1 thread: 22M insn/s (~88 MB/s)

# armv5te

Instruction decoder for the ARMv5TE instruction set, inspired by [ppc750cl](https://github.com/encounter/ppc750cl).

## Disassembler

The [`/disasm/`](/disasm/) module can disassemble ARM instructions in the ARMv5TE instruction set.

- It accepts all 2^32 possible instructions without returning errors.
- No promises that the output is 100% correct.
  - Some illegal instructions may not be parsed as illegal.

### Usage

```rust
use armv5te::arm::{Argument, Ins, Opcode, ParsedIns, Reg};

let ins = Ins::new(0xe5902268);
assert_eq!(ins.op, Opcode::Ldr);

let parsed = ParsedIns::parse(ins);
assert_eq!(parsed.args[0], Argument::Reg(Reg::R2));
assert_eq!(parsed.args[1], Argument::RegDeref(Reg::R0));
assert_eq!(parsed.args[2], Argument::Offset((0x268, 12)));
assert_eq!(parsed.to_string(), "ldr r2, [r0, #0x268]");
```

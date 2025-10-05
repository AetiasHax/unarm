# unarm

Parser for the ARM instruction set, inspired by [powerpc-rs](https://github.com/encounter/powerpc-rs). It currently supports
the following versions:

- ARMv4
- ARMv4T
- ARMv5T
- ARMv5TE
- ARMv5TEJ
- ARMv6
- ARMv6K

It also supports these extensions:

- VFPv2

## Contents

- [About](#about)
- [Performance](#performance)
- [Usage](#usage)
  - [Parsing one instruction](#parsing-one-instruction)
  - [4-byte Thumb instructions](#4-byte-thumb-instructions)
  - [The FormatIns trait](#the-formatins-trait)

## About

- Most of the parser is generated from [`isa.yaml`](/generator/assets/isa.yaml) by the [`/generator/`](/generator/) module.
- It accepts all 2^32 possible ARM instructions and 2^16 Thumb instructions (plus the 4-byte instructions) without errors.
- No promises that the output is 100% correct.
  - Some illegal instructions may not be parsed as illegal.
  - Some instructions may not stringify correctly.
  - (more, probably)

## Performance

Tested on all 2^32 ARM and Thumb instructions in ARMv6K using the [`/fuzz/`](/fuzz/) module on a single thread.

AMD Ryzen 7 7700X:
| Test                      | Duration | Throughput |
|:-------------------------:|:--------:|:----------:|
| Parse ARM                 | 27.11s   | ~604 MB/s  |
| Parse and stringify ARM   | 379.15s  | ~43 MB/s   |
| Parse Thumb               | 20.89s   | ~392 MB/s  |
| Parse and stringify Thumb | 305.58s  | ~27 MB/s   |

## Usage

### Parsing one instruction

```rust
use unarm::*;

let pc = 0;
let options = Options::default();
let ins = parse_arm(0xe5902268, pc, &options);
assert_eq!(
    ins,
    Ins::Ldr {
        cond: Cond::Al,
        rd: Reg::R2,
        addr: AddrLdrStr::Pre {
            rn: Reg::R0,
            offset: LdrStrOffset::Imm(0x268),
            writeback: false
        }
    }
);
assert_eq!(ins.display(&options).to_string(), "ldr r2, [r0, #0x268]");
```

### 4-byte Thumb instructions

Some instructions in Thumb are 4 bytes long instead of 2. To parse them properly, put the second
half of the instruction in the upper half of the code passed to `parse_thumb`. 

```rust
let first = 0xf099;
let second = 0xe866;
let (ins, _size) = parse_thumb(first | (second << 16), 0, &options);
```

You can do this for 2-byte instructions as well by passing two consecutive instructions in the same way. `parse_thumb` returns both the parsed instruction and its size, so you can tell if only one or both of the 16-bit words were parsed.

### The FormatIns trait

The `FormatIns` trait is used for formatting an instruction. You can implement this trait yourself to
customize the formatted output. Here is an example:

```rust
pub struct MyFormatter {
    options: unarm::Options,
}

// Write is a supertrait of FormatIns
impl std::fmt::Write for MyFormatter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        println!("{s}");
        Ok(())
    }
}

impl unarm::FormatIns for MyFormatter {
    fn options(&self) -> &unarm::Options {
        &self.options
    }

    // Override the default behavior
    fn write_reg(&mut self, reg: unarm::Reg) -> core::fmt::Result {
        // Add custom behavior here
        self.write_str("REG:")?;
        reg.write(self)
    }

    // Override any other trait functions as needed
}

let mut formatter = MyFormatter { options: Default::default() };
let ins = parse_arm(0xe5902268, 0, &formatter.options);
formatter.write_ins(&ins).unwrap();
```

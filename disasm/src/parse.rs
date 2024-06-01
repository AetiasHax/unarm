use crate::{
    args::{Argument, Arguments},
    v5te::{arm, thumb},
};

#[derive(Clone, Copy, Debug)]
pub struct Parser<'a> {
    pub version: ArmVersion,
    pub mode: ParseMode,
    pub address: u32,
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(version: ArmVersion, mode: ParseMode, address: u32, data: &'a [u8]) -> Self {
        Self {
            version,
            mode,
            address,
            data,
        }
    }

    fn read_code(&mut self) -> Option<u32> {
        let ins_size = self.mode.instruction_size();
        if self.data.len() < ins_size {
            return None;
        }
        let code = match ins_size {
            2 => u16::from_le_bytes([self.data[0], self.data[1]]) as u32,
            4 => u32::from_le_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]),
            _ => return None,
        };
        self.data = &self.data[ins_size..];
        self.address += ins_size as u32;
        Some(code)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (u32, Op, ParsedIns);

    fn next(&mut self) -> Option<Self::Item> {
        let address = self.address;
        let code = self.read_code()?;

        let (op, ins) = match (self.version, self.mode) {
            (ArmVersion::V5Te, ParseMode::Arm) => {
                let ins = arm::Ins::new(code);
                (Op::ArmV5Te(ins.op), ins.parse())
            }
            (ArmVersion::V5Te, ParseMode::Thumb) => {
                let ins = thumb::Ins::new(code);
                let op = Op::ThumbV5Te(ins.op);
                let parsed = ins.parse();
                if ins.is_half_bl() {
                    let code = self.read_code()?;
                    let ins = thumb::Ins::new(code);
                    let second = ins.parse();
                    let combined = parsed.combine_thumb_bl(&second);
                    (op, combined)
                } else {
                    (op, parsed)
                }
            }
            (_, ParseMode::Data) => {
                let mut args = Arguments::default();
                args[0] = Argument::UImm(code);
                (Op::Data, ParsedIns { mnemonic: ".word", args })
            }
        };

        Some((address, op, ins))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArmVersion {
    V5Te,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseMode {
    Arm,
    Thumb,
    Data,
}

impl ParseMode {
    pub fn instruction_size(self) -> usize {
        match self {
            Self::Arm => 4,
            Self::Thumb => 2,
            Self::Data => 4,
        }
    }

    pub fn from_mapping_symbol(sym: &str) -> Option<Self> {
        match sym {
            "$a" => Some(Self::Arm),
            "$t" => Some(Self::Thumb),
            "$d" => Some(Self::Data),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    ArmV5Te(arm::Opcode),
    ThumbV5Te(thumb::Opcode),
    Data,
}

impl Op {
    pub fn id(self) -> u16 {
        match self {
            Self::ArmV5Te(x) => x as u16,
            Self::ThumbV5Te(x) => x as u16,
            Self::Data => u16::MAX,
        }
    }
}

#[derive(Default, Debug)]
pub struct ParsedIns {
    pub mnemonic: &'static str,
    pub args: Arguments,
}

impl ParsedIns {
    pub fn args_iter(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().take_while(|a| **a != Argument::None)
    }

    /// Combines a pair of Thumb BL/BL or BL/BLX half-instructions into a full 32-bit instruction
    pub fn combine_thumb_bl(&self, second: &Self) -> Self {
        match (self.args[0], second.args[0]) {
            (Argument::SImm(high), Argument::UImm(low)) => {
                let dest = (high + (low as i32)) << 9 >> 9;
                let mut args = Arguments::default();
                args[0] = Argument::BranchDest(dest);
                Self {
                    mnemonic: second.mnemonic,
                    args,
                }
            }
            _ => Self {
                mnemonic: "<illegal>",
                args: Arguments::default(),
            },
        }
    }
}

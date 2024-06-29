use crate::args::{Argument, Arguments};
#[cfg(feature = "v4t")]
use crate::v4t;
#[cfg(feature = "v5te")]
use crate::v5te;
#[cfg(feature = "v6k")]
use crate::v6k;

#[derive(Clone, Copy, Debug)]
pub struct Parser<'a> {
    pub version: ArmVersion,
    pub mode: ParseMode,
    pub address: u32,
    pub endian: Endian,
    pub flags: ParseFlags,
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(version: ArmVersion, mode: ParseMode, address: u32, endian: Endian, flags: ParseFlags, data: &'a [u8]) -> Self {
        Self {
            version,
            mode,
            address,
            endian,
            flags,
            data,
        }
    }

    fn read_code(&mut self) -> Option<(u32, u32)> {
        let ins_size = self.mode.instruction_size(self.address);
        if self.data.len() < ins_size {
            return None;
        }
        let code = match (self.endian, ins_size) {
            (Endian::Little, 2) => u16::from_le_bytes([self.data[0], self.data[1]]) as u32,
            (Endian::Little, 4) => u32::from_le_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]),
            (Endian::Big, 2) => u16::from_be_bytes([self.data[0], self.data[1]]) as u32,
            (Endian::Big, 4) => u32::from_be_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]),
            _ => return None,
        };
        self.data = &self.data[ins_size..];
        self.address += ins_size as u32;
        Some((ins_size as u32, code))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParseFlags {
    pub ual: bool,
}

impl Default for ParseFlags {
    fn default() -> Self {
        Self { ual: true }
    }
}

macro_rules! parse_arm {
    ($self:expr, $module:ident, $op:ident, $code:expr) => {{
        let ins = $module::arm::Ins::new($code, &$self.flags);
        (Op::$op(ins.op), ins.parse(&$self.flags))
    }};
}

macro_rules! parse_thumb {
    ($self:expr, $module:ident, $op:ident, $code:expr) => {{
        let ins = $module::thumb::Ins::new($code, &$self.flags);
        let op = Op::$op(ins.op);
        let parsed = ins.parse(&$self.flags);
        if ins.is_half_bl() {
            let (_, code) = $self.read_code()?;
            let ins = $module::thumb::Ins::new(code, &$self.flags);
            let second = ins.parse(&$self.flags);
            let combined = parsed.combine_thumb_bl(&second);
            (op, combined)
        } else {
            (op, parsed)
        }
    }};
}

impl<'a> Iterator for Parser<'a> {
    type Item = (u32, Op, ParsedIns);

    fn next(&mut self) -> Option<Self::Item> {
        let address = self.address;
        let (ins_size, code) = self.read_code()?;

        let (op, ins) = match (self.version, self.mode) {
            #[cfg(all(feature = "v4t", feature = "arm"))]
            (ArmVersion::V4T, ParseMode::Arm) => parse_arm!(self, v4t, ArmV4T, code),
            #[cfg(all(feature = "v4t", feature = "thumb"))]
            (ArmVersion::V4T, ParseMode::Thumb) => parse_thumb!(self, v4t, ThumbV4T, code),
            #[cfg(all(feature = "v5te", feature = "arm"))]
            (ArmVersion::V5Te, ParseMode::Arm) => parse_arm!(self, v5te, ArmV5Te, code),
            #[cfg(all(feature = "v5te", feature = "thumb"))]
            (ArmVersion::V5Te, ParseMode::Thumb) => parse_thumb!(self, v5te, ThumbV5Te, code),
            #[cfg(all(feature = "v6k", feature = "arm"))]
            (ArmVersion::V6K, ParseMode::Arm) => parse_arm!(self, v6k, ArmV6K, code),
            #[cfg(all(feature = "v6k", feature = "thumb"))]
            (ArmVersion::V6K, ParseMode::Thumb) => parse_thumb!(self, v6k, ThumbV6K, code),
            (_, ParseMode::Data) => {
                let mut args = Arguments::default();
                args[0] = Argument::UImm(code);
                let mnemonic = if ins_size == 4 { ".word" } else { ".hword" };
                (Op::Data, ParsedIns { mnemonic, args })
            }
        };

        Some((address, op, ins))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArmVersion {
    #[cfg(feature = "v4t")]
    V4T,
    #[cfg(feature = "v5te")]
    V5Te,
    #[cfg(feature = "v6k")]
    V6K,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseMode {
    #[cfg(feature = "arm")]
    Arm,
    #[cfg(feature = "thumb")]
    Thumb,
    Data,
}

impl ParseMode {
    pub fn instruction_size(self, address: u32) -> usize {
        match self {
            #[cfg(feature = "arm")]
            Self::Arm => 4,
            #[cfg(feature = "thumb")]
            Self::Thumb => 2,
            Self::Data => 4 - (address as usize & 2),
        }
    }

    pub fn from_mapping_symbol(sym: &str) -> Option<Self> {
        match sym {
            #[cfg(feature = "arm")]
            "$a" => Some(Self::Arm),
            #[cfg(feature = "thumb")]
            "$t" => Some(Self::Thumb),
            "$d" => Some(Self::Data),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    #[cfg(all(feature = "v4t", feature = "arm"))]
    ArmV4T(v4t::arm::Opcode),
    #[cfg(all(feature = "v4t", feature = "thumb"))]
    ThumbV4T(v4t::thumb::Opcode),
    #[cfg(all(feature = "v5te", feature = "arm"))]
    ArmV5Te(v5te::arm::Opcode),
    #[cfg(all(feature = "v5te", feature = "thumb"))]
    ThumbV5Te(v5te::thumb::Opcode),
    #[cfg(all(feature = "v6k", feature = "arm"))]
    ArmV6K(v6k::arm::Opcode),
    #[cfg(all(feature = "v6k", feature = "thumb"))]
    ThumbV6K(v6k::thumb::Opcode),
    Data,
}

impl Op {
    pub fn id(self) -> u16 {
        match self {
            #[cfg(all(feature = "v4t", feature = "arm"))]
            Self::ArmV4T(x) => x as u16,
            #[cfg(all(feature = "v4t", feature = "thumb"))]
            Self::ThumbV4T(x) => x as u16,
            #[cfg(all(feature = "v5te", feature = "arm"))]
            Self::ArmV5Te(x) => x as u16,
            #[cfg(all(feature = "v5te", feature = "thumb"))]
            Self::ThumbV5Te(x) => x as u16,
            #[cfg(all(feature = "v6k", feature = "arm"))]
            Self::ArmV6K(x) => x as u16,
            #[cfg(all(feature = "v6k", feature = "thumb"))]
            Self::ThumbV6K(x) => x as u16,
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

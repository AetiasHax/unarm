use crate::args::{Argument, Arguments};

#[cfg(feature = "arm")]
use crate::arm;
#[cfg(feature = "thumb")]
use crate::thumb;

#[derive(Clone, Copy, Debug)]
pub struct Parser<'a> {
    pub mode: ParseMode,
    pub address: u32,
    pub endian: Endian,
    pub flags: ParseFlags,
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(mode: ParseMode, address: u32, endian: Endian, flags: ParseFlags, data: &'a [u8]) -> Self {
        Self {
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

    pub fn seek_forward(&mut self, address: u32) {
        if address < self.address {
            panic!("cannot seek backwards using Parser::seek_forward");
        }
        let diff = address - self.address;
        self.data = &self.data[diff as usize..];
        self.address = address;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParseFlags {
    pub ual: bool,
    pub version: ArmVersion,
}

impl Default for ParseFlags {
    fn default() -> Self {
        Self {
            ual: true,
            version: ArmVersion::default(),
        }
    }
}

macro_rules! parse_arm {
    ($self:expr, $code:expr) => {{
        let ins = arm::Ins::new($code, &$self.flags);
        (Ins::Arm(ins), ins.parse(&$self.flags))
    }};
}

macro_rules! parse_thumb {
    ($self:expr, $code:expr) => {{
        let ins = thumb::Ins::new($code, &$self.flags);
        let parsed = ins.parse(&$self.flags);
        if ins.is_half_bl() {
            let (_, code) = $self.read_code()?;
            let ins = thumb::Ins::new(code, &$self.flags);
            let second = ins.parse(&$self.flags);
            let combined = parsed.combine_thumb_bl(&second);
            (Ins::Thumb(ins), combined)
        } else {
            (Ins::Thumb(ins), parsed)
        }
    }};
}

impl Iterator for Parser<'_> {
    type Item = (u32, Ins, ParsedIns);

    fn next(&mut self) -> Option<Self::Item> {
        let address = self.address;
        let (ins_size, code) = self.read_code()?;

        let (op, ins) = match self.mode {
            #[cfg(feature = "arm")]
            ParseMode::Arm => parse_arm!(self, code),
            #[cfg(feature = "thumb")]
            ParseMode::Thumb => parse_thumb!(self, code),
            ParseMode::Data => {
                let mut args = Arguments::default();
                args[0] = Argument::UImm(code);
                let mnemonic = if ins_size == 4 { ".word" } else { ".hword" };
                (Ins::Data, ParsedIns { mnemonic, args })
            }
        };

        Some((address, op, ins))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Default)]
pub enum ArmVersion {
    #[cfg(feature = "v4t")]
    #[cfg_attr(all(feature = "v4t", not(feature = "v5te"), not(feature = "v6k")), default)]
    V4T,
    #[cfg(feature = "v5te")]
    #[cfg_attr(all(feature = "v5te", not(feature = "v6k")), default)]
    V5Te,
    #[cfg(feature = "v6k")]
    #[default]
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
pub enum Ins {
    #[cfg(feature = "arm")]
    Arm(arm::Ins),
    #[cfg(feature = "thumb")]
    Thumb(thumb::Ins),
    Data,
}

impl Ins {
    pub fn opcode_id(self) -> u16 {
        match self {
            #[cfg(feature = "arm")]
            Self::Arm(x) => x.op as u16,
            #[cfg(feature = "thumb")]
            Self::Thumb(x) => x.op as u16,
            Self::Data => u16::MAX,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

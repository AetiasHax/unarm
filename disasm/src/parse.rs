use crate::{
    args::{Argument, Arguments},
    arm::{self, parse::ParseArmV5Te},
    thumb::{self, parse::ParseThumbV5Te},
};

pub struct Parsers {
    pub arm: Box<dyn Parse>,
    pub thumb: Box<dyn Parse>,
}

impl Parsers {
    pub fn from_version(version: ArmVersion) -> Self {
        match version {
            ArmVersion::ARMv5TE => Parsers {
                arm: Box::new(ParseArmV5Te),
                thumb: Box::new(ParseThumbV5Te),
            },
        }
    }
}

pub enum ArmVersion {
    ARMv5TE,
}

pub trait Parse {
    fn version(&self) -> ArmVersion;
    fn thumb(&self) -> bool;

    fn parse(&self, code: u32) -> ParsedIns;
}

#[derive(Default)]
pub struct ParsedIns {
    pub mnemonic: &'static str,
    pub args: Arguments,
}

impl ParsedIns {
    pub fn parse_arm_v5te(ins: arm::Ins) -> Self {
        let mut out = Self::default();
        arm::parse(&mut out, ins);
        out
    }
    pub fn parse_thumb_v5te(ins: thumb::Ins) -> Self {
        let mut out = Self::default();
        thumb::parse(&mut out, ins);
        out
    }

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

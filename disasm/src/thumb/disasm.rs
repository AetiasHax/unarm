use std::fmt::{self, Display, Formatter};

use crate::{
    args::{Argument, Reg},
    thumb::generated::{parse, Arguments, Opcode},
};

#[derive(Clone, Copy)]
pub struct Ins {
    pub code: u32,
    pub op: Opcode,
}

impl Ins {
    pub fn new(code: u32) -> Self {
        let op = Opcode::find(code);
        Self { code, op }
    }

    /// Returns whether this is a BL half-instruction and should be combined with the upcoming instruction
    pub fn is_half_bl(&self) -> bool {
        self.op == Opcode::BlH
    }
}

#[derive(Default)]
pub struct ParsedIns {
    pub mnemonic: &'static str,
    pub args: Arguments,
}

impl ParsedIns {
    pub fn parse(ins: Ins) -> Self {
        let mut out = Self::default();
        parse(&mut out, ins);
        out
    }

    pub fn args_iter(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().take_while(|a| **a != Argument::None)
    }

    /// Combines a pair of BL/BL or BL/BLX half-instructions into a full 32-bit instruction
    pub fn combine_bl(&self, second: &Self) -> Self {
        match (self.args[0], second.args[0]) {
            (Argument::SImm(high), Argument::UImm(low)) => {
                let dest = (high + (low as i32)) << 9 >> 9;
                Self {
                    mnemonic: second.mnemonic,
                    args: [Argument::BranchDest(dest), Argument::None, Argument::None],
                }
            }
            _ => Self {
                mnemonic: "<illegal>",
                args: [Argument::None, Argument::None, Argument::None],
            },
        }
    }
}

impl Display for ParsedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.mnemonic)?;
        let mut comma = false;
        let mut deref = false;
        for arg in self.args_iter() {
            if comma {
                write!(f, ", ")?;
            }
            if let Argument::Reg(Reg {
                deref: true,
                reg,
                writeback,
            }) = arg
            {
                deref = true;
                write!(f, "[{}", reg)?;
                if *writeback {
                    write!(f, "!")?;
                }
            } else {
                write!(f, "{}", arg)?;
            }
            comma = true;
        }
        if deref {
            write!(f, "]")?;
        }
        Ok(())
    }
}

pub struct InsIter<'a> {
    pub address: u32,
    pub data: &'a [u8],
}

impl<'a> InsIter<'a> {
    pub fn new(data: &'a [u8], address: u32) -> Self {
        Self { address, data }
    }
}

impl<'a> Iterator for InsIter<'a> {
    type Item = (u32, Ins);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 2 {
            return None;
        }
        let bytes = [self.data[0], self.data[1]];
        let ins = Ins::new(u16::from_le_bytes(bytes) as u32);
        let addr = self.address;
        self.address += 2;
        self.data = &self.data[2..];
        Some((addr, ins))
    }
}

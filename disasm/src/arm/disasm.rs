use std::fmt::{self, Display, Formatter};

use crate::{
    args::{Argument, Arguments, OffsetImm, OffsetReg, Reg},
    arm::generated::{parse, Opcode},
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
}

impl Display for ParsedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.mnemonic)?;
        let mut comma = false;
        let mut deref = false;
        let mut writeback = false;
        for arg in self.args_iter() {
            if deref {
                match arg {
                    Argument::OffsetImm(OffsetImm {
                        post_indexed: true,
                        value: _,
                    })
                    | Argument::OffsetReg(OffsetReg {
                        add: _,
                        post_indexed: true,
                        reg: _,
                    })
                    | Argument::CoOption(_) => {
                        deref = false;
                        write!(f, "]")?;
                        if writeback {
                            write!(f, "!")?;
                            writeback = false;
                        }
                    }
                    _ => {}
                }
            }
            if comma {
                write!(f, ", ")?;
            }
            if let Argument::Reg(Reg {
                deref: true,
                reg,
                writeback: wb,
            }) = arg
            {
                deref = true;
                writeback = *wb;
                write!(f, "[{}", reg)?;
            } else {
                write!(f, "{}", arg)?;
            }
            comma = true;
        }
        if deref {
            write!(f, "]")?;
            if writeback {
                write!(f, "!")?;
            }
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
        if self.data.len() < 4 {
            return None;
        }
        let bytes = [self.data[0], self.data[1], self.data[2], self.data[3]];
        let ins = Ins::new(u32::from_le_bytes(bytes));
        let addr = self.address;
        self.address += 4;
        self.data = &self.data[4..];
        Some((addr, ins))
    }
}

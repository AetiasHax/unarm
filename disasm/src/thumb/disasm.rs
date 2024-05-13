use std::fmt::{self, Display, Formatter};

use crate::{
    display::SignedHex,
    thumb::generated::{parse, Argument, Arguments, Opcode, Reg},
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
        for arg in self.args_iter() {
            if comma {
                write!(f, ", ")?;
            }
            match arg {
                Argument::PcDeref | Argument::SpDeref | Argument::RegDeref(_) => {
                    deref = true;
                    write!(f, "[")?;
                }
                _ => {}
            }
            write!(f, "{}", arg)?;
            comma = true;
        }
        if deref {
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Argument::None => Ok(()),
            Argument::Reg(x) => write!(f, "{}", x),
            Argument::RegWb(x) => write!(f, "{}!", x),
            Argument::RegDeref(x) => write!(f, "{}", x),
            Argument::RegOffset(x) => write!(f, "{}", x),
            Argument::Pc => write!(f, "pc"),
            Argument::PcDeref => write!(f, "pc"),
            Argument::Sp => write!(f, "sp"),
            Argument::SpDeref => write!(f, "sp"),
            Argument::RegList(x) | Argument::RegListPc(x) => {
                write!(f, "{{")?;
                let mut first = true;
                for i in 0..8 {
                    if (x & (1 << i)) != 0 {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", Reg::parse(i))?;
                        first = false;
                    }
                }
                if (x & 0x100) != 0 {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "pc")?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            Argument::UImm(x) => write!(f, "#0x{:x}", x),
            Argument::SImm((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
            Argument::Offset((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Reg::Illegal => write!(f, "<illegal>"),
            Reg::R0 => write!(f, "r0"),
            Reg::R1 => write!(f, "r1"),
            Reg::R2 => write!(f, "r2"),
            Reg::R3 => write!(f, "r3"),
            Reg::R4 => write!(f, "r4"),
            Reg::R5 => write!(f, "r5"),
            Reg::R6 => write!(f, "r6"),
            Reg::R7 => write!(f, "r7"),
            Reg::R8 => write!(f, "r8"),
            Reg::R9 => write!(f, "r9"),
            Reg::R10 => write!(f, "r10"),
            Reg::Fp => write!(f, "fp"),
            Reg::Ip => write!(f, "ip"),
            Reg::Sp => write!(f, "sp"),
            Reg::Lr => write!(f, "lr"),
            Reg::Pc => write!(f, "pc"),
        }
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

use std::fmt::{self, Display, Formatter};

use crate::{
    generated::{parse, Argument, Arguments, FieldMask, Opcode, RegOffset, RegPostOffset, ShiftImm, ShiftReg},
    CoReg, Reg, Shift, StatusMask, StatusReg,
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
                    Argument::PostOffset(_) | Argument::RegPostOffset(_) | Argument::CoOption(_) => {
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
            match arg {
                Argument::RegDeref(_) => {
                    deref = true;
                    write!(f, "[")?
                }
                Argument::RegDerefWb(_) => {
                    deref = true;
                    writeback = true;
                    write!(f, "[")?
                }
                _ => {}
            }
            write!(f, "{}", arg)?;
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

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Argument::None => Ok(()),
            Argument::Reg(x) => write!(f, "{}", x),
            Argument::RegWb(x) => write!(f, "{}!", x),
            Argument::RegList(x) | Argument::RegListC(x) => {
                write!(f, "{{")?;
                let mut first = true;
                for i in 0..16 {
                    if (x & (1 << i)) != 0 {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", Reg::parse(i))?;
                        first = false;
                    }
                }
                write!(f, "}}")?;
                if matches!(self, Argument::RegListC(_)) {
                    write!(f, "^")?;
                }
                Ok(())
            }
            Argument::CoReg(x) => write!(f, "{}", x),
            Argument::StatusReg(x) => write!(f, "{}", x),
            Argument::UImm(x) => write!(f, "#0x{:x}", x),
            Argument::SImm((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
            Argument::Offset((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
            Argument::CoOption(x) => write!(f, "{{0x{:x}}}", x),
            Argument::CoOpcode(x) => write!(f, "#{}", x),
            Argument::CoprocNum(x) => write!(f, "p{}", x),
            Argument::ShiftImm(x) => write!(f, "{}", x),
            Argument::ShiftReg(x) => write!(f, "{}", x),
            Argument::Rrx => write!(f, "rrx"),
            Argument::RegOffset(x) => write!(f, "{}", x),
            Argument::FieldMask(x) => write!(f, "{}", x),
            Argument::RegDeref(x) => write!(f, "{}", x),
            Argument::RegDerefWb(x) => write!(f, "{}", x),
            Argument::PostOffset((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
            Argument::RegPostOffset(x) => write!(f, "{}", x),
            Argument::BranchDest((x, size)) => write!(f, "{}", SignedHex { value: *x, size: *size }),
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

impl Display for CoReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CoReg::Illegal => write!(f, "<illegal>"),
            CoReg::C0 => write!(f, "c0"),
            CoReg::C1 => write!(f, "c1"),
            CoReg::C2 => write!(f, "c2"),
            CoReg::C3 => write!(f, "c3"),
            CoReg::C4 => write!(f, "c4"),
            CoReg::C5 => write!(f, "c5"),
            CoReg::C6 => write!(f, "c6"),
            CoReg::C7 => write!(f, "c7"),
            CoReg::C8 => write!(f, "c8"),
            CoReg::C9 => write!(f, "c9"),
            CoReg::C10 => write!(f, "c10"),
            CoReg::C11 => write!(f, "c11"),
            CoReg::C12 => write!(f, "c12"),
            CoReg::C13 => write!(f, "c13"),
            CoReg::C14 => write!(f, "c14"),
            CoReg::C15 => write!(f, "c15"),
        }
    }
}

impl Display for StatusReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StatusReg::Illegal => write!(f, "<illegal>"),
            StatusReg::Cpsr => write!(f, "cpsr"),
            StatusReg::Spsr => write!(f, "spsr"),
        }
    }
}

impl Display for StatusMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StatusMask::Illegal => write!(f, "<illegal>"),
            StatusMask::C => write!(f, "c"),
            StatusMask::X => write!(f, "x"),
            StatusMask::Xc => write!(f, "xc"),
            StatusMask::S => write!(f, "s"),
            StatusMask::Sc => write!(f, "sc"),
            StatusMask::Sx => write!(f, "sx"),
            StatusMask::Sxc => write!(f, "sxc"),
            StatusMask::F => write!(f, "f"),
            StatusMask::Fc => write!(f, "fc"),
            StatusMask::Fx => write!(f, "fx"),
            StatusMask::Fxc => write!(f, "fxc"),
            StatusMask::Fs => write!(f, "fs"),
            StatusMask::Fsc => write!(f, "fsc"),
            StatusMask::Fsx => write!(f, "fsx"),
            StatusMask::Fsxc => write!(f, "fsxc"),
        }
    }
}

impl Display for Shift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Shift::Illegal => write!(f, "<illegal>"),
            Shift::Lsl => write!(f, "lsl"),
            Shift::Lsr => write!(f, "lsr"),
            Shift::Asr => write!(f, "asr"),
            Shift::Ror => write!(f, "ror"),
        }
    }
}

impl Display for ShiftImm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} #0x{:x}", self.op, self.imm)
    }
}

impl Display for ShiftReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.op, self.reg)
    }
}

impl Display for RegOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.add {
            write!(f, "-")?;
        }
        write!(f, "{}", self.reg)
    }
}

impl Display for RegPostOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.add {
            write!(f, "-")?;
        }
        write!(f, "{}", self.reg)
    }
}

impl Display for FieldMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.reg, self.mask)
    }
}

struct SignedHex {
    value: i32,
    size: u8,
}

impl Display for SignedHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hex = format!("{:08x}", self.value.abs() & ((1 << self.size as i32) - 1));
        let chars = self.size.div_ceil(4);
        let mut hex: String = hex.chars().skip((8 - chars).into()).skip_while(|ch| *ch == '0').collect();
        if hex.is_empty() {
            hex += "0";
        }
        write!(f, "#")?;
        // if self.value & (1 << (self.size - 1)) != 0 {
        if self.value.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x{}", hex)
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

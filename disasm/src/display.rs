use std::fmt::{self, Display, Formatter};

use crate::{
    args::{
        Argument, CoReg, CpsrFlags, CpsrMode, Endian, OffsetImm, OffsetReg, Reg, Register, Shift, ShiftImm, ShiftReg,
        StatusMask, StatusReg,
    },
    parse::ParsedIns,
};

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

pub struct SignedHex(i32);

impl Display for SignedHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#")?;
        if self.0.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x{:x}", self.0.abs())
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Argument::None => Ok(()),
            Argument::Reg(reg) => {
                write!(f, "{}", reg.reg)?;
                if reg.writeback {
                    write!(f, "!")?;
                }
                Ok(())
            }
            Argument::RegList(list) => {
                write!(f, "{{")?;
                let mut first = true;
                for i in 0..16 {
                    if (list.regs & (1 << i)) != 0 {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", Register::parse(i))?;
                        first = false;
                    }
                }
                write!(f, "}}")?;
                if list.user_mode {
                    write!(f, "^")?;
                }
                Ok(())
            }
            Argument::CoReg(x) => write!(f, "{}", x),
            Argument::StatusReg(x) => write!(f, "{}", x),
            Argument::UImm(x) => write!(f, "#0x{:x}", x),
            Argument::SImm(x) => write!(f, "{}", SignedHex(*x)),
            Argument::OffsetImm(x) => write!(f, "{}", SignedHex(x.value)),
            Argument::CoOption(x) => write!(f, "{{0x{:x}}}", x),
            Argument::CoOpcode(x) => write!(f, "#{}", x),
            Argument::CoprocNum(x) => write!(f, "p{}", x),
            Argument::ShiftImm(x) => write!(f, "{}", x),
            Argument::ShiftReg(x) => write!(f, "{}", x),
            Argument::OffsetReg(x) => write!(f, "{}", x),
            Argument::BranchDest(x) => write!(f, "{}", SignedHex(*x)),
            Argument::StatusMask(x) => write!(f, "{}", x),
            Argument::Shift(x) => write!(f, "{}", x),
            Argument::SatImm(x) => write!(f, "#{}", x),
            Argument::CpsrMode(x) => write!(f, "{}", x),
            Argument::CpsrFlags(x) => write!(f, "{}", x),
            Argument::Endian(x) => write!(f, "{}", x),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Register::Illegal => write!(f, "<illegal>"),
            Register::R0 => write!(f, "r0"),
            Register::R1 => write!(f, "r1"),
            Register::R2 => write!(f, "r2"),
            Register::R3 => write!(f, "r3"),
            Register::R4 => write!(f, "r4"),
            Register::R5 => write!(f, "r5"),
            Register::R6 => write!(f, "r6"),
            Register::R7 => write!(f, "r7"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::Fp => write!(f, "fp"),
            Register::Ip => write!(f, "ip"),
            Register::Sp => write!(f, "sp"),
            Register::Lr => write!(f, "lr"),
            Register::Pc => write!(f, "pc"),
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
        write!(f, "{}_", self.reg)?;
        if self.flags {
            write!(f, "f")?;
        }
        if self.status {
            write!(f, "s")?;
        }
        if self.extension {
            write!(f, "x")?;
        }
        if self.control {
            write!(f, "c")?;
        }
        Ok(())
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
            Shift::Rrx => write!(f, "rrx"),
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

impl Display for OffsetReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.add {
            write!(f, "-")?;
        }
        write!(f, "{}", self.reg)
    }
}

impl Display for CpsrMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#0x{:x}", self.mode)?;
        if self.writeback {
            write!(f, "!")?;
        }
        Ok(())
    }
}

impl Display for CpsrFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.a {
            write!(f, "a")?;
        }
        if self.f {
            write!(f, "f")?;
        }
        if self.i {
            write!(f, "i")?;
        }
        if !self.a && !self.f && !self.i {
            write!(f, "none")?;
        }
        Ok(())
    }
}

impl Display for Endian {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Endian::Illegal => write!(f, "<illegal>"),
            Endian::Le => write!(f, "le"),
            Endian::Be => write!(f, "be"),
        }
    }
}

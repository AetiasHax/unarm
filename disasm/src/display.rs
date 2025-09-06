use crate::*;
impl BranchTarget {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let Self { addr } = self;
        f.write_str("#")?;
        if *addr < 0 {
            write!(f, "-{:#x}", - addr)?;
        } else {
            write!(f, "{:#x}", addr)?;
        }
        Ok(())
    }
}
impl Cond {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Eq => {
                f.write_str("eq")?;
            }
            Self::Ne => {
                f.write_str("ne")?;
            }
            Self::Hs => {
                f.write_str("hs")?;
            }
            Self::Lo => {
                f.write_str("lo")?;
            }
            Self::Mi => {
                f.write_str("mi")?;
            }
            Self::Pl => {
                f.write_str("pl")?;
            }
            Self::Vs => {
                f.write_str("vs")?;
            }
            Self::Vc => {
                f.write_str("vc")?;
            }
            Self::Hi => {
                f.write_str("hi")?;
            }
            Self::Ls => {
                f.write_str("ls")?;
            }
            Self::Ge => {
                f.write_str("ge")?;
            }
            Self::Lt => {
                f.write_str("lt")?;
            }
            Self::Gt => {
                f.write_str("gt")?;
            }
            Self::Le => {
                f.write_str("le")?;
            }
            Self::Al => {}
        }
        Ok(())
    }
}
impl Reg {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::R0 => {
                if options.av {
                    f.write_str("a1")?;
                } else {
                    f.write_str("r0")?;
                }
            }
            Self::R1 => {
                if options.av {
                    f.write_str("a2")?;
                } else {
                    f.write_str("r1")?;
                }
            }
            Self::R2 => {
                if options.av {
                    f.write_str("a3")?;
                } else {
                    f.write_str("r2")?;
                }
            }
            Self::R3 => {
                if options.av {
                    f.write_str("a4")?;
                } else {
                    f.write_str("r3")?;
                }
            }
            Self::R4 => {
                if options.av {
                    f.write_str("v1")?;
                } else {
                    f.write_str("r4")?;
                }
            }
            Self::R5 => {
                if options.av {
                    f.write_str("v2")?;
                } else {
                    f.write_str("r5")?;
                }
            }
            Self::R6 => {
                if options.av {
                    f.write_str("v3")?;
                } else {
                    f.write_str("r6")?;
                }
            }
            Self::R7 => {
                if options.av {
                    f.write_str("v4")?;
                } else {
                    f.write_str("r7")?;
                }
            }
            Self::R8 => {
                if options.av {
                    f.write_str("v5")?;
                } else {
                    f.write_str("r8")?;
                }
            }
            Self::R9 => {
                if options.r9_use == R9Use::R9 {
                    if options.av {
                        f.write_str("v6")?;
                    } else {
                        f.write_str("r9")?;
                    }
                } else {
                    if options.r9_use == R9Use::Sb {
                        f.write_str("sb")?;
                    } else {
                        f.write_str("tr")?;
                    }
                }
            }
            Self::R10 => {
                if options.sl {
                    f.write_str("sl")?;
                } else {
                    if options.av {
                        f.write_str("v7")?;
                    } else {
                        f.write_str("r10")?;
                    }
                }
            }
            Self::R11 => {
                if options.fp {
                    f.write_str("fp")?;
                } else {
                    if options.av {
                        f.write_str("v8")?;
                    } else {
                        f.write_str("r11")?;
                    }
                }
            }
            Self::R12 => {
                if options.ip {
                    f.write_str("ip")?;
                } else {
                    f.write_str("r12")?;
                }
            }
            Self::Sp => {
                f.write_str("sp")?;
            }
            Self::Lr => {
                f.write_str("lr")?;
            }
            Self::Pc => {
                f.write_str("pc")?;
            }
        }
        Ok(())
    }
}
impl ShiftOp {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Lsl => {
                f.write_str("lsl")?;
            }
            Self::Lsr => {
                f.write_str("lsr")?;
            }
            Self::Asr => {
                f.write_str("asr")?;
            }
            Self::Ror => {
                f.write_str("ror")?;
            }
        }
        Ok(())
    }
}
impl Op2 {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Imm(data) => {
                f.write_str("#")?;
                write!(f, "{:#x}", data)?;
            }
            Self::ShiftReg { rm, shift_op, rs } => {
                rm.fmt(options, f)?;
                f.write_str(", ")?;
                shift_op.fmt(options, f)?;
                f.write_str(" ")?;
                rs.fmt(options, f)?;
            }
            Self::ShiftImm { rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    rm.fmt(options, f)?;
                } else {
                    if *imm == 0 && *shift_op == ShiftOp::Ror {
                        rm.fmt(options, f)?;
                        f.write_str(", rrx")?;
                    } else {
                        rm.fmt(options, f)?;
                        f.write_str(", ")?;
                        shift_op.fmt(options, f)?;
                        f.write_str(" #")?;
                        write!(f, "{:#x}", imm)?;
                    }
                }
            }
        }
        Ok(())
    }
}
impl Ins {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("adc")?;
                    if *s {
                        f.write_str("s")?;
                    }
                    cond.fmt(options, f)?;
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                } else {
                    f.write_str("adc")?;
                    cond.fmt(options, f)?;
                    if *s {
                        f.write_str("s")?;
                    }
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                }
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("add")?;
                    if *s {
                        f.write_str("s")?;
                    }
                    cond.fmt(options, f)?;
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                } else {
                    f.write_str("add")?;
                    cond.fmt(options, f)?;
                    if *s {
                        f.write_str("s")?;
                    }
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                }
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("and")?;
                    if *s {
                        f.write_str("s")?;
                    }
                    cond.fmt(options, f)?;
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                } else {
                    f.write_str("and")?;
                    cond.fmt(options, f)?;
                    if *s {
                        f.write_str("s")?;
                    }
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                }
            }
            Ins::B { cond, target } => {
                f.write_str("b")?;
                cond.fmt(options, f)?;
                f.write_str(" ")?;
                target.fmt(options, f)?;
            }
            Ins::Illegal => {
                f.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
}
impl Ins {
    pub fn display<'a>(&'a self, options: &'a Options) -> DisplayIns<'a> {
        DisplayIns { ins: self, options }
    }
}
pub struct DisplayIns<'a> {
    ins: &'a Ins,
    options: &'a Options,
}
impl<'a> core::fmt::Display for DisplayIns<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.ins.fmt(self.options, f)
    }
}

use crate::*;
pub trait Write: core::fmt::Write {
    fn options(&self) -> &Options;
    fn write_opcode(&mut self, opcode: &str) -> core::fmt::Result {
        self.write_str(opcode)
    }
    fn write_space(&mut self) -> core::fmt::Result {
        self.write_str(" ")
    }
    fn write_separator(&mut self) -> core::fmt::Result {
        self.write_str(", ")
    }
    fn write_s(&mut self, s: bool) -> core::fmt::Result {
        if s {
            self.write_str("s")?;
        }
        Ok(())
    }
    fn write_uimm(&mut self, uimm: u32) -> core::fmt::Result {
        write!(self, "{:#x}", uimm)?;
        Ok(())
    }
    fn write_simm(&mut self, simm: i32) -> core::fmt::Result {
        if simm < 0 {
            write!(self, "-{:#x}", - simm)?;
        } else {
            write!(self, "{:#x}", simm)?;
        }
        Ok(())
    }
    fn write_branch_target(&mut self, branch_target: BranchTarget) -> core::fmt::Result {
        branch_target.write(self)?;
        Ok(())
    }
    fn write_cond(&mut self, cond: Cond) -> core::fmt::Result {
        cond.write(self)?;
        Ok(())
    }
    fn write_reg(&mut self, reg: Reg) -> core::fmt::Result {
        reg.write(self)?;
        Ok(())
    }
    fn write_shift_op(&mut self, shift_op: ShiftOp) -> core::fmt::Result {
        shift_op.write(self)?;
        Ok(())
    }
    fn write_op2(&mut self, op2: Op2) -> core::fmt::Result {
        op2.write(self)?;
        Ok(())
    }
    fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
        ins.write(self)
    }
}
impl BranchTarget {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        let Self { addr } = self;
        f.write_str("#")?;
        f.write_uimm(*addr)?;
        Ok(())
    }
}
impl Cond {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
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
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
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
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
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
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Self::Imm(data) => {
                f.write_str("#")?;
                f.write_uimm(*data)?;
            }
            Self::ShiftReg { rm, shift_op, rs } => {
                f.write_reg(*rm)?;
                f.write_separator()?;
                f.write_shift_op(*shift_op)?;
                f.write_space()?;
                f.write_reg(*rs)?;
            }
            Self::ShiftImm { rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    f.write_reg(*rm)?;
                } else {
                    if *imm == 0 && *shift_op == ShiftOp::Ror {
                        f.write_reg(*rm)?;
                        f.write_separator()?;
                        f.write_str("rrx")?;
                    } else {
                        f.write_reg(*rm)?;
                        f.write_separator()?;
                        f.write_shift_op(*shift_op)?;
                        f.write_space()?;
                        f.write_str("#")?;
                        f.write_uimm(*imm)?;
                    }
                }
            }
        }
        Ok(())
    }
}
impl Ins {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("adc")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                } else {
                    f.write_str("adc")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                }
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("add")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                } else {
                    f.write_str("add")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                }
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("and")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                } else {
                    f.write_str("and")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                    f.write_space()?;
                    f.write_reg(*rd)?;
                    f.write_separator()?;
                    f.write_reg(*rn)?;
                    f.write_separator()?;
                    f.write_op2(*op2)?;
                }
            }
            Ins::B { cond, target } => {
                f.write_str("b")?;
                f.write_cond(*cond)?;
                f.write_space()?;
                f.write_branch_target(*target)?;
            }
            Ins::Illegal => {
                f.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
}

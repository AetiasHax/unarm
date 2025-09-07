use crate::*;
pub trait Write: core::fmt::Write {
    fn options(&self) -> &Options;
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
    fn write_blx_target(&mut self, blx_target: BlxTarget) -> core::fmt::Result {
        blx_target.write(self)?;
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
    fn write_coproc(&mut self, coproc: Coproc) -> core::fmt::Result {
        coproc.write(self)?;
        Ok(())
    }
    fn write_co_reg(&mut self, co_reg: CoReg) -> core::fmt::Result {
        co_reg.write(self)?;
        Ok(())
    }
    fn write_op2(&mut self, op2: Op2) -> core::fmt::Result {
        op2.write(self)?;
        Ok(())
    }
    fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
        ins.write_opcode(self)?;
        ins.write_params(self)?;
        Ok(())
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
impl BlxTarget {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Self::Direct(target) => {
                f.write_branch_target(*target)?;
            }
            Self::Indirect(rm) => {
                f.write_reg(*rm)?;
            }
        }
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
impl Coproc {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Self::P0 => {
                f.write_str("p0")?;
            }
            Self::P1 => {
                f.write_str("p1")?;
            }
            Self::P2 => {
                f.write_str("p2")?;
            }
            Self::P3 => {
                f.write_str("p3")?;
            }
            Self::P4 => {
                f.write_str("p4")?;
            }
            Self::P5 => {
                f.write_str("p5")?;
            }
            Self::P6 => {
                f.write_str("p6")?;
            }
            Self::P7 => {
                f.write_str("p7")?;
            }
            Self::P8 => {
                f.write_str("p8")?;
            }
            Self::P9 => {
                f.write_str("p9")?;
            }
            Self::P10 => {
                f.write_str("p10")?;
            }
            Self::P11 => {
                f.write_str("p11")?;
            }
            Self::P12 => {
                f.write_str("p12")?;
            }
            Self::P13 => {
                f.write_str("p13")?;
            }
            Self::P14 => {
                f.write_str("p14")?;
            }
            Self::P15 => {
                f.write_str("p15")?;
            }
        }
        Ok(())
    }
}
impl CoReg {
    pub fn write<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Self::C0 => {
                f.write_str("c0")?;
            }
            Self::C1 => {
                f.write_str("c1")?;
            }
            Self::C2 => {
                f.write_str("c2")?;
            }
            Self::C3 => {
                f.write_str("c3")?;
            }
            Self::C4 => {
                f.write_str("c4")?;
            }
            Self::C5 => {
                f.write_str("c5")?;
            }
            Self::C6 => {
                f.write_str("c6")?;
            }
            Self::C7 => {
                f.write_str("c7")?;
            }
            Self::C8 => {
                f.write_str("c8")?;
            }
            Self::C9 => {
                f.write_str("c9")?;
            }
            Self::C10 => {
                f.write_str("c10")?;
            }
            Self::C11 => {
                f.write_str("c11")?;
            }
            Self::C12 => {
                f.write_str("c12")?;
            }
            Self::C13 => {
                f.write_str("c13")?;
            }
            Self::C14 => {
                f.write_str("c14")?;
            }
            Self::C15 => {
                f.write_str("c15")?;
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
            Self::Imm(imm) => {
                f.write_str("#")?;
                f.write_uimm(*imm)?;
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
    pub fn write_opcode<F>(&self, f: &mut F) -> core::fmt::Result
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
                } else {
                    f.write_str("adc")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                }
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("add")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                } else {
                    f.write_str("add")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                }
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("and")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                } else {
                    f.write_str("and")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                }
            }
            Ins::B { cond, target } => {
                f.write_str("b")?;
                f.write_cond(*cond)?;
            }
            Ins::Bic { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("bic")?;
                    f.write_s(*s)?;
                    f.write_cond(*cond)?;
                } else {
                    f.write_str("bic")?;
                    f.write_cond(*cond)?;
                    f.write_s(*s)?;
                }
            }
            Ins::Bkpt { imm } => {
                f.write_str("bkpt")?;
            }
            Ins::Bl { cond, target } => {
                f.write_str("bl")?;
                f.write_cond(*cond)?;
            }
            Ins::Blx { cond, target } => {
                f.write_str("blx")?;
                f.write_cond(*cond)?;
            }
            Ins::Bx { cond, rm } => {
                f.write_str("bx")?;
                f.write_cond(*cond)?;
            }
            Ins::Bxj { cond, rm } => {
                f.write_str("bxj")?;
                f.write_cond(*cond)?;
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                f.write_str("cdp")?;
                f.write_cond(*cond)?;
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                f.write_str("cdp2")?;
            }
            Ins::Clrex {} => {
                f.write_str("clrex")?;
            }
            Ins::Illegal => {
                f.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
    pub fn write_params<F>(&self, f: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = f.options();
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                f.write_space()?;
                f.write_reg(*rd)?;
                f.write_separator()?;
                f.write_reg(*rn)?;
                f.write_separator()?;
                f.write_op2(*op2)?;
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                f.write_space()?;
                f.write_reg(*rd)?;
                f.write_separator()?;
                f.write_reg(*rn)?;
                f.write_separator()?;
                f.write_op2(*op2)?;
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                f.write_space()?;
                f.write_reg(*rd)?;
                f.write_separator()?;
                f.write_reg(*rn)?;
                f.write_separator()?;
                f.write_op2(*op2)?;
            }
            Ins::B { cond, target } => {
                f.write_space()?;
                f.write_branch_target(*target)?;
            }
            Ins::Bic { s, cond, rd, rn, op2 } => {
                f.write_space()?;
                f.write_reg(*rd)?;
                f.write_separator()?;
                f.write_reg(*rn)?;
                f.write_separator()?;
                f.write_op2(*op2)?;
            }
            Ins::Bkpt { imm } => {
                f.write_space()?;
                f.write_str("#")?;
                f.write_uimm(*imm)?;
            }
            Ins::Bl { cond, target } => {
                f.write_space()?;
                f.write_branch_target(*target)?;
            }
            Ins::Blx { cond, target } => {
                f.write_space()?;
                f.write_blx_target(*target)?;
            }
            Ins::Bx { cond, rm } => {
                f.write_space()?;
                f.write_reg(*rm)?;
            }
            Ins::Bxj { cond, rm } => {
                f.write_space()?;
                f.write_reg(*rm)?;
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                f.write_space()?;
                f.write_coproc(*coproc)?;
                f.write_separator()?;
                f.write_str("#")?;
                f.write_uimm(*opc1)?;
                f.write_separator()?;
                f.write_co_reg(*crd)?;
                f.write_separator()?;
                f.write_co_reg(*crn)?;
                f.write_separator()?;
                f.write_co_reg(*crm)?;
                f.write_separator()?;
                f.write_str("#")?;
                f.write_uimm(*opc2)?;
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                f.write_space()?;
                f.write_coproc(*coproc)?;
                f.write_separator()?;
                f.write_str("#")?;
                f.write_uimm(*opc1)?;
                f.write_separator()?;
                f.write_co_reg(*crd)?;
                f.write_separator()?;
                f.write_co_reg(*crn)?;
                f.write_separator()?;
                f.write_co_reg(*crm)?;
                f.write_separator()?;
                f.write_str("#")?;
                f.write_uimm(*opc2)?;
            }
            Ins::Clrex {} => {}
            Ins::Illegal => {
                f.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
}

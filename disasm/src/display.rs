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
    fn write_l(&mut self, l: bool) -> core::fmt::Result {
        if l {
            self.write_str("l")?;
        }
        Ok(())
    }
    fn write_wb(&mut self, wb: bool) -> core::fmt::Result {
        if wb {
            self.write_str("!")?;
        }
        Ok(())
    }
    fn write_user_mode(&mut self, user_mode: bool) -> core::fmt::Result {
        if user_mode {
            self.write_str("^")?;
        }
        Ok(())
    }
    fn write_subtract(&mut self, subtract: bool) -> core::fmt::Result {
        if subtract {
            self.write_str("-")?;
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
    fn write_reg_list(&mut self, reg_list: RegList) -> core::fmt::Result {
        reg_list.write(self)?;
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
    fn write_cps_effect(&mut self, cps_effect: CpsEffect) -> core::fmt::Result {
        cps_effect.write(self)?;
        Ok(())
    }
    fn write_aif_flags(&mut self, aif_flags: AifFlags) -> core::fmt::Result {
        aif_flags.write(self)?;
        Ok(())
    }
    fn write_addr_ldc_stc(&mut self, addr_ldc_stc: AddrLdcStc) -> core::fmt::Result {
        addr_ldc_stc.write(self)?;
        Ok(())
    }
    fn write_ldm_stm_mode(&mut self, ldm_stm_mode: LdmStmMode) -> core::fmt::Result {
        ldm_stm_mode.write(self)?;
        Ok(())
    }
    fn write_addr_ldr_str(&mut self, addr_ldr_str: AddrLdrStr) -> core::fmt::Result {
        addr_ldr_str.write(self)?;
        Ok(())
    }
    fn write_ldr_str_offset(
        &mut self,
        ldr_str_offset: LdrStrOffset,
    ) -> core::fmt::Result {
        ldr_str_offset.write(self)?;
        Ok(())
    }
    fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
        ins.write_opcode(self)?;
        ins.write_params(self)?;
        Ok(())
    }
}
impl BranchTarget {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        let Self { addr } = self;
        formatter.write_str("#")?;
        formatter.write_uimm(*addr)?;
        Ok(())
    }
}
impl BlxTarget {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Direct(target) => {
                formatter.write_branch_target(*target)?;
            }
            Self::Indirect(rm) => {
                formatter.write_reg(*rm)?;
            }
        }
        Ok(())
    }
}
impl Cond {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Eq => {
                formatter.write_str("eq")?;
            }
            Self::Ne => {
                formatter.write_str("ne")?;
            }
            Self::Hs => {
                formatter.write_str("hs")?;
            }
            Self::Lo => {
                formatter.write_str("lo")?;
            }
            Self::Mi => {
                formatter.write_str("mi")?;
            }
            Self::Pl => {
                formatter.write_str("pl")?;
            }
            Self::Vs => {
                formatter.write_str("vs")?;
            }
            Self::Vc => {
                formatter.write_str("vc")?;
            }
            Self::Hi => {
                formatter.write_str("hi")?;
            }
            Self::Ls => {
                formatter.write_str("ls")?;
            }
            Self::Ge => {
                formatter.write_str("ge")?;
            }
            Self::Lt => {
                formatter.write_str("lt")?;
            }
            Self::Gt => {
                formatter.write_str("gt")?;
            }
            Self::Le => {
                formatter.write_str("le")?;
            }
            Self::Al => {}
        }
        Ok(())
    }
}
impl Reg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::R0 => {
                if options.av {
                    formatter.write_str("a1")?;
                } else {
                    formatter.write_str("r0")?;
                }
            }
            Self::R1 => {
                if options.av {
                    formatter.write_str("a2")?;
                } else {
                    formatter.write_str("r1")?;
                }
            }
            Self::R2 => {
                if options.av {
                    formatter.write_str("a3")?;
                } else {
                    formatter.write_str("r2")?;
                }
            }
            Self::R3 => {
                if options.av {
                    formatter.write_str("a4")?;
                } else {
                    formatter.write_str("r3")?;
                }
            }
            Self::R4 => {
                if options.av {
                    formatter.write_str("v1")?;
                } else {
                    formatter.write_str("r4")?;
                }
            }
            Self::R5 => {
                if options.av {
                    formatter.write_str("v2")?;
                } else {
                    formatter.write_str("r5")?;
                }
            }
            Self::R6 => {
                if options.av {
                    formatter.write_str("v3")?;
                } else {
                    formatter.write_str("r6")?;
                }
            }
            Self::R7 => {
                if options.av {
                    formatter.write_str("v4")?;
                } else {
                    formatter.write_str("r7")?;
                }
            }
            Self::R8 => {
                if options.av {
                    formatter.write_str("v5")?;
                } else {
                    formatter.write_str("r8")?;
                }
            }
            Self::R9 => {
                if options.r9_use == R9Use::R9 {
                    if options.av {
                        formatter.write_str("v6")?;
                    } else {
                        formatter.write_str("r9")?;
                    }
                } else {
                    if options.r9_use == R9Use::Sb {
                        formatter.write_str("sb")?;
                    } else {
                        formatter.write_str("tr")?;
                    }
                }
            }
            Self::R10 => {
                if options.sl {
                    formatter.write_str("sl")?;
                } else {
                    if options.av {
                        formatter.write_str("v7")?;
                    } else {
                        formatter.write_str("r10")?;
                    }
                }
            }
            Self::R11 => {
                if options.fp {
                    formatter.write_str("fp")?;
                } else {
                    if options.av {
                        formatter.write_str("v8")?;
                    } else {
                        formatter.write_str("r11")?;
                    }
                }
            }
            Self::R12 => {
                if options.ip {
                    formatter.write_str("ip")?;
                } else {
                    formatter.write_str("r12")?;
                }
            }
            Self::Sp => {
                formatter.write_str("sp")?;
            }
            Self::Lr => {
                formatter.write_str("lr")?;
            }
            Self::Pc => {
                formatter.write_str("pc")?;
            }
        }
        Ok(())
    }
}
impl ShiftOp {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Lsl => {
                formatter.write_str("lsl")?;
            }
            Self::Lsr => {
                formatter.write_str("lsr")?;
            }
            Self::Asr => {
                formatter.write_str("asr")?;
            }
            Self::Ror => {
                formatter.write_str("ror")?;
            }
        }
        Ok(())
    }
}
impl Coproc {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::P0 => {
                formatter.write_str("p0")?;
            }
            Self::P1 => {
                formatter.write_str("p1")?;
            }
            Self::P2 => {
                formatter.write_str("p2")?;
            }
            Self::P3 => {
                formatter.write_str("p3")?;
            }
            Self::P4 => {
                formatter.write_str("p4")?;
            }
            Self::P5 => {
                formatter.write_str("p5")?;
            }
            Self::P6 => {
                formatter.write_str("p6")?;
            }
            Self::P7 => {
                formatter.write_str("p7")?;
            }
            Self::P8 => {
                formatter.write_str("p8")?;
            }
            Self::P9 => {
                formatter.write_str("p9")?;
            }
            Self::P10 => {
                formatter.write_str("p10")?;
            }
            Self::P11 => {
                formatter.write_str("p11")?;
            }
            Self::P12 => {
                formatter.write_str("p12")?;
            }
            Self::P13 => {
                formatter.write_str("p13")?;
            }
            Self::P14 => {
                formatter.write_str("p14")?;
            }
            Self::P15 => {
                formatter.write_str("p15")?;
            }
        }
        Ok(())
    }
}
impl CoReg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::C0 => {
                formatter.write_str("c0")?;
            }
            Self::C1 => {
                formatter.write_str("c1")?;
            }
            Self::C2 => {
                formatter.write_str("c2")?;
            }
            Self::C3 => {
                formatter.write_str("c3")?;
            }
            Self::C4 => {
                formatter.write_str("c4")?;
            }
            Self::C5 => {
                formatter.write_str("c5")?;
            }
            Self::C6 => {
                formatter.write_str("c6")?;
            }
            Self::C7 => {
                formatter.write_str("c7")?;
            }
            Self::C8 => {
                formatter.write_str("c8")?;
            }
            Self::C9 => {
                formatter.write_str("c9")?;
            }
            Self::C10 => {
                formatter.write_str("c10")?;
            }
            Self::C11 => {
                formatter.write_str("c11")?;
            }
            Self::C12 => {
                formatter.write_str("c12")?;
            }
            Self::C13 => {
                formatter.write_str("c13")?;
            }
            Self::C14 => {
                formatter.write_str("c14")?;
            }
            Self::C15 => {
                formatter.write_str("c15")?;
            }
        }
        Ok(())
    }
}
impl Op2 {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Imm(imm) => {
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Self::ShiftReg { rm, shift_op, rs } => {
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_shift_op(*shift_op)?;
                formatter.write_space()?;
                formatter.write_reg(*rs)?;
            }
            Self::ShiftImm { rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    formatter.write_reg(*rm)?;
                } else {
                    if *imm == 0 && *shift_op == ShiftOp::Ror {
                        formatter.write_reg(*rm)?;
                        formatter.write_separator()?;
                        formatter.write_str("rrx")?;
                    } else {
                        formatter.write_reg(*rm)?;
                        formatter.write_separator()?;
                        formatter.write_shift_op(*shift_op)?;
                        formatter.write_space()?;
                        formatter.write_str("#")?;
                        formatter.write_uimm(*imm)?;
                    }
                }
            }
        }
        Ok(())
    }
}
impl CpsEffect {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::SetMode => {}
            Self::Ie => {
                formatter.write_str("ie")?;
            }
            Self::Id => {
                formatter.write_str("id")?;
            }
        }
        Ok(())
    }
}
impl AifFlags {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        let Self { a, i, f } = self;
        if *a {
            formatter.write_str("a")?;
        }
        if *i {
            formatter.write_str("i")?;
        }
        if *f {
            formatter.write_str("f")?;
        }
        Ok(())
    }
}
impl AddrLdcStc {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Pre { rn, offset, writeback } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_simm(*offset)?;
                formatter.write_str("]")?;
                formatter.write_wb(*writeback)?;
            }
            Self::Post { rn, offset } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("], #")?;
                formatter.write_simm(*offset)?;
            }
            Self::Unidx { rn, option } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("], {")?;
                formatter.write_uimm(*option)?;
                formatter.write_str("}")?;
            }
        }
        Ok(())
    }
}
impl LdmStmMode {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Da => {
                formatter.write_str("da")?;
            }
            Self::Ia => {
                if options.ual {} else {
                    formatter.write_str("ia")?;
                }
            }
            Self::Db => {
                formatter.write_str("db")?;
            }
            Self::Ib => {
                formatter.write_str("ib")?;
            }
        }
        Ok(())
    }
}
impl AddrLdrStr {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Pre { rn, offset, writeback } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_ldr_str_offset(*offset)?;
                formatter.write_str("]")?;
                formatter.write_wb(*writeback)?;
            }
            Self::Post { rn, offset } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("], ")?;
                formatter.write_ldr_str_offset(*offset)?;
            }
        }
        Ok(())
    }
}
impl LdrStrOffset {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Self::Imm(offset) => {
                formatter.write_str("#")?;
                formatter.write_simm(*offset)?;
            }
            Self::Reg { subtract, rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    formatter.write_subtract(*subtract)?;
                    formatter.write_reg(*rm)?;
                } else {
                    if *imm == 0 && *shift_op == ShiftOp::Ror {
                        formatter.write_subtract(*subtract)?;
                        formatter.write_reg(*rm)?;
                        formatter.write_separator()?;
                        formatter.write_str("rrx")?;
                    } else {
                        formatter.write_subtract(*subtract)?;
                        formatter.write_reg(*rm)?;
                        formatter.write_separator()?;
                        formatter.write_shift_op(*shift_op)?;
                        formatter.write_space()?;
                        formatter.write_str("#")?;
                        formatter.write_uimm(*imm)?;
                    }
                }
            }
        }
        Ok(())
    }
}
impl Ins {
    pub fn write_opcode<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                if options.ual {
                    formatter.write_str("adc")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("adc")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                if options.ual {
                    formatter.write_str("add")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("add")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                if options.ual {
                    formatter.write_str("and")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("and")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::B { cond, target } => {
                formatter.write_str("b")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Bic { s, cond, rd, rn, op2 } => {
                if options.ual {
                    formatter.write_str("bic")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("bic")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Bkpt { imm } => {
                formatter.write_str("bkpt")?;
            }
            Ins::Bl { cond, target } => {
                formatter.write_str("bl")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Blx { cond, target } => {
                formatter.write_str("blx")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Bx { cond, rm } => {
                formatter.write_str("bx")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Bxj { cond, rm } => {
                formatter.write_str("bxj")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                formatter.write_str("cdp")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                formatter.write_str("cdp2")?;
            }
            Ins::Clrex {} => {
                formatter.write_str("clrex")?;
            }
            Ins::Clz { cond, rd, rm } => {
                formatter.write_str("clz")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Cmn { cond, rn, op2 } => {
                formatter.write_str("cmn")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Cmp { cond, rn, op2 } => {
                formatter.write_str("cmp")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Cps { effect, aif, mode } => {
                formatter.write_str("cps")?;
                formatter.write_cps_effect(*effect)?;
            }
            Ins::Csdb { cond } => {
                formatter.write_str("csdb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Eor { s, cond, rd, rn, op2 } => {
                if options.ual {
                    formatter.write_str("eor")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("eor")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                if options.ual {
                    formatter.write_str("ldc")?;
                    formatter.write_l(*l)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldc")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_l(*l)?;
                }
            }
            Ins::Ldc2 { l, coproc, crd, dest } => {
                formatter.write_str("ldc2")?;
                formatter.write_l(*l)?;
            }
            Ins::Ldm { mode, cond, rn, writeback, regs, user_mode } => {
                if options.ual {
                    formatter.write_str("ldm")?;
                    formatter.write_ldm_stm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldm")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_ldm_stm_mode(*mode)?;
                }
            }
            Ins::Ldr { cond, rd, addr } => {
                formatter.write_str("ldr")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Illegal => {
                formatter.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
    pub fn write_params<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let options = formatter.options();
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::And { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::B { cond, target } => {
                formatter.write_space()?;
                formatter.write_branch_target(*target)?;
            }
            Ins::Bic { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Bkpt { imm } => {
                formatter.write_space()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Ins::Bl { cond, target } => {
                formatter.write_space()?;
                formatter.write_branch_target(*target)?;
            }
            Ins::Blx { cond, target } => {
                formatter.write_space()?;
                formatter.write_blx_target(*target)?;
            }
            Ins::Bx { cond, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Bxj { cond, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Clrex {} => {}
            Ins::Clz { cond, rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Cmn { cond, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Cmp { cond, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Cps { effect, aif, mode } => {
                formatter.write_space()?;
                if *effect == CpsEffect::SetMode {
                    formatter.write_str("#")?;
                    formatter.write_uimm(*mode)?;
                } else {
                    if *mode == 0 {
                        formatter.write_aif_flags(*aif)?;
                    } else {
                        formatter.write_aif_flags(*aif)?;
                        formatter.write_separator()?;
                        formatter.write_str("#")?;
                        formatter.write_uimm(*mode)?;
                    }
                }
            }
            Ins::Csdb { cond } => {}
            Ins::Eor { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldc_stc(*dest)?;
            }
            Ins::Ldc2 { l, coproc, crd, dest } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldc_stc(*dest)?;
            }
            Ins::Ldm { mode, cond, rn, writeback, regs, user_mode } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_reg_list(*regs)?;
                formatter.write_user_mode(*user_mode)?;
            }
            Ins::Ldr { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::Illegal => {
                formatter.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
}

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::needless_else)]
#![allow(clippy::explicit_auto_deref)]
#![allow(unused_variables)]
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
    fn write_thumb(&mut self, thumb: bool) -> core::fmt::Result {
        if thumb {
            self.write_str("thumb")?;
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
    fn write_status_reg(&mut self, status_reg: StatusReg) -> core::fmt::Result {
        status_reg.write(self)?;
        Ok(())
    }
    fn write_status_fields(&mut self, status_fields: StatusFields) -> core::fmt::Result {
        status_fields.write(self)?;
        Ok(())
    }
    fn write_msr_op2(&mut self, msr_op2: MsrOp2) -> core::fmt::Result {
        msr_op2.write(self)?;
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
    fn write_shift_reg(&mut self, shift_reg: ShiftReg) -> core::fmt::Result {
        shift_reg.write(self)?;
        Ok(())
    }
    fn write_shift_imm(&mut self, shift_imm: ShiftImm) -> core::fmt::Result {
        shift_imm.write(self)?;
        Ok(())
    }
    fn write_op2_shift(&mut self, op2_shift: Op2Shift) -> core::fmt::Result {
        op2_shift.write(self)?;
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
    fn write_addr_ldr_str_post(
        &mut self,
        addr_ldr_str_post: AddrLdrStrPost,
    ) -> core::fmt::Result {
        addr_ldr_str_post.write(self)?;
        Ok(())
    }
    fn write_ldr_str_offset(
        &mut self,
        ldr_str_offset: LdrStrOffset,
    ) -> core::fmt::Result {
        ldr_str_offset.write(self)?;
        Ok(())
    }
    fn write_addr_misc_load(
        &mut self,
        addr_misc_load: AddrMiscLoad,
    ) -> core::fmt::Result {
        addr_misc_load.write(self)?;
        Ok(())
    }
    fn write_misc_load_offset(
        &mut self,
        misc_load_offset: MiscLoadOffset,
    ) -> core::fmt::Result {
        misc_load_offset.write(self)?;
        Ok(())
    }
    fn write_srs_rfe_mode(&mut self, srs_rfe_mode: SrsRfeMode) -> core::fmt::Result {
        srs_rfe_mode.write(self)?;
        Ok(())
    }
    fn write_endianness(&mut self, endianness: Endianness) -> core::fmt::Result {
        endianness.write(self)?;
        Ok(())
    }
    fn write_reg_side(&mut self, reg_side: RegSide) -> core::fmt::Result {
        reg_side.write(self)?;
        Ok(())
    }
    fn write_swap_rm(&mut self, swap_rm: bool) -> core::fmt::Result {
        if swap_rm {
            self.write_str("x")?;
        }
        Ok(())
    }
    fn write_round(&mut self, round: bool) -> core::fmt::Result {
        if round {
            self.write_str("r")?;
        }
        Ok(())
    }
    fn write_round_zero(&mut self, round_zero: bool) -> core::fmt::Result {
        if round_zero {
            if self.options().ual {
                self.write_str("r")?;
            } else {
                self.write_str("z")?;
            }
        }
        Ok(())
    }
    fn write_sreg(&mut self, sreg: Sreg) -> core::fmt::Result {
        sreg.write(self)?;
        Ok(())
    }
    fn write_dreg(&mut self, dreg: Dreg) -> core::fmt::Result {
        dreg.write(self)?;
        Ok(())
    }
    fn write_quiet_nan_exc(&mut self, quiet_nan_exc: bool) -> core::fmt::Result {
        if quiet_nan_exc {
            self.write_str("e")?;
        }
        Ok(())
    }
    fn write_vcmp_f32_op2(&mut self, vcmp_f32_op2: VcmpF32Op2) -> core::fmt::Result {
        vcmp_f32_op2.write(self)?;
        Ok(())
    }
    fn write_vcmp_f64_op2(&mut self, vcmp_f64_op2: VcmpF64Op2) -> core::fmt::Result {
        vcmp_f64_op2.write(self)?;
        Ok(())
    }
    fn write_sreg_list(&mut self, sreg_list: SregList) -> core::fmt::Result {
        sreg_list.write(self)?;
        Ok(())
    }
    fn write_dreg_list(&mut self, dreg_list: DregList) -> core::fmt::Result {
        dreg_list.write(self)?;
        Ok(())
    }
    fn write_dreg_index(&mut self, dreg_index: DregIndex) -> core::fmt::Result {
        dreg_index.write(self)?;
        Ok(())
    }
    fn write_fpscr(&mut self, fpscr: Fpscr) -> core::fmt::Result {
        fpscr.write(self)?;
        Ok(())
    }
    fn write_vldm_vstm_mode(
        &mut self,
        vldm_vstm_mode: VldmVstmMode,
    ) -> core::fmt::Result {
        vldm_vstm_mode.write(self)?;
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
        match self {
            Self::R0 => {
                if formatter.options().av {
                    formatter.write_str("a1")?;
                } else {
                    formatter.write_str("r0")?;
                }
            }
            Self::R1 => {
                if formatter.options().av {
                    formatter.write_str("a2")?;
                } else {
                    formatter.write_str("r1")?;
                }
            }
            Self::R2 => {
                if formatter.options().av {
                    formatter.write_str("a3")?;
                } else {
                    formatter.write_str("r2")?;
                }
            }
            Self::R3 => {
                if formatter.options().av {
                    formatter.write_str("a4")?;
                } else {
                    formatter.write_str("r3")?;
                }
            }
            Self::R4 => {
                if formatter.options().av {
                    formatter.write_str("v1")?;
                } else {
                    formatter.write_str("r4")?;
                }
            }
            Self::R5 => {
                if formatter.options().av {
                    formatter.write_str("v2")?;
                } else {
                    formatter.write_str("r5")?;
                }
            }
            Self::R6 => {
                if formatter.options().av {
                    formatter.write_str("v3")?;
                } else {
                    formatter.write_str("r6")?;
                }
            }
            Self::R7 => {
                if formatter.options().av {
                    formatter.write_str("v4")?;
                } else {
                    formatter.write_str("r7")?;
                }
            }
            Self::R8 => {
                if formatter.options().av {
                    formatter.write_str("v5")?;
                } else {
                    formatter.write_str("r8")?;
                }
            }
            Self::R9 => {
                match formatter.options().r9_use {
                    R9Use::R9 => {
                        if formatter.options().av {
                            formatter.write_str("v6")?;
                        } else {
                            formatter.write_str("r9")?;
                        }
                    }
                    R9Use::Sb => {
                        formatter.write_str("sb")?;
                    }
                    R9Use::Tr => {
                        formatter.write_str("tr")?;
                    }
                }
            }
            Self::R10 => {
                if formatter.options().sl {
                    formatter.write_str("sl")?;
                } else {
                    if formatter.options().av {
                        formatter.write_str("v7")?;
                    } else {
                        formatter.write_str("r10")?;
                    }
                }
            }
            Self::R11 => {
                if formatter.options().fp {
                    formatter.write_str("fp")?;
                } else {
                    if formatter.options().av {
                        formatter.write_str("v8")?;
                    } else {
                        formatter.write_str("r11")?;
                    }
                }
            }
            Self::R12 => {
                if formatter.options().ip {
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
impl StatusReg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Cpsr => {
                formatter.write_str("cpsr")?;
            }
            Self::Spsr => {
                formatter.write_str("spsr")?;
            }
        }
        Ok(())
    }
}
impl StatusFields {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self { reg, c, x, s, f } = self;
        formatter.write_status_reg(*reg)?;
        formatter.write_str("_")?;
        if *f {
            formatter.write_str("f")?;
        }
        if *x {
            formatter.write_str("x")?;
        }
        if *s {
            formatter.write_str("s")?;
        }
        if *c {
            formatter.write_str("c")?;
        }
        Ok(())
    }
}
impl MsrOp2 {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Imm(imm) => {
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Self::Reg(reg) => {
                formatter.write_reg(*reg)?;
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
        match self {
            Self::Imm(imm) => {
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Self::ShiftReg(shift_reg) => {
                formatter.write_shift_reg(*shift_reg)?;
            }
            Self::ShiftImm(shift_imm) => {
                formatter.write_shift_imm(*shift_imm)?;
            }
        }
        Ok(())
    }
}
impl ShiftReg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self { rm, shift_op, rs } = self;
        formatter.write_reg(*rm)?;
        formatter.write_separator()?;
        formatter.write_shift_op(*shift_op)?;
        formatter.write_space()?;
        formatter.write_reg(*rs)?;
        Ok(())
    }
}
impl ShiftImm {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self { rm, shift_op, imm } = self;
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
        Ok(())
    }
}
impl Op2Shift {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Imm(imm) => {
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Self::Reg(reg) => {
                formatter.write_reg(*reg)?;
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
                formatter.write_str("]")?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_simm(*offset)?;
            }
            Self::Unidx { rn, option } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
                formatter.write_separator()?;
                formatter.write_str("{")?;
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
        match self {
            Self::Da => {
                formatter.write_str("da")?;
            }
            Self::Ia => {
                if formatter.options().ual {} else {
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
        match self {
            Self::Pre { rn, offset, writeback } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_ldr_str_offset(*offset)?;
                formatter.write_str("]")?;
                formatter.write_wb(*writeback)?;
            }
            Self::Post(addr_ldr_str_post) => {
                formatter.write_addr_ldr_str_post(*addr_ldr_str_post)?;
            }
        }
        Ok(())
    }
}
impl AddrLdrStrPost {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self { rn, offset } = self;
        formatter.write_str("[")?;
        formatter.write_reg(*rn)?;
        formatter.write_str("]")?;
        formatter.write_separator()?;
        formatter.write_ldr_str_offset(*offset)?;
        Ok(())
    }
}
impl LdrStrOffset {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
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
impl AddrMiscLoad {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Pre { rn, offset, writeback } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_misc_load_offset(*offset)?;
                formatter.write_str("]")?;
                formatter.write_wb(*writeback)?;
            }
            Self::Post { rn, offset } => {
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
                formatter.write_separator()?;
                formatter.write_misc_load_offset(*offset)?;
            }
        }
        Ok(())
    }
}
impl MiscLoadOffset {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Imm(offset) => {
                formatter.write_str("#")?;
                formatter.write_simm(*offset)?;
            }
            Self::Reg { rm, subtract } => {
                formatter.write_subtract(*subtract)?;
                formatter.write_reg(*rm)?;
            }
        }
        Ok(())
    }
}
impl SrsRfeMode {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Da => {
                formatter.write_str("da")?;
            }
            Self::Ia => {
                formatter.write_str("ia")?;
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
impl Endianness {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Le => {
                formatter.write_str("le")?;
            }
            Self::Be => {
                formatter.write_str("be")?;
            }
        }
        Ok(())
    }
}
impl RegSide {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Bottom => {
                formatter.write_str("b")?;
            }
            Self::Top => {
                formatter.write_str("t")?;
            }
        }
        Ok(())
    }
}
impl Sreg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::S0 => {
                formatter.write_str("s0")?;
            }
            Self::S1 => {
                formatter.write_str("s1")?;
            }
            Self::S2 => {
                formatter.write_str("s2")?;
            }
            Self::S3 => {
                formatter.write_str("s3")?;
            }
            Self::S4 => {
                formatter.write_str("s4")?;
            }
            Self::S5 => {
                formatter.write_str("s5")?;
            }
            Self::S6 => {
                formatter.write_str("s6")?;
            }
            Self::S7 => {
                formatter.write_str("s7")?;
            }
            Self::S8 => {
                formatter.write_str("s8")?;
            }
            Self::S9 => {
                formatter.write_str("s9")?;
            }
            Self::S10 => {
                formatter.write_str("s10")?;
            }
            Self::S11 => {
                formatter.write_str("s11")?;
            }
            Self::S12 => {
                formatter.write_str("s12")?;
            }
            Self::S13 => {
                formatter.write_str("s13")?;
            }
            Self::S14 => {
                formatter.write_str("s14")?;
            }
            Self::S15 => {
                formatter.write_str("s15")?;
            }
            Self::S16 => {
                formatter.write_str("s16")?;
            }
            Self::S17 => {
                formatter.write_str("s17")?;
            }
            Self::S18 => {
                formatter.write_str("s18")?;
            }
            Self::S19 => {
                formatter.write_str("s19")?;
            }
            Self::S20 => {
                formatter.write_str("s20")?;
            }
            Self::S21 => {
                formatter.write_str("s21")?;
            }
            Self::S22 => {
                formatter.write_str("s22")?;
            }
            Self::S23 => {
                formatter.write_str("s23")?;
            }
            Self::S24 => {
                formatter.write_str("s24")?;
            }
            Self::S25 => {
                formatter.write_str("s25")?;
            }
            Self::S26 => {
                formatter.write_str("s26")?;
            }
            Self::S27 => {
                formatter.write_str("s27")?;
            }
            Self::S28 => {
                formatter.write_str("s28")?;
            }
            Self::S29 => {
                formatter.write_str("s29")?;
            }
            Self::S30 => {
                formatter.write_str("s30")?;
            }
            Self::S31 => {
                formatter.write_str("s31")?;
            }
        }
        Ok(())
    }
}
impl Dreg {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::D0 => {
                formatter.write_str("d0")?;
            }
            Self::D1 => {
                formatter.write_str("d1")?;
            }
            Self::D2 => {
                formatter.write_str("d2")?;
            }
            Self::D3 => {
                formatter.write_str("d3")?;
            }
            Self::D4 => {
                formatter.write_str("d4")?;
            }
            Self::D5 => {
                formatter.write_str("d5")?;
            }
            Self::D6 => {
                formatter.write_str("d6")?;
            }
            Self::D7 => {
                formatter.write_str("d7")?;
            }
            Self::D8 => {
                formatter.write_str("d8")?;
            }
            Self::D9 => {
                formatter.write_str("d9")?;
            }
            Self::D10 => {
                formatter.write_str("d10")?;
            }
            Self::D11 => {
                formatter.write_str("d11")?;
            }
            Self::D12 => {
                formatter.write_str("d12")?;
            }
            Self::D13 => {
                formatter.write_str("d13")?;
            }
            Self::D14 => {
                formatter.write_str("d14")?;
            }
            Self::D15 => {
                formatter.write_str("d15")?;
            }
            Self::D16 => {
                formatter.write_str("d16")?;
            }
            Self::D17 => {
                formatter.write_str("d17")?;
            }
            Self::D18 => {
                formatter.write_str("d18")?;
            }
            Self::D19 => {
                formatter.write_str("d19")?;
            }
            Self::D20 => {
                formatter.write_str("d20")?;
            }
            Self::D21 => {
                formatter.write_str("d21")?;
            }
            Self::D22 => {
                formatter.write_str("d22")?;
            }
            Self::D23 => {
                formatter.write_str("d23")?;
            }
            Self::D24 => {
                formatter.write_str("d24")?;
            }
            Self::D25 => {
                formatter.write_str("d25")?;
            }
            Self::D26 => {
                formatter.write_str("d26")?;
            }
            Self::D27 => {
                formatter.write_str("d27")?;
            }
            Self::D28 => {
                formatter.write_str("d28")?;
            }
            Self::D29 => {
                formatter.write_str("d29")?;
            }
            Self::D30 => {
                formatter.write_str("d30")?;
            }
            Self::D31 => {
                formatter.write_str("d31")?;
            }
        }
        Ok(())
    }
}
impl VcmpF32Op2 {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Zero => {
                formatter.write_str("#0.0")?;
            }
            Self::Reg(sm) => {
                formatter.write_sreg(*sm)?;
            }
        }
        Ok(())
    }
}
impl VcmpF64Op2 {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Zero => {
                formatter.write_str("#0.0")?;
            }
            Self::Reg(dm) => {
                formatter.write_dreg(*dm)?;
            }
        }
        Ok(())
    }
}
impl DregIndex {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self { dreg, index } = self;
        if formatter.options().ual {
            formatter.write_dreg(*dreg)?;
            formatter.write_str("[")?;
            formatter.write_uimm(*index)?;
            formatter.write_str("]")?;
        } else {
            formatter.write_dreg(*dreg)?;
        }
        Ok(())
    }
}
impl Fpscr {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        let Self {} = self;
        formatter.write_str("fpscr")?;
        Ok(())
    }
}
impl VldmVstmMode {
    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Self::Ia => {
                formatter.write_str("ia")?;
            }
            Self::Db => {
                formatter.write_str("db")?;
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
        match self {
            Ins::Illegal => formatter.write_str("<illegal>")?,
            Ins::Adc { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("adc")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("adc")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Add { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("add")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("add")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::And { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("and")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("and")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Asr { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("asr")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("asr")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::B { cond, target } => {
                formatter.write_str("b")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Bic { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("bic")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("bic")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
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
            Ins::Dbg { cond, option } => {
                formatter.write_str("dbg")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Eor { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("eor")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("eor")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                if formatter.options().ual {
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
                if formatter.options().ual {
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
            Ins::Ldrb { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrb")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("b")?;
                }
            }
            Ins::Ldrbt { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrbt")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("bt")?;
                }
            }
            Ins::Ldrd { cond, rd, rd2, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrd")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("d")?;
                }
            }
            Ins::Ldrex { cond, rd, rn } => {
                formatter.write_str("ldrex")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ldrexb { cond, rd, rn } => {
                formatter.write_str("ldrexb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                formatter.write_str("ldrexd")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ldrexh { cond, rd, rn } => {
                formatter.write_str("ldrexh")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ldrh { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrh")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("h")?;
                }
            }
            Ins::Ldrsb { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrsb")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("sb")?;
                }
            }
            Ins::Ldrsh { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrsh")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("sh")?;
                }
            }
            Ins::Ldrt { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("ldrt")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ldr")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("t")?;
                }
            }
            Ins::Lsl { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("lsl")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("lsl")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Lsr { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_str("lsr")?;
                formatter.write_s(*s)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_str("mcr")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_str("mcr2")?;
            }
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                formatter.write_str("mcrr")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                formatter.write_str("mcrr2")?;
            }
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                if formatter.options().ual {
                    formatter.write_str("mla")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("mla")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("mov")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("mov")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_str("mrc")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_str("mrc2")?;
            }
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                formatter.write_str("mrrc")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                formatter.write_str("mrrc2")?;
            }
            Ins::Mrs { cond, rd, status_reg } => {
                formatter.write_str("mrs")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Msr { cond, status_fields, op2 } => {
                formatter.write_str("msr")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Mul { s, thumb, cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("mul")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("mul")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Mvn { s, thumb, cond, rd, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("mvn")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("mvn")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Neg { rd, rm } => {
                formatter.write_str("neg")?;
            }
            Ins::Nop { cond } => {
                formatter.write_str("nop")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("orr")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("orr")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                formatter.write_str("pkhbt")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                formatter.write_str("pkhtb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Pld { addr } => {
                formatter.write_str("pld")?;
            }
            Ins::Pop { cond, regs } => {
                formatter.write_str("pop")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Push { cond, regs } => {
                formatter.write_str("push")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qadd { cond, rd, rm, rn } => {
                formatter.write_str("qadd")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qadd16 { cond, rd, rn, rm } => {
                formatter.write_str("qadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qadd8 { cond, rd, rn, rm } => {
                formatter.write_str("qadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qasx { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("qasx")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("qaddsubx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Qdadd { cond, rd, rm, rn } => {
                formatter.write_str("qdadd")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qdsub { cond, rd, rm, rn } => {
                formatter.write_str("qdsub")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qsax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("qsax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("qsubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Qsub { cond, rd, rm, rn } => {
                formatter.write_str("qsub")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qsub16 { cond, rd, rn, rm } => {
                formatter.write_str("qsub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Qsub8 { cond, rd, rn, rm } => {
                formatter.write_str("qsub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Rev { cond, rd, rm } => {
                formatter.write_str("rev")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Rev16 { cond, rd, rm } => {
                formatter.write_str("rev16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Revsh { cond, rd, rm } => {
                formatter.write_str("revsh")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Rfe { addr_mode, rn, writeback } => {
                formatter.write_str("rfe")?;
                formatter.write_srs_rfe_mode(*addr_mode)?;
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("ror")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ror")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Rrx { s, cond, rd, rm } => {
                formatter.write_str("rrx")?;
                formatter.write_s(*s)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("rsb")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("rsb")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("rsc")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("rsc")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Sadd16 { cond, rd, rn, rm } => {
                formatter.write_str("sadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sadd8 { cond, rd, rn, rm } => {
                formatter.write_str("sadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sasx { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("sasx")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("saddsubx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("sbc")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("sbc")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Sel { cond, rd, rn, rm } => {
                formatter.write_str("sel")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Setend { endian } => {
                formatter.write_str("setend")?;
            }
            Ins::Sev { cond } => {
                formatter.write_str("sev")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Shadd16 { cond, rd, rn, rm } => {
                formatter.write_str("shadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Shadd8 { cond, rd, rn, rm } => {
                formatter.write_str("shadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Shasx { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("shasx")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("shaddsubx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Shsax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("shsax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("shsubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Shsub16 { cond, rd, rn, rm } => {
                formatter.write_str("shsub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Shsub8 { cond, rd, rn, rm } => {
                formatter.write_str("shsub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                formatter.write_str("smla")?;
                formatter.write_reg_side(*rn_side)?;
                formatter.write_reg_side(*rm_side)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                formatter.write_str("smlad")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("smlal")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("smlal")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                formatter.write_str("smlal")?;
                formatter.write_reg_side(*rn_side)?;
                formatter.write_reg_side(*rm_side)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                formatter.write_str("smlald")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                formatter.write_str("smlaw")?;
                formatter.write_reg_side(*rm_side)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                formatter.write_str("smlsd")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                formatter.write_str("smlsld")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                formatter.write_str("smmla")?;
                formatter.write_round(*round)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                formatter.write_str("smmls")?;
                formatter.write_round(*round)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smmul { round, cond, rd, rn, rm } => {
                formatter.write_str("smmul")?;
                formatter.write_round(*round)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                formatter.write_str("smuad")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                formatter.write_str("smul")?;
                formatter.write_reg_side(*rn_side)?;
                formatter.write_reg_side(*rm_side)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("smull")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("smull")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                formatter.write_str("smulw")?;
                formatter.write_reg_side(*rm_side)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                formatter.write_str("smusd")?;
                formatter.write_swap_rm(*swap_rm)?;
                formatter.write_cond(*cond)?;
            }
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                formatter.write_str("srs")?;
                formatter.write_srs_rfe_mode(*addr_mode)?;
            }
            Ins::Ssat { cond, rd, imm, op2 } => {
                formatter.write_str("ssat")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ssat16 { cond, rd, imm, rn } => {
                formatter.write_str("ssat16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ssax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("ssax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("ssubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Ssub16 { cond, rd, rn, rm } => {
                formatter.write_str("ssub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Ssub8 { cond, rd, rn, rm } => {
                formatter.write_str("ssub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Stc { l, cond, coproc, crd, dest } => {
                if formatter.options().ual {
                    formatter.write_str("stc")?;
                    formatter.write_l(*l)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("stc")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_l(*l)?;
                }
            }
            Ins::Stc2 { l, coproc, crd, dest } => {
                formatter.write_str("stc2")?;
                formatter.write_l(*l)?;
            }
            Ins::Stm { mode, cond, rn, writeback, regs, user_mode } => {
                if formatter.options().ual {
                    formatter.write_str("stm")?;
                    formatter.write_ldm_stm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("stm")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_ldm_stm_mode(*mode)?;
                }
            }
            Ins::Str { cond, rd, addr } => {
                formatter.write_str("str")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Strb { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("strb")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("str")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("b")?;
                }
            }
            Ins::Strbt { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("strbt")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("str")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("bt")?;
                }
            }
            Ins::Strd { cond, rd, rd2, addr } => {
                if formatter.options().ual {
                    formatter.write_str("strd")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("str")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("d")?;
                }
            }
            Ins::Strex { cond, rd, rm, rn } => {
                formatter.write_str("strex")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Strexb { cond, rd, rm, rn } => {
                formatter.write_str("strexb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                formatter.write_str("strexd")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Strexh { cond, rd, rm, rn } => {
                formatter.write_str("strexh")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Strh { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("strh")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("str")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("h")?;
                }
            }
            Ins::Strt { cond, rd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("strt")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("str")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str("t")?;
                }
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("sub")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("sub")?;
                    formatter.write_cond(*cond)?;
                    if !*thumb {
                        formatter.write_s(*s)?;
                    } else {}
                }
            }
            Ins::Svc { cond, imm } => {
                if formatter.options().ual {
                    formatter.write_str("svc")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("swi")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Swp { cond, rd, rd2, rn } => {
                formatter.write_str("swp")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Swpb { cond, rd, rd2, rn } => {
                formatter.write_str("swpb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                formatter.write_str("sxtab")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                formatter.write_str("sxtab16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                formatter.write_str("sxtah")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxtb { cond, rd, rm, rotate } => {
                formatter.write_str("sxtb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                formatter.write_str("sxtb16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Sxth { cond, rd, rm, rotate } => {
                formatter.write_str("sxth")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Teq { cond, rn, op2 } => {
                formatter.write_str("teq")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Tst { cond, rn, op2 } => {
                formatter.write_str("tst")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uadd16 { cond, rd, rn, rm } => {
                formatter.write_str("uadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uadd8 { cond, rd, rn, rm } => {
                formatter.write_str("uadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uasx { cond, rd, rn, rm } => {
                formatter.write_str("uasx")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Udf { imm } => {
                formatter.write_str("udf")?;
            }
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                formatter.write_str("uhadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                formatter.write_str("uhadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uhasx { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("uhasx")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("uhaddsubx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Uhsax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("uhsax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("uhsubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                formatter.write_str("uhsub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                formatter.write_str("uhsub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_str("umaal")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("umlal")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("umlal")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("umull")?;
                    formatter.write_s(*s)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("umull")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_s(*s)?;
                }
            }
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                formatter.write_str("uqadd16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                formatter.write_str("uqadd8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uqasx { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("uqasx")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("uqaddsubx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Uqsax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("uqsax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("uqsubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                formatter.write_str("uqsub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                formatter.write_str("uqsub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usad8 { cond, rd, rn, rm } => {
                formatter.write_str("usad8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                formatter.write_str("usada8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usat { cond, rd, imm, op2 } => {
                formatter.write_str("usat")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usat16 { cond, rd, imm, rn } => {
                formatter.write_str("usat16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usax { cond, rd, rn, rm } => {
                if formatter.options().ual {
                    formatter.write_str("usax")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("usubaddx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Usub16 { cond, rd, rn, rm } => {
                formatter.write_str("usub16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Usub8 { cond, rd, rn, rm } => {
                formatter.write_str("usub8")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                formatter.write_str("uxtab")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                formatter.write_str("uxtab16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                formatter.write_str("uxtah")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxtb { cond, rd, rm, rotate } => {
                formatter.write_str("uxtb")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                formatter.write_str("uxtb16")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Uxth { cond, rd, rm, rotate } => {
                formatter.write_str("uxth")?;
                formatter.write_cond(*cond)?;
            }
            Ins::VabsF32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vabs")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fabss")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VabsF64 { cond, dd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vabs")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fabsd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VaddF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vadd")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fadds")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VaddF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vadd")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("faddd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcmpF32 { quiet_nan_exc, cond, sd, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("vcmp")?;
                    formatter.write_quiet_nan_exc(*quiet_nan_exc)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fcmp")?;
                    formatter.write_quiet_nan_exc(*quiet_nan_exc)?;
                    if *op2 == VcmpF32Op2::Zero {
                        formatter.write_str("z")?;
                    } else {}
                    formatter.write_str("s")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcmpF64 { quiet_nan_exc, cond, dd, op2 } => {
                if formatter.options().ual {
                    formatter.write_str("vcmp")?;
                    formatter.write_quiet_nan_exc(*quiet_nan_exc)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fcmp")?;
                    formatter.write_quiet_nan_exc(*quiet_nan_exc)?;
                    if *op2 == VcmpF64Op2::Zero {
                        formatter.write_str("z")?;
                    } else {}
                    formatter.write_str("d")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF32F64 { cond, sd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32.f64")?;
                } else {
                    formatter.write_str("fcvtsd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF32S32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32.s32")?;
                } else {
                    formatter.write_str("fsitos")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF32U32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32.u32")?;
                } else {
                    formatter.write_str("fuitos")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF64F32 { cond, dd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64.f32")?;
                } else {
                    formatter.write_str("fcvtds")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF64S32 { cond, dd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64.s32")?;
                } else {
                    formatter.write_str("fsitod")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtF64U32 { cond, dd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64.u32")?;
                } else {
                    formatter.write_str("fuitod")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".s32.f32")?;
                } else {
                    formatter.write_str("ftosi")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_str("s")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".s32.f64")?;
                } else {
                    formatter.write_str("ftosi")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_str("d")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".u32.f32")?;
                } else {
                    formatter.write_str("ftoui")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_str("s")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vcvt")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".u32.f64")?;
                } else {
                    formatter.write_str("ftoui")?;
                    formatter.write_round_zero(*round_zero)?;
                    formatter.write_str("d")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VdivF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vdiv")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fdivs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VdivF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vdiv")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fdivd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                if formatter.options().ual {
                    formatter.write_str("vldm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fldm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_str("s")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                if formatter.options().ual {
                    formatter.write_str("vldm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fldm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_str("d")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VldrF32 { cond, sd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("vldr")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("flds")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VldrF64 { cond, dd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("vldr")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fldd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vmla")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fmacs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vmla")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fmacd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vmls")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fmscs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vmls")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fmscd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Vmov32Reg { cond, dd, rt } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".32")?;
                } else {
                    if (*dd).index == 0 {
                        formatter.write_str("fmdlr")?;
                        formatter.write_cond(*cond)?;
                    } else {
                        formatter.write_str("fmdhr")?;
                        formatter.write_cond(*cond)?;
                    }
                }
            }
            Ins::VmovF32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fcpys")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovF32Reg { cond, sn, rt } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmsr")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovF64 { cond, dd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fcpyd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovReg32 { cond, rt, dn } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".32")?;
                } else {
                    if (*dn).index == 0 {
                        formatter.write_str("fmrdl")?;
                        formatter.write_cond(*cond)?;
                    } else {
                        formatter.write_str("fmrdh")?;
                        formatter.write_cond(*cond)?;
                    }
                }
            }
            Ins::VmovRegF32 { cond, rt, sn } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmrs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmrrs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmsrr")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmrrd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                if formatter.options().ual {
                    formatter.write_str("vmov")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmdrr")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Vmrs { cond, rd, fpscr } => {
                if formatter.options().ual {
                    formatter.write_str("vmrs")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmrx")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Vmsr { cond, fpscr, rd } => {
                if formatter.options().ual {
                    formatter.write_str("vmsr")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fmxr")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmulF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vmul")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fmuls")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VmulF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vmul")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fmuld")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnegF32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vneg")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fnegs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnegF64 { cond, dd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vneg")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fnegd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmla")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fnmacs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmla")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fnmacd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmls")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fnmscs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmls")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fnmscd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmul")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fnmuls")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vnmul")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fnmuld")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VpopF32 { cond, regs } => {
                formatter.write_str("vpop")?;
                formatter.write_cond(*cond)?;
            }
            Ins::VpopF64 { cond, regs } => {
                formatter.write_str("vpop")?;
                formatter.write_cond(*cond)?;
            }
            Ins::VpushF32 { cond, regs } => {
                formatter.write_str("vpush")?;
                formatter.write_cond(*cond)?;
            }
            Ins::VpushF64 { cond, regs } => {
                formatter.write_str("vpush")?;
                formatter.write_cond(*cond)?;
            }
            Ins::VsqrtF32 { cond, sd, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vsqrt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fsqrts")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VsqrtF64 { cond, dd, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vsqrt")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fsqrtd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                if formatter.options().ual {
                    formatter.write_str("vstm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fstm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_str("s")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                if formatter.options().ual {
                    formatter.write_str("vstm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fstm")?;
                    formatter.write_vldm_vstm_mode(*mode)?;
                    formatter.write_str("d")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VstrF32 { cond, sd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("vstr")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fsts")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VstrF64 { cond, dd, addr } => {
                if formatter.options().ual {
                    formatter.write_str("vstr")?;
                    formatter.write_cond(*cond)?;
                } else {
                    formatter.write_str("fstd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VsubF32 { cond, sd, sn, sm } => {
                if formatter.options().ual {
                    formatter.write_str("vsub")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f32")?;
                } else {
                    formatter.write_str("fsubs")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::VsubF64 { cond, dd, dn, dm } => {
                if formatter.options().ual {
                    formatter.write_str("vsub")?;
                    formatter.write_cond(*cond)?;
                    formatter.write_str(".f64")?;
                } else {
                    formatter.write_str("fsubd")?;
                    formatter.write_cond(*cond)?;
                }
            }
            Ins::Wfe { cond } => {
                formatter.write_str("wfe")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Wfi { cond } => {
                formatter.write_str("wfi")?;
                formatter.write_cond(*cond)?;
            }
            Ins::Yield { cond } => {
                formatter.write_str("yield")?;
                formatter.write_cond(*cond)?;
            }
        }
        Ok(())
    }
    pub fn write_params<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        match self {
            Ins::Illegal => {}
            Ins::Adc { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::Add { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb || *rd != *rn {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::And { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::Asr { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb || *rd != *rn {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                }
            }
            Ins::B { cond, target } => {
                formatter.write_space()?;
                formatter.write_branch_target(*target)?;
            }
            Ins::Bic { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
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
            Ins::Dbg { cond, option } => {
                formatter.write_space()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*option)?;
            }
            Ins::Eor { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
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
            Ins::Ldrb { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::Ldrbt { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str_post(*addr)?;
            }
            Ins::Ldrd { cond, rd, rd2, addr } => {
                formatter.write_space()?;
                if formatter.options().ual {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rd2)?;
                    formatter.write_separator()?;
                    formatter.write_addr_misc_load(*addr)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_addr_misc_load(*addr)?;
                }
            }
            Ins::Ldrex { cond, rd, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Ldrexb { cond, rd, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Ldrexh { cond, rd, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Ldrh { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_misc_load(*addr)?;
            }
            Ins::Ldrsb { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_misc_load(*addr)?;
            }
            Ins::Ldrsh { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_misc_load(*addr)?;
            }
            Ins::Ldrt { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str_post(*addr)?;
            }
            Ins::Lsl { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb || *rd != *rn {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                }
            }
            Ins::Lsr { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb || *rd != *rn {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                }
            }
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
            }
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
            }
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc1)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crn)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc2)?;
            }
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
            }
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*opc)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crm)?;
            }
            Ins::Mrs { cond, rd, status_reg } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_status_reg(*status_reg)?;
            }
            Ins::Msr { cond, status_fields, op2 } => {
                formatter.write_space()?;
                formatter.write_status_fields(*status_fields)?;
                formatter.write_separator()?;
                formatter.write_msr_op2(*op2)?;
            }
            Ins::Mul { s, thumb, cond, rd, rn, rm } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rm)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                }
            }
            Ins::Mvn { s, thumb, cond, rd, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Neg { rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Nop { cond } => {}
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *shift != 0 {
                    formatter.write_separator()?;
                    formatter.write_shift_op(*shift_op)?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*shift)?;
                } else {}
            }
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_shift_op(*shift_op)?;
                formatter.write_space()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*shift)?;
            }
            Ins::Pld { addr } => {
                formatter.write_space()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::Pop { cond, regs } => {
                formatter.write_space()?;
                formatter.write_reg_list(*regs)?;
            }
            Ins::Push { cond, regs } => {
                formatter.write_space()?;
                formatter.write_reg_list(*regs)?;
            }
            Ins::Qadd { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Qadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Qadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Qasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Qdadd { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Qdsub { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Qsax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Qsub { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Qsub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Qsub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Rev { cond, rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Rev16 { cond, rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Revsh { cond, rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Rfe { addr_mode, rn, writeback } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2_shift(*op2)?;
                }
            }
            Ins::Rrx { s, cond, rd, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Sadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Sadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Sasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::Sel { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Setend { endian } => {
                formatter.write_space()?;
                formatter.write_endianness(*endian)?;
            }
            Ins::Sev { cond } => {}
            Ins::Shadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Shadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Shasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Shsax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Shsub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Shsub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Smmul { round, cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*mode)?;
            }
            Ins::Ssat { cond, rd, imm, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
                formatter.write_separator()?;
                formatter.write_shift_imm(*op2)?;
            }
            Ins::Ssat16 { cond, rd, imm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Ssax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Ssub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Ssub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Stc { l, cond, coproc, crd, dest } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldc_stc(*dest)?;
            }
            Ins::Stc2 { l, coproc, crd, dest } => {
                formatter.write_space()?;
                formatter.write_coproc(*coproc)?;
                formatter.write_separator()?;
                formatter.write_co_reg(*crd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldc_stc(*dest)?;
            }
            Ins::Stm { mode, cond, rn, writeback, regs, user_mode } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_reg_list(*regs)?;
                formatter.write_user_mode(*user_mode)?;
            }
            Ins::Str { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::Strb { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::Strbt { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str_post(*addr)?;
            }
            Ins::Strd { cond, rd, rd2, addr } => {
                formatter.write_space()?;
                if formatter.options().ual {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rd2)?;
                    formatter.write_separator()?;
                    formatter.write_addr_misc_load(*addr)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_addr_misc_load(*addr)?;
                }
            }
            Ins::Strex { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Strexb { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm2)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Strexh { cond, rd, rm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Strh { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_misc_load(*addr)?;
            }
            Ins::Strt { cond, rd, addr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str_post(*addr)?;
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                formatter.write_space()?;
                if formatter.options().ual || !*thumb || *rd != *rn {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_reg(*rn)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                } else {
                    formatter.write_reg(*rd)?;
                    formatter.write_separator()?;
                    formatter.write_op2(*op2)?;
                }
            }
            Ins::Svc { cond, imm } => {
                formatter.write_space()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Ins::Swp { cond, rd, rd2, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Swpb { cond, rd, rd2, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd2)?;
                formatter.write_separator()?;
                formatter.write_str("[")?;
                formatter.write_reg(*rn)?;
                formatter.write_str("]")?;
            }
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Sxtb { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Sxth { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Teq { cond, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Tst { cond, rn, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_op2(*op2)?;
            }
            Ins::Uadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Udf { imm } => {
                formatter.write_space()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
            }
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uhasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uhsax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd_lo)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd_hi)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqasx { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqsax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Usad8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                formatter.write_separator()?;
                formatter.write_reg(*ra)?;
            }
            Ins::Usat { cond, rd, imm, op2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
                formatter.write_separator()?;
                formatter.write_shift_imm(*op2)?;
            }
            Ins::Usat16 { cond, rd, imm, rn } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_str("#")?;
                formatter.write_uimm(*imm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
            }
            Ins::Usax { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Usub16 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Usub8 { cond, rd, rn, rm } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
            }
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Uxtb { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::Uxth { cond, rd, rm, rotate } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rm)?;
                if *rotate != 0 {
                    formatter.write_separator()?;
                    formatter.write_str("ror")?;
                    formatter.write_space()?;
                    formatter.write_str("#")?;
                    formatter.write_uimm(*rotate)?;
                } else {}
            }
            Ins::VabsF32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VabsF64 { cond, dd, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VaddF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VaddF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VcmpF32 { quiet_nan_exc, cond, sd, op2 } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                if formatter.options().ual || *op2 != VcmpF32Op2::Zero {
                    formatter.write_separator()?;
                    formatter.write_vcmp_f32_op2(*op2)?;
                } else {}
            }
            Ins::VcmpF64 { quiet_nan_exc, cond, dd, op2 } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                if formatter.options().ual || *op2 != VcmpF64Op2::Zero {
                    formatter.write_separator()?;
                    formatter.write_vcmp_f64_op2(*op2)?;
                } else {}
            }
            Ins::VcvtF32F64 { cond, sd, dm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VcvtF32S32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtF32U32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtF64F32 { cond, dd, sm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtF64S32 { cond, dd, sm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtF64U32 { cond, dd, sm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VdivF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VdivF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_sreg_list(*regs)?;
            }
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_dreg_list(*regs)?;
            }
            Ins::VldrF32 { cond, sd, addr } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::VldrF64 { cond, dd, addr } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::Vmov32Reg { cond, dd, rt } => {
                formatter.write_space()?;
                formatter.write_dreg_index(*dd)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt)?;
            }
            Ins::VmovF32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VmovF32Reg { cond, sn, rt } => {
                formatter.write_space()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt)?;
            }
            Ins::VmovF64 { cond, dd, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VmovReg32 { cond, rt, dn } => {
                formatter.write_space()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_dreg_index(*dn)?;
            }
            Ins::VmovRegF32 { cond, rt, sn } => {
                formatter.write_space()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
            }
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                formatter.write_space()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt2)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm2)?;
            }
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                formatter.write_space()?;
                formatter.write_sreg(*sm)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm2)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt2)?;
            }
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                formatter.write_space()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt2)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                formatter.write_space()?;
                formatter.write_dreg(*dm)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt)?;
                formatter.write_separator()?;
                formatter.write_reg(*rt2)?;
            }
            Ins::Vmrs { cond, rd, fpscr } => {
                formatter.write_space()?;
                formatter.write_reg(*rd)?;
                formatter.write_separator()?;
                formatter.write_fpscr(*fpscr)?;
            }
            Ins::Vmsr { cond, fpscr, rd } => {
                formatter.write_space()?;
                formatter.write_fpscr(*fpscr)?;
                formatter.write_separator()?;
                formatter.write_reg(*rd)?;
            }
            Ins::VmulF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VmulF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VnegF32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VnegF64 { cond, dd, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VpopF32 { cond, regs } => {
                formatter.write_space()?;
                formatter.write_sreg_list(*regs)?;
            }
            Ins::VpopF64 { cond, regs } => {
                formatter.write_space()?;
                formatter.write_dreg_list(*regs)?;
            }
            Ins::VpushF32 { cond, regs } => {
                formatter.write_space()?;
                formatter.write_sreg_list(*regs)?;
            }
            Ins::VpushF64 { cond, regs } => {
                formatter.write_space()?;
                formatter.write_dreg_list(*regs)?;
            }
            Ins::VsqrtF32 { cond, sd, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VsqrtF64 { cond, dd, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_sreg_list(*regs)?;
            }
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                formatter.write_space()?;
                formatter.write_reg(*rn)?;
                formatter.write_wb(*writeback)?;
                formatter.write_separator()?;
                formatter.write_dreg_list(*regs)?;
            }
            Ins::VstrF32 { cond, sd, addr } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::VstrF64 { cond, dd, addr } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_addr_ldr_str(*addr)?;
            }
            Ins::VsubF32 { cond, sd, sn, sm } => {
                formatter.write_space()?;
                formatter.write_sreg(*sd)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sn)?;
                formatter.write_separator()?;
                formatter.write_sreg(*sm)?;
            }
            Ins::VsubF64 { cond, dd, dn, dm } => {
                formatter.write_space()?;
                formatter.write_dreg(*dd)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dn)?;
                formatter.write_separator()?;
                formatter.write_dreg(*dm)?;
            }
            Ins::Wfe { cond } => {}
            Ins::Wfi { cond } => {}
            Ins::Yield { cond } => {}
        }
        Ok(())
    }
}

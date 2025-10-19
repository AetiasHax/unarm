#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(clippy::single_match)]
use crate::*;
impl Ins {
    /// Returns a [`DefsUses`] object containing all the registers this instruction
    /// defines, in no particular order.
    pub fn defs(&self) -> DefsUses {
        let mut defs = DefsUses::new();
        match self {
            Ins::Adc { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Add { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::And { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Asr { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::B { cond, target } => {
                defs.push(Reg::Pc);
            }
            Ins::Bic { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Bl { cond, target } => {
                defs.push(Reg::Pc);
            }
            Ins::Blx { cond, target } => {
                defs.push(Reg::Pc);
            }
            Ins::Bx { cond, rm } => {
                defs.push(Reg::Pc);
            }
            Ins::Bxj { cond, rm } => {
                defs.push(Reg::Pc);
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                defs.push(*crd);
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                defs.push(*crd);
            }
            Ins::Clz { cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Eor { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                defs.push(*crd);
                dest.defs(&mut defs);
            }
            Ins::Ldc2 { l, coproc, crd, dest } => {
                defs.push(*crd);
                dest.defs(&mut defs);
            }
            Ins::Ldm { mode, cond, rn, writeback, regs, user_mode } => {
                defs.push(*regs);
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::Ldr { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Ldrb { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Ldrbt { cond, rd, addr } => {
                defs.push(*rd);
            }
            Ins::Ldrd { cond, rd, rd2, addr } => {
                defs.push(*rd);
                defs.push(*rd2);
                addr.defs(&mut defs);
            }
            Ins::Ldrex { cond, rd, rn } => {
                defs.push(*rd);
            }
            Ins::Ldrexb { cond, rd, rn } => {
                defs.push(*rd);
            }
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            Ins::Ldrexh { cond, rd, rn } => {
                defs.push(*rd);
            }
            Ins::Ldrh { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Ldrsb { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Ldrsh { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Ldrt { cond, rd, addr } => {
                defs.push(*rd);
            }
            Ins::Lsl { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Lsr { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*crn);
                defs.push(*crm);
            }
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*crn);
                defs.push(*crm);
            }
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                defs.push(*crm);
            }
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                defs.push(*crm);
            }
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                defs.push(*rd);
            }
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*rd);
                defs.push(*crm);
            }
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*rd);
                defs.push(*crm);
            }
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            Ins::Mrs { cond, rd, status_reg } => {
                defs.push(*rd);
            }
            Ins::Msr { cond, status_fields, op2 } => {
                defs.push(*status_fields);
            }
            Ins::Mul { s, thumb, cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Mvn { s, thumb, cond, rd, op2 } => {
                defs.push(*rd);
            }
            Ins::Neg { rd, rm } => {
                defs.push(*rd);
            }
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                defs.push(*rd);
            }
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                defs.push(*rd);
            }
            Ins::Pld { addr } => {
                addr.defs(&mut defs);
            }
            Ins::Pop { cond, regs } => {
                defs.push(Reg::Sp);
                defs.push(*regs);
            }
            Ins::Push { cond, regs } => {
                defs.push(Reg::Sp);
            }
            Ins::Qadd { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Qadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Qadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Qasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Qdadd { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Qdsub { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Qsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Qsub { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Qsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Qsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Rev { cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Rev16 { cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Revsh { cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Rfe { addr_mode, rn, writeback } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Rrx { s, cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Sadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Sadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Sasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Sel { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Shsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                defs.push(*rd);
            }
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                defs.push(*rd);
            }
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                defs.push(*rd);
            }
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                defs.push(*rd);
            }
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            Ins::Smmul { round, cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                defs.push(*rd);
            }
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                defs.push(*rd);
            }
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                defs.push(*rd);
            }
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                defs.push(*rd);
            }
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::Ssat { cond, rd, imm, op2 } => {
                defs.push(*rd);
            }
            Ins::Ssat16 { cond, rd, imm, rn } => {
                defs.push(*rd);
            }
            Ins::Ssax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Ssub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Ssub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Stc { l, cond, coproc, crd, dest } => {
                dest.defs(&mut defs);
            }
            Ins::Stc2 { l, coproc, crd, dest } => {
                dest.defs(&mut defs);
            }
            Ins::Stm { mode, cond, rn, writeback, regs, user_mode } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::Str { cond, rd, addr } => {
                addr.defs(&mut defs);
            }
            Ins::Strb { cond, rd, addr } => {
                addr.defs(&mut defs);
            }
            Ins::Strd { cond, rd, rd2, addr } => {
                addr.defs(&mut defs);
            }
            Ins::Strex { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Strexb { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                defs.push(*rd);
            }
            Ins::Strexh { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Strh { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            Ins::Strt { cond, rd, addr } => {
                defs.push(*rd);
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Swp { cond, rd, rd2, rn } => {
                defs.push(*rd);
            }
            Ins::Swpb { cond, rd, rd2, rn } => {
                defs.push(*rd);
            }
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Sxtb { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Sxth { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uqasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uqsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Usad8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            Ins::Usat { cond, rd, imm, op2 } => {
                defs.push(*rd);
            }
            Ins::Usat16 { cond, rd, imm, rn } => {
                defs.push(*rd);
            }
            Ins::Usax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Usub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Usub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uxtb { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::Uxth { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            Ins::VabsF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VabsF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            Ins::VaddF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VaddF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VcvtF32F64 { cond, sd, dm } => {
                defs.push(*sd);
            }
            Ins::VcvtF32S32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VcvtF32U32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VcvtF64F32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            Ins::VcvtF64S32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            Ins::VcvtF64U32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                defs.push(*sd);
            }
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                defs.push(*sd);
            }
            Ins::VdivF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VdivF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                defs.push(*regs);
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                defs.push(*regs);
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::VldrF32 { cond, sd, addr } => {
                defs.push(*sd);
                addr.defs(&mut defs);
            }
            Ins::VldrF64 { cond, dd, addr } => {
                defs.push(*dd);
                addr.defs(&mut defs);
            }
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::Vmov32Reg { cond, dd, rt } => {
                defs.push(*dd);
            }
            Ins::VmovF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VmovF32Reg { cond, sn, rt } => {
                defs.push(*sn);
            }
            Ins::VmovF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            Ins::VmovReg32 { cond, rt, dn } => {
                defs.push(*rt);
            }
            Ins::VmovRegF32 { cond, rt, sn } => {
                defs.push(*rt);
            }
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                defs.push(*rt);
                defs.push(*rt2);
            }
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                defs.push(*sm);
                defs.push(*sm2);
            }
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                defs.push(*rt);
                defs.push(*rt2);
            }
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                defs.push(*dm);
            }
            Ins::Vmrs { cond, rd, fpscr } => {
                defs.push(*rd);
            }
            Ins::Vmsr { cond, fpscr, rd } => {
                defs.push(*fpscr);
            }
            Ins::VmulF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VmulF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VnegF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VnegF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            Ins::VpopF32 { cond, regs } => {
                defs.push(Reg::Sp);
                defs.push(*regs);
            }
            Ins::VpopF64 { cond, regs } => {
                defs.push(Reg::Sp);
                defs.push(*regs);
            }
            Ins::VpushF32 { cond, regs } => {
                defs.push(Reg::Sp);
            }
            Ins::VpushF64 { cond, regs } => {
                defs.push(Reg::Sp);
            }
            Ins::VsqrtF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            Ins::VsqrtF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::VstrF32 { cond, sd, addr } => {
                addr.defs(&mut defs);
            }
            Ins::VstrF64 { cond, dd, addr } => {
                addr.defs(&mut defs);
            }
            Ins::VsubF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            Ins::VsubF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            _ => {}
        }
        defs
    }
    /// Returns a [`DefsUses`] object containing all the registers this instruction
    /// uses, in no particular order.
    pub fn uses(&self) -> DefsUses {
        let mut uses = DefsUses::new();
        match self {
            Ins::Adc { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Add { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::And { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Asr { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Bic { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Blx { cond, target } => {
                target.uses(&mut uses);
            }
            Ins::Bx { cond, rm } => {
                uses.push(*rm);
            }
            Ins::Bxj { cond, rm } => {
                uses.push(*rm);
            }
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            Ins::Clz { cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Cmn { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Cmp { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Eor { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                dest.uses(&mut uses);
            }
            Ins::Ldc2 { l, coproc, crd, dest } => {
                dest.uses(&mut uses);
            }
            Ins::Ldm { mode, cond, rn, writeback, regs, user_mode } => {
                uses.push(*rn);
            }
            Ins::Ldr { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrb { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrbt { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrd { cond, rd, rd2, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrex { cond, rd, rn } => {
                uses.push(*rn);
            }
            Ins::Ldrexb { cond, rd, rn } => {
                uses.push(*rn);
            }
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                uses.push(*rn);
            }
            Ins::Ldrexh { cond, rd, rn } => {
                uses.push(*rn);
            }
            Ins::Ldrh { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrsb { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrsh { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Ldrt { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Lsl { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Lsr { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*rd);
                uses.push(*crm);
            }
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*rd);
                uses.push(*crm);
            }
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                uses.push(*rd);
                uses.push(*rd2);
            }
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                uses.push(*rd);
                uses.push(*rd2);
            }
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                op2.uses(&mut uses);
            }
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                uses.push(*crm);
            }
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                uses.push(*crm);
            }
            Ins::Mrs { cond, rd, status_reg } => {
                uses.push(*status_reg);
            }
            Ins::Msr { cond, status_fields, op2 } => {
                op2.uses(&mut uses);
            }
            Ins::Mul { s, thumb, cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Mvn { s, thumb, cond, rd, op2 } => {
                op2.uses(&mut uses);
            }
            Ins::Neg { rd, rm } => {
                uses.push(*rm);
            }
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Pld { addr } => {
                addr.uses(&mut uses);
            }
            Ins::Pop { cond, regs } => {
                uses.push(Reg::Sp);
            }
            Ins::Push { cond, regs } => {
                uses.push(Reg::Sp);
                uses.push(*regs);
            }
            Ins::Qadd { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Qadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Qadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Qasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Qdadd { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Qdsub { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Qsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Qsub { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Qsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Qsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Rev { cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Rev16 { cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Revsh { cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Rfe { addr_mode, rn, writeback } => {
                uses.push(*rn);
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Rrx { s, cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Sadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Sel { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Shsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Smmul { round, cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                uses.push(*rn);
            }
            Ins::Ssat { cond, rd, imm, op2 } => {
                op2.uses(&mut uses);
            }
            Ins::Ssax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Ssub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Ssub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Stc { l, cond, coproc, crd, dest } => {
                uses.push(*crd);
                dest.uses(&mut uses);
            }
            Ins::Stc2 { l, coproc, crd, dest } => {
                uses.push(*crd);
                dest.uses(&mut uses);
            }
            Ins::Stm { mode, cond, rn, writeback, regs, user_mode } => {
                uses.push(*regs);
                uses.push(*rn);
            }
            Ins::Str { cond, rd, addr } => {
                uses.push(*rd);
                addr.uses(&mut uses);
            }
            Ins::Strb { cond, rd, addr } => {
                uses.push(*rd);
                addr.uses(&mut uses);
            }
            Ins::Strbt { cond, rd, addr } => {
                uses.push(*rd);
                addr.uses(&mut uses);
            }
            Ins::Strd { cond, rd, rd2, addr } => {
                uses.push(*rd);
                uses.push(*rd2);
                addr.uses(&mut uses);
            }
            Ins::Strex { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Strexb { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*rm2);
            }
            Ins::Strexh { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Strh { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Strt { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Swp { cond, rd, rd2, rn } => {
                uses.push(*rd2);
                uses.push(*rn);
            }
            Ins::Swpb { cond, rd, rd2, rn } => {
                uses.push(*rd2);
                uses.push(*rn);
            }
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sxtb { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::Sxth { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::Teq { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Tst { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Uadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
            }
            Ins::Uadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Usad8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Usat { cond, rd, imm, op2 } => {
                op2.uses(&mut uses);
            }
            Ins::Usat16 { cond, rd, imm, rn } => {
                uses.push(*rn);
            }
            Ins::Usax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Usub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Usub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Uxtb { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::Uxth { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            Ins::VabsF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VabsF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            Ins::VaddF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VaddF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VcmpF32 { nan_exc, cond, sd, op2 } => {
                uses.push(*sd);
                op2.uses(&mut uses);
            }
            Ins::VcmpF64 { nan_exc, cond, dd, op2 } => {
                uses.push(*dd);
                op2.uses(&mut uses);
            }
            Ins::VcvtF32F64 { cond, sd, dm } => {
                uses.push(*dm);
            }
            Ins::VcvtF32S32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtF32U32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtF64F32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtF64S32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtF64U32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                uses.push(*dm);
            }
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                uses.push(*dm);
            }
            Ins::VdivF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VdivF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                uses.push(*rn);
            }
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                uses.push(*rn);
            }
            Ins::VldrF32 { cond, sd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::VldrF64 { cond, dd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::Vmov32Reg { cond, dd, rt } => {
                uses.push(*rt);
            }
            Ins::VmovF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VmovF32Reg { cond, sn, rt } => {
                uses.push(*rt);
            }
            Ins::VmovF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            Ins::VmovReg32 { cond, rt, dn } => {
                uses.push(*dn);
            }
            Ins::VmovRegF32 { cond, rt, sn } => {
                uses.push(*sn);
            }
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                uses.push(*sm);
                uses.push(*sm2);
            }
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                uses.push(*rt);
                uses.push(*rt2);
            }
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                uses.push(*dm);
            }
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                uses.push(*rt);
                uses.push(*rt2);
            }
            Ins::Vmrs { cond, rd, fpscr } => {
                uses.push(*fpscr);
            }
            Ins::Vmsr { cond, fpscr, rd } => {
                uses.push(*rd);
            }
            Ins::VmulF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VmulF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VnegF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VnegF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            Ins::VpopF32 { cond, regs } => {
                uses.push(Reg::Sp);
            }
            Ins::VpopF64 { cond, regs } => {
                uses.push(Reg::Sp);
            }
            Ins::VpushF32 { cond, regs } => {
                uses.push(Reg::Sp);
                uses.push(*regs);
            }
            Ins::VpushF64 { cond, regs } => {
                uses.push(Reg::Sp);
                uses.push(*regs);
            }
            Ins::VsqrtF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            Ins::VsqrtF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                uses.push(*regs);
                uses.push(*rn);
            }
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                uses.push(*regs);
                uses.push(*rn);
            }
            Ins::VstrF32 { cond, sd, addr } => {
                uses.push(*sd);
                addr.uses(&mut uses);
            }
            Ins::VstrF64 { cond, dd, addr } => {
                uses.push(*dd);
                addr.uses(&mut uses);
            }
            Ins::VsubF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            Ins::VsubF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            _ => {}
        }
        uses
    }
}
impl BlxTarget {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Indirect(rm) => {
                uses.push(*rm);
            }
            _ => {}
        }
    }
}
impl MsrOp2 {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg(reg) => {
                uses.push(*reg);
            }
            _ => {}
        }
    }
}
impl Op2 {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::ShiftReg(shift_reg) => {
                shift_reg.uses(uses);
            }
            Self::ShiftImm(shift_imm) => {
                shift_imm.uses(uses);
            }
            _ => {}
        }
    }
}
impl ShiftReg {
    fn uses(&self, uses: &mut DefsUses) {
        let Self { rm, shift_op, rs } = self;
        uses.push(*rm);
        uses.push(*rs);
    }
}
impl ShiftImm {
    fn uses(&self, uses: &mut DefsUses) {
        let Self { rm, shift_op, imm } = self;
        uses.push(*rm);
    }
}
impl Op2Shift {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg(reg) => {
                uses.push(*reg);
            }
            _ => {}
        }
    }
}
impl AddrLdcStc {
    fn defs(&self, defs: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            _ => {}
        }
    }
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                uses.push(*rn);
            }
            Self::Post { rn, offset } => {
                uses.push(*rn);
            }
            Self::Unidx { rn, option } => {
                uses.push(*rn);
            }
            _ => {}
        }
    }
}
impl AddrLdrStr {
    fn defs(&self, defs: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            _ => {}
        }
    }
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                uses.push(*rn);
                offset.uses(uses);
            }
            Self::Post(addr_ldr_str_post) => {
                addr_ldr_str_post.uses(uses);
            }
            _ => {}
        }
    }
}
impl AddrLdrStrPost {
    fn uses(&self, uses: &mut DefsUses) {
        let Self { rn, offset } = self;
        uses.push(*rn);
        offset.uses(uses);
    }
}
impl LdrStrOffset {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg { subtract, rm, shift_op, imm } => {
                uses.push(*rm);
            }
            _ => {}
        }
    }
}
impl AddrMiscLoad {
    fn defs(&self, defs: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            _ => {}
        }
    }
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Pre { rn, offset, writeback } => {
                uses.push(*rn);
                offset.uses(uses);
            }
            Self::Post { rn, offset } => {
                uses.push(*rn);
                offset.uses(uses);
            }
            _ => {}
        }
    }
}
impl MiscLoadOffset {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg { subtract, rm } => {
                uses.push(*rm);
            }
            _ => {}
        }
    }
}
impl VcmpF32Op2 {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg(sm) => {
                uses.push(*sm);
            }
            _ => {}
        }
    }
}
impl VcmpF64Op2 {
    fn uses(&self, uses: &mut DefsUses) {
        match self {
            Self::Reg(dm) => {
                uses.push(*dm);
            }
            _ => {}
        }
    }
}

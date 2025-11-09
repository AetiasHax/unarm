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
            #[cfg(
                any(
                    feature = "v5t",
                    feature = "v5te",
                    feature = "v5tej",
                    feature = "v6",
                    feature = "v6k"
                )
            )]
            Ins::Blx { cond, target } => {
                defs.push(Reg::Pc);
            }
            #[cfg(
                any(
                    feature = "v4t",
                    feature = "v5t",
                    feature = "v5te",
                    feature = "v5tej",
                    feature = "v6",
                    feature = "v6k"
                )
            )]
            Ins::Bx { cond, rm } => {
                defs.push(Reg::Pc);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(feature = "v5tej", feature = "v6", feature = "v6k")
                )
            )]
            Ins::Bxj { cond, rm } => {
                defs.push(Reg::Pc);
            }
            #[cfg(feature = "arm")]
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                defs.push(*crd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                defs.push(*crd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Clz { cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Eor { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                defs.push(*crd);
                dest.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(feature = "arm")]
            Ins::Ldrbt { cond, rd, addr } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Ldrd { cond, rd, rd2, addr } => {
                defs.push(*rd);
                defs.push(*rd2);
                addr.defs(&mut defs);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ldrex { cond, rd, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexb { cond, rd, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
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
            #[cfg(feature = "arm")]
            Ins::Ldrt { cond, rd, addr } => {
                defs.push(*rd);
            }
            Ins::Lsl { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            Ins::Lsr { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*crn);
                defs.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*crn);
                defs.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                defs.push(*crm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                defs.push(*crm);
            }
            #[cfg(feature = "arm")]
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*rd);
                defs.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                defs.push(*rd);
                defs.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                defs.push(*rd);
                defs.push(*rd2);
            }
            #[cfg(feature = "arm")]
            Ins::Mrs { cond, rd, status_reg } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Msr { cond, status_fields, op2 } => {
                defs.push(*status_fields);
            }
            Ins::Mul { s, thumb, cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Mvn { s, thumb, cond, rd, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "thumb")]
            Ins::Neg { rd, rm } => {
                defs.push(*rd);
            }
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qadd { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qdadd { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qdsub { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qsub { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev { cond, rd, rm } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev16 { cond, rd, rm } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Revsh { cond, rd, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Rfe { addr_mode, rn, writeback } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Rrx { s, cond, rd, rm } => {
                defs.push(*rd);
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sel { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmul { round, cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat { cond, rd, imm, op2 } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat16 { cond, rd, imm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Stc { l, cond, coproc, crd, dest } => {
                dest.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Strd { cond, rd, rd2, addr } => {
                addr.defs(&mut defs);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Strex { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexb { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexh { cond, rd, rm, rn } => {
                defs.push(*rd);
            }
            Ins::Strh { cond, rd, addr } => {
                defs.push(*rd);
                addr.defs(&mut defs);
            }
            #[cfg(feature = "arm")]
            Ins::Strt { cond, rd, addr } => {
                defs.push(*rd);
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Swp { cond, rd, rd2, rn } => {
                defs.push(*rd);
            }
            #[cfg(feature = "arm")]
            Ins::Swpb { cond, rd, rd2, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxtb { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxth { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(feature = "arm")]
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(feature = "arm")]
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                defs.push(*rd_lo);
                defs.push(*rd_hi);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqasx { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usad8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat { cond, rd, imm, op2 } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat16 { cond, rd, imm, rn } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usax { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub16 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub8 { cond, rd, rn, rm } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxtb { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxth { cond, rd, rm, rotate } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VabsF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VabsF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VaddF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VaddF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32F64 { cond, sd, dm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32S32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32U32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64F32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64S32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64U32 { cond, dd, sm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VdivF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VdivF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                defs.push(*regs);
                if *writeback {
                    defs.push(*rn);
                }
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                defs.push(*regs);
                if *writeback {
                    defs.push(*rn);
                }
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldrF32 { cond, sd, addr } => {
                defs.push(*sd);
                addr.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldrF64 { cond, dd, addr } => {
                defs.push(*dd);
                addr.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmov32Reg { cond, dd, rt } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32Reg { cond, sn, rt } => {
                defs.push(*sn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovReg32 { cond, rt, dn } => {
                defs.push(*rt);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF32 { cond, rt, sn } => {
                defs.push(*rt);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                defs.push(*rt);
                defs.push(*rt2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                defs.push(*sm);
                defs.push(*sm2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                defs.push(*rt);
                defs.push(*rt2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                defs.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmrs { cond, rd, fpscr } => {
                defs.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmsr { cond, fpscr, rd } => {
                defs.push(*fpscr);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmulF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmulF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnegF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnegF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpopF32 { cond, regs } => {
                defs.push(Reg::Sp);
                defs.push(*regs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpopF64 { cond, regs } => {
                defs.push(Reg::Sp);
                defs.push(*regs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpushF32 { cond, regs } => {
                defs.push(Reg::Sp);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpushF64 { cond, regs } => {
                defs.push(Reg::Sp);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsqrtF32 { cond, sd, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsqrtF64 { cond, dd, dm } => {
                defs.push(*dd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                if *writeback {
                    defs.push(*rn);
                }
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstrF32 { cond, sd, addr } => {
                addr.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstrF64 { cond, dd, addr } => {
                addr.defs(&mut defs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsubF32 { cond, sd, sn, sm } => {
                defs.push(*sd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(
                any(
                    feature = "v5t",
                    feature = "v5te",
                    feature = "v5tej",
                    feature = "v6",
                    feature = "v6k"
                )
            )]
            Ins::Blx { cond, target } => {
                target.uses(&mut uses);
            }
            #[cfg(
                any(
                    feature = "v4t",
                    feature = "v5t",
                    feature = "v5te",
                    feature = "v5tej",
                    feature = "v6",
                    feature = "v6k"
                )
            )]
            Ins::Bx { cond, rm } => {
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(feature = "v5tej", feature = "v6", feature = "v6k")
                )
            )]
            Ins::Bxj { cond, rm } => {
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Cdp { cond, coproc, opc1, crd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Cdp2 { coproc, opc1, crd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(feature = "arm")]
            Ins::Ldc { l, cond, coproc, crd, dest } => {
                dest.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(feature = "arm")]
            Ins::Ldrbt { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Ldrd { cond, rd, rd2, addr } => {
                addr.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ldrex { cond, rd, rn } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexb { cond, rd, rn } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexd { cond, rd, rd2, rn } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
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
            #[cfg(feature = "arm")]
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
            #[cfg(feature = "arm")]
            Ins::Mcr { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*rd);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mcr2 { coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*rd);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mcrr { cond, coproc, opc, rd, rd2, crm } => {
                uses.push(*rd);
                uses.push(*rd2);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Mcrr2 { coproc, opc, rd, rd2, crm } => {
                uses.push(*rd);
                uses.push(*rd2);
            }
            #[cfg(feature = "arm")]
            Ins::Mla { s, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            Ins::Mov { s, thumb, cond, rd, op2 } => {
                op2.uses(&mut uses);
            }
            #[cfg(feature = "arm")]
            Ins::Mrc { cond, coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mrc2 { coproc, opc1, rd, crn, crm, opc2 } => {
                uses.push(*crn);
                uses.push(*crm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Mrrc { cond, coproc, opc, rd, rd2, crm } => {
                uses.push(*crm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Mrrc2 { coproc, opc, rd, rd2, crm } => {
                uses.push(*crm);
            }
            #[cfg(feature = "arm")]
            Ins::Mrs { cond, rd, status_reg } => {
                uses.push(*status_reg);
            }
            #[cfg(feature = "arm")]
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
            #[cfg(feature = "thumb")]
            Ins::Neg { rd, rm } => {
                uses.push(*rm);
            }
            Ins::Orr { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhbt { cond, rd, rn, rm, shift_op, shift } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhtb { cond, rd, rn, rm, shift_op, shift } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qadd { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qdadd { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qdsub { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Qsub { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev { cond, rd, rm } => {
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev16 { cond, rd, rm } => {
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Revsh { cond, rd, rm } => {
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Rfe { addr_mode, rn, writeback } => {
                uses.push(*rn);
            }
            Ins::Ror { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(feature = "arm")]
            Ins::Rrx { s, cond, rd, rm } => {
                uses.push(*rm);
            }
            Ins::Rsb { s, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(feature = "arm")]
            Ins::Rsc { s, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            Ins::Sbc { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sel { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smla { cond, rd, rn, rn_side, rm, rm_side, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlad { cond, rd, rn, rm, swap_rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(feature = "arm")]
            Ins::Smlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::SmlalHalf { cond, rd_lo, rd_hi, rn, rn_side, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlald { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smlaw { cond, rd, rn, rm, rm_side, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsd { cond, rd, rn, rm, swap_rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsld { cond, rd_lo, rd_hi, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmla { round, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmls { round, cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmul { round, cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smuad { cond, rd, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smul { cond, rd, rn, rn_side, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Smull { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Smulw { cond, rd, rn, rm, rm_side } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smusd { cond, rd, rn, rm, swap_rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Srs { addr_mode, rn, writeback, mode } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat { cond, rd, imm, op2 } => {
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Stc { l, cond, coproc, crd, dest } => {
                uses.push(*crd);
                dest.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5t",
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
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
            #[cfg(feature = "arm")]
            Ins::Strbt { cond, rd, addr } => {
                uses.push(*rd);
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Strd { cond, rd, rd2, addr } => {
                uses.push(*rd);
                uses.push(*rd2);
                addr.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Strex { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexb { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexd { cond, rd, rm, rm2, rn } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*rm2);
            }
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexh { cond, rd, rm, rn } => {
                uses.push(*rm);
                uses.push(*rn);
            }
            Ins::Strh { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            #[cfg(feature = "arm")]
            Ins::Strt { cond, rd, addr } => {
                addr.uses(&mut uses);
            }
            Ins::Sub { s, thumb, cond, rd, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(feature = "arm")]
            Ins::Swp { cond, rd, rd2, rn } => {
                uses.push(*rd2);
                uses.push(*rn);
            }
            #[cfg(feature = "arm")]
            Ins::Swpb { cond, rd, rd2, rn } => {
                uses.push(*rd2);
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab16 { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtah { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxtb { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtb16 { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxth { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Teq { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            Ins::Tst { cond, rn, op2 } => {
                uses.push(*rn);
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Umaal { cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Umlal { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(feature = "arm")]
            Ins::Umull { s, cond, rd_lo, rd_hi, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqasx { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usad8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usada8 { cond, rd, rn, rm, ra } => {
                uses.push(*rn);
                uses.push(*rm);
                uses.push(*ra);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat { cond, rd, imm, op2 } => {
                op2.uses(&mut uses);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat16 { cond, rd, imm, rn } => {
                uses.push(*rn);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usax { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub16 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub8 { cond, rd, rn, rm } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab16 { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtah { cond, rd, rn, rm, rotate } => {
                uses.push(*rn);
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxtb { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtb16 { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxth { cond, rd, rm, rotate } => {
                uses.push(*rm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VabsF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VabsF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VaddF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VaddF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcmpF32 { nan_exc, cond, sd, op2 } => {
                uses.push(*sd);
                op2.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcmpF64 { nan_exc, cond, dd, op2 } => {
                uses.push(*dd);
                op2.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32F64 { cond, sd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32S32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF32U32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64F32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64S32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtF64U32 { cond, dd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtS32F32 { round_zero, cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtS32F64 { round_zero, cond, sd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtU32F32 { round_zero, cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VcvtU32F64 { round_zero, cond, sd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VdivF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VdivF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldmF32 { mode, cond, rn, writeback, regs } => {
                uses.push(*rn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldmF64 { mode, cond, rn, writeback, regs } => {
                uses.push(*rn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldrF32 { cond, sd, addr } => {
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VldrF64 { cond, dd, addr } => {
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlaF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlaF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlsF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmlsF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmov32Reg { cond, dd, rt } => {
                uses.push(*rt);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32Reg { cond, sn, rt } => {
                uses.push(*rt);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovReg32 { cond, rt, dn } => {
                uses.push(*dn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF32 { cond, rt, sn } => {
                uses.push(*sn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF32Dual { cond, rt, rt2, sm, sm2 } => {
                uses.push(*sm);
                uses.push(*sm2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF32RegDual { cond, sm, sm2, rt, rt2 } => {
                uses.push(*rt);
                uses.push(*rt2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovRegF64 { cond, rt, rt2, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmovF64Reg { cond, dm, rt, rt2 } => {
                uses.push(*rt);
                uses.push(*rt2);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmrs { cond, rd, fpscr } => {
                uses.push(*fpscr);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::Vmsr { cond, fpscr, rd } => {
                uses.push(*rd);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmulF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VmulF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnegF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnegF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlaF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlaF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlsF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmlsF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmulF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VnmulF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpopF32 { cond, regs } => {
                uses.push(Reg::Sp);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpopF64 { cond, regs } => {
                uses.push(Reg::Sp);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpushF32 { cond, regs } => {
                uses.push(Reg::Sp);
                uses.push(*regs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VpushF64 { cond, regs } => {
                uses.push(Reg::Sp);
                uses.push(*regs);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsqrtF32 { cond, sd, sm } => {
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsqrtF64 { cond, dd, dm } => {
                uses.push(*dm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstmF32 { mode, cond, rn, writeback, regs } => {
                uses.push(*regs);
                uses.push(*rn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstmF64 { mode, cond, rn, writeback, regs } => {
                uses.push(*regs);
                uses.push(*rn);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstrF32 { cond, sd, addr } => {
                uses.push(*sd);
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VstrF64 { cond, dd, addr } => {
                uses.push(*dd);
                addr.uses(&mut uses);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsubF32 { cond, sd, sn, sm } => {
                uses.push(*sn);
                uses.push(*sm);
            }
            #[cfg(
                all(
                    feature = "arm",
                    feature = "vfp_v2",
                    any(
                        feature = "v5te",
                        feature = "v5tej",
                        feature = "v6",
                        feature = "v6k"
                    )
                )
            )]
            Ins::VsubF64 { cond, dd, dn, dm } => {
                uses.push(*dn);
                uses.push(*dm);
            }
            _ => {}
        }
        uses
    }
}
#[cfg(
    any(
        feature = "v5t",
        feature = "v5te",
        feature = "v5tej",
        feature = "v6",
        feature = "v6k"
    )
)]
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
#[cfg(feature = "arm")]
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
#[cfg(feature = "arm")]
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
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
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
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
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

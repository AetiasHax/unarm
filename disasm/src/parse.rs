#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::eq_op)]
#![allow(clippy::double_parens)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::derivable_impls)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::*;
impl BranchTarget {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        Self {
            addr: pc.wrapping_add((value)),
        }
    }
}
impl BlxTarget {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0xf000000) == 0xb000000 {
            Self::Direct(
                BranchTarget::parse(
                    (((((((value) & 0xffffff) << 2) | ((((value) >> 24) & 0x1) << 1))
                        as i32) << 6 >> 6) as u32)
                        .wrapping_add(8),
                    pc,
                ),
            )
        } else if (value & 0xffffff0) == 0x12fff30 {
            Self::Indirect(Reg::parse(((value) & 0xf), pc))
        } else {
            panic!();
        }
    }
}
impl Cond {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::Eq,
            0x1 => Self::Ne,
            0x2 => Self::Hs,
            0x3 => Self::Lo,
            0x4 => Self::Mi,
            0x5 => Self::Pl,
            0x6 => Self::Vs,
            0x7 => Self::Vc,
            0x8 => Self::Hi,
            0x9 => Self::Ls,
            0xa => Self::Ge,
            0xb => Self::Lt,
            0xc => Self::Gt,
            0xd => Self::Le,
            0xe => Self::Al,
            _ => panic!(),
        }
    }
}
impl Reg {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::R0,
            0x1 => Self::R1,
            0x2 => Self::R2,
            0x3 => Self::R3,
            0x4 => Self::R4,
            0x5 => Self::R5,
            0x6 => Self::R6,
            0x7 => Self::R7,
            0x8 => Self::R8,
            0x9 => Self::R9,
            0xa => Self::R10,
            0xb => Self::R11,
            0xc => Self::R12,
            0xd => Self::Sp,
            0xe => Self::Lr,
            0xf => Self::Pc,
            _ => panic!(),
        }
    }
}
impl StatusReg {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::Cpsr,
            0x1 => Self::Spsr,
            _ => panic!(),
        }
    }
}
impl StatusFields {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        Self {
            reg: StatusReg::parse((((value) >> 22) & 0x1), pc),
            c: (((value) >> 16) & 0x1) != 0,
            x: (((value) >> 17) & 0x1) != 0,
            s: (((value) >> 18) & 0x1) != 0,
            f: (((value) >> 19) & 0x1) != 0,
        }
    }
}
impl MsrOp2 {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x2000000) == 0x2000000 {
            Some(Self::Imm(((value) & 0xff).rotate_right((((value) >> 8) & 0xf))))
        } else if (value & 0x20000f0) == 0x0 {
            if value & 0xf00 != 0 {
                return None;
            }
            Some(Self::Reg(Reg::parse(((value) & 0xf), pc)))
        } else {
            panic!();
        }
    }
}
impl ShiftOp {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::Lsl,
            0x1 => Self::Lsr,
            0x2 => Self::Asr,
            0x3 => Self::Ror,
            _ => panic!(),
        }
    }
}
impl Coproc {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::P0,
            0x1 => Self::P1,
            0x2 => Self::P2,
            0x3 => Self::P3,
            0x4 => Self::P4,
            0x5 => Self::P5,
            0x6 => Self::P6,
            0x7 => Self::P7,
            0x8 => Self::P8,
            0x9 => Self::P9,
            0xa => Self::P10,
            0xb => Self::P11,
            0xc => Self::P12,
            0xd => Self::P13,
            0xe => Self::P14,
            0xf => Self::P15,
            _ => panic!(),
        }
    }
}
impl CoReg {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::C0,
            0x1 => Self::C1,
            0x2 => Self::C2,
            0x3 => Self::C3,
            0x4 => Self::C4,
            0x5 => Self::C5,
            0x6 => Self::C6,
            0x7 => Self::C7,
            0x8 => Self::C8,
            0x9 => Self::C9,
            0xa => Self::C10,
            0xb => Self::C11,
            0xc => Self::C12,
            0xd => Self::C13,
            0xe => Self::C14,
            0xf => Self::C15,
            _ => panic!(),
        }
    }
}
impl Op2 {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x2000000) == 0x2000000 {
            Self::Imm(((value) & 0xff).rotate_right((((value) >> 8) & 0xf) << 1))
        } else if (value & 0x2000010) == 0x10 {
            Self::ShiftReg {
                rm: Reg::parse(((value) & 0xf), pc),
                shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc),
                rs: Reg::parse((((value) >> 8) & 0xf), pc),
            }
        } else if (value & 0x2000010) == 0x0 {
            Self::ShiftImm {
                rm: Reg::parse(((value) & 0xf), pc),
                shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc),
                imm: (((value) >> 7) & 0x1f),
            }
        } else {
            panic!();
        }
    }
}
impl Op2Shift {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x10) == 0x0 {
            Self::Imm(
                if (((value) >> 7) & 0x1f) != 0 { (((value) >> 7) & 0x1f) } else { 32 },
            )
        } else if (value & 0x90) == 0x10 {
            Self::Reg(Reg::parse((((value) >> 8) & 0xf), pc))
        } else {
            panic!();
        }
    }
}
impl CpsEffect {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x3) == 0x0 {
            Self::SetMode
        } else if (value & 0x3) == 0x2 {
            Self::Ie
        } else if (value & 0x3) == 0x3 {
            Self::Id
        } else {
            panic!();
        }
    }
}
impl AifFlags {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        Self {
            a: (((value) >> 2) & 0x1) != 0,
            i: (((value) >> 1) & 0x1) != 0,
            f: ((value) & 0x1) != 0,
        }
    }
}
impl AddrLdcStc {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x1000000) == 0x1000000 {
            Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                offset: ((if (((value) >> 23) & 0x1) == 0 {
                    -((((value) & 0xff) << 2) as i32)
                } else {
                    (((value) & 0xff) << 2) as i32
                })) as i32,
                writeback: ((((value) >> 21) & 0x1)) != 0,
            }
        } else if (value & 0x1200000) == 0x200000 {
            Self::Post {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                offset: ((if (((value) >> 23) & 0x1) == 0 {
                    -((((value) & 0xff) << 2) as i32)
                } else {
                    (((value) & 0xff) << 2) as i32
                })) as i32,
            }
        } else if (value & 0x1a00000) == 0x800000 {
            Self::Unidx {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                option: ((value) & 0xff),
            }
        } else {
            panic!();
        }
    }
}
impl LdmStmMode {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        match value {
            0x0 => Self::Da,
            0x1 => Self::Ia,
            0x2 => Self::Db,
            0x3 => Self::Ib,
            _ => panic!(),
        }
    }
}
impl AddrLdrStr {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x1000000) == 0x1000000 {
            Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                offset: LdrStrOffset::parse((value), pc),
                writeback: ((((value) >> 21) & 0x1)) != 0,
            }
        } else if (value & 0x1200000) == 0x0 {
            Self::Post(AddrLdrStrPost::parse((value), pc))
        } else {
            panic!();
        }
    }
}
impl AddrLdrStrPost {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        Self {
            rn: Reg::parse((((value) >> 16) & 0xf), pc),
            offset: LdrStrOffset::parse((value), pc),
        }
    }
}
impl LdrStrOffset {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        if (value & 0x2000000) == 0x0 {
            Self::Imm(
                ((if (((value) >> 23) & 0x1) == 0 {
                    -(((value) & 0xfff) as i32)
                } else {
                    ((value) & 0xfff) as i32
                })) as i32,
            )
        } else if (value & 0x2000000) == 0x2000000 {
            Self::Reg {
                subtract: ((((value) >> 23) & 0x1) ^ 1) != 0,
                rm: Reg::parse(((value) & 0xf), pc),
                shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc),
                imm: if (((value) >> 5) & 0x3) == 1 && (((value) >> 7) & 0x1f) == 0 {
                    0x20
                } else {
                    (((value) >> 7) & 0x1f)
                },
            }
        } else {
            panic!();
        }
    }
}
impl AddrMiscLoad {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x1000000) == 0x1000000 {
            Some(Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                offset: MiscLoadOffset::parse((value), pc)?,
                writeback: ((((value) >> 21) & 0x1)) != 0,
            })
        } else if (value & 0x1200000) == 0x0 {
            Some(Self::Post {
                rn: Reg::parse((((value) >> 16) & 0xf), pc),
                offset: MiscLoadOffset::parse((value), pc)?,
            })
        } else {
            panic!();
        }
    }
}
impl MiscLoadOffset {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0xe400090) == 0x400090 {
            Some(
                Self::Imm(
                    ((if (((value) >> 23) & 0x1) == 0 {
                        -((((((value) >> 8) & 0xf) << 4) | ((value) & 0xf)) as i32)
                    } else {
                        (((((value) >> 8) & 0xf) << 4) | ((value) & 0xf)) as i32
                    })) as i32,
                ),
            )
        } else if (value & 0xe400090) == 0x90 {
            if value & 0xf00 != 0 {
                return None;
            }
            Some(Self::Reg {
                rm: Reg::parse(((value) & 0xf), pc),
                subtract: ((((value) >> 23) & 0x1) ^ 1) != 0,
            })
        } else {
            panic!();
        }
    }
}
impl Default for BranchTarget {
    fn default() -> Self {
        Self { addr: 0 }
    }
}
impl Default for Cond {
    fn default() -> Self {
        Self::Al
    }
}
impl Default for ShiftOp {
    fn default() -> Self {
        Self::Lsl
    }
}
impl Default for AifFlags {
    fn default() -> Self {
        Self {
            a: false,
            i: false,
            f: false,
        }
    }
}
impl Default for LdmStmMode {
    fn default() -> Self {
        Self::Ia
    }
}
pub fn parse_arm(ins: u32, pc: u32) -> Option<Ins> {
    if (ins & 0xfff00ff) == 0x3200014 {
        parse_arm_csdb_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xe1200070 {
        parse_arm_bkpt_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xf5700010 {
        parse_arm_clrex_0(ins as u32, pc)
    } else if (ins & 0xfe00ff0) == 0x1a00000 {
        parse_arm_mov_1(ins as u32, pc)
    } else if (ins & 0xfff10020) == 0xf1000000 {
        parse_arm_cps_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00040 {
        parse_arm_asr_0(ins as u32, pc)
    } else if (ins & 0xe5f00f0) == 0x4f00d0 {
        parse_arm_ldrd_1(ins as u32, pc)
    } else if (ins & 0xff010f0) == 0x1b00090 {
        parse_arm_ldrexd_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00000 {
        parse_arm_lsl_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00020 {
        parse_arm_lsr_0(ins as u32, pc)
    } else if (ins & 0xfb002f0) == 0x1000000 {
        parse_arm_mrs_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200030 {
        parse_arm_blx_1(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200010 {
        parse_arm_bx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200020 {
        parse_arm_bxj_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1600010 {
        parse_arm_clz_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1900090 {
        parse_arm_ldrex_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1d00090 {
        parse_arm_ldrexb_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1f00090 {
        parse_arm_ldrexh_0(ins as u32, pc)
    } else if (ins & 0xfff00000) == 0xfc400000 {
        parse_arm_mcrr2_0(ins as u32, pc)
    } else if (ins & 0xfff00000) == 0xfc500000 {
        parse_arm_mrrc2_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0x200090 {
        parse_arm_mla_0(ins as u32, pc)
    } else if (ins & 0xff100010) == 0xfe000010 {
        parse_arm_mcr2_0(ins as u32, pc)
    } else if (ins & 0xff100010) == 0xfe100010 {
        parse_arm_mrc2_0(ins as u32, pc)
    } else if (ins & 0xff000010) == 0xfe000000 {
        parse_arm_cdp2_0(ins as u32, pc)
    } else if (ins & 0xfe100000) == 0xfc100000 {
        parse_arm_ldc2_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0xd0 {
        parse_arm_ldrd_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000b0 {
        parse_arm_ldrh_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000d0 {
        parse_arm_ldrsb_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000f0 {
        parse_arm_ldrsh_0(ins as u32, pc)
    } else if (ins & 0xff00000) == 0xc400000 {
        parse_arm_mcrr_0(ins as u32, pc)
    } else if (ins & 0xff00000) == 0xc500000 {
        parse_arm_mrrc_0(ins as u32, pc)
    } else if (ins & 0xe708000) == 0x8500000 {
        parse_arm_ldm_1(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1500000 {
        parse_arm_cmp_0(ins as u32, pc)
    } else if (ins & 0xfe00000) == 0x3a00000 {
        parse_arm_mov_0(ins as u32, pc)
    } else if (ins & 0xf700000) == 0x4700000 {
        parse_arm_ldrbt_0(ins as u32, pc)
    } else if (ins & 0xfe000000) == 0xfa000000 {
        parse_arm_blx_0(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1700000 {
        parse_arm_cmn_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x800000 {
        parse_arm_add_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x1c00000 {
        parse_arm_bic_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x0 {
        parse_arm_and_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x200000 {
        parse_arm_eor_0(ins as u32, pc)
    } else if (ins & 0xd700000) == 0x4300000 {
        parse_arm_ldrt_0(ins as u32, pc)
    } else if (ins & 0xf100010) == 0xe000010 {
        parse_arm_mcr_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0xa00000 {
        parse_arm_adc_0(ins as u32, pc)
    } else if (ins & 0xe508000) == 0x8508000 {
        parse_arm_ldm_2(ins as u32, pc)
    } else if (ins & 0xf100010) == 0xe100010 {
        parse_arm_mrc_0(ins as u32, pc)
    } else if (ins & 0xdb00000) == 0x1200000 {
        parse_arm_msr_0(ins as u32, pc)
    } else if (ins & 0xf000010) == 0xe000000 {
        parse_arm_cdp_0(ins as u32, pc)
    } else if (ins & 0xe500000) == 0x8100000 {
        parse_arm_ldm_0(ins as u32, pc)
    } else if (ins & 0xe100000) == 0xc100000 {
        parse_arm_ldc_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4100000 {
        parse_arm_ldr_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4500000 {
        parse_arm_ldrb_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xb000000 {
        parse_arm_bl_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xa000000 {
        parse_arm_b_0(ins as u32, pc)
    } else {
        None
    }
}
pub fn parse_thumb(ins: u16, next: Option<u16>, pc: u32) -> Option<Ins> {
    if (ins & 0xffe8) == 0xb660 {
        parse_thumb_cps_0(ins as u32, pc)
    } else if (ins & 0xff78) == 0x4468 {
        parse_thumb_add_6(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4485 {
        parse_thumb_add_7(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4780 {
        parse_thumb_blx_1(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4700 {
        parse_thumb_bx_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4140 {
        parse_thumb_adc_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x0 {
        parse_thumb_mov_2(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4000 {
        parse_thumb_and_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4100 {
        parse_thumb_asr_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4380 {
        parse_thumb_bic_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x42c0 {
        parse_thumb_cmn_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4280 {
        parse_thumb_cmp_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4040 {
        parse_thumb_eor_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4080 {
        parse_thumb_lsl_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x40c0 {
        parse_thumb_lsr_1(ins as u32, pc)
    } else if (ins & 0xff80) == 0xb000 {
        parse_thumb_add_5(ins as u32, pc)
    } else if (ins & 0xff00) == 0xbe00 {
        parse_thumb_bkpt_0(ins as u32, pc)
    } else if let Some(next) = next && (ins & 0xf800) == 0xf000
        && (next & 0xd000) == 0xd000
    {
        parse_thumb_bl_0(((ins as u32) << 16) | (next as u32), pc)
    } else if let Some(next) = next && (ins & 0xf800) == 0xf000
        && (next & 0xd000) == 0xc000
    {
        parse_thumb_blx_0(((ins as u32) << 16) | (next as u32), pc)
    } else if (ins & 0xff00) == 0x4400 {
        parse_thumb_add_3(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4500 {
        parse_thumb_cmp_2(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4600 {
        parse_thumb_mov_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1800 {
        parse_thumb_add_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5800 {
        parse_thumb_ldr_3(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5c00 {
        parse_thumb_ldrb_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1c00 {
        parse_thumb_add_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5a00 {
        parse_thumb_ldrh_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5600 {
        parse_thumb_ldrsb_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5e00 {
        parse_thumb_ldrsh_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x8800 {
        parse_thumb_ldrh_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x3000 {
        parse_thumb_add_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa000 {
        parse_thumb_add_8(ins as u32, pc)
    } else if (ins & 0xf800) == 0x7800 {
        parse_thumb_ldrb_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x2800 {
        parse_thumb_cmp_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xe000 {
        parse_thumb_b_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x1000 {
        parse_thumb_asr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa800 {
        parse_thumb_add_4(ins as u32, pc)
    } else if (ins & 0xf800) == 0x0 {
        parse_thumb_lsl_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xc800 {
        parse_thumb_ldm_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x800 {
        parse_thumb_lsr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x6800 {
        parse_thumb_ldr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x2000 {
        parse_thumb_mov_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x9800 {
        parse_thumb_ldr_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x4800 {
        parse_thumb_ldr_2(ins as u32, pc)
    } else if (ins & 0xf000) == 0xd000 {
        parse_thumb_b_0(ins as u32, pc)
    } else {
        None
    }
}
fn parse_arm_adc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Adc { s, cond, rd, rn, op2 })
}
fn parse_thumb_adc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Adc { s, cond, rd, rn, op2 })
}
fn parse_arm_add_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2::Imm(((value) >> 6) & 0x7);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(((value) >> 8) & 0x7, pc);
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_2(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 6) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_3(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_4(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_5(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::Imm(((value) & 0x7f) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_6(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_7(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_8(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(15, pc);
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_arm_and_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::And { s, cond, rd, rn, op2 })
}
fn parse_thumb_and_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::And { s, cond, rd, rn, op2 })
}
fn parse_arm_asr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse((value) & 0xf, pc);
    let op2 = Op2Shift::parse(value, pc);
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_thumb_asr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2Shift::Imm(
        if (((value) >> 6) & 0x1f) != 0 { (((value) >> 6) & 0x1f) } else { 32 },
    );
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_thumb_asr_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc));
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_arm_b_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xffffff) << 2) as i32) << 6 >> 6) as u32).wrapping_add(8),
        pc,
    );
    Some(Ins::B { cond, target })
}
fn parse_thumb_b_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 8) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xff) << 1) as i32) << 23 >> 23) as u32).wrapping_add(4),
        pc,
    );
    Some(Ins::B { cond, target })
}
fn parse_thumb_b_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let target = BranchTarget::parse(
        ((((((value) & 0x7ff) << 1) as i32) << 20 >> 20) as u32).wrapping_add(4),
        pc,
    );
    Some(Ins::B { cond, target })
}
fn parse_arm_bic_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Bic { s, cond, rd, rn, op2 })
}
fn parse_thumb_bic_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Bic { s, cond, rd, rn, op2 })
}
fn parse_arm_bkpt_0(value: u32, pc: u32) -> Option<Ins> {
    let imm = ((((value) >> 8) & 0xfff) << 4) | ((value) & 0xf);
    Some(Ins::Bkpt { imm })
}
fn parse_thumb_bkpt_0(value: u32, pc: u32) -> Option<Ins> {
    let imm = (value) & 0xff;
    Some(Ins::Bkpt { imm })
}
fn parse_arm_bl_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xffffff) << 2) as i32) << 6 >> 6) as u32).wrapping_add(8),
        pc,
    );
    Some(Ins::Bl { cond, target })
}
fn parse_thumb_bl_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let target = BranchTarget::parse(
        ((((((((value) >> 16) & 0x7ff) << 12) | (((value) & 0x7ff) << 1)) as i32) << 9
            >> 9) as u32)
            .wrapping_add(4),
        pc,
    );
    Some(Ins::Bl { cond, target })
}
fn parse_arm_blx_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let target = BlxTarget::Direct(
        BranchTarget::parse(
            (((((((value) & 0xffffff) << 2) | ((((value) >> 24) & 0x1) << 1)) as i32)
                << 6 >> 6) as u32)
                .wrapping_add(8),
            pc,
        ),
    );
    Some(Ins::Blx { cond, target })
}
fn parse_arm_blx_1(value: u32, pc: u32) -> Option<Ins> {
    if (((value) >> 8) & 0xfff) != 0xfff {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BlxTarget::Indirect(Reg::parse((value) & 0xf, pc));
    Some(Ins::Blx { cond, target })
}
fn parse_thumb_blx_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let target = BlxTarget::Direct(
        BranchTarget::parse(
            ((((((((value) >> 16) & 0x7ff) << 12) | ((((value) >> 1) & 0x3ff) << 2))
                as i32) << 9 >> 9) as u32)
                .wrapping_add(4),
            pc,
        ),
    );
    Some(Ins::Blx { cond, target })
}
fn parse_thumb_blx_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let target = BlxTarget::Indirect(Reg::parse(((value) >> 3) & 0xf, pc));
    Some(Ins::Blx { cond, target })
}
fn parse_arm_bx_0(value: u32, pc: u32) -> Option<Ins> {
    if (((value) >> 8) & 0xfff) != 0xfff {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Some(Ins::Bx { cond, rm })
}
fn parse_thumb_bx_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rm = Reg::parse(((value) >> 3) & 0xf, pc);
    Some(Ins::Bx { cond, rm })
}
fn parse_arm_bxj_0(value: u32, pc: u32) -> Option<Ins> {
    if (((value) >> 8) & 0xfff) != 0xfff {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Some(Ins::Bxj { cond, rm })
}
fn parse_arm_cdp_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Cdp {
        cond,
        coproc,
        opc1,
        crd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_cdp2_0(value: u32, pc: u32) -> Option<Ins> {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Cdp2 {
        coproc,
        opc1,
        crd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_clrex_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xfff0f != 0xff00f {
        return None;
    }
    Some(Ins::Clrex {})
}
fn parse_arm_clz_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f00 != 0xf0f00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Some(Ins::Clz { cond, rd, rm })
}
fn parse_arm_cmn_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Cmn { cond, rn, op2 })
}
fn parse_thumb_cmn_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Cmn { cond, rn, op2 })
}
fn parse_arm_cmp_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc);
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_2(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_arm_cps_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xfe00 != 0 {
        return None;
    }
    let effect = CpsEffect::parse(((value) >> 18) & 0x3, pc);
    let aif = AifFlags::parse(((value) >> 6) & 0x7, pc);
    let mode = (value) & 0x1f;
    Some(Ins::Cps { effect, aif, mode })
}
fn parse_thumb_cps_0(value: u32, pc: u32) -> Option<Ins> {
    let effect = CpsEffect::parse(((value) >> 4) & 0x3, pc);
    let aif = AifFlags::parse((value) & 0x7, pc);
    let mode = 0;
    Some(Ins::Cps { effect, aif, mode })
}
fn parse_arm_csdb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    Some(Ins::Csdb { cond })
}
fn parse_arm_eor_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Eor { s, cond, rd, rn, op2 })
}
fn parse_thumb_eor_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Eor { s, cond, rd, rn, op2 })
}
fn parse_arm_ldc_0(value: u32, pc: u32) -> Option<Ins> {
    let l = (((value) >> 22) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let dest = AddrLdcStc::parse(value, pc);
    Some(Ins::Ldc {
        l,
        cond,
        coproc,
        crd,
        dest,
    })
}
fn parse_arm_ldc2_0(value: u32, pc: u32) -> Option<Ins> {
    let l = (((value) >> 22) & 0x1) != 0;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let dest = AddrLdcStc::parse(value, pc);
    Some(Ins::Ldc2 { l, coproc, crd, dest })
}
fn parse_arm_ldm_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc);
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = RegList::parse((value) & 0xffff);
    let user_mode = (0) != 0;
    Some(Ins::Ldm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_arm_ldm_1(value: u32, pc: u32) -> Option<Ins> {
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc);
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let writeback = (0) != 0;
    let regs = RegList::parse((value) & 0x7fff);
    let user_mode = (1) != 0;
    Some(Ins::Ldm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_arm_ldm_2(value: u32, pc: u32) -> Option<Ins> {
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc);
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = RegList::parse((value) & 0xffff);
    let user_mode = (1) != 0;
    Some(Ins::Ldm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_thumb_ldm_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = LdmStmMode::default();
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc);
    let writeback = ((!((value) & 0xff) >> (((value) >> 8) & 0x7)) & 1) != 0;
    let regs = RegList::parse((value) & 0xff);
    let user_mode = (0) != 0;
    Some(Ins::Ldm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_arm_ldr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrLdrStr::parse(value, pc);
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(13, pc),
        offset: LdrStrOffset::Imm((((value) & 0xff) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_2(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(15, pc),
        offset: LdrStrOffset::Imm((((value) & 0xff) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_3(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc),
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_arm_ldrb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrLdrStr::parse(value, pc);
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_thumb_ldrb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f)) as i32),
        writeback: false,
    };
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_thumb_ldrb_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc),
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_arm_ldrbt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrLdrStr::parse(value, pc);
    Some(Ins::Ldrbt { cond, rd, addr })
}
fn parse_arm_ldrd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x1000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc);
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrd { cond, rd, rd2, addr })
}
fn parse_arm_ldrd_1(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x01201000 != 0x01000000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc);
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrd { cond, rd, rd2, addr })
}
fn parse_arm_ldrex_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    Some(Ins::Ldrex { cond, rd, rn })
}
fn parse_arm_ldrexb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    Some(Ins::Ldrexb { cond, rd, rn })
}
fn parse_arm_ldrexd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    Some(Ins::Ldrexd { cond, rd, rd2, rn })
}
fn parse_arm_ldrexh_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    Some(Ins::Ldrexh { cond, rd, rn })
}
fn parse_arm_ldrh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_thumb_ldrh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: MiscLoadOffset::Imm(((((value) >> 6) & 0x1f) << 1) as i32),
        writeback: false,
    };
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_thumb_ldrh_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc),
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_arm_ldrsb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrsb { cond, rd, addr })
}
fn parse_thumb_ldrsb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc),
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrsb { cond, rd, addr })
}
fn parse_arm_ldrsh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrsh { cond, rd, addr })
}
fn parse_thumb_ldrsh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc),
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc),
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrsh { cond, rd, addr })
}
fn parse_arm_ldrt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let addr = AddrLdrStr::Post(AddrLdrStrPost::parse(value, pc));
    Some(Ins::Ldrt { cond, rd, addr })
}
fn parse_arm_lsl_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse((value) & 0xf, pc);
    let op2 = Op2Shift::parse(value, pc);
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsl_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2Shift::Imm(((value) >> 6) & 0x1f);
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsl_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc));
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_arm_lsr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse((value) & 0xf, pc);
    let op2 = Op2Shift::parse(value, pc);
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2Shift::Imm(
        if (((value) >> 6) & 0x1f) != 0 { (((value) >> 6) & 0x1f) } else { 32 },
    );
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsr_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc));
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_arm_mcr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Mcr {
        cond,
        coproc,
        opc1,
        rd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_mcr2_0(value: u32, pc: u32) -> Option<Ins> {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Mcr2 {
        coproc,
        opc1,
        rd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_mcrr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc = ((value) >> 20) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    Some(Ins::Mcrr {
        cond,
        coproc,
        opc,
        rd,
        rd2,
        crm,
    })
}
fn parse_arm_mcrr2_0(value: u32, pc: u32) -> Option<Ins> {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc = ((value) >> 20) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    Some(Ins::Mcrr2 {
        coproc,
        opc,
        rd,
        rd2,
        crm,
    })
}
fn parse_arm_mla_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 16) & 0xf, pc);
    let rn = Reg::parse((value) & 0xf, pc);
    let rm = Reg::parse(((value) >> 8) & 0xf, pc);
    let ra = Reg::parse(((value) >> 12) & 0xf, pc);
    Some(Ins::Mla {
        s,
        cond,
        rd,
        rn,
        rm,
        ra,
    })
}
fn parse_arm_mov_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_arm_mov_1(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (0) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_2(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_arm_mrc_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Mrc {
        cond,
        coproc,
        opc1,
        rd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_mrc2_0(value: u32, pc: u32) -> Option<Ins> {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Some(Ins::Mrc2 {
        coproc,
        opc1,
        rd,
        crn,
        crm,
        opc2,
    })
}
fn parse_arm_mrrc_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc = ((value) >> 4) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    Some(Ins::Mrrc {
        cond,
        coproc,
        opc,
        rd,
        rd2,
        crm,
    })
}
fn parse_arm_mrrc2_0(value: u32, pc: u32) -> Option<Ins> {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc = ((value) >> 4) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    Some(Ins::Mrrc2 {
        coproc,
        opc,
        rd,
        rd2,
        crm,
    })
}
fn parse_arm_mrs_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x000f0d0f != 0x000f0000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let status_reg = StatusReg::parse(((value) >> 22) & 0x1, pc);
    Some(Ins::Mrs { cond, rd, status_reg })
}
fn parse_arm_msr_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let status_fields = StatusFields::parse(value, pc);
    let op2 = MsrOp2::parse(value, pc)?;
    Some(Ins::Msr {
        cond,
        status_fields,
        op2,
    })
}

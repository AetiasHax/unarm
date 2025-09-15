#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::eq_op)]
#![allow(clippy::double_parens)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::needless_else)]
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
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0xf000000) == 0xb000000 {
            Some(
                Self::Direct(
                    BranchTarget::parse(
                        (((((((value) & 0xffffff) << 2) | ((((value) >> 24) & 0x1) << 1))
                            as i32) << 6 >> 6) as u32)
                            .wrapping_add(8),
                        pc,
                    ),
                ),
            )
        } else if (value & 0xffffff0) == 0x12fff30 {
            Some(Self::Indirect(Reg::parse(((value) & 0xf), pc)?))
        } else {
            None
        }
    }
}
impl Cond {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Eq),
            0x1 => Some(Self::Ne),
            0x2 => Some(Self::Hs),
            0x3 => Some(Self::Lo),
            0x4 => Some(Self::Mi),
            0x5 => Some(Self::Pl),
            0x6 => Some(Self::Vs),
            0x7 => Some(Self::Vc),
            0x8 => Some(Self::Hi),
            0x9 => Some(Self::Ls),
            0xa => Some(Self::Ge),
            0xb => Some(Self::Lt),
            0xc => Some(Self::Gt),
            0xd => Some(Self::Le),
            0xe => Some(Self::Al),
            _ => None,
        }
    }
}
impl Reg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::R0),
            0x1 => Some(Self::R1),
            0x2 => Some(Self::R2),
            0x3 => Some(Self::R3),
            0x4 => Some(Self::R4),
            0x5 => Some(Self::R5),
            0x6 => Some(Self::R6),
            0x7 => Some(Self::R7),
            0x8 => Some(Self::R8),
            0x9 => Some(Self::R9),
            0xa => Some(Self::R10),
            0xb => Some(Self::R11),
            0xc => Some(Self::R12),
            0xd => Some(Self::Sp),
            0xe => Some(Self::Lr),
            0xf => Some(Self::Pc),
            _ => None,
        }
    }
}
impl StatusReg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Cpsr),
            0x1 => Some(Self::Spsr),
            _ => None,
        }
    }
}
impl StatusFields {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        Some(Self {
            reg: StatusReg::parse((((value) >> 22) & 0x1), pc)?,
            c: (((value) >> 16) & 0x1) != 0,
            x: (((value) >> 17) & 0x1) != 0,
            s: (((value) >> 18) & 0x1) != 0,
            f: (((value) >> 19) & 0x1) != 0,
        })
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
            Some(Self::Reg(Reg::parse(((value) & 0xf), pc)?))
        } else {
            None
        }
    }
}
impl ShiftOp {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Lsl),
            0x1 => Some(Self::Lsr),
            0x2 => Some(Self::Asr),
            0x3 => Some(Self::Ror),
            _ => None,
        }
    }
}
impl Coproc {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::P0),
            0x1 => Some(Self::P1),
            0x2 => Some(Self::P2),
            0x3 => Some(Self::P3),
            0x4 => Some(Self::P4),
            0x5 => Some(Self::P5),
            0x6 => Some(Self::P6),
            0x7 => Some(Self::P7),
            0x8 => Some(Self::P8),
            0x9 => Some(Self::P9),
            0xa => Some(Self::P10),
            0xb => Some(Self::P11),
            0xc => Some(Self::P12),
            0xd => Some(Self::P13),
            0xe => Some(Self::P14),
            0xf => Some(Self::P15),
            _ => None,
        }
    }
}
impl CoReg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::C0),
            0x1 => Some(Self::C1),
            0x2 => Some(Self::C2),
            0x3 => Some(Self::C3),
            0x4 => Some(Self::C4),
            0x5 => Some(Self::C5),
            0x6 => Some(Self::C6),
            0x7 => Some(Self::C7),
            0x8 => Some(Self::C8),
            0x9 => Some(Self::C9),
            0xa => Some(Self::C10),
            0xb => Some(Self::C11),
            0xc => Some(Self::C12),
            0xd => Some(Self::C13),
            0xe => Some(Self::C14),
            0xf => Some(Self::C15),
            _ => None,
        }
    }
}
impl Op2 {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x2000000) == 0x2000000 {
            Some(Self::Imm(((value) & 0xff).rotate_right((((value) >> 8) & 0xf) << 1)))
        } else if (value & 0x2000010) == 0x10 {
            Some(Self::ShiftReg(ShiftReg::parse(((value) & 0xfff), pc)?))
        } else if (value & 0x2000010) == 0x0 {
            Some(Self::ShiftImm(ShiftImm::parse(((value) & 0xfff), pc)?))
        } else {
            None
        }
    }
}
impl ShiftReg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        Some(Self {
            rm: Reg::parse(((value) & 0xf), pc)?,
            shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc)?,
            rs: Reg::parse((((value) >> 8) & 0xf), pc)?,
        })
    }
}
impl ShiftImm {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        Some(Self {
            rm: Reg::parse(((value) & 0xf), pc)?,
            shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc)?,
            imm: (((value) >> 7) & 0x1f),
        })
    }
}
impl Op2Shift {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x10) == 0x0 {
            Some(
                Self::Imm(
                    if (((value) >> 7) & 0x1f) != 0 {
                        (((value) >> 7) & 0x1f)
                    } else {
                        32
                    },
                ),
            )
        } else if (value & 0x90) == 0x10 {
            Some(Self::Reg(Reg::parse((((value) >> 8) & 0xf), pc)?))
        } else {
            None
        }
    }
}
impl CpsEffect {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x3) == 0x0 {
            Some(Self::SetMode)
        } else if (value & 0x3) == 0x2 {
            Some(Self::Ie)
        } else if (value & 0x3) == 0x3 {
            Some(Self::Id)
        } else {
            None
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
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x1000000) == 0x1000000 {
            Some(Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                offset: ((if (((value) >> 23) & 0x1) == 0 {
                    -((((value) & 0xff) << 2) as i32)
                } else {
                    (((value) & 0xff) << 2) as i32
                })) as i32,
                writeback: ((((value) >> 21) & 0x1)) != 0,
            })
        } else if (value & 0x1200000) == 0x200000 {
            Some(Self::Post {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                offset: ((if (((value) >> 23) & 0x1) == 0 {
                    -((((value) & 0xff) << 2) as i32)
                } else {
                    (((value) & 0xff) << 2) as i32
                })) as i32,
            })
        } else if (value & 0x1a00000) == 0x800000 {
            Some(Self::Unidx {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                option: ((value) & 0xff),
            })
        } else {
            None
        }
    }
}
impl LdmStmMode {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Da),
            0x1 => Some(Self::Ia),
            0x2 => Some(Self::Db),
            0x3 => Some(Self::Ib),
            _ => None,
        }
    }
}
impl AddrLdrStr {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x1000000) == 0x1000000 {
            Some(Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                offset: LdrStrOffset::parse((value), pc)?,
                writeback: ((((value) >> 21) & 0x1)) != 0,
            })
        } else if (value & 0x1200000) == 0x0 {
            Some(Self::Post(AddrLdrStrPost::parse((value), pc)?))
        } else {
            None
        }
    }
}
impl AddrLdrStrPost {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        Some(Self {
            rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
            offset: LdrStrOffset::parse((value), pc)?,
        })
    }
}
impl LdrStrOffset {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x2000000) == 0x0 {
            Some(
                Self::Imm(
                    ((if (((value) >> 23) & 0x1) == 0 {
                        -(((value) & 0xfff) as i32)
                    } else {
                        ((value) & 0xfff) as i32
                    })) as i32,
                ),
            )
        } else if (value & 0x2000000) == 0x2000000 {
            Some(Self::Reg {
                subtract: ((((value) >> 23) & 0x1) ^ 1) != 0,
                rm: Reg::parse(((value) & 0xf), pc)?,
                shift_op: ShiftOp::parse((((value) >> 5) & 0x3), pc)?,
                imm: if (((value) >> 5) & 0x3) == 1 && (((value) >> 7) & 0x1f) == 0 {
                    0x20
                } else {
                    (((value) >> 7) & 0x1f)
                },
            })
        } else {
            None
        }
    }
}
impl AddrMiscLoad {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x1000000) == 0x1000000 {
            Some(Self::Pre {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                offset: MiscLoadOffset::parse((value), pc)?,
                writeback: ((((value) >> 21) & 0x1)) != 0,
            })
        } else if (value & 0x1200000) == 0x0 {
            Some(Self::Post {
                rn: Reg::parse((((value) >> 16) & 0xf), pc)?,
                offset: MiscLoadOffset::parse((value), pc)?,
            })
        } else {
            None
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
                rm: Reg::parse(((value) & 0xf), pc)?,
                subtract: ((((value) >> 23) & 0x1) ^ 1) != 0,
            })
        } else {
            None
        }
    }
}
impl SrsRfeMode {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Da),
            0x1 => Some(Self::Ia),
            0x2 => Some(Self::Db),
            0x3 => Some(Self::Ib),
            _ => None,
        }
    }
}
impl Endianness {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Le),
            0x1 => Some(Self::Be),
            _ => None,
        }
    }
}
impl RegSide {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Bottom),
            0x1 => Some(Self::Top),
            _ => None,
        }
    }
}
impl Sreg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::S0),
            0x1 => Some(Self::S1),
            0x2 => Some(Self::S2),
            0x3 => Some(Self::S3),
            0x4 => Some(Self::S4),
            0x5 => Some(Self::S5),
            0x6 => Some(Self::S6),
            0x7 => Some(Self::S7),
            0x8 => Some(Self::S8),
            0x9 => Some(Self::S9),
            0xa => Some(Self::S10),
            0xb => Some(Self::S11),
            0xc => Some(Self::S12),
            0xd => Some(Self::S13),
            0xe => Some(Self::S14),
            0xf => Some(Self::S15),
            0x10 => Some(Self::S16),
            0x11 => Some(Self::S17),
            0x12 => Some(Self::S18),
            0x13 => Some(Self::S19),
            0x14 => Some(Self::S20),
            0x15 => Some(Self::S21),
            0x16 => Some(Self::S22),
            0x17 => Some(Self::S23),
            0x18 => Some(Self::S24),
            0x19 => Some(Self::S25),
            0x1a => Some(Self::S26),
            0x1b => Some(Self::S27),
            0x1c => Some(Self::S28),
            0x1d => Some(Self::S29),
            0x1e => Some(Self::S30),
            0x1f => Some(Self::S31),
            _ => None,
        }
    }
}
impl Dreg {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::D0),
            0x1 => Some(Self::D1),
            0x2 => Some(Self::D2),
            0x3 => Some(Self::D3),
            0x4 => Some(Self::D4),
            0x5 => Some(Self::D5),
            0x6 => Some(Self::D6),
            0x7 => Some(Self::D7),
            0x8 => Some(Self::D8),
            0x9 => Some(Self::D9),
            0xa => Some(Self::D10),
            0xb => Some(Self::D11),
            0xc => Some(Self::D12),
            0xd => Some(Self::D13),
            0xe => Some(Self::D14),
            0xf => Some(Self::D15),
            0x10 => Some(Self::D16),
            0x11 => Some(Self::D17),
            0x12 => Some(Self::D18),
            0x13 => Some(Self::D19),
            0x14 => Some(Self::D20),
            0x15 => Some(Self::D21),
            0x16 => Some(Self::D22),
            0x17 => Some(Self::D23),
            0x18 => Some(Self::D24),
            0x19 => Some(Self::D25),
            0x1a => Some(Self::D26),
            0x1b => Some(Self::D27),
            0x1c => Some(Self::D28),
            0x1d => Some(Self::D29),
            0x1e => Some(Self::D30),
            0x1f => Some(Self::D31),
            _ => None,
        }
    }
}
impl VcmpF32Op2 {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x3f) == 0x0 {
            Some(Self::Zero)
        } else if (value & 0x10) == 0x0 {
            Some(
                Self::Reg(
                    Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?,
                ),
            )
        } else {
            None
        }
    }
}
impl VcmpF64Op2 {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        if (value & 0x3f) == 0x0 {
            Some(Self::Zero)
        } else if (value & 0x10) == 0x0 {
            Some(
                Self::Reg(
                    Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?,
                ),
            )
        } else {
            None
        }
    }
}
impl DregIndex {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        Some(Self {
            dreg: Dreg::parse(
                ((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf),
                pc,
            )?,
            index: (((value) >> 21) & 0x1),
        })
    }
}
impl Fpscr {
    pub(crate) fn parse(value: u32, pc: u32) -> Self {
        Self {}
    }
}
impl VldmVstmMode {
    pub(crate) fn parse(value: u32, pc: u32) -> Option<Self> {
        match value {
            0x1 => Some(Self::Ia),
            0x2 => Some(Self::Db),
            _ => None,
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
impl Default for SrsRfeMode {
    fn default() -> Self {
        Self::Ia
    }
}
impl Default for Fpscr {
    fn default() -> Self {
        Self {}
    }
}
pub fn parse_arm(ins: u32, pc: u32) -> Option<Ins> {
    if (ins & 0xfff0fff) == 0x49d0004 {
        parse_arm_pop_1(ins as u32, pc)
    } else if (ins & 0xfff0fff) == 0x52d0004 {
        parse_arm_push_1(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200002 {
        parse_arm_wfe_0(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200001 {
        parse_arm_yield_0(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200014 {
        parse_arm_csdb_0(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200000 {
        parse_arm_nop_0(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200004 {
        parse_arm_sev_0(ins as u32, pc)
    } else if (ins & 0xfff00ff) == 0x3200003 {
        parse_arm_wfi_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb00bc0 {
        parse_arm_vabs_f64_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb00ac0 {
        parse_arm_vabs_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb70bc0 {
        parse_arm_vcvt_f32_f64_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb70ac0 {
        parse_arm_vcvt_f64_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb00a40 {
        parse_arm_vmov_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb00b40 {
        parse_arm_vmov_f64_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb10a40 {
        parse_arm_vneg_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb10b40 {
        parse_arm_vneg_f64_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb10ac0 {
        parse_arm_vsqrt_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0fd0) == 0xeb10bc0 {
        parse_arm_vsqrt_f64_0(ins as u32, pc)
    } else if (ins & 0xfff100f0) == 0xf1010000 {
        parse_arm_setend_0(ins as u32, pc)
    } else if (ins & 0xfbf0f50) == 0xeb40a40 {
        parse_arm_vcmp_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0f50) == 0xeb40b40 {
        parse_arm_vcmp_f64_0(ins as u32, pc)
    } else if (ins & 0xfff0f10) == 0xef10a10 {
        parse_arm_vmrs_0(ins as u32, pc)
    } else if (ins & 0xfff0f10) == 0xee10a10 {
        parse_arm_vmsr_0(ins as u32, pc)
    } else if (ins & 0xfbc0fd0) == 0xeb80a40 {
        parse_arm_vcvt_f32_u32_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xf5700010 {
        parse_arm_clrex_0(ins as u32, pc)
    } else if (ins & 0xfbc0fd0) == 0xeb80ac0 {
        parse_arm_vcvt_f32_s32_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xe1200070 {
        parse_arm_bkpt_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x6af0070 {
        parse_arm_sxtb_0(ins as u32, pc)
    } else if (ins & 0xfbc0fd0) == 0xeb80bc0 {
        parse_arm_vcvt_f64_s32_0(ins as u32, pc)
    } else if (ins & 0xfbc0fd0) == 0xeb80b40 {
        parse_arm_vcvt_f64_u32_0(ins as u32, pc)
    } else if (ins & 0xfbd0f50) == 0xebd0a40 {
        parse_arm_vcvt_s32_f32_0(ins as u32, pc)
    } else if (ins & 0xfbd0f50) == 0xebd0b40 {
        parse_arm_vcvt_s32_f64_0(ins as u32, pc)
    } else if (ins & 0xfbd0f50) == 0xebc0a40 {
        parse_arm_vcvt_u32_f32_0(ins as u32, pc)
    } else if (ins & 0xfbd0f50) == 0xebc0b40 {
        parse_arm_vcvt_u32_f64_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x68f0070 {
        parse_arm_sxtb16_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x6bf0070 {
        parse_arm_sxth_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xe7f000f0 {
        parse_arm_udf_0(ins as u32, pc)
    } else if (ins & 0xff0f0f0) == 0x780f010 {
        parse_arm_usad8_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x6ef0070 {
        parse_arm_uxtb_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x6cf0070 {
        parse_arm_uxtb16_0(ins as u32, pc)
    } else if (ins & 0xfff00f0) == 0x6ff0070 {
        parse_arm_uxth_0(ins as u32, pc)
    } else if (ins & 0xff0f0d0) == 0x750f010 {
        parse_arm_smmul_0(ins as u32, pc)
    } else if (ins & 0xff0f0d0) == 0x700f010 {
        parse_arm_smuad_0(ins as u32, pc)
    } else if (ins & 0xff0f0d0) == 0x700f050 {
        parse_arm_smusd_0(ins as u32, pc)
    } else if (ins & 0xff00fd0) == 0xc500a10 {
        parse_arm_vmov_reg_f32_dual_0(ins as u32, pc)
    } else if (ins & 0xff00fd0) == 0xc400a10 {
        parse_arm_vmov_f32_reg_dual_0(ins as u32, pc)
    } else if (ins & 0xff00fd0) == 0xc500b10 {
        parse_arm_vmov_reg_f64_0(ins as u32, pc)
    } else if (ins & 0xff00fd0) == 0xc400b10 {
        parse_arm_vmov_f64_reg_0(ins as u32, pc)
    } else if (ins & 0xfbf0f00) == 0xcbd0a00 {
        parse_arm_vpop_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0f00) == 0xcbd0b00 {
        parse_arm_vpop_f64_0(ins as u32, pc)
    } else if (ins & 0xfbf0f00) == 0xd2d0a00 {
        parse_arm_vpush_f32_0(ins as u32, pc)
    } else if (ins & 0xfbf0f00) == 0xd2d0b00 {
        parse_arm_vpush_f64_0(ins as u32, pc)
    } else if (ins & 0xfe00ff0) == 0x1a00000 {
        parse_arm_mov_1(ins as u32, pc)
    } else if (ins & 0xfe00ff0) == 0x1a00060 {
        parse_arm_rrx_0(ins as u32, pc)
    } else if (ins & 0xfff10020) == 0xf1000000 {
        parse_arm_cps_0(ins as u32, pc)
    } else if (ins & 0xfd00f70) == 0xe000b10 {
        parse_arm_vmov_32_reg_0(ins as u32, pc)
    } else if (ins & 0xfd00f70) == 0xe100b10 {
        parse_arm_vmov_reg_32_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00040 {
        parse_arm_asr_0(ins as u32, pc)
    } else if (ins & 0xe5f00f0) == 0x4f00d0 {
        parse_arm_ldrd_1(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00060 {
        parse_arm_ror_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe300a00 {
        parse_arm_vadd_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe300b00 {
        parse_arm_vadd_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe800a00 {
        parse_arm_vdiv_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe800b00 {
        parse_arm_vdiv_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe200a00 {
        parse_arm_vmul_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe200b00 {
        parse_arm_vmul_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe000a00 {
        parse_arm_vmla_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe000b00 {
        parse_arm_vmla_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe100a40 {
        parse_arm_vnmla_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe100b40 {
        parse_arm_vnmla_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe100a00 {
        parse_arm_vnmls_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe100b00 {
        parse_arm_vnmls_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe200a40 {
        parse_arm_vnmul_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe200b40 {
        parse_arm_vnmul_f64_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe000a40 {
        parse_arm_vmls_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe000b40 {
        parse_arm_vmls_f64_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00000 {
        parse_arm_lsl_0(ins as u32, pc)
    } else if (ins & 0xff00f10) == 0xe000a10 {
        parse_arm_vmov_f32_reg_0(ins as u32, pc)
    } else if (ins & 0xfef0060) == 0x1a00020 {
        parse_arm_lsr_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe300a40 {
        parse_arm_vsub_f32_0(ins as u32, pc)
    } else if (ins & 0xfb00f50) == 0xe300b40 {
        parse_arm_vsub_f64_0(ins as u32, pc)
    } else if (ins & 0xff00f10) == 0xe100a10 {
        parse_arm_vmov_reg_f32_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1e00090 {
        parse_arm_strexh_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6100090 {
        parse_arm_sadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6100030 {
        parse_arm_sasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x68000b0 {
        parse_arm_sel_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6300010 {
        parse_arm_shadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6300090 {
        parse_arm_shadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6300030 {
        parse_arm_shasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6300050 {
        parse_arm_shsax_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6300070 {
        parse_arm_shsub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x63000f0 {
        parse_arm_shsub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6a00030 {
        parse_arm_ssat16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6100050 {
        parse_arm_ssax_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6100070 {
        parse_arm_ssub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x61000f0 {
        parse_arm_ssub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1800090 {
        parse_arm_strex_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1c00090 {
        parse_arm_strexb_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1a00090 {
        parse_arm_strexd_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1000090 {
        parse_arm_swp_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1400090 {
        parse_arm_swpb_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6a00070 {
        parse_arm_sxtab_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6800070 {
        parse_arm_sxtab16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6b00070 {
        parse_arm_sxtah_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6500010 {
        parse_arm_uadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6500090 {
        parse_arm_uadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6500030 {
        parse_arm_uasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6700010 {
        parse_arm_uhadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6700090 {
        parse_arm_uhadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6700030 {
        parse_arm_uhasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6700050 {
        parse_arm_uhsax_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6700070 {
        parse_arm_uhsub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x67000f0 {
        parse_arm_uhsub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x400090 {
        parse_arm_umaal_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6600010 {
        parse_arm_uqadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6600090 {
        parse_arm_uqadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6600030 {
        parse_arm_uqasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6600070 {
        parse_arm_uqsub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x66000f0 {
        parse_arm_uqsub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x7800010 {
        parse_arm_usada8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6e00030 {
        parse_arm_usat16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6500050 {
        parse_arm_usax_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6500070 {
        parse_arm_usub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x65000f0 {
        parse_arm_usub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6e00070 {
        parse_arm_uxtab_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6c00070 {
        parse_arm_uxtab16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6f00070 {
        parse_arm_uxtah_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1900090 {
        parse_arm_ldrex_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1d00090 {
        parse_arm_ldrexb_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1b00090 {
        parse_arm_ldrexd_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1f00090 {
        parse_arm_ldrexh_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200030 {
        parse_arm_blx_1(ins as u32, pc)
    } else if (ins & 0xfff00000) == 0xfc400000 {
        parse_arm_mcrr2_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200010 {
        parse_arm_bx_0(ins as u32, pc)
    } else if (ins & 0xfff00000) == 0xfc500000 {
        parse_arm_mrrc2_0(ins as u32, pc)
    } else if (ins & 0xfb002f0) == 0x1000000 {
        parse_arm_mrs_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200020 {
        parse_arm_bxj_0(ins as u32, pc)
    } else if (ins & 0xfff0000) == 0x8bd0000 {
        parse_arm_pop_0(ins as u32, pc)
    } else if (ins & 0xfff0000) == 0x92d0000 {
        parse_arm_push_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1000050 {
        parse_arm_qadd_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6200010 {
        parse_arm_qadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6200090 {
        parse_arm_qadd8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6200030 {
        parse_arm_qasx_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1400050 {
        parse_arm_qdadd_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1600050 {
        parse_arm_qdsub_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6200050 {
        parse_arm_qsax_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1200050 {
        parse_arm_qsub_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6200070 {
        parse_arm_qsub16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x62000f0 {
        parse_arm_qsub8_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6b00030 {
        parse_arm_rev_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6b000b0 {
        parse_arm_rev16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6f000b0 {
        parse_arm_revsh_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x1600010 {
        parse_arm_clz_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6100010 {
        parse_arm_sadd16_0(ins as u32, pc)
    } else if (ins & 0xff000f0) == 0x6600050 {
        parse_arm_uqsax_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0xa00090 {
        parse_arm_umlal_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x7000010 {
        parse_arm_smlad_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0xe00090 {
        parse_arm_smlal_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x7400010 {
        parse_arm_smlald_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0x200090 {
        parse_arm_mla_0(ins as u32, pc)
    } else if (ins & 0xff000b0) == 0x1200080 {
        parse_arm_smlaw_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x7000050 {
        parse_arm_smlsd_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x7400050 {
        parse_arm_smlsld_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0x90 {
        parse_arm_mul_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x7500010 {
        parse_arm_smmla_0(ins as u32, pc)
    } else if (ins & 0xff00070) == 0x6800010 {
        parse_arm_pkhbt_0(ins as u32, pc)
    } else if (ins & 0xff00070) == 0x6800050 {
        parse_arm_pkhtb_0(ins as u32, pc)
    } else if (ins & 0xff000d0) == 0x75000d0 {
        parse_arm_smmls_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0xc00090 {
        parse_arm_smull_0(ins as u32, pc)
    } else if (ins & 0xff000b0) == 0x12000a0 {
        parse_arm_smulw_0(ins as u32, pc)
    } else if (ins & 0xfe000f0) == 0x800090 {
        parse_arm_umull_0(ins as u32, pc)
    } else if (ins & 0xff00090) == 0x1000080 {
        parse_arm_smla_0(ins as u32, pc)
    } else if (ins & 0xff00090) == 0x1400080 {
        parse_arm_smlal_half_0(ins as u32, pc)
    } else if (ins & 0xff100010) == 0xfe000010 {
        parse_arm_mcr2_0(ins as u32, pc)
    } else if (ins & 0xf300f00) == 0xd100a00 {
        parse_arm_vldr_f32_0(ins as u32, pc)
    } else if (ins & 0xf300f00) == 0xd100b00 {
        parse_arm_vldr_f64_0(ins as u32, pc)
    } else if (ins & 0xff100010) == 0xfe100010 {
        parse_arm_mrc2_0(ins as u32, pc)
    } else if (ins & 0xfd700000) == 0xf5500000 {
        parse_arm_pld_0(ins as u32, pc)
    } else if (ins & 0xff00090) == 0x1600080 {
        parse_arm_smul_0(ins as u32, pc)
    } else if (ins & 0xf300f00) == 0xd000a00 {
        parse_arm_vstr_f32_0(ins as u32, pc)
    } else if (ins & 0xf300f00) == 0xd000b00 {
        parse_arm_vstr_f64_0(ins as u32, pc)
    } else if (ins & 0xfe00030) == 0x6e00010 {
        parse_arm_usat_0(ins as u32, pc)
    } else if (ins & 0xff000010) == 0xfe000000 {
        parse_arm_cdp2_0(ins as u32, pc)
    } else if (ins & 0xfe500000) == 0xf8400000 {
        parse_arm_srs_0(ins as u32, pc)
    } else if (ins & 0xfe00030) == 0x6a00010 {
        parse_arm_ssat_0(ins as u32, pc)
    } else if (ins & 0xfe500000) == 0xf8100000 {
        parse_arm_rfe_0(ins as u32, pc)
    } else if (ins & 0xe100f00) == 0xc000a00 {
        parse_arm_vstm_f32_0(ins as u32, pc)
    } else if (ins & 0xff00000) == 0xc400000 {
        parse_arm_mcrr_0(ins as u32, pc)
    } else if (ins & 0xe100f00) == 0xc100a00 {
        parse_arm_vldm_f32_0(ins as u32, pc)
    } else if (ins & 0xe100f00) == 0xc100b00 {
        parse_arm_vldm_f64_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0xf0 {
        parse_arm_strd_0(ins as u32, pc)
    } else if (ins & 0xff00000) == 0xc500000 {
        parse_arm_mrrc_0(ins as u32, pc)
    } else if (ins & 0xfe100000) == 0xfc000000 {
        parse_arm_stc2_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0xd0 {
        parse_arm_ldrd_0(ins as u32, pc)
    } else if (ins & 0xfe100000) == 0xfc100000 {
        parse_arm_ldc2_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0xb0 {
        parse_arm_strh_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000b0 {
        parse_arm_ldrh_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000d0 {
        parse_arm_ldrsb_0(ins as u32, pc)
    } else if (ins & 0xe1000f0) == 0x1000f0 {
        parse_arm_ldrsh_0(ins as u32, pc)
    } else if (ins & 0xe100f00) == 0xc000b00 {
        parse_arm_vstm_f64_0(ins as u32, pc)
    } else if (ins & 0xfe000000) == 0xfa000000 {
        parse_arm_blx_0(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1500000 {
        parse_arm_cmp_0(ins as u32, pc)
    } else if (ins & 0xfe00000) == 0x3a00000 {
        parse_arm_mov_0(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1300000 {
        parse_arm_teq_0(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1100000 {
        parse_arm_tst_0(ins as u32, pc)
    } else if (ins & 0xe708000) == 0x8500000 {
        parse_arm_ldm_1(ins as u32, pc)
    } else if (ins & 0xdf00000) == 0x1700000 {
        parse_arm_cmn_0(ins as u32, pc)
    } else if (ins & 0xd700000) == 0x4300000 {
        parse_arm_ldrt_0(ins as u32, pc)
    } else if (ins & 0xe508000) == 0x8508000 {
        parse_arm_ldm_2(ins as u32, pc)
    } else if (ins & 0xd700000) == 0x4700000 {
        parse_arm_ldrbt_0(ins as u32, pc)
    } else if (ins & 0xd700000) == 0x4600000 {
        parse_arm_strbt_0(ins as u32, pc)
    } else if (ins & 0xf100010) == 0xe100010 {
        parse_arm_mrc_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0xa00000 {
        parse_arm_adc_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x1c00000 {
        parse_arm_bic_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x0 {
        parse_arm_and_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x800000 {
        parse_arm_add_0(ins as u32, pc)
    } else if (ins & 0xd700000) == 0x4200000 {
        parse_arm_strt_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x400000 {
        parse_arm_sub_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x200000 {
        parse_arm_eor_0(ins as u32, pc)
    } else if (ins & 0xdb00000) == 0x1200000 {
        parse_arm_msr_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x1e00000 {
        parse_arm_mvn_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x1800000 {
        parse_arm_orr_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0xc00000 {
        parse_arm_sbc_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x600000 {
        parse_arm_rsb_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0xe00000 {
        parse_arm_rsc_0(ins as u32, pc)
    } else if (ins & 0xf100010) == 0xe000010 {
        parse_arm_mcr_0(ins as u32, pc)
    } else if (ins & 0xf000010) == 0xe000000 {
        parse_arm_cdp_0(ins as u32, pc)
    } else if (ins & 0xe500000) == 0x8100000 {
        parse_arm_ldm_0(ins as u32, pc)
    } else if (ins & 0xe100000) == 0xc100000 {
        parse_arm_ldc_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xb000000 {
        parse_arm_bl_0(ins as u32, pc)
    } else if (ins & 0xe100000) == 0x8000000 {
        parse_arm_stm_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4000000 {
        parse_arm_str_0(ins as u32, pc)
    } else if (ins & 0xe100000) == 0xc000000 {
        parse_arm_stc_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4400000 {
        parse_arm_strb_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4100000 {
        parse_arm_ldr_0(ins as u32, pc)
    } else if (ins & 0xc500000) == 0x4500000 {
        parse_arm_ldrb_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xf000000 {
        parse_arm_svc_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xa000000 {
        parse_arm_b_0(ins as u32, pc)
    } else {
        None
    }
}
pub fn parse_thumb(ins: u16, next: Option<u16>, pc: u32) -> Option<Ins> {
    if (ins & 0xffff) == 0xbf00 {
        parse_thumb_nop_0(ins as u32, pc)
    } else if (ins & 0xff78) == 0x4468 {
        parse_thumb_add_6(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4485 {
        parse_thumb_add_7(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4780 {
        parse_thumb_blx_1(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4700 {
        parse_thumb_bx_0(ins as u32, pc)
    } else if (ins & 0xffe8) == 0xb660 {
        parse_thumb_cps_0(ins as u32, pc)
    } else if (ins & 0xffe0) == 0xb640 {
        parse_thumb_setend_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xb2c0 {
        parse_thumb_uxtb_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x42c0 {
        parse_thumb_cmn_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4280 {
        parse_thumb_cmp_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4380 {
        parse_thumb_bic_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4040 {
        parse_thumb_eor_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4080 {
        parse_thumb_lsl_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x40c0 {
        parse_thumb_lsr_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x0 {
        parse_thumb_mov_2(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4340 {
        parse_thumb_mul_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x43c0 {
        parse_thumb_mvn_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4000 {
        parse_thumb_and_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4300 {
        parse_thumb_orr_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xba00 {
        parse_thumb_rev_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xba40 {
        parse_thumb_rev16_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xbac0 {
        parse_thumb_revsh_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x41c0 {
        parse_thumb_ror_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4240 {
        parse_thumb_rsb_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4180 {
        parse_thumb_sbc_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4100 {
        parse_thumb_asr_1(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4140 {
        parse_thumb_adc_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xb280 {
        parse_thumb_uxth_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xb240 {
        parse_thumb_sxtb_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0xb200 {
        parse_thumb_sxth_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4200 {
        parse_thumb_tst_0(ins as u32, pc)
    } else if (ins & 0xff80) == 0xb000 {
        parse_thumb_add_5(ins as u32, pc)
    } else if (ins & 0xff80) == 0xb080 {
        parse_thumb_sub_3(ins as u32, pc)
    } else if (ins & 0xff00) == 0xdf00 {
        parse_thumb_svc_0(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4400 {
        parse_thumb_add_3(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4600 {
        parse_thumb_mov_1(ins as u32, pc)
    } else if let Some(next) = next && (ins & 0xf800) == 0xf000
        && (next & 0xd000) == 0xd000
    {
        parse_thumb_bl_0(((ins as u32) << 16) | (next as u32), pc)
    } else if let Some(next) = next && (ins & 0xf800) == 0xf000
        && (next & 0xd000) == 0xc000
    {
        parse_thumb_blx_0(((ins as u32) << 16) | (next as u32), pc)
    } else if (ins & 0xff00) == 0xbe00 {
        parse_thumb_bkpt_0(ins as u32, pc)
    } else if (ins & 0xff00) == 0xde00 {
        parse_thumb_udf_0(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4500 {
        parse_thumb_cmp_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5400 {
        parse_thumb_strb_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5e00 {
        parse_thumb_ldrsh_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1800 {
        parse_thumb_add_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0xbc00 {
        parse_thumb_pop_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0xb400 {
        parse_thumb_push_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5800 {
        parse_thumb_ldr_3(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5c00 {
        parse_thumb_ldrb_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5000 {
        parse_thumb_str_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5200 {
        parse_thumb_strh_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1e00 {
        parse_thumb_sub_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1a00 {
        parse_thumb_sub_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5a00 {
        parse_thumb_ldrh_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x5600 {
        parse_thumb_ldrsb_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1c00 {
        parse_thumb_add_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xe000 {
        parse_thumb_b_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x7800 {
        parse_thumb_ldrb_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x1000 {
        parse_thumb_asr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xc000 {
        parse_thumb_stm_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x6000 {
        parse_thumb_str_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x9000 {
        parse_thumb_str_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa800 {
        parse_thumb_add_4(ins as u32, pc)
    } else if (ins & 0xf800) == 0x7000 {
        parse_thumb_strb_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x8800 {
        parse_thumb_ldrh_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x8000 {
        parse_thumb_strh_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa000 {
        parse_thumb_add_8(ins as u32, pc)
    } else if (ins & 0xf800) == 0xc800 {
        parse_thumb_ldm_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x3800 {
        parse_thumb_sub_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x6800 {
        parse_thumb_ldr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x9800 {
        parse_thumb_ldr_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x4800 {
        parse_thumb_ldr_2(ins as u32, pc)
    } else if (ins & 0xf800) == 0x2800 {
        parse_thumb_cmp_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x0 {
        parse_thumb_lsl_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x3000 {
        parse_thumb_add_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x800 {
        parse_thumb_lsr_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x2000 {
        parse_thumb_mov_0(ins as u32, pc)
    } else if (ins & 0xf000) == 0xd000 {
        parse_thumb_b_0(ins as u32, pc)
    } else {
        None
    }
}
fn parse_arm_adc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Adc { s, cond, rd, rn, op2 })
}
fn parse_thumb_adc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Adc { s, cond, rd, rn, op2 })
}
fn parse_arm_add_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2::Imm(((value) >> 6) & 0x7);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_2(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_3(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?;
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_4(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let rn = Reg::parse(13, pc)?;
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_5(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc)?;
    let rn = Reg::parse(13, pc)?;
    let op2 = Op2::Imm(((value) & 0x7f) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_6(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?;
    let rn = Reg::parse(13, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_7(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc)?;
    let rn = Reg::parse(13, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_thumb_add_8(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let rn = Reg::parse(15, pc)?;
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Some(Ins::Add { s, cond, rd, rn, op2 })
}
fn parse_arm_and_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::And { s, cond, rd, rn, op2 })
}
fn parse_thumb_and_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::And { s, cond, rd, rn, op2 })
}
fn parse_arm_asr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let op2 = Op2Shift::parse(value, pc)?;
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_thumb_asr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2Shift::Imm(
        if (((value) >> 6) & 0x1f) != 0 { (((value) >> 6) & 0x1f) } else { 32 },
    );
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_thumb_asr_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc)?);
    Some(Ins::Asr { s, cond, rd, rn, op2 })
}
fn parse_arm_b_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let target = BranchTarget::parse(
        ((((((value) & 0xffffff) << 2) as i32) << 6 >> 6) as u32).wrapping_add(8),
        pc,
    );
    Some(Ins::B { cond, target })
}
fn parse_thumb_b_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 8) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Bic { s, cond, rd, rn, op2 })
}
fn parse_thumb_bic_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let target = BlxTarget::Indirect(Reg::parse((value) & 0xf, pc)?);
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
    let target = BlxTarget::Indirect(Reg::parse(((value) >> 3) & 0xf, pc)?);
    Some(Ins::Blx { cond, target })
}
fn parse_arm_bx_0(value: u32, pc: u32) -> Option<Ins> {
    if (((value) >> 8) & 0xfff) != 0xfff {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Bx { cond, rm })
}
fn parse_thumb_bx_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rm = Reg::parse(((value) >> 3) & 0xf, pc)?;
    Some(Ins::Bx { cond, rm })
}
fn parse_arm_bxj_0(value: u32, pc: u32) -> Option<Ins> {
    if (((value) >> 8) & 0xfff) != 0xfff {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Bxj { cond, rm })
}
fn parse_arm_cdp_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Clz { cond, rd, rm })
}
fn parse_arm_cmn_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Cmn { cond, rn, op2 })
}
fn parse_thumb_cmn_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Cmn { cond, rn, op2 })
}
fn parse_arm_cmp_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_thumb_cmp_2(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Cmp { cond, rn, op2 })
}
fn parse_arm_cps_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xfe00 != 0 {
        return None;
    }
    let effect = CpsEffect::parse(((value) >> 18) & 0x3, pc)?;
    let aif = AifFlags::parse(((value) >> 6) & 0x7, pc);
    let mode = (value) & 0x1f;
    Some(Ins::Cps { effect, aif, mode })
}
fn parse_thumb_cps_0(value: u32, pc: u32) -> Option<Ins> {
    let effect = CpsEffect::parse(((value) >> 4) & 0x3, pc)?;
    let aif = AifFlags::parse((value) & 0x7, pc);
    let mode = 0;
    Some(Ins::Cps { effect, aif, mode })
}
fn parse_arm_csdb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Csdb { cond })
}
fn parse_arm_eor_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Eor { s, cond, rd, rn, op2 })
}
fn parse_thumb_eor_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Eor { s, cond, rd, rn, op2 })
}
fn parse_arm_ldc_0(value: u32, pc: u32) -> Option<Ins> {
    let l = (((value) >> 22) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let dest = AddrLdcStc::parse(value, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let dest = AddrLdcStc::parse(value, pc)?;
    Some(Ins::Ldc2 { l, coproc, crd, dest })
}
fn parse_arm_ldm_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xffff == 0 {
        return None;
    }
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
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
    if value & 0xffff == 0 {
        return None;
    }
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
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
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
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
    if value & 0xff == 0 {
        return None;
    }
    let mode = LdmStmMode::default();
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::parse(value, pc)?;
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(13, pc)?,
        offset: LdrStrOffset::Imm((((value) & 0xff) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_2(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(15, pc)?,
        offset: LdrStrOffset::Imm((((value) & 0xff) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_thumb_ldr_3(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Ldr { cond, rd, addr })
}
fn parse_arm_ldrb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::parse(value, pc)?;
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_thumb_ldrb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f)) as i32),
        writeback: false,
    };
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_thumb_ldrb_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Ldrb { cond, rd, addr })
}
fn parse_arm_ldrbt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::Post(AddrLdrStrPost::parse(value, pc)?);
    Some(Ins::Ldrbt { cond, rd, addr })
}
fn parse_arm_ldrd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x1000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrd { cond, rd, rd2, addr })
}
fn parse_arm_ldrd_1(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x01201000 != 0x01000000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrd { cond, rd, rd2, addr })
}
fn parse_arm_ldrex_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Ldrex { cond, rd, rn })
}
fn parse_arm_ldrexb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Ldrexb { cond, rd, rn })
}
fn parse_arm_ldrexd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Ldrexd { cond, rd, rd2, rn })
}
fn parse_arm_ldrexh_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f != 0xf0f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Ldrexh { cond, rd, rn })
}
fn parse_arm_ldrh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_thumb_ldrh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Imm(((((value) >> 6) & 0x1f) << 1) as i32),
        writeback: false,
    };
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_thumb_ldrh_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrh { cond, rd, addr })
}
fn parse_arm_ldrsb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrsb { cond, rd, addr })
}
fn parse_thumb_ldrsb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrsb { cond, rd, addr })
}
fn parse_arm_ldrsh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Ldrsh { cond, rd, addr })
}
fn parse_thumb_ldrsh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Ldrsh { cond, rd, addr })
}
fn parse_arm_ldrt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::Post(AddrLdrStrPost::parse(value, pc)?);
    Some(Ins::Ldrt { cond, rd, addr })
}
fn parse_arm_lsl_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let op2 = Op2Shift::parse(value, pc)?;
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsl_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2Shift::Imm(((value) >> 6) & 0x1f);
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsl_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc)?);
    Some(Ins::Lsl { s, cond, rd, rn, op2 })
}
fn parse_arm_lsr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let op2 = Op2Shift::parse(value, pc)?;
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2Shift::Imm(
        if (((value) >> 6) & 0x1f) != 0 { (((value) >> 6) & 0x1f) } else { 32 },
    );
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_thumb_lsr_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc)?);
    Some(Ins::Lsr { s, cond, rd, rn, op2 })
}
fn parse_arm_mcr_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc = ((value) >> 20) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc = ((value) >> 20) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_arm_mov_1(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (0) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_thumb_mov_2(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Mov { s, cond, rd, op2 })
}
fn parse_arm_mrc_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc1 = ((value) >> 21) & 0x7;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc = ((value) >> 4) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let opc = ((value) >> 4) & 0xf;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let crm = CoReg::parse((value) & 0xf, pc)?;
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
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let status_reg = StatusReg::parse(((value) >> 22) & 0x1, pc)?;
    Some(Ins::Mrs { cond, rd, status_reg })
}
fn parse_arm_msr_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let status_fields = StatusFields::parse(value, pc)?;
    let op2 = MsrOp2::parse(value, pc)?;
    Some(Ins::Msr {
        cond,
        status_fields,
        op2,
    })
}
fn parse_arm_mul_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Mul { s, cond, rd, rn, rm })
}
fn parse_thumb_mul_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let rm = Reg::parse((value) & 0x7, pc)?;
    Some(Ins::Mul { s, cond, rd, rn, rm })
}
fn parse_arm_mvn_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Mvn { s, cond, rd, op2 })
}
fn parse_thumb_mvn_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Mvn { s, cond, rd, op2 })
}
fn parse_arm_nop_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Nop { cond })
}
fn parse_thumb_nop_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    Some(Ins::Nop { cond })
}
fn parse_arm_orr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Orr { s, cond, rd, rn, op2 })
}
fn parse_thumb_orr_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Orr { s, cond, rd, rn, op2 })
}
fn parse_arm_pkhbt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let shift_op = ShiftOp::parse(0, pc)?;
    let shift = ((value) >> 7) & 0x1f;
    Some(Ins::Pkhbt {
        cond,
        rd,
        rn,
        rm,
        shift_op,
        shift,
    })
}
fn parse_arm_pkhtb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let shift_op = ShiftOp::parse(2, pc)?;
    let shift = if (((value) >> 7) & 0x1f) != 0 { (((value) >> 7) & 0x1f) } else { 32 };
    Some(Ins::Pkhtb {
        cond,
        rd,
        rn,
        rm,
        shift_op,
        shift,
    })
}
fn parse_arm_pld_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0xf000 {
        return None;
    }
    let addr = AddrLdrStr::parse(value, pc)?;
    Some(Ins::Pld { addr })
}
fn parse_arm_pop_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xffff == 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = RegList::parse((value) & 0xffff);
    Some(Ins::Pop { cond, regs })
}
fn parse_arm_pop_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = RegList::parse(1 << (((value) >> 12) & 0xf));
    Some(Ins::Pop { cond, regs })
}
fn parse_thumb_pop_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x1ff == 0 {
        return None;
    }
    let cond = Cond::parse(14, pc)?;
    let regs = RegList::parse(((((value) >> 8) & 0x1) << 15) | ((value) & 0xff));
    Some(Ins::Pop { cond, regs })
}
fn parse_arm_push_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xffff == 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = RegList::parse((value) & 0xffff);
    Some(Ins::Push { cond, regs })
}
fn parse_arm_push_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = RegList::parse(1 << (((value) >> 12) & 0xf));
    Some(Ins::Push { cond, regs })
}
fn parse_thumb_push_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x1ff == 0 {
        return None;
    }
    let cond = Cond::parse(14, pc)?;
    let regs = RegList::parse(((((value) >> 8) & 0x1) << 14) | ((value) & 0xff));
    Some(Ins::Push { cond, regs })
}
fn parse_arm_qadd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Qadd { cond, rd, rm, rn })
}
fn parse_arm_qadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qadd16 { cond, rd, rn, rm })
}
fn parse_arm_qadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qadd8 { cond, rd, rn, rm })
}
fn parse_arm_qasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qasx { cond, rd, rn, rm })
}
fn parse_arm_qdadd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Qdadd { cond, rd, rm, rn })
}
fn parse_arm_qdsub_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Qdsub { cond, rd, rm, rn })
}
fn parse_arm_qsax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qsax { cond, rd, rn, rm })
}
fn parse_arm_qsub_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Qsub { cond, rd, rm, rn })
}
fn parse_arm_qsub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qsub16 { cond, rd, rn, rm })
}
fn parse_arm_qsub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Qsub8 { cond, rd, rn, rm })
}
fn parse_arm_rev_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f00 != 0xf0f00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Rev { cond, rd, rm })
}
fn parse_thumb_rev_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(14, pc)?;
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    Some(Ins::Rev { cond, rd, rm })
}
fn parse_arm_rev16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f00 != 0xf0f00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Rev16 { cond, rd, rm })
}
fn parse_thumb_rev16_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(14, pc)?;
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    Some(Ins::Rev16 { cond, rd, rm })
}
fn parse_arm_revsh_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0f00 != 0xf0f00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Revsh { cond, rd, rm })
}
fn parse_thumb_revsh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(14, pc)?;
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    Some(Ins::Revsh { cond, rd, rm })
}
fn parse_arm_rfe_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xffff != 0x0a00 {
        return None;
    }
    let addr_mode = SrsRfeMode::parse(((value) >> 23) & 0x3, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    Some(Ins::Rfe {
        addr_mode,
        rn,
        writeback,
    })
}
fn parse_arm_ror_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let op2 = Op2Shift::parse(value, pc)?;
    Some(Ins::Ror { s, cond, rd, rn, op2 })
}
fn parse_thumb_ror_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2Shift::Reg(Reg::parse(((value) >> 3) & 0x7, pc)?);
    Some(Ins::Ror { s, cond, rd, rn, op2 })
}
fn parse_arm_rrx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf0000 != 0 {
        return None;
    }
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Rrx { s, cond, rd, rm })
}
fn parse_arm_rsb_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Rsb { s, cond, rd, rn, op2 })
}
fn parse_thumb_rsb_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2::Imm(0);
    Some(Ins::Rsb { s, cond, rd, rn, op2 })
}
fn parse_arm_rsc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Rsc { s, cond, rd, rn, op2 })
}
fn parse_arm_sadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Sadd16 { cond, rd, rn, rm })
}
fn parse_arm_sadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Sadd8 { cond, rd, rn, rm })
}
fn parse_arm_sasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Sasx { cond, rd, rn, rm })
}
fn parse_arm_sbc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Sbc { s, cond, rd, rn, op2 })
}
fn parse_thumb_sbc_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Sbc { s, cond, rd, rn, op2 })
}
fn parse_arm_sel_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Sel { cond, rd, rn, rm })
}
fn parse_arm_setend_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xefd0f != 0 {
        return None;
    }
    let endian = Endianness::parse(((value) >> 9) & 0x1, pc)?;
    Some(Ins::Setend { endian })
}
fn parse_thumb_setend_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x17 != 0x10 {
        return None;
    }
    let endian = Endianness::parse(((value) >> 3) & 0x1, pc)?;
    Some(Ins::Setend { endian })
}
fn parse_arm_sev_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Sev { cond })
}
fn parse_arm_shadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shadd16 { cond, rd, rn, rm })
}
fn parse_arm_shadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shadd8 { cond, rd, rn, rm })
}
fn parse_arm_shasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shasx { cond, rd, rn, rm })
}
fn parse_arm_shsax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shsax { cond, rd, rn, rm })
}
fn parse_arm_shsub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shsub16 { cond, rd, rn, rm })
}
fn parse_arm_shsub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Shsub8 { cond, rd, rn, rm })
}
fn parse_arm_smla_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rn_side = RegSide::parse(((value) >> 5) & 0x1, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let rm_side = RegSide::parse(((value) >> 6) & 0x1, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smla {
        cond,
        rd,
        rn,
        rn_side,
        rm,
        rm_side,
        ra,
    })
}
fn parse_arm_smlad_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smlad {
        cond,
        rd,
        rn,
        rm,
        swap_rm,
        ra,
    })
}
fn parse_arm_smlal_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Smlal {
        s,
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
    })
}
fn parse_arm_smlal_half_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rn_side = RegSide::parse(((value) >> 5) & 0x1, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let rm_side = RegSide::parse(((value) >> 6) & 0x1, pc)?;
    Some(Ins::SmlalHalf {
        cond,
        rd_lo,
        rd_hi,
        rn,
        rn_side,
        rm,
        rm_side,
    })
}
fn parse_arm_smlald_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    Some(Ins::Smlald {
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
        swap_rm,
    })
}
fn parse_arm_smlaw_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let rm_side = RegSide::parse(((value) >> 6) & 0x1, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smlaw {
        cond,
        rd,
        rn,
        rm,
        rm_side,
        ra,
    })
}
fn parse_arm_smlsd_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smlsd {
        cond,
        rd,
        rn,
        rm,
        swap_rm,
        ra,
    })
}
fn parse_arm_smlsld_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    Some(Ins::Smlsld {
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
        swap_rm,
    })
}
fn parse_arm_smmla_0(value: u32, pc: u32) -> Option<Ins> {
    let round = (((value) >> 5) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smmla {
        round,
        cond,
        rd,
        rn,
        rm,
        ra,
    })
}
fn parse_arm_smmls_0(value: u32, pc: u32) -> Option<Ins> {
    let round = (((value) >> 5) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Smmls {
        round,
        cond,
        rd,
        rn,
        rm,
        ra,
    })
}
fn parse_arm_smmul_0(value: u32, pc: u32) -> Option<Ins> {
    let round = (((value) >> 5) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Smmul {
        round,
        cond,
        rd,
        rn,
        rm,
    })
}
fn parse_arm_smuad_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    Some(Ins::Smuad {
        cond,
        rd,
        rn,
        rm,
        swap_rm,
    })
}
fn parse_arm_smul_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rn_side = RegSide::parse(((value) >> 5) & 0x1, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let rm_side = RegSide::parse(((value) >> 6) & 0x1, pc)?;
    Some(Ins::Smul {
        cond,
        rd,
        rn,
        rn_side,
        rm,
        rm_side,
    })
}
fn parse_arm_smull_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Smull {
        s,
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
    })
}
fn parse_arm_smulw_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let rm_side = RegSide::parse(((value) >> 6) & 0x1, pc)?;
    Some(Ins::Smulw {
        cond,
        rd,
        rn,
        rm,
        rm_side,
    })
}
fn parse_arm_smusd_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let swap_rm = (((value) >> 5) & 0x1) != 0;
    Some(Ins::Smusd {
        cond,
        rd,
        rn,
        rm,
        swap_rm,
    })
}
fn parse_arm_srs_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xfffe0 != 0xd0500 {
        return None;
    }
    let addr_mode = SrsRfeMode::parse(((value) >> 23) & 0x3, pc)?;
    let rn = Reg::parse(13, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let mode = (value) & 0x1f;
    Some(Ins::Srs {
        addr_mode,
        rn,
        writeback,
        mode,
    })
}
fn parse_arm_ssat_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let imm = (((value) >> 16) & 0x1f).wrapping_add(1);
    let op2 = ShiftImm::parse((value) & 0xfff, pc)?;
    Some(Ins::Ssat { cond, rd, imm, op2 })
}
fn parse_arm_ssat16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let imm = (((value) >> 16) & 0x1f).wrapping_add(1);
    let rn = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Ssat16 { cond, rd, imm, rn })
}
fn parse_arm_ssax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Ssax { cond, rd, rn, rm })
}
fn parse_arm_ssub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Ssub16 { cond, rd, rn, rm })
}
fn parse_arm_ssub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Ssub8 { cond, rd, rn, rm })
}
fn parse_arm_stc_0(value: u32, pc: u32) -> Option<Ins> {
    let l = (((value) >> 22) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let dest = AddrLdcStc::parse(value, pc)?;
    Some(Ins::Stc {
        l,
        cond,
        coproc,
        crd,
        dest,
    })
}
fn parse_arm_stc2_0(value: u32, pc: u32) -> Option<Ins> {
    let l = (((value) >> 22) & 0x1) != 0;
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc)?;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc)?;
    let dest = AddrLdcStc::parse(value, pc)?;
    Some(Ins::Stc2 { l, coproc, crd, dest })
}
fn parse_arm_stm_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xffff == 0 {
        return None;
    }
    let mode = LdmStmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = RegList::parse((value) & 0xffff);
    let user_mode = (((value) >> 22) & 0x1) != 0;
    Some(Ins::Stm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_thumb_stm_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff == 0 {
        return None;
    }
    let mode = LdmStmMode::default();
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let writeback = (1) != 0;
    let regs = RegList::parse((value) & 0xff);
    let user_mode = (0) != 0;
    Some(Ins::Stm {
        mode,
        cond,
        rn,
        writeback,
        regs,
        user_mode,
    })
}
fn parse_arm_str_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::parse(value, pc)?;
    Some(Ins::Str { cond, rd, addr })
}
fn parse_thumb_str_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Str { cond, rd, addr })
}
fn parse_thumb_str_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(13, pc)?,
        offset: LdrStrOffset::Imm((((value) & 0xff) << 2) as i32),
        writeback: false,
    };
    Some(Ins::Str { cond, rd, addr })
}
fn parse_thumb_str_2(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Str { cond, rd, addr })
}
fn parse_arm_strb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::parse(value, pc)?;
    Some(Ins::Strb { cond, rd, addr })
}
fn parse_thumb_strb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Imm(((((value) >> 6) & 0x1f)) as i32),
        writeback: false,
    };
    Some(Ins::Strb { cond, rd, addr })
}
fn parse_thumb_strb_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: LdrStrOffset::Reg {
            subtract: false,
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            shift_op: ShiftOp::default(),
            imm: 0,
        },
        writeback: false,
    };
    Some(Ins::Strb { cond, rd, addr })
}
fn parse_arm_strbt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::Post(AddrLdrStrPost::parse(value, pc)?);
    Some(Ins::Strbt { cond, rd, addr })
}
fn parse_arm_strd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x1000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((((value) >> 12) & 0xf).wrapping_add(1), pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Strd { cond, rd, rd2, addr })
}
fn parse_arm_strex_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Strex { cond, rd, rm, rn })
}
fn parse_arm_strexb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Strexb { cond, rd, rm, rn })
}
fn parse_arm_strexd_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf01 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rm2 = Reg::parse(((value) & 0xf).wrapping_add(1), pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Strexd {
        cond,
        rd,
        rm,
        rm2,
        rn,
    })
}
fn parse_arm_strexh_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf01 != 0xf01 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Strexh { cond, rd, rm, rn })
}
fn parse_arm_strh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrMiscLoad::parse(value, pc)?;
    Some(Ins::Strh { cond, rd, addr })
}
fn parse_thumb_strh_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Imm(((((value) >> 6) & 0x1f) << 1) as i32),
        writeback: false,
    };
    Some(Ins::Strh { cond, rd, addr })
}
fn parse_thumb_strh_1(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let addr = AddrMiscLoad::Pre {
        rn: Reg::parse(((value) >> 3) & 0x7, pc)?,
        offset: MiscLoadOffset::Reg {
            rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
            subtract: false,
        },
        writeback: false,
    };
    Some(Ins::Strh { cond, rd, addr })
}
fn parse_arm_strt_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let addr = AddrLdrStr::Post(AddrLdrStrPost::parse(value, pc)?);
    Some(Ins::Strt { cond, rd, addr })
}
fn parse_arm_sub_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Sub { s, cond, rd, rn, op2 })
}
fn parse_thumb_sub_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2::Imm(((value) >> 6) & 0x7);
    Some(Ins::Sub { s, cond, rd, rn, op2 })
}
fn parse_thumb_sub_1(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 8) & 0x7, pc)?;
    let op2 = Op2::Imm((value) & 0xff);
    Some(Ins::Sub { s, cond, rd, rn, op2 })
}
fn parse_thumb_sub_2(value: u32, pc: u32) -> Option<Ins> {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rn = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 6) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Sub { s, cond, rd, rn, op2 })
}
fn parse_thumb_sub_3(value: u32, pc: u32) -> Option<Ins> {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc)?;
    let rn = Reg::parse(13, pc)?;
    let op2 = Op2::Imm(((value) & 0x7f) << 2);
    Some(Ins::Sub { s, cond, rd, rn, op2 })
}
fn parse_arm_svc_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let imm = (value) & 0xffffff;
    Some(Ins::Svc { cond, imm })
}
fn parse_thumb_svc_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let imm = (value) & 0xff;
    Some(Ins::Svc { cond, imm })
}
fn parse_arm_swp_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Swp { cond, rd, rd2, rn })
}
fn parse_arm_swpb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd2 = Reg::parse((value) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::Swpb { cond, rd, rd2, rn })
}
fn parse_arm_sxtab_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxtab {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_sxtab16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxtab16 {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_sxtah_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxtah {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_sxtb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxtb { cond, rd, rm, rotate })
}
fn parse_thumb_sxtb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let rotate = 0;
    Some(Ins::Sxtb { cond, rd, rm, rotate })
}
fn parse_arm_sxtb16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxtb16 {
        cond,
        rd,
        rm,
        rotate,
    })
}
fn parse_arm_sxth_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Sxth { cond, rd, rm, rotate })
}
fn parse_thumb_sxth_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let rotate = 0;
    Some(Ins::Sxth { cond, rd, rm, rotate })
}
fn parse_arm_teq_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Teq { cond, rn, op2 })
}
fn parse_arm_tst_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf000 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let op2 = Op2::parse(value, pc)?;
    Some(Ins::Tst { cond, rn, op2 })
}
fn parse_thumb_tst_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc)?;
    let op2 = Op2::ShiftImm(ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc)?,
        shift_op: ShiftOp::default(),
        imm: 0,
    });
    Some(Ins::Tst { cond, rn, op2 })
}
fn parse_arm_uadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uadd16 { cond, rd, rn, rm })
}
fn parse_arm_uadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uadd8 { cond, rd, rn, rm })
}
fn parse_arm_uasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uasx { cond, rd, rn, rm })
}
fn parse_arm_udf_0(value: u32, pc: u32) -> Option<Ins> {
    let imm = ((((value) >> 8) & 0xfff) << 4) | ((value) & 0xf);
    Some(Ins::Udf { imm })
}
fn parse_thumb_udf_0(value: u32, pc: u32) -> Option<Ins> {
    let imm = (value) & 0xff;
    Some(Ins::Udf { imm })
}
fn parse_arm_uhadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhadd16 { cond, rd, rn, rm })
}
fn parse_arm_uhadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhadd8 { cond, rd, rn, rm })
}
fn parse_arm_uhasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhasx { cond, rd, rn, rm })
}
fn parse_arm_uhsax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhsax { cond, rd, rn, rm })
}
fn parse_arm_uhsub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhsub16 { cond, rd, rn, rm })
}
fn parse_arm_uhsub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uhsub8 { cond, rd, rn, rm })
}
fn parse_arm_umaal_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Umaal {
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
    })
}
fn parse_arm_umlal_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Umlal {
        s,
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
    })
}
fn parse_arm_umull_0(value: u32, pc: u32) -> Option<Ins> {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd_lo = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rd_hi = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Umull {
        s,
        cond,
        rd_lo,
        rd_hi,
        rn,
        rm,
    })
}
fn parse_arm_uqadd16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqadd16 { cond, rd, rn, rm })
}
fn parse_arm_uqadd8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqadd8 { cond, rd, rn, rm })
}
fn parse_arm_uqasx_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqasx { cond, rd, rn, rm })
}
fn parse_arm_uqsax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqsax { cond, rd, rn, rm })
}
fn parse_arm_uqsub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqsub16 { cond, rd, rn, rm })
}
fn parse_arm_uqsub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Uqsub8 { cond, rd, rn, rm })
}
fn parse_arm_usad8_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    Some(Ins::Usad8 { cond, rd, rn, rm })
}
fn parse_arm_usada8_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rn = Reg::parse((value) & 0xf, pc)?;
    let rm = Reg::parse(((value) >> 8) & 0xf, pc)?;
    let ra = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Usada8 {
        cond,
        rd,
        rn,
        rm,
        ra,
    })
}
fn parse_arm_usat_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let imm = (((value) >> 16) & 0x1f);
    let op2 = ShiftImm::parse((value) & 0xfff, pc)?;
    Some(Ins::Usat { cond, rd, imm, op2 })
}
fn parse_arm_usat16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let imm = (((value) >> 16) & 0x1f);
    let rn = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Usat16 { cond, rd, imm, rn })
}
fn parse_arm_usax_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Usax { cond, rd, rn, rm })
}
fn parse_arm_usub16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Usub16 { cond, rd, rn, rm })
}
fn parse_arm_usub8_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf00 != 0xf00 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    Some(Ins::Usub8 { cond, rd, rn, rm })
}
fn parse_arm_uxtab_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxtab {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_uxtab16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxtab16 {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_uxtah_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxtah {
        cond,
        rd,
        rn,
        rm,
        rotate,
    })
}
fn parse_arm_uxtb_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxtb { cond, rd, rm, rotate })
}
fn parse_thumb_uxtb_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let rotate = 0;
    Some(Ins::Uxtb { cond, rd, rm, rotate })
}
fn parse_arm_uxtb16_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxtb16 {
        cond,
        rd,
        rm,
        rotate,
    })
}
fn parse_arm_uxth_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x300 != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rm = Reg::parse((value) & 0xf, pc)?;
    let rotate = (((value) >> 10) & 0x3) << 3;
    Some(Ins::Uxth { cond, rd, rm, rotate })
}
fn parse_thumb_uxth_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc)?;
    let rm = Reg::parse(((value) >> 3) & 0x7, pc)?;
    let rotate = 0;
    Some(Ins::Uxth { cond, rd, rm, rotate })
}
fn parse_arm_vabs_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VabsF32 { cond, sd, sm })
}
fn parse_arm_vabs_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VabsF64 { cond, dd, dm })
}
fn parse_arm_vadd_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VaddF32 { cond, sd, sn, sm })
}
fn parse_arm_vadd_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VaddF64 { cond, dd, dn, dm })
}
fn parse_arm_vcmp_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let quiet_nan_exc = (((value) >> 7) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let op2 = VcmpF32Op2::parse((value) & 0x3f, pc)?;
    Some(Ins::VcmpF32 {
        quiet_nan_exc,
        cond,
        sd,
        op2,
    })
}
fn parse_arm_vcmp_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let quiet_nan_exc = (((value) >> 7) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let op2 = VcmpF64Op2::parse((value) & 0x3f, pc)?;
    Some(Ins::VcmpF64 {
        quiet_nan_exc,
        cond,
        dd,
        op2,
    })
}
fn parse_arm_vcvt_f32_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VcvtF32F64 { cond, sd, dm })
}
fn parse_arm_vcvt_f32_s32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtF32S32 { cond, sd, sm })
}
fn parse_arm_vcvt_f32_u32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtF32U32 { cond, sd, sm })
}
fn parse_arm_vcvt_f64_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtF64F32 { cond, dd, sm })
}
fn parse_arm_vcvt_f64_s32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtF64S32 { cond, dd, sm })
}
fn parse_arm_vcvt_f64_u32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtF64U32 { cond, dd, sm })
}
fn parse_arm_vcvt_s32_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let round_zero = ((((value) >> 7) & 0x1) ^ 1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtS32F32 {
        round_zero,
        cond,
        sd,
        sm,
    })
}
fn parse_arm_vcvt_s32_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let round_zero = ((((value) >> 7) & 0x1) ^ 1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VcvtS32F64 {
        round_zero,
        cond,
        sd,
        dm,
    })
}
fn parse_arm_vcvt_u32_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let round_zero = ((((value) >> 7) & 0x1) ^ 1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VcvtU32F32 {
        round_zero,
        cond,
        sd,
        sm,
    })
}
fn parse_arm_vcvt_u32_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let round_zero = ((((value) >> 7) & 0x1) ^ 1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VcvtU32F64 {
        round_zero,
        cond,
        sd,
        dm,
    })
}
fn parse_arm_vdiv_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VdivF32 { cond, sd, sn, sm })
}
fn parse_arm_vdiv_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VdivF64 { cond, dd, dn, dm })
}
fn parse_arm_vldm_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = VldmVstmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = SregList::parse(value);
    Some(Ins::VldmF32 {
        mode,
        cond,
        rn,
        writeback,
        regs,
    })
}
fn parse_arm_vldm_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = VldmVstmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = DregList::parse(value);
    Some(Ins::VldmF64 {
        mode,
        cond,
        rn,
        writeback,
        regs,
    })
}
fn parse_arm_vldr_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 16) & 0xf, pc)?,
        offset: LdrStrOffset::Imm(
            ((if (((value) >> 23) & 0x1) == 0 {
                -((((value) & 0xff) << 2) as i32)
            } else {
                (((value) & 0xff) << 2) as i32
            })) as i32,
        ),
        writeback: false,
    };
    Some(Ins::VldrF32 { cond, sd, addr })
}
fn parse_arm_vldr_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 16) & 0xf, pc)?,
        offset: LdrStrOffset::Imm(
            ((if (((value) >> 23) & 0x1) == 0 {
                -((((value) & 0xff) << 2) as i32)
            } else {
                (((value) & 0xff) << 2) as i32
            })) as i32,
        ),
        writeback: false,
    };
    Some(Ins::VldrF64 { cond, dd, addr })
}
fn parse_arm_vmla_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VmlaF32 { cond, sd, sn, sm })
}
fn parse_arm_vmla_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VmlaF64 { cond, dd, dn, dm })
}
fn parse_arm_vmls_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VmlsF32 { cond, sd, sn, sm })
}
fn parse_arm_vmls_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VmlsF64 { cond, dd, dn, dm })
}
fn parse_arm_vmov_32_reg_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = DregIndex::parse(value, pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Vmov32Reg { cond, dd, rt })
}
fn parse_arm_vmov_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VmovF32 { cond, sd, sm })
}
fn parse_arm_vmov_f32_reg_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x6f != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::VmovF32Reg { cond, sn, rt })
}
fn parse_arm_vmov_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VmovF64 { cond, dd, dm })
}
fn parse_arm_vmov_reg_32_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xf != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let dn = DregIndex::parse(value, pc)?;
    Some(Ins::VmovReg32 { cond, rt, dn })
}
fn parse_arm_vmov_reg_f32_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x6f != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    Some(Ins::VmovRegF32 { cond, rt, sn })
}
fn parse_arm_vmov_reg_f32_dual_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x2f == 0x2f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rt2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    let sm2 = Sreg::parse(
        ((((value) & 0xf) << 1) | (((value) >> 5) & 0x1)).wrapping_add(1),
        pc,
    )?;
    Some(Ins::VmovRegF32Dual {
        cond,
        rt,
        rt2,
        sm,
        sm2,
    })
}
fn parse_arm_vmov_f32_reg_dual_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0x2f == 0x2f {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    let sm2 = Sreg::parse(
        ((((value) & 0xf) << 1) | (((value) >> 5) & 0x1)).wrapping_add(1),
        pc,
    )?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rt2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::VmovF32RegDual {
        cond,
        sm,
        sm2,
        rt,
        rt2,
    })
}
fn parse_arm_vmov_reg_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rt2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VmovRegF64 {
        cond,
        rt,
        rt2,
        dm,
    })
}
fn parse_arm_vmov_f64_reg_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    let rt = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let rt2 = Reg::parse(((value) >> 16) & 0xf, pc)?;
    Some(Ins::VmovF64Reg {
        cond,
        dm,
        rt,
        rt2,
    })
}
fn parse_arm_vmrs_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xef != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    let fpscr = Fpscr::default();
    Some(Ins::Vmrs { cond, rd, fpscr })
}
fn parse_arm_vmsr_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xef != 0 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let fpscr = Fpscr::default();
    let rd = Reg::parse(((value) >> 12) & 0xf, pc)?;
    Some(Ins::Vmsr { cond, fpscr, rd })
}
fn parse_arm_vmul_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VmulF32 { cond, sd, sn, sm })
}
fn parse_arm_vmul_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VmulF64 { cond, dd, dn, dm })
}
fn parse_arm_vneg_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VnegF32 { cond, sd, sm })
}
fn parse_arm_vneg_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VnegF64 { cond, dd, dm })
}
fn parse_arm_vnmla_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VnmlaF32 { cond, sd, sn, sm })
}
fn parse_arm_vnmla_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VnmlaF64 { cond, dd, dn, dm })
}
fn parse_arm_vnmls_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VnmlsF32 { cond, sd, sn, sm })
}
fn parse_arm_vnmls_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VnmlsF64 { cond, dd, dn, dm })
}
fn parse_arm_vnmul_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VnmulF32 { cond, sd, sn, sm })
}
fn parse_arm_vnmul_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VnmulF64 { cond, dd, dn, dm })
}
fn parse_arm_vpop_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = SregList::parse(value);
    Some(Ins::VpopF32 { cond, regs })
}
fn parse_arm_vpop_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = DregList::parse(value);
    Some(Ins::VpopF64 { cond, regs })
}
fn parse_arm_vpush_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = SregList::parse(value);
    Some(Ins::VpushF32 { cond, regs })
}
fn parse_arm_vpush_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let regs = DregList::parse(value);
    Some(Ins::VpushF64 { cond, regs })
}
fn parse_arm_vsqrt_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VsqrtF32 { cond, sd, sm })
}
fn parse_arm_vsqrt_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VsqrtF64 { cond, dd, dm })
}
fn parse_arm_vstm_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = VldmVstmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = SregList::parse(value);
    Some(Ins::VstmF32 {
        mode,
        cond,
        rn,
        writeback,
        regs,
    })
}
fn parse_arm_vstm_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let mode = VldmVstmMode::parse(((value) >> 23) & 0x3, pc)?;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let rn = Reg::parse(((value) >> 16) & 0xf, pc)?;
    let writeback = (((value) >> 21) & 0x1) != 0;
    let regs = DregList::parse(value);
    Some(Ins::VstmF64 {
        mode,
        cond,
        rn,
        writeback,
        regs,
    })
}
fn parse_arm_vstr_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 16) & 0xf, pc)?,
        offset: LdrStrOffset::Imm(
            ((if (((value) >> 23) & 0x1) == 0 {
                -((((value) & 0xff) << 2) as i32)
            } else {
                (((value) & 0xff) << 2) as i32
            })) as i32,
        ),
        writeback: false,
    };
    Some(Ins::VstrF32 { cond, sd, addr })
}
fn parse_arm_vstr_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let addr = AddrLdrStr::Pre {
        rn: Reg::parse(((value) >> 16) & 0xf, pc)?,
        offset: LdrStrOffset::Imm(
            ((if (((value) >> 23) & 0x1) == 0 {
                -((((value) & 0xff) << 2) as i32)
            } else {
                (((value) & 0xff) << 2) as i32
            })) as i32,
        ),
        writeback: false,
    };
    Some(Ins::VstrF64 { cond, dd, addr })
}
fn parse_arm_vsub_f32_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let sd = Sreg::parse(((((value) >> 12) & 0xf) << 1) | (((value) >> 22) & 0x1), pc)?;
    let sn = Sreg::parse(((((value) >> 16) & 0xf) << 1) | (((value) >> 7) & 0x1), pc)?;
    let sm = Sreg::parse((((value) & 0xf) << 1) | (((value) >> 5) & 0x1), pc)?;
    Some(Ins::VsubF32 { cond, sd, sn, sm })
}
fn parse_arm_vsub_f64_0(value: u32, pc: u32) -> Option<Ins> {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    let dd = Dreg::parse(((((value) >> 22) & 0x1) << 4) | (((value) >> 12) & 0xf), pc)?;
    let dn = Dreg::parse(((((value) >> 7) & 0x1) << 4) | (((value) >> 16) & 0xf), pc)?;
    let dm = Dreg::parse(((((value) >> 5) & 0x1) << 4) | ((value) & 0xf), pc)?;
    Some(Ins::VsubF64 { cond, dd, dn, dm })
}
fn parse_arm_wfe_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Wfe { cond })
}
fn parse_arm_wfi_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Wfi { cond })
}
fn parse_arm_yield_0(value: u32, pc: u32) -> Option<Ins> {
    if value & 0xff00 != 0xf000 {
        return None;
    }
    let cond = Cond::parse(((value) >> 28) & 0xf, pc)?;
    Some(Ins::Yield { cond })
}

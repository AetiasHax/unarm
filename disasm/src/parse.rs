use crate::*;
impl BranchTarget {
    fn parse(value: u32, pc: u32) -> Self {
        Self {
            addr: pc.wrapping_add((value)),
        }
    }
}
impl BlxTarget {
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
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
impl ShiftOp {
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
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
impl CpsEffect {
    fn parse(value: u32, pc: u32) -> Self {
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
    fn parse(value: u32, pc: u32) -> Self {
        Self {
            a: (((value) >> 2) & 0x1) != 0,
            i: (((value) >> 1) & 0x1) != 0,
            f: ((value) & 0x1) != 0,
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
pub fn parse_arm(ins: u32, pc: u32) -> Ins {
    if (ins & 0xffffffff) == 0xf57ff01f {
        parse_arm_clrex_0(ins as u32, pc)
    } else if (ins & 0xffffff0) == 0x12fff30 {
        parse_arm_blx_1(ins as u32, pc)
    } else if (ins & 0xffffff0) == 0x12fff10 {
        parse_arm_bx_0(ins as u32, pc)
    } else if (ins & 0xffffff0) == 0x12fff20 {
        parse_arm_bxj_0(ins as u32, pc)
    } else if (ins & 0xfff1fe20) == 0xf1000000 {
        parse_arm_cps_0(ins as u32, pc)
    } else if (ins & 0xfff0ff0) == 0x16f0f10 {
        parse_arm_clz_0(ins as u32, pc)
    } else if (ins & 0xfff000f0) == 0xe1200070 {
        parse_arm_bkpt_0(ins as u32, pc)
    } else if (ins & 0xdf0f000) == 0x1700000 {
        parse_arm_cmn_0(ins as u32, pc)
    } else if (ins & 0xdf0f000) == 0x1500000 {
        parse_arm_cmp_0(ins as u32, pc)
    } else if (ins & 0xff000010) == 0xfe000000 {
        parse_arm_cdp2_0(ins as u32, pc)
    } else if (ins & 0xfe000000) == 0xfa000000 {
        parse_arm_blx_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0xa00000 {
        parse_arm_adc_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x800000 {
        parse_arm_add_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x0 {
        parse_arm_and_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x1c00000 {
        parse_arm_bic_0(ins as u32, pc)
    } else if (ins & 0xf000010) == 0xe000000 {
        parse_arm_cdp_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xa000000 {
        parse_arm_b_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xb000000 {
        parse_arm_bl_0(ins as u32, pc)
    } else {
        Ins::Illegal
    }
}
pub fn parse_thumb(ins: u16, next: Option<u16>, pc: u32) -> Ins {
    if (ins & 0xff78) == 0x4468 {
        parse_thumb_add_6(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4485 {
        parse_thumb_add_7(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4780 {
        parse_thumb_blx_1(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4700 {
        parse_thumb_bx_0(ins as u32, pc)
    } else if (ins & 0xffe8) == 0xb660 {
        parse_thumb_cps_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4140 {
        parse_thumb_adc_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4000 {
        parse_thumb_and_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4380 {
        parse_thumb_bic_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x42c0 {
        parse_thumb_cmn_0(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4280 {
        parse_thumb_cmp_1(ins as u32, pc)
    } else if (ins & 0xff80) == 0xb000 {
        parse_thumb_add_5(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4400 {
        parse_thumb_add_3(ins as u32, pc)
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
    } else if (ins & 0xff00) == 0x4500 {
        parse_thumb_cmp_2(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1c00 {
        parse_thumb_add_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1800 {
        parse_thumb_add_2(ins as u32, pc)
    } else if (ins & 0xf800) == 0x3000 {
        parse_thumb_add_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa800 {
        parse_thumb_add_4(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa000 {
        parse_thumb_add_8(ins as u32, pc)
    } else if (ins & 0xf800) == 0xe000 {
        parse_thumb_b_1(ins as u32, pc)
    } else if (ins & 0xf800) == 0x2800 {
        parse_thumb_cmp_0(ins as u32, pc)
    } else if (ins & 0xf000) == 0xd000 {
        parse_thumb_b_0(ins as u32, pc)
    } else {
        Ins::Illegal
    }
}
fn parse_arm_adc_0(value: u32, pc: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::Adc { s, cond, rd, rn, op2 }
}
fn parse_thumb_adc_0(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Adc { s, cond, rd, rn, op2 }
}
fn parse_arm_add_0(value: u32, pc: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_0(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2::Imm(((value) >> 6) & 0x7);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_1(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(((value) >> 8) & 0x7, pc);
    let op2 = Op2::Imm((value) & 0xff);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_2(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse(((value) >> 3) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 6) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_3(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_4(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_5(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::Imm(((value) & 0x7f) << 2);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_6(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_7(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(13, pc);
    let rn = Reg::parse(13, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_8(value: u32, pc: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::parse(((value) >> 8) & 0x7, pc);
    let rn = Reg::parse(15, pc);
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_arm_and_0(value: u32, pc: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::And { s, cond, rd, rn, op2 }
}
fn parse_thumb_and_0(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::And { s, cond, rd, rn, op2 }
}
fn parse_arm_b_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xffffff) << 2) as i32) << 6 >> 6) as u32).wrapping_add(8),
        pc,
    );
    Ins::B { cond, target }
}
fn parse_thumb_b_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 8) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xff) << 1) as i32) << 23 >> 23) as u32).wrapping_add(4),
        pc,
    );
    Ins::B { cond, target }
}
fn parse_thumb_b_1(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let target = BranchTarget::parse(
        ((((((value) & 0x7ff) << 1) as i32) << 20 >> 20) as u32).wrapping_add(4),
        pc,
    );
    Ins::B { cond, target }
}
fn parse_arm_bic_0(value: u32, pc: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::Bic { s, cond, rd, rn, op2 }
}
fn parse_thumb_bic_0(value: u32, pc: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::parse((value) & 0x7, pc);
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Bic { s, cond, rd, rn, op2 }
}
fn parse_arm_bkpt_0(value: u32, pc: u32) -> Ins {
    let imm = ((((value) >> 8) & 0xfff) << 4) | ((value) & 0xf);
    Ins::Bkpt { imm }
}
fn parse_thumb_bkpt_0(value: u32, pc: u32) -> Ins {
    let imm = (value) & 0xff;
    Ins::Bkpt { imm }
}
fn parse_arm_bl_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BranchTarget::parse(
        ((((((value) & 0xffffff) << 2) as i32) << 6 >> 6) as u32).wrapping_add(8),
        pc,
    );
    Ins::Bl { cond, target }
}
fn parse_thumb_bl_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let target = BranchTarget::parse(
        ((((((((value) >> 16) & 0x7ff) << 12) | (((value) & 0x7ff) << 1)) as i32) << 9
            >> 9) as u32)
            .wrapping_add(4),
        pc,
    );
    Ins::Bl { cond, target }
}
fn parse_arm_blx_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let target = BlxTarget::Direct(
        BranchTarget::parse(
            (((((((value) & 0xffffff) << 2) | ((((value) >> 24) & 0x1) << 1)) as i32)
                << 6 >> 6) as u32)
                .wrapping_add(8),
            pc,
        ),
    );
    Ins::Blx { cond, target }
}
fn parse_arm_blx_1(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let target = BlxTarget::Indirect(Reg::parse((value) & 0xf, pc));
    Ins::Blx { cond, target }
}
fn parse_thumb_blx_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let target = BlxTarget::Direct(
        BranchTarget::parse(
            ((((((((value) >> 16) & 0x7ff) << 12) | ((((value) >> 1) & 0x3ff) << 2))
                as i32) << 9 >> 9) as u32)
                .wrapping_add(4),
            pc,
        ),
    );
    Ins::Blx { cond, target }
}
fn parse_thumb_blx_1(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let target = BlxTarget::Indirect(Reg::parse(((value) >> 3) & 0xf, pc));
    Ins::Blx { cond, target }
}
fn parse_arm_bx_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Ins::Bx { cond, rm }
}
fn parse_thumb_bx_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let rm = Reg::parse(((value) >> 3) & 0xf, pc);
    Ins::Bx { cond, rm }
}
fn parse_arm_bxj_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Ins::Bxj { cond, rm }
}
fn parse_arm_cdp_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Ins::Cdp {
        cond,
        coproc,
        opc1,
        crd,
        crn,
        crm,
        opc2,
    }
}
fn parse_arm_cdp2_0(value: u32, pc: u32) -> Ins {
    let coproc = Coproc::parse(((value) >> 8) & 0xf, pc);
    let opc1 = ((value) >> 20) & 0xf;
    let crd = CoReg::parse(((value) >> 12) & 0xf, pc);
    let crn = CoReg::parse(((value) >> 16) & 0xf, pc);
    let crm = CoReg::parse((value) & 0xf, pc);
    let opc2 = ((value) >> 5) & 0x7;
    Ins::Cdp2 {
        coproc,
        opc1,
        crd,
        crn,
        crm,
        opc2,
    }
}
fn parse_arm_clrex_0(value: u32, pc: u32) -> Ins {
    Ins::Clrex {}
}
fn parse_arm_clz_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rd = Reg::parse(((value) >> 12) & 0xf, pc);
    let rm = Reg::parse((value) & 0xf, pc);
    Ins::Clz { cond, rd, rm }
}
fn parse_arm_cmn_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::Cmn { cond, rn, op2 }
}
fn parse_thumb_cmn_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Cmn { cond, rn, op2 }
}
fn parse_arm_cmp_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::parse(((value) >> 28) & 0xf, pc);
    let rn = Reg::parse(((value) >> 16) & 0xf, pc);
    let op2 = Op2::parse(value, pc);
    Ins::Cmp { cond, rn, op2 }
}
fn parse_thumb_cmp_0(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let rn = Reg::parse(((value) >> 8) & 0x7, pc);
    let op2 = Op2::Imm((value) & 0xff);
    Ins::Cmp { cond, rn, op2 }
}
fn parse_thumb_cmp_1(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let rn = Reg::parse((value) & 0x7, pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0x7, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Cmp { cond, rn, op2 }
}
fn parse_thumb_cmp_2(value: u32, pc: u32) -> Ins {
    let cond = Cond::default();
    let rn = Reg::parse(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7), pc);
    let op2 = Op2::ShiftImm {
        rm: Reg::parse(((value) >> 3) & 0xf, pc),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Cmp { cond, rn, op2 }
}
fn parse_arm_cps_0(value: u32, pc: u32) -> Ins {
    let effect = CpsEffect::parse(((value) >> 18) & 0x3, pc);
    let aif = AifFlags::parse(((value) >> 6) & 0x7, pc);
    let mode = (value) & 0x1f;
    Ins::Cps { effect, aif, mode }
}
fn parse_thumb_cps_0(value: u32, pc: u32) -> Ins {
    let effect = CpsEffect::parse(((value) >> 4) & 0x3, pc);
    let aif = AifFlags::parse((value) & 0x7, pc);
    let mode = 0;
    Ins::Cps { effect, aif, mode }
}

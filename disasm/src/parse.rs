use crate::*;
impl BranchTarget {
    fn parse(value: u32, pc: u32) -> Self {
        Self {
            addr: pc.wrapping_add((value)),
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
pub fn parse_arm(ins: u32, pc: u32) -> Ins {
    if (ins & 0xde00000) == 0xa00000 {
        parse_arm_adc_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x800000 {
        parse_arm_add_0(ins as u32, pc)
    } else if (ins & 0xde00000) == 0x0 {
        parse_arm_and_0(ins as u32, pc)
    } else if (ins & 0xf000000) == 0xa000000 {
        parse_arm_b_0(ins as u32, pc)
    } else {
        Ins::Illegal
    }
}
pub fn parse_thumb(ins: u16, next: Option<u16>, pc: u32) -> Ins {
    if (ins & 0xffc0) == 0x4140 {
        parse_thumb_adc_0(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1c00 {
        parse_thumb_add_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0x3000 {
        parse_thumb_add_1(ins as u32, pc)
    } else if (ins & 0xfe00) == 0x1800 {
        parse_thumb_add_2(ins as u32, pc)
    } else if (ins & 0xff00) == 0x4400 {
        parse_thumb_add_3(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa800 {
        parse_thumb_add_4(ins as u32, pc)
    } else if (ins & 0xff80) == 0xb000 {
        parse_thumb_add_5(ins as u32, pc)
    } else if (ins & 0xff78) == 0x4468 {
        parse_thumb_add_6(ins as u32, pc)
    } else if (ins & 0xff87) == 0x4485 {
        parse_thumb_add_7(ins as u32, pc)
    } else if (ins & 0xf800) == 0xa000 {
        parse_thumb_add_8(ins as u32, pc)
    } else if (ins & 0xffc0) == 0x4000 {
        parse_thumb_and_0(ins as u32, pc)
    } else if (ins & 0xf000) == 0xd000 {
        parse_thumb_b_0(ins as u32, pc)
    } else if (ins & 0xf800) == 0xe000 {
        parse_thumb_b_1(ins as u32, pc)
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

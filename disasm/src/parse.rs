pub struct Options {
    ///The version of ARM to use
    pub version: Version,
    ///If true, r0-r3 and r4-11 will display as a1-a4 and v1-v8 respectively
    pub av: bool,
    ///How R9 should be displayed
    pub r9_use: R9Use,
    ///If true, R10 will display as SL (stack limit)
    pub sl: bool,
    ///If true, R11 will display as FP (frame pointer)
    pub fp: bool,
    ///If true, R12 will display as IP (intra-procedure call scratch register)
    pub ip: bool,
    ///If true, use Unified Assembly Language syntax (UAL), otherwise use divided syntax
    pub ual: bool,
}
pub enum Version {
    V4,
    V4T,
    V5,
    V5T,
    V5Te,
    V6K,
}
pub enum R9Use {
    ///General purpose register
    R9,
    ///Static base (SB), used for position-independent data
    Sb,
    ///TLS register (TR), used for thread-local storage
    Tr,
}
pub enum Cond {
    ///Equal
    Eq,
    ///Not equal
    Ne,
    ///Unsigned higher or same
    Hs,
    ///Unsigned lower
    Lo,
    ///Minus/negative
    Mi,
    ///Plus/positive
    Pl,
    ///Overflow set
    Vs,
    ///Overflow clear
    Vc,
    ///Unsigned higher
    Hi,
    ///Unsigned lower or same
    Ls,
    ///Signed greater than or equal
    Ge,
    ///Signed less than
    Lt,
    ///Signed greater than
    Gt,
    ///Signed less than or equal
    Le,
    ///Always
    Al,
}
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    Sp,
    Lr,
    Pc,
}
pub enum ShiftOp {
    ///Logical shift left
    Lsl,
    ///Logical shift right
    Lsr,
    ///Arithmetic shift right
    Asr,
    ///Rotate right
    Ror,
}
pub enum Op2 {
    ///Immediate
    Imm(u32),
    ///Register shifted by register
    ShiftReg { rm: Reg, shift_op: ShiftOp, rs: Reg },
    ///Register shifted by immediate
    ShiftImm { rm: Reg, shift_op: ShiftOp, imm: u32 },
}
impl From<u32> for Cond {
    fn from(value: u32) -> Self {
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
impl From<u32> for Reg {
    fn from(value: u32) -> Self {
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
impl From<u32> for ShiftOp {
    fn from(value: u32) -> Self {
        match value {
            0x0 => Self::Lsl,
            0x1 => Self::Lsr,
            0x2 => Self::Asr,
            0x3 => Self::Ror,
            _ => panic!(),
        }
    }
}
impl From<u32> for Op2 {
    fn from(value: u32) -> Self {
        if (value & 0x2000000) == 0x2000000 {
            Self::Imm(value & 0xfff)
        } else if (value & 0x2000010) == 0x10 {
            Self::ShiftReg {
                rm: Reg::from(value & 0xf),
                shift_op: ShiftOp::from((value >> 5) & 0x3),
                rs: Reg::from((value >> 8) & 0xf),
            }
        } else if (value & 0x2000010) == 0x0 {
            Self::ShiftImm {
                rm: Reg::from(value & 0xf),
                shift_op: ShiftOp::from((value >> 5) & 0x3),
                imm: (value >> 7) & 0x1f,
            }
        } else {
            panic!();
        }
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
pub enum Ins {
    Adc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
}
fn parse_arm_adc_0(value: u32) -> Ins {
    let s = ((value >> 20) & 0x1) != 0;
    let cond = Cond::from((value >> 28) & 0xf);
    let rd = Reg::from((value >> 12) & 0xf);
    let rn = Reg::from((value >> 16) & 0xf);
    let op2 = Op2::from(value);
    Ins::Adc { s, cond, rd, rn, op2 }
}
fn parse_thumb_adc_0(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(value & 0x7);
    let rn = Reg::from(value & 0x7);
    let op2 = Op2::ShiftImm {
        rm: Reg::from((value >> 3) & 0x7),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Adc { s, cond, rd, rn, op2 }
}

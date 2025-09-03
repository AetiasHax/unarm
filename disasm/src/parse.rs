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
#[derive(PartialEq, Eq)]
pub enum R9Use {
    ///General purpose register
    R9,
    ///Static base (SB), used for position-independent data
    Sb,
    ///TLS register (TR), used for thread-local storage
    Tr,
}
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
            Self::Imm(((value) & 0xff).rotate_right((((value) >> 8) & 0xf) << 1))
        } else if (value & 0x2000010) == 0x10 {
            Self::ShiftReg {
                rm: Reg::from((value) & 0xf),
                shift_op: ShiftOp::from(((value) >> 5) & 0x3),
                rs: Reg::from(((value) >> 8) & 0xf),
            }
        } else if (value & 0x2000010) == 0x0 {
            Self::ShiftImm {
                rm: Reg::from((value) & 0xf),
                shift_op: ShiftOp::from(((value) >> 5) & 0x3),
                imm: (((value) >> 7) & 0x1f),
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
impl Cond {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
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
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
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
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
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
impl Op2 {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Imm(data) => {
                f.write_str("#")?;
                write!(f, "{:#x}", data)?;
            }
            Self::ShiftReg { rm, shift_op, rs } => {
                rm.fmt(options, f)?;
                f.write_str(", ")?;
                shift_op.fmt(options, f)?;
                f.write_str(" ")?;
                rs.fmt(options, f)?;
            }
            Self::ShiftImm { rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    rm.fmt(options, f)?;
                } else {
                    if *imm == 0 && *shift_op == ShiftOp::Ror {
                        rm.fmt(options, f)?;
                        f.write_str(", rrx")?;
                    } else {
                        rm.fmt(options, f)?;
                        f.write_str(", ")?;
                        shift_op.fmt(options, f)?;
                        f.write_str(" #")?;
                        write!(f, "{:#x}", imm)?;
                    }
                }
            }
        }
        Ok(())
    }
}
pub enum Ins {
    Adc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    Add { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    Illegal,
}
impl Ins {
    pub fn fmt(
        &self,
        options: &Options,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("adc")?;
                    if *s {
                        f.write_str("s")?;
                    }
                    cond.fmt(options, f)?;
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                } else {
                    f.write_str("adc")?;
                    cond.fmt(options, f)?;
                    if *s {
                        f.write_str("s")?;
                    }
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                }
            }
            Ins::Add { s, cond, rd, rn, op2 } => {
                if options.ual {
                    f.write_str("add")?;
                    if *s {
                        f.write_str("s")?;
                    }
                    cond.fmt(options, f)?;
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                } else {
                    f.write_str("add")?;
                    cond.fmt(options, f)?;
                    if *s {
                        f.write_str("s")?;
                    }
                    f.write_str(" ")?;
                    rd.fmt(options, f)?;
                    f.write_str(", ")?;
                    rn.fmt(options, f)?;
                    f.write_str(", ")?;
                    op2.fmt(options, f)?;
                }
            }
            Ins::Illegal => {
                f.write_str("<illegal>")?;
            }
        }
        Ok(())
    }
}
impl Ins {
    pub fn display<'a>(&'a self, options: &'a Options) -> DisplayIns<'a> {
        DisplayIns { ins: self, options }
    }
}
pub struct DisplayIns<'a> {
    ins: &'a Ins,
    options: &'a Options,
}
impl<'a> core::fmt::Display for DisplayIns<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.ins.fmt(self.options, f)
    }
}
pub fn parse_arm(ins: u32) -> Ins {
    if (ins & 0xde00000) == 0xa00000 {
        parse_arm_adc_0(ins as u32)
    } else if (ins & 0xde00000) == 0x800000 {
        parse_arm_add_0(ins as u32)
    } else {
        Ins::Illegal
    }
}
pub fn parse_thumb(ins: u16, next: Option<u16>) -> Ins {
    if (ins & 0xffc0) == 0x4140 {
        parse_thumb_adc_0(ins as u32)
    } else if (ins & 0xfe00) == 0x1c00 {
        parse_thumb_add_0(ins as u32)
    } else if (ins & 0xf800) == 0x3000 {
        parse_thumb_add_1(ins as u32)
    } else if (ins & 0xfe00) == 0x1800 {
        parse_thumb_add_2(ins as u32)
    } else if (ins & 0xff00) == 0x4400 {
        parse_thumb_add_3(ins as u32)
    } else if (ins & 0xf800) == 0xa800 {
        parse_thumb_add_4(ins as u32)
    } else if (ins & 0xff80) == 0xb000 {
        parse_thumb_add_5(ins as u32)
    } else if (ins & 0xff78) == 0x4468 {
        parse_thumb_add_6(ins as u32)
    } else if (ins & 0xff87) == 0x4485 {
        parse_thumb_add_7(ins as u32)
    } else if (ins & 0xf800) == 0xa000 {
        parse_thumb_add_8(ins as u32)
    } else {
        Ins::Illegal
    }
}
fn parse_arm_adc_0(value: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::from(((value) >> 28) & 0xf);
    let rd = Reg::from(((value) >> 12) & 0xf);
    let rn = Reg::from(((value) >> 16) & 0xf);
    let op2 = Op2::from(value);
    Ins::Adc { s, cond, rd, rn, op2 }
}
fn parse_thumb_adc_0(value: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::from((value) & 0x7);
    let rn = Reg::from((value) & 0x7);
    let op2 = Op2::ShiftImm {
        rm: Reg::from(((value) >> 3) & 0x7),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Adc { s, cond, rd, rn, op2 }
}
fn parse_arm_add_0(value: u32) -> Ins {
    let s = (((value) >> 20) & 0x1) != 0;
    let cond = Cond::from(((value) >> 28) & 0xf);
    let rd = Reg::from(((value) >> 12) & 0xf);
    let rn = Reg::from(((value) >> 16) & 0xf);
    let op2 = Op2::from(value);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_0(value: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::from((value) & 0x7);
    let rn = Reg::from(((value) >> 3) & 0x7);
    let op2 = Op2::Imm(((value) >> 6) & 0x7);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_1(value: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::from(((value) >> 8) & 0x7);
    let rn = Reg::from(((value) >> 8) & 0x7);
    let op2 = Op2::Imm((value) & 0xff);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_2(value: u32) -> Ins {
    let s = (1) != 0;
    let cond = Cond::default();
    let rd = Reg::from((value) & 0x7);
    let rn = Reg::from(((value) >> 3) & 0x7);
    let op2 = Op2::ShiftImm {
        rm: Reg::from(((value) >> 6) & 0x7),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_3(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7));
    let rn = Reg::from(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7));
    let op2 = Op2::ShiftImm {
        rm: Reg::from(((value) >> 3) & 0xf),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_4(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(((value) >> 8) & 0x7);
    let rn = Reg::from(13);
    let op2 = Op2::Imm(((value) & 0xff) << 2);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_5(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(13);
    let rn = Reg::from(13);
    let op2 = Op2::Imm(((value) & 0x7f) << 2);
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_6(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7));
    let rn = Reg::from(13);
    let op2 = Op2::ShiftImm {
        rm: Reg::from(((((value) >> 7) & 0x1) << 3) | ((value) & 0x7)),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_7(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(13);
    let rn = Reg::from(13);
    let op2 = Op2::ShiftImm {
        rm: Reg::from(((value) >> 3) & 0xf),
        shift_op: ShiftOp::default(),
        imm: 0,
    };
    Ins::Add { s, cond, rd, rn, op2 }
}
fn parse_thumb_add_8(value: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(((value) >> 8) & 0x7);
    let rn = Reg::from(15);
    let op2 = Op2::Imm((value) & 0xff);
    Ins::Add { s, cond, rd, rn, op2 }
}

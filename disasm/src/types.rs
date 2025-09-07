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
    V5Tej,
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
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BranchTarget {
    pub addr: u32,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlxTarget {
    ///Direct target
    Direct(BranchTarget),
    ///Indirect target
    Indirect(Reg),
}
#[derive(PartialEq, Eq, Clone, Copy)]
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
#[derive(PartialEq, Eq, Clone, Copy)]
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
#[derive(PartialEq, Eq, Clone, Copy)]
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
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Coproc {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CoReg {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Op2 {
    ///Immediate
    Imm(u32),
    ///Register shifted by register
    ShiftReg { rm: Reg, shift_op: ShiftOp, rs: Reg },
    ///Register shifted by immediate
    ShiftImm { rm: Reg, shift_op: ShiftOp, imm: u32 },
}
pub enum Ins {
    Adc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    Add { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    And { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    B { cond: Cond, target: BranchTarget },
    Bic { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    Bkpt { imm: u32 },
    Bl { cond: Cond, target: BranchTarget },
    Blx { cond: Cond, target: BlxTarget },
    Bx { cond: Cond, rm: Reg },
    Bxj { cond: Cond, rm: Reg },
    Cdp {
        cond: Cond,
        coproc: Coproc,
        opc1: u32,
        crd: CoReg,
        crn: CoReg,
        crm: CoReg,
        opc2: u32,
    },
    Cdp2 { coproc: Coproc, opc1: u32, crd: CoReg, crn: CoReg, crm: CoReg, opc2: u32 },
    Clrex {},
    Illegal,
}

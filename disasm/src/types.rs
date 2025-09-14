#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::*;
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
pub enum StatusReg {
    ///Current program status register
    Cpsr,
    ///Saved program status register
    Spsr,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct StatusFields {
    pub reg: StatusReg,
    pub c: bool,
    pub x: bool,
    pub s: bool,
    pub f: bool,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MsrOp2 {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
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
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Op2Shift {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CpsEffect {
    ///Set mode
    SetMode,
    ///Interrupt enable
    Ie,
    ///Interrupt disable
    Id,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct AifFlags {
    pub a: bool,
    pub i: bool,
    pub f: bool,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddrLdcStc {
    ///Pre-indexed
    Pre { rn: Reg, offset: i32, writeback: bool },
    ///Post-indexed
    Post { rn: Reg, offset: i32 },
    ///Unindexed
    Unidx { rn: Reg, option: u32 },
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LdmStmMode {
    ///Decrement After
    Da,
    ///Increment After
    Ia,
    ///Decrement Before
    Db,
    ///Increment Before
    Ib,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddrLdrStr {
    ///Pre-indexed
    Pre { rn: Reg, offset: LdrStrOffset, writeback: bool },
    ///Post-indexed
    Post(AddrLdrStrPost),
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct AddrLdrStrPost {
    pub rn: Reg,
    pub offset: LdrStrOffset,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LdrStrOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg { subtract: bool, rm: Reg, shift_op: ShiftOp, imm: u32 },
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddrMiscLoad {
    ///Pre-indexed
    Pre { rn: Reg, offset: MiscLoadOffset, writeback: bool },
    Post { rn: Reg, offset: MiscLoadOffset },
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MiscLoadOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg { rm: Reg, subtract: bool },
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SrsRfeMode {
    ///Decrement After
    Da,
    ///Increment After
    Ia,
    ///Decrement Before
    Db,
    ///Increment Before
    Ib,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    ///Little-endian
    Le,
    ///Big-endian
    Be,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RegSide {
    ///Bottom halfword
    Bottom,
    ///Top halfword
    Top,
}
pub enum Ins {
    ///Add with Carry
    Adc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Add
    Add { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Bitwise AND
    And { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Arithmetic Shift Right
    Asr { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Branch
    B { cond: Cond, target: BranchTarget },
    ///Bit Clear
    Bic { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Breakpoint
    Bkpt { imm: u32 },
    ///Branch with Link
    Bl { cond: Cond, target: BranchTarget },
    ///Branch with Link and Exchange
    Blx { cond: Cond, target: BlxTarget },
    ///Branch and Exchange
    Bx { cond: Cond, rm: Reg },
    ///Branch and Exchange Jazelle
    Bxj { cond: Cond, rm: Reg },
    ///Coprocessor Data Processing
    Cdp {
        cond: Cond,
        coproc: Coproc,
        opc1: u32,
        crd: CoReg,
        crn: CoReg,
        crm: CoReg,
        opc2: u32,
    },
    ///Coprocessor Data Processing (extended)
    Cdp2 { coproc: Coproc, opc1: u32, crd: CoReg, crn: CoReg, crm: CoReg, opc2: u32 },
    ///Clear Exclusive
    Clrex {},
    ///Count Leading Zeros
    Clz { cond: Cond, rd: Reg, rm: Reg },
    ///Compare Negative
    Cmn { cond: Cond, rn: Reg, op2: Op2 },
    ///Compare
    Cmp { cond: Cond, rn: Reg, op2: Op2 },
    ///Change Processor State
    Cps { effect: CpsEffect, aif: AifFlags, mode: u32 },
    ///Consume of Speculative Data Barrier
    Csdb { cond: Cond },
    ///Bitwise Exclusive OR
    Eor { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Load Coprocessor
    Ldc { l: bool, cond: Cond, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
    ///Load Coprocessor (extended)
    Ldc2 { l: bool, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
    ///Load Multiple
    Ldm {
        mode: LdmStmMode,
        cond: Cond,
        rn: Reg,
        writeback: bool,
        regs: RegList,
        user_mode: bool,
    },
    ///Load Register
    Ldr { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Load Register Byte
    Ldrb { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Load Register Byte with Translation
    Ldrbt { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Load Register Dual
    Ldrd { cond: Cond, rd: Reg, rd2: Reg, addr: AddrMiscLoad },
    ///Load Register Exclusive
    Ldrex { cond: Cond, rd: Reg, rn: Reg },
    ///Load Register Exclusive Byte
    Ldrexb { cond: Cond, rd: Reg, rn: Reg },
    ///Load Register Exclusive Doubleword
    Ldrexd { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    ///Load Register Exclusive Halfword
    Ldrexh { cond: Cond, rd: Reg, rn: Reg },
    ///Load Register Halfword
    Ldrh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Load Register Signed Byte
    Ldrsb { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Load Register Signed Halfword
    Ldrsh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Load Register with Translation
    Ldrt { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Logical Shift Left
    Lsl { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Logical Shift Right
    Lsr { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Move to Coprocessor from ARM Register
    Mcr {
        cond: Cond,
        coproc: Coproc,
        opc1: u32,
        rd: Reg,
        crn: CoReg,
        crm: CoReg,
        opc2: u32,
    },
    ///Move to Coprocessor from ARM Register (extended)
    Mcr2 { coproc: Coproc, opc1: u32, rd: Reg, crn: CoReg, crm: CoReg, opc2: u32 },
    ///Move to Coprocessor from two ARM Registers
    Mcrr { cond: Cond, coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    ///Move to Coprocessor from two ARM Registers (extended)
    Mcrr2 { coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    ///Multiply Accumulate
    Mla { s: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    ///Move
    Mov { s: bool, cond: Cond, rd: Reg, op2: Op2 },
    ///Move to ARM Register from Coprocessor
    Mrc {
        cond: Cond,
        coproc: Coproc,
        opc1: u32,
        rd: Reg,
        crn: CoReg,
        crm: CoReg,
        opc2: u32,
    },
    ///Move to ARM Register from Coprocessor (extended)
    Mrc2 { coproc: Coproc, opc1: u32, rd: Reg, crn: CoReg, crm: CoReg, opc2: u32 },
    ///Move to two ARM Registers from Coprocessor
    Mrrc { cond: Cond, coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    ///Move to two ARM Registers from Coprocessor (extended)
    Mrrc2 { coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    ///Move to Register from Status register
    Mrs { cond: Cond, rd: Reg, status_reg: StatusReg },
    ///Move to Status register
    Msr { cond: Cond, status_fields: StatusFields, op2: MsrOp2 },
    ///Multiply
    Mul { s: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Move Negative
    Mvn { s: bool, cond: Cond, rd: Reg, op2: Op2 },
    ///No Operation
    Nop { cond: Cond },
    ///Bitwise OR
    Orr { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Pack Halfword Bottom Top
    Pkhbt { cond: Cond, rd: Reg, rn: Reg, rm: Reg, shift_op: ShiftOp, shift: u32 },
    ///Pack Halfword Top Bottom
    Pkhtb { cond: Cond, rd: Reg, rn: Reg, rm: Reg, shift_op: ShiftOp, shift: u32 },
    ///Preload Data
    Pld { addr: AddrLdrStr },
    ///Pop multiple registers
    Pop { cond: Cond, regs: RegList },
    ///Push multiple registers
    Push { cond: Cond, regs: RegList },
    ///Saturating Add
    Qadd { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Saturating Add two 16-bit values
    Qadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Saturating Add four 8-bit values
    Qadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Saturating Add and Subtract with Exchange
    Qasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Saturating Double and Add
    Qdadd { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Saturating Double and Subtract
    Qdsub { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Saturating Subtract and Add with Exchange
    Qsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Saturating Subtract
    Qsub { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Saturating Subtract two 16-bit values
    Qsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Saturating Subtract four 8-bit values
    Qsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Reverse bytes in word
    Rev { cond: Cond, rd: Reg, rm: Reg },
    ///Reverse bytes in packed halfwords
    Rev16 { cond: Cond, rd: Reg, rm: Reg },
    ///Reverse bytes in signed halfword
    Revsh { cond: Cond, rd: Reg, rm: Reg },
    ///Return From Exception
    Rfe { mode: SrsRfeMode, rn: Reg, writeback: bool },
    ///Rotate Right
    Ror { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Rotate Right with Extend
    Rrx { s: bool, cond: Cond, rd: Reg, rm: Reg },
    ///Reverse Subtract
    Rsb { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Reverse Subtract with Carry
    Rsc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Signed Add two 16-bit values
    Sadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Add four 8-bit values
    Sadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Add and Subtract with Exchange
    Sasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Subtract with Carry
    Sbc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Select
    Sel { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Set Endianness
    Setend { endian: Endianness },
    ///Send Event
    Sev { cond: Cond },
    ///Signed Halving Add two 16-bit values
    Shadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Halving Add four 8-bit values
    Shadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Halving Add and Subtract with Exchange
    Shasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Halving Subtract and Add with Exchange
    Shsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Halving Subtract two 16-bit values
    Shsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Halving Subtract four 8-bit values
    Shsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Multiply Accumulate halfwords
    Smla {
        cond: Cond,
        rd: Reg,
        rn: Reg,
        rn_side: RegSide,
        rm: Reg,
        rm_side: RegSide,
        ra: Reg,
    },
    ///Signed Multiply Accumulate Dual
    Smlad { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool, ra: Reg },
    ///Signed Multiply Accumulate Long
    Smlal { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    ///Signed Multiply Accumulate Long halfwords
    Smlal_half {
        cond: Cond,
        rd_lo: Reg,
        rd_hi: Reg,
        rn: Reg,
        rn_side: RegSide,
        rm: Reg,
        rm_side: RegSide,
    },
    ///Signed Multiply Accumulate Long Dual
    Smlald { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    ///Signed Multiply Accumulate Word by halfword
    Smlaw { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rm_side: RegSide, ra: Reg },
    ///Signed Multiply Subtract Dual
    Smlsd { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool, ra: Reg },
    ///Signed Multiply Subtract Long Dual
    Smlsld { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    ///Signed Most significant word Multiply Accumulate
    Smmla { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    ///Signed Most significant word Multiply Subtract
    Smmls { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    ///Signed Most significant word Multiply
    Smmul { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Multiply Add Dual
    Smuad { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    ///Signed Multiply
    Smul { cond: Cond, rd: Reg, rn: Reg, rn_side: RegSide, rm: Reg, rm_side: RegSide },
    ///Signed Multiply Long
    Smull { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    ///Signed Multiply Word by halfword
    Smulw { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rm_side: RegSide },
    ///Signed Multiply Subtract Dual
    Smusd { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool },
}

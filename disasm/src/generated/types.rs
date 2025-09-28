#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::*;
#[derive(Clone, Copy)]
pub struct Options {
    ///The version of ARM to use
    pub version: Version,
    ///The extensions to enable
    pub extensions: Extensions,
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
#[derive(Debug, Clone, Copy)]
pub enum Version {
    V4,
    V4T,
    V5T,
    V5Te,
    V5Tej,
    V6,
    V6K,
}
impl Version {
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }
}
#[derive(Clone, Copy)]
pub struct Versions(u8);
impl Versions {
    pub const fn none() -> Self {
        Self(0)
    }
    pub const fn all() -> Self {
        Self(u8::MAX)
    }
    pub fn with(self, version: Version) -> Self {
        Self(self.0 | version.bit())
    }
    pub const fn of(versions: &[Version]) -> Self {
        let mut mask = 0;
        let mut i = 0;
        loop {
            if i >= versions.len() {
                break;
            }
            mask |= versions[i].bit();
            i += 1;
        }
        Self(mask)
    }
    pub fn has(self, version: Version) -> bool {
        (self.0 & version.bit()) != 0
    }
}
#[derive(Clone, Copy)]
pub enum Extension {
    VfpV2,
}
impl Extension {
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }
}
#[derive(Clone, Copy)]
pub struct Extensions(u8);
impl Extensions {
    pub const fn none() -> Self {
        Self(0)
    }
    pub const fn all() -> Self {
        Self(u8::MAX)
    }
    pub fn with(self, extension: Extension) -> Self {
        Self(self.0 | extension.bit())
    }
    pub const fn of(extensions: &[Extension]) -> Self {
        let mut mask = 0;
        let mut i = 0;
        loop {
            if i >= extensions.len() {
                break;
            }
            mask |= extensions[i].bit();
            i += 1;
        }
        Self(mask)
    }
    pub fn has_all(self, extensions: Extensions) -> bool {
        (self.0 & extensions.0) == self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum R9Use {
    ///General purpose register
    R9,
    ///Static base (SB), used for position-independent data
    Sb,
    ///TLS register (TR), used for thread-local storage
    Tr,
}
///The direct destination address of a branch instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BranchTarget {
    pub addr: u32,
}
///The destination of a BLX instruction, which can be direct (immediate) or indirect (register)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BlxTarget {
    ///Direct target
    Direct(BranchTarget),
    ///Indirect target
    Indirect(Reg),
}
///Mnemonic suffix, specifies the condition for whether to execute the instruction
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///General-purpose register
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///Status register
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StatusReg {
    ///Current program status register
    Cpsr,
    ///Saved program status register
    Spsr,
}
///Status register with field masks
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StatusFields {
    pub reg: StatusReg,
    pub c: bool,
    pub x: bool,
    pub s: bool,
    pub f: bool,
}
///Second operand of the MSR instruction, can be an immediate or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MsrOp2 {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
}
///Shift operation
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///Coprocessor
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///Coprocessor register
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///Second operand of a data-processing operation, can be an immediate, an immediate-shifted register or a register-shifted register.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Op2 {
    ///Immediate
    Imm(u32),
    ///Register shifted by register
    ShiftReg(ShiftReg),
    ///Register shifted by immediate
    ShiftImm(ShiftImm),
}
///Register shifted by another register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftReg {
    pub rm: Reg,
    pub shift_op: ShiftOp,
    pub rs: Reg,
}
///Register shifted by an immediate
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftImm {
    pub rm: Reg,
    pub shift_op: ShiftOp,
    pub imm: u32,
}
///Second operand of a shift instruction, can be an immediate or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Op2Shift {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
}
///Mnemonic suffix for CPS, specifies whether to enable/disable interrupt bits or just set the processor mode
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CpsEffect {
    ///Set mode
    SetMode,
    ///Interrupt enable
    Ie,
    ///Interrupt disable
    Id,
}
///In a CPS instruction, specifies which interrupt bits to enable or disable
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AifFlags {
    pub a: bool,
    pub i: bool,
    pub f: bool,
}
///The memory address of an LDC/STC instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrLdcStc {
    ///Pre-indexed
    Pre { rn: Reg, offset: i32, writeback: bool },
    ///Post-indexed
    Post { rn: Reg, offset: i32 },
    ///Unindexed
    Unidx { rn: Reg, option: u32 },
}
///Mnemonic suffix for LDM/STM, specifies how to step the base address
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///The memory address of an LDR(B)/STR(B)/PLD instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrLdrStr {
    ///Pre-indexed
    Pre { rn: Reg, offset: LdrStrOffset, writeback: bool },
    ///Post-indexed
    Post(AddrLdrStrPost),
}
///A post-indexed memory address for LDR(B)(T)/STR(B)(T)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AddrLdrStrPost {
    pub rn: Reg,
    pub offset: LdrStrOffset,
}
///The offset value in the memory address of a LDR(B)/STR(B) instruction, can be an immediate or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LdrStrOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg { subtract: bool, rm: Reg, shift_op: ShiftOp, imm: u32 },
}
///The memory address of a miscellaneous load/store instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrMiscLoad {
    ///Pre-indexed
    Pre { rn: Reg, offset: MiscLoadOffset, writeback: bool },
    Post { rn: Reg, offset: MiscLoadOffset },
}
///The offset value in the memory address of a miscellaneous load/store instruction, can be an immediate or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MiscLoadOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg { rm: Reg, subtract: bool },
}
///Mnemonic suffix for SRS/RFE, specifies how to step the stack pointer
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
///Used by SETEND, specifies the endianness for data accesses
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Endianness {
    ///Little-endian
    Le,
    ///Big-endian
    Be,
}
///Mnemonic suffix, specifies which half of a register to use as an operand
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RegSide {
    ///Bottom halfword
    Bottom,
    ///Top halfword
    Top,
}
///General-purpose register for single-precision floating-point numbers
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Sreg {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S20,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,
}
///General-purpose register for double-precision floating-point numbers
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Dreg {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
}
///Second operand of a VCMP.F32 instruction, can be zero or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VcmpF32Op2 {
    ///Compare with zero
    Zero,
    ///Compare with register
    Reg(Sreg),
}
///Second operand of a VCMP.F64 instruction, can be zero or a register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VcmpF64Op2 {
    ///Compare with zero
    Zero,
    ///Compare with register
    Reg(Dreg),
}
///A double-precision floating-point register and index (0 or 1) to move to/from
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DregIndex {
    pub dreg: Dreg,
    pub index: u32,
}
///Floating-Point Status and Control Register
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Fpscr {}
///Mnemonic suffix for VLDM/VSTM, specifies how to step the base address
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VldmVstmMode {
    ///Increment After
    Ia,
    ///Decrement Before
    Db,
}
#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ins {
    ///Add with Carry
    Adc { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Add
    Add { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Bitwise AND
    And { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Arithmetic Shift Right
    Asr { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Branch
    B { cond: Cond, target: BranchTarget },
    ///Bit Clear
    Bic { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
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
    ///Debug Hint
    Dbg { cond: Cond, option: u32 },
    ///Bitwise Exclusive OR
    Eor { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
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
    Ldrbt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
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
    Ldrt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    ///Logical Shift Left
    Lsl { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Logical Shift Right
    Lsr { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
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
    Mov { s: bool, thumb: bool, cond: Cond, rd: Reg, op2: Op2 },
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
    Mul { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Move Negative
    Mvn { s: bool, thumb: bool, cond: Cond, rd: Reg, op2: Op2 },
    ///Negate
    Neg { rd: Reg, rm: Reg },
    ///No Operation
    Nop { cond: Cond },
    ///Bitwise OR
    Orr { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
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
    Rfe { addr_mode: SrsRfeMode, rn: Reg, writeback: bool },
    ///Rotate Right
    Ror { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
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
    Sbc { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
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
    SmlalHalf {
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
    ///Store Return State
    Srs { addr_mode: SrsRfeMode, rn: Reg, writeback: bool, mode: u32 },
    ///Signed Saturate
    Ssat { cond: Cond, rd: Reg, imm: u32, op2: ShiftImm },
    ///Signed Saturate two 16-bit values
    Ssat16 { cond: Cond, rd: Reg, imm: u32, rn: Reg },
    ///Signed Subtract and Add with Exchange
    Ssax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Subtract two 16-bit values
    Ssub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Signed Subtract four 8-bit values
    Ssub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Store Coprocessor
    Stc { l: bool, cond: Cond, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
    ///Store Coprocessor (extended)
    Stc2 { l: bool, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
    ///Store Multiple
    Stm {
        mode: LdmStmMode,
        cond: Cond,
        rn: Reg,
        writeback: bool,
        regs: RegList,
        user_mode: bool,
    },
    ///Store Register
    Str { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Store Register Byte
    Strb { cond: Cond, rd: Reg, addr: AddrLdrStr },
    ///Store Register Byte with Translation
    Strbt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    ///Store Register Dual
    Strd { cond: Cond, rd: Reg, rd2: Reg, addr: AddrMiscLoad },
    ///Store Register Exclusive
    Strex { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Store Register Exclusive Byte
    Strexb { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Store Register Exclusive Doubleword
    Strexd { cond: Cond, rd: Reg, rm: Reg, rm2: Reg, rn: Reg },
    ///Store Register Exclusive Halfword
    Strexh { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Store Register Halfword
    Strh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Store Register with Translation
    Strt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    ///Subtract
    Sub { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Supervisor Call
    Svc { cond: Cond, imm: u32 },
    ///Swap
    Swp { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    ///Swap Byte
    Swpb { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    ///Sign Extend and Add Byte
    Sxtab { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Sign Extend to 16 bits and Add Byte
    Sxtab16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Sign Extend and Add Halfword
    Sxtah { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Sign Extend Byte
    Sxtb { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Sign Extend Byte to 16 bits
    Sxtb16 { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Sign Extend Halfword
    Sxth { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Test Equivalence
    Teq { cond: Cond, rn: Reg, op2: Op2 },
    ///Test
    Tst { cond: Cond, rn: Reg, op2: Op2 },
    ///Unsigned Add two 16-bit values
    Uadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Add four 8-bit values
    Uadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Add and Subtract with Exchange
    Uasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Undefined Permanently
    Udf { imm: u32 },
    ///Unsigned Halving Add two 16-bit values
    Uhadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Halving Add four 8-bit values
    Uhadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Halving Add and Subtract with Exchange
    Uhasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Halving Subtract and Add with Exchange
    Uhsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Halving Subtract two 16-bit values
    Uhsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Halving Subtract four 8-bit values
    Uhsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Multiply Accumulate Accumulate Long
    Umaal { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    ///Unsigned Multiply Accumulate Long
    Umlal { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    ///Unsigned Multiply Long
    Umull { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Add two 16-bit values
    Uqadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Add four 8-bit values
    Uqadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Add and Subtract with Exchange
    Uqasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Subtract and Add with Exchange
    Uqsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Subtract two 16-bit values
    Uqsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Saturating Subtract four 8-bit values
    Uqsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Sum of Absolute Differences for four 8-bit values
    Usad8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Sum of Absolute Differences and Accumulate four 8-bit values
    Usada8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    ///Unsigned Saturate
    Usat { cond: Cond, rd: Reg, imm: u32, op2: ShiftImm },
    ///Unsigned Saturate two 16-bit values
    Usat16 { cond: Cond, rd: Reg, imm: u32, rn: Reg },
    ///Unsigned Subtract and Add with Exchange
    Usax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Subtract two 16-bit values
    Usub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Subtract four 8-bit values
    Usub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Unsigned Extend and Add Byte
    Uxtab { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Unsigned Extend to 16 bits and Add Byte
    Uxtab16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Unsigned Extend and Add Halfword
    Uxtah { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    ///Unsigned Extend Byte
    Uxtb { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Unsigned Extend Byte to 16 bits
    Uxtb16 { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Unsigned Extend Halfword
    Uxth { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    ///Vector Absolute 32-bit
    VabsF32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Absolute 64-bit
    VabsF64 { cond: Cond, dd: Dreg, dm: Dreg },
    ///Vector Add 32-bit
    VaddF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Add 64-bit
    VaddF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Compare 32-bit
    VcmpF32 { nan_exc: bool, cond: Cond, sd: Sreg, op2: VcmpF32Op2 },
    ///Vector Compare 64-bit
    VcmpF64 { nan_exc: bool, cond: Cond, dd: Dreg, op2: VcmpF64Op2 },
    ///Vector Convert to Floating-point 32-bit from Floating-point 64-bit
    VcvtF32F64 { cond: Cond, sd: Sreg, dm: Dreg },
    ///Vector Convert to Floating-point 32-bit from Signed 32-bit integer
    VcvtF32S32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Convert to Floating-point 32-bit from Unsigned 32-bit integer
    VcvtF32U32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Convert to Floating-point 64-bit from Floating-point 32-bit
    VcvtF64F32 { cond: Cond, dd: Dreg, sm: Sreg },
    ///Vector Convert to Floating-point 64-bit from Signed 32-bit integer
    VcvtF64S32 { cond: Cond, dd: Dreg, sm: Sreg },
    ///Vector Convert to Floating-point 64-bit from Unsigned 32-bit integer
    VcvtF64U32 { cond: Cond, dd: Dreg, sm: Sreg },
    ///Vector Convert to Signed 32-bit integer from Floating-point 32-bit
    VcvtS32F32 { round_zero: bool, cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Convert to Signed 32-bit integer from Floating-point 64-bit
    VcvtS32F64 { round_zero: bool, cond: Cond, sd: Sreg, dm: Dreg },
    ///Vector Convert to Unsigned 32-bit integer from Floating-point 32-bit
    VcvtU32F32 { round_zero: bool, cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Convert to Unsigned 32-bit integer from Floating-point 64-bit
    VcvtU32F64 { round_zero: bool, cond: Cond, sd: Sreg, dm: Dreg },
    ///Vector Divide Floating-point 32-bit
    VdivF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Divide Floating-point 64-bit
    VdivF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Load Multiple Floating-point 32-bit
    VldmF32 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: SregList },
    ///Vector Load Multiple Floating-point 64-bit
    VldmF64 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: DregList },
    ///Vector Load Register Floating-point 32-bit
    VldrF32 { cond: Cond, sd: Sreg, addr: AddrLdrStr },
    ///Vector Load Register Floating-point 64-bit
    VldrF64 { cond: Cond, dd: Dreg, addr: AddrLdrStr },
    ///Vector Multiply Accumulate Floating-point 32-bit
    VmlaF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Multiply Accumulate Floating-point 64-bit
    VmlaF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Multiply Subtract Floating-point 32-bit
    VmlsF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Multiply Subtract Floating-point 64-bit
    VmlsF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Move to 32-bit scalar from register
    Vmov32Reg { cond: Cond, dd: DregIndex, rt: Reg },
    ///Vector Move Floating-point 32-bit
    VmovF32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Move to Floating-point 32-bit from register
    VmovF32Reg { cond: Cond, sn: Sreg, rt: Reg },
    ///Vector Move Floating-point 64-bit
    VmovF64 { cond: Cond, dd: Dreg, dm: Dreg },
    ///Vector Move to register from 32-bit scalar
    VmovReg32 { cond: Cond, rt: Reg, dn: DregIndex },
    ///Vector Move to register from floating-point 32-bit
    VmovRegF32 { cond: Cond, rt: Reg, sn: Sreg },
    ///Vector Move to two registers from two floating-point 32-bit
    VmovRegF32Dual { cond: Cond, rt: Reg, rt2: Reg, sm: Sreg, sm2: Sreg },
    ///Vector Move to two floating-point 32-bit from two registers
    VmovF32RegDual { cond: Cond, sm: Sreg, sm2: Sreg, rt: Reg, rt2: Reg },
    ///Vector Move to two registers from one floating-point 64-bit
    VmovRegF64 { cond: Cond, rt: Reg, rt2: Reg, dm: Dreg },
    ///Vector Move to one floating-point 64-bit from two registers
    VmovF64Reg { cond: Cond, dm: Dreg, rt: Reg, rt2: Reg },
    ///Vector Move to Register from Status register
    Vmrs { cond: Cond, rd: Reg, fpscr: Fpscr },
    ///Vector Move to Status register from Register
    Vmsr { cond: Cond, fpscr: Fpscr, rd: Reg },
    ///Vector Multiply Floating-point 32-bit
    VmulF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Multiply Floating-point 64-bit
    VmulF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Negate Floating-point 32-bit
    VnegF32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Negate Floating-point 64-bit
    VnegF64 { cond: Cond, dd: Dreg, dm: Dreg },
    ///Vector Negate Multiply Accumulate Floating-point 32-bit
    VnmlaF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Negate Multiply Accumulate Floating-point 64-bit
    VnmlaF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Negate Multiply Subtract Floating-point 32-bit
    VnmlsF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Negate Multiply Subtract Floating-point 64-bit
    VnmlsF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Negate Multiply Floating-point 32-bit
    VnmulF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Negate Multiply Floating-point 64-bit
    VnmulF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Vector Pop Floating-point 32-bit
    VpopF32 { cond: Cond, regs: SregList },
    ///Vector Pop Floating-point 64-bit
    VpopF64 { cond: Cond, regs: DregList },
    ///Vector Push Floating-point 32-bit
    VpushF32 { cond: Cond, regs: SregList },
    ///Vector Push Floating-point 64-bit
    VpushF64 { cond: Cond, regs: DregList },
    ///Vector Square Root Floating-point 32-bit
    VsqrtF32 { cond: Cond, sd: Sreg, sm: Sreg },
    ///Vector Square Root Floating-point 64-bit
    VsqrtF64 { cond: Cond, dd: Dreg, dm: Dreg },
    ///Vector Store Multiple Floating-point 32-bit
    VstmF32 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: SregList },
    ///Vector Store Multiple Floating-point 64-bit
    VstmF64 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: DregList },
    ///Vector Store Register Floating-point 32-bit
    VstrF32 { cond: Cond, sd: Sreg, addr: AddrLdrStr },
    ///Vector Store Register Floating-point 64-bit
    VstrF64 { cond: Cond, dd: Dreg, addr: AddrLdrStr },
    ///Vector Subtract 32-bit
    VsubF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    ///Vector Subtract 64-bit
    VsubF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    ///Wait For Event
    Wfe { cond: Cond },
    ///Wait For Interrupt
    Wfi { cond: Cond },
    ///Yield
    Yield { cond: Cond },
    Word(u32),
    HalfWord(u16),
    Byte(u8),
    Illegal,
}

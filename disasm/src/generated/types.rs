#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::*;
#[derive(Debug, Clone)]
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
impl Default for Options {
    fn default() -> Self {
        Self {
            version: Version::default(),
            extensions: Extensions::default(),
            av: false,
            r9_use: R9Use::default(),
            sl: false,
            fp: false,
            ip: false,
            ual: true,
        }
    }
}
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Version {
    #[cfg(feature = "v4")]
    V4,
    #[cfg(feature = "v4t")]
    V4T,
    #[cfg(feature = "v5t")]
    V5T,
    #[cfg(feature = "v5te")]
    V5Te,
    #[cfg(feature = "v5tej")]
    V5Tej,
    #[cfg(feature = "v6")]
    V6,
    #[cfg(feature = "v6k")]
    V6K,
}
impl Version {
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
impl Default for Version {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "v6k")] return Self::V6K;
        #[cfg(feature = "v6")] return Self::V6;
        #[cfg(feature = "v5tej")] return Self::V5Tej;
        #[cfg(feature = "v5te")] return Self::V5Te;
        #[cfg(feature = "v5t")] return Self::V5T;
        #[cfg(feature = "v4t")] return Self::V4T;
        #[cfg(feature = "v4")] return Self::V4;
    }
}
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Extension {
    #[cfg(feature = "vfp_v2")]
    VfpV2,
}
impl Extension {
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
impl Default for Extensions {
    fn default() -> Self {
        Self::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum R9Use {
    #[default]
    ///General purpose register
    R9,
    ///Static base (SB), used for position-independent data
    Sb,
    ///TLS register (TR), used for thread-local storage
    Tr,
}
///The direct destination address of a branch instruction
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BranchTarget {
    pub addr: u32,
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
///The destination of a BLX instruction, which can be direct (immediate) or indirect (register)
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BlxTarget {
    ///Direct target
    Direct(BranchTarget),
    ///Indirect target
    Indirect(Reg),
}
///Mnemonic suffix, specifies the condition for whether to execute the instruction
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
    ///General-purpose or SB (static base) or TR (TLS register)
    R9,
    ///General-purpose or SL (stack limit)
    R10,
    ///General-purpose or FP (frame pointer)
    R11,
    ///General-purpose or IP (intra-procedure call scratch register)
    R12,
    ///Stack pointer
    Sp,
    ///Link register
    Lr,
    ///Program counter
    Pc,
}
#[cfg(feature = "arm")]
///Status register
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StatusReg {
    ///Current program status register
    Cpsr,
    ///Saved program status register
    Spsr,
}
#[cfg(feature = "arm")]
///Status register with field masks
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StatusFields {
    pub reg: StatusReg,
    ///Control field mask
    pub c: bool,
    ///Extension field mask
    pub x: bool,
    ///Status field mask
    pub s: bool,
    ///Flags field mask
    pub f: bool,
}
#[cfg(feature = "arm")]
///Second operand of the MSR instruction, can be an immediate or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MsrOp2 {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
}
///Shift operation
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[cfg(feature = "arm")]
///Coprocessor
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[cfg(feature = "arm")]
///Coprocessor register
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Op2 {
    ///Immediate
    Imm(Op2Imm),
    ///Register shifted by register
    ShiftReg(ShiftReg),
    ///Register shifted by immediate
    ShiftImm(ShiftImm),
}
///Immediate second operand
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Op2Imm {
    ///Immediate value, already rotated by `rotate_imm`
    pub imm: u32,
    ///Immediate rotation amount
    pub rotate_imm: u32,
}
///Register shifted by another register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftReg {
    ///Register being shifted
    pub rm: Reg,
    ///Shift operation to apply
    pub shift_op: ShiftOp,
    ///Register to shift by
    pub rs: Reg,
}
///Register shifted by an immediate
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ShiftImm {
    ///Register being shifted
    pub rm: Reg,
    ///Shift operation to apply
    pub shift_op: ShiftOp,
    ///Immediate to shift by
    pub imm: u32,
}
///Second operand of a shift instruction, can be an immediate or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Op2Shift {
    ///Immediate
    Imm(u32),
    ///Register
    Reg(Reg),
}
#[cfg(any(feature = "v6", feature = "v6k"))]
///Mnemonic suffix for CPS, specifies whether to enable/disable interrupt bits or just set the processor mode
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CpsEffect {
    ///Set mode
    SetMode,
    ///Interrupt enable
    Ie,
    ///Interrupt disable
    Id,
}
#[cfg(any(feature = "v6", feature = "v6k"))]
///In a CPS instruction, specifies which interrupt bits to enable or disable
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AifFlags {
    ///Imprecise data abort bit
    pub a: bool,
    ///IRQ interrupt bit
    pub i: bool,
    ///FIQ interrupt bit
    pub f: bool,
}
#[cfg(feature = "arm")]
///The memory address of an LDC/STC instruction
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrLdcStc {
    ///Pre-indexed
    Pre {
        ///Base register
        rn: Reg,
        offset: i32,
        ///If true, write the last accessed address back to the base register
        writeback: bool,
    },
    ///Post-indexed
    Post {
        ///Base register
        rn: Reg,
        offset: i32,
    },
    ///Unindexed
    Unidx {
        ///Base register
        rn: Reg,
        ///Additional options to the coprocessor
        option: u32,
    },
}
///Mnemonic suffix for LDM/STM, specifies how to step the base address
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrLdrStr {
    ///Pre-indexed
    Pre {
        ///Base register
        rn: Reg,
        offset: LdrStrOffset,
        ///If true, write the last accessed address back to the base register
        writeback: bool,
    },
    ///Post-indexed
    Post(AddrLdrStrPost),
}
///A post-indexed memory address for LDR(B)(T)/STR(B)(T)
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AddrLdrStrPost {
    ///Base register
    pub rn: Reg,
    pub offset: LdrStrOffset,
}
///The offset value in the memory address of a LDR(B)/STR(B) instruction, can be an immediate or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LdrStrOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg {
        ///If true, subtract Rm from the base register instead of adding
        subtract: bool,
        ///Index register
        rm: Reg,
        ///Shift operation to apply to Rm
        shift_op: ShiftOp,
        ///Immediate to shift by
        imm: u32,
    },
}
///The memory address of a miscellaneous load/store instruction
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddrMiscLoad {
    ///Pre-indexed
    Pre {
        ///Base register
        rn: Reg,
        offset: MiscLoadOffset,
        ///If true, write the last accessed address back to the base register
        writeback: bool,
    },
    ///Post-indexed
    Post {
        ///Base register
        rn: Reg,
        offset: MiscLoadOffset,
    },
}
///The offset value in the memory address of a miscellaneous load/store instruction, can be an immediate or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MiscLoadOffset {
    ///Immediate offset
    Imm(i32),
    ///Register offset
    Reg {
        ///If true, subtract Rm from the base register instead of adding
        subtract: bool,
        ///Index register
        rm: Reg,
    },
}
#[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
///Mnemonic suffix for SRS/RFE, specifies how to step the stack pointer
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[cfg(any(feature = "v6", feature = "v6k"))]
///Used by SETEND, specifies the endianness for data accesses
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Endianness {
    ///Little-endian
    Le,
    ///Big-endian
    Be,
}
#[cfg(
    all(
        feature = "arm",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///Mnemonic suffix, specifies which half of a register to use as an operand
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RegSide {
    ///Bottom halfword
    Bottom,
    ///Top halfword
    Top,
}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///General-purpose register for single-precision floating-point numbers
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///General-purpose register for double-precision floating-point numbers
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,
}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///Second operand of a VCMP.F32 instruction, can be zero or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VcmpF32Op2 {
    ///Compare with zero
    Zero,
    ///Compare with register
    Reg(Sreg),
}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///Second operand of a VCMP.F64 instruction, can be zero or a register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VcmpF64Op2 {
    ///Compare with zero
    Zero,
    ///Compare with register
    Reg(Dreg),
}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///A double-precision floating-point register and index (0 or 1) to move to/from
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DregIndex {
    pub dreg: Dreg,
    pub index: u32,
}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///Floating-Point Status and Control Register
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Fpscr {}
#[cfg(
    all(
        feature = "arm",
        feature = "vfp_v2",
        any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
    )
)]
///Mnemonic suffix for VLDM/VSTM, specifies how to step the base address
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VldmVstmMode {
    ///Increment After
    Ia,
    ///Decrement Before
    Db,
}
#[repr(u16)]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    #[cfg(
        any(
            feature = "v5t",
            feature = "v5te",
            feature = "v5tej",
            feature = "v6",
            feature = "v6k"
        )
    )]
    ///Breakpoint
    Bkpt { imm: u32 },
    ///Branch with Link
    Bl { cond: Cond, target: BranchTarget },
    #[cfg(
        any(
            feature = "v5t",
            feature = "v5te",
            feature = "v5tej",
            feature = "v6",
            feature = "v6k"
        )
    )]
    ///Branch with Link and Exchange
    Blx { cond: Cond, target: BlxTarget },
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
    ///Branch and Exchange
    Bx { cond: Cond, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v5tej", feature = "v6", feature = "v6k")))]
    ///Branch and Exchange Jazelle
    Bxj { cond: Cond, rm: Reg },
    #[cfg(feature = "arm")]
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
    ///Coprocessor Data Processing (extended)
    Cdp2 { coproc: Coproc, opc1: u32, crd: CoReg, crn: CoReg, crm: CoReg, opc2: u32 },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Clear Exclusive
    Clrex {},
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
    ///Count Leading Zeros
    Clz { cond: Cond, rd: Reg, rm: Reg },
    ///Compare Negative
    Cmn { cond: Cond, rn: Reg, op2: Op2 },
    ///Compare
    Cmp { cond: Cond, rn: Reg, op2: Op2 },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Change Processor State
    Cps { effect: CpsEffect, aif: AifFlags, mode: u32 },
    #[cfg(feature = "arm")]
    ///Consume of Speculative Data Barrier
    Csdb { cond: Cond },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Debug Hint
    Dbg { cond: Cond, option: u32 },
    ///Bitwise Exclusive OR
    Eor { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    #[cfg(feature = "arm")]
    ///Load Coprocessor
    Ldc { l: bool, cond: Cond, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
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
    #[cfg(feature = "arm")]
    ///Load Register Byte with Translation
    Ldrbt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Load Register Dual
    Ldrd { cond: Cond, rd: Reg, rd2: Reg, addr: AddrMiscLoad },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Load Register Exclusive
    Ldrex { cond: Cond, rd: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Load Register Exclusive Byte
    Ldrexb { cond: Cond, rd: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Load Register Exclusive Doubleword
    Ldrexd { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Load Register Exclusive Halfword
    Ldrexh { cond: Cond, rd: Reg, rn: Reg },
    ///Load Register Halfword
    Ldrh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Load Register Signed Byte
    Ldrsb { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    ///Load Register Signed Halfword
    Ldrsh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    #[cfg(feature = "arm")]
    ///Load Register with Translation
    Ldrt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    ///Logical Shift Left
    Lsl { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    ///Logical Shift Right
    Lsr { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    #[cfg(feature = "arm")]
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
    ///Move to Coprocessor from ARM Register (extended)
    Mcr2 { coproc: Coproc, opc1: u32, rd: Reg, crn: CoReg, crm: CoReg, opc2: u32 },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Move to Coprocessor from two ARM Registers
    Mcrr { cond: Cond, coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Move to Coprocessor from two ARM Registers (extended)
    Mcrr2 { coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    #[cfg(feature = "arm")]
    ///Multiply Accumulate
    Mla { s: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    ///Move
    Mov { s: bool, thumb: bool, cond: Cond, rd: Reg, op2: Op2 },
    #[cfg(feature = "arm")]
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
    ///Move to ARM Register from Coprocessor (extended)
    Mrc2 { coproc: Coproc, opc1: u32, rd: Reg, crn: CoReg, crm: CoReg, opc2: u32 },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Move to two ARM Registers from Coprocessor
    Mrrc { cond: Cond, coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Move to two ARM Registers from Coprocessor (extended)
    Mrrc2 { coproc: Coproc, opc: u32, rd: Reg, rd2: Reg, crm: CoReg },
    #[cfg(feature = "arm")]
    ///Move to Register from Status register
    Mrs { cond: Cond, rd: Reg, status_reg: StatusReg },
    #[cfg(feature = "arm")]
    ///Move to Status register
    Msr { cond: Cond, status_fields: StatusFields, op2: MsrOp2 },
    ///Multiply
    Mul { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Move Negative
    Mvn { s: bool, thumb: bool, cond: Cond, rd: Reg, op2: Op2 },
    #[cfg(feature = "thumb")]
    ///Negate
    Neg { rd: Reg, rm: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///No Operation
    Nop { cond: Cond },
    ///Bitwise OR
    Orr { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Pack Halfword Bottom Top
    Pkhbt { cond: Cond, rd: Reg, rn: Reg, rm: Reg, shift_op: ShiftOp, shift: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Pack Halfword Top Bottom
    Pkhtb { cond: Cond, rd: Reg, rn: Reg, rm: Reg, shift_op: ShiftOp, shift: u32 },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Preload Data
    Pld { addr: AddrLdrStr },
    ///Pop multiple registers
    Pop { cond: Cond, regs: RegList },
    ///Push multiple registers
    Push { cond: Cond, regs: RegList },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Saturating Add
    Qadd { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Add two 16-bit values
    Qadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Add four 8-bit values
    Qadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Add and Subtract with Exchange
    Qasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Saturating Double and Add
    Qdadd { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Saturating Double and Subtract
    Qdsub { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Subtract and Add with Exchange
    Qsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Saturating Subtract
    Qsub { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Subtract two 16-bit values
    Qsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Saturating Subtract four 8-bit values
    Qsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Reverse bytes in word
    Rev { cond: Cond, rd: Reg, rm: Reg },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Reverse bytes in packed halfwords
    Rev16 { cond: Cond, rd: Reg, rm: Reg },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Reverse bytes in signed halfword
    Revsh { cond: Cond, rd: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Return From Exception
    Rfe { addr_mode: SrsRfeMode, rn: Reg, writeback: bool },
    ///Rotate Right
    Ror { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2Shift },
    #[cfg(feature = "arm")]
    ///Rotate Right with Extend
    Rrx { s: bool, cond: Cond, rd: Reg, rm: Reg },
    ///Reverse Subtract
    Rsb { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    #[cfg(feature = "arm")]
    ///Reverse Subtract with Carry
    Rsc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Add two 16-bit values
    Sadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Add four 8-bit values
    Sadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Add and Subtract with Exchange
    Sasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    ///Subtract with Carry
    Sbc { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Select
    Sel { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Set Endianness
    Setend { endian: Endianness },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Send Event
    Sev { cond: Cond },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Add two 16-bit values
    Shadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Add four 8-bit values
    Shadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Add and Subtract with Exchange
    Shasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Subtract and Add with Exchange
    Shsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Subtract two 16-bit values
    Shsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Halving Subtract four 8-bit values
    Shsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
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
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Accumulate Dual
    Smlad { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool, ra: Reg },
    #[cfg(feature = "arm")]
    ///Signed Multiply Accumulate Long
    Smlal { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
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
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Accumulate Long Dual
    Smlald { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Signed Multiply Accumulate Word by halfword
    Smlaw { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rm_side: RegSide, ra: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Subtract Dual
    Smlsd { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool, ra: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Subtract Long Dual
    Smlsld { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Most significant word Multiply Accumulate
    Smmla { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Most significant word Multiply Subtract
    Smmls { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Most significant word Multiply
    Smmul { round: bool, cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Add Dual
    Smuad { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Signed Multiply
    Smul { cond: Cond, rd: Reg, rn: Reg, rn_side: RegSide, rm: Reg, rm_side: RegSide },
    #[cfg(feature = "arm")]
    ///Signed Multiply Long
    Smull { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Signed Multiply Word by halfword
    Smulw { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rm_side: RegSide },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Multiply Subtract Dual
    Smusd { cond: Cond, rd: Reg, rn: Reg, rm: Reg, swap_rm: bool },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Store Return State
    Srs { addr_mode: SrsRfeMode, rn: Reg, writeback: bool, mode: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Saturate
    Ssat { cond: Cond, rd: Reg, imm: u32, op2: ShiftImm },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Saturate two 16-bit values
    Ssat16 { cond: Cond, rd: Reg, imm: u32, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Subtract and Add with Exchange
    Ssax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Subtract two 16-bit values
    Ssub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Signed Subtract four 8-bit values
    Ssub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(feature = "arm")]
    ///Store Coprocessor
    Stc { l: bool, cond: Cond, coproc: Coproc, crd: CoReg, dest: AddrLdcStc },
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
    #[cfg(feature = "arm")]
    ///Store Register Byte with Translation
    Strbt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    #[cfg(
        all(
            feature = "arm",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Store Register Dual
    Strd { cond: Cond, rd: Reg, rd2: Reg, addr: AddrMiscLoad },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Store Register Exclusive
    Strex { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Store Register Exclusive Byte
    Strexb { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Store Register Exclusive Doubleword
    Strexd { cond: Cond, rd: Reg, rm: Reg, rm2: Reg, rn: Reg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Store Register Exclusive Halfword
    Strexh { cond: Cond, rd: Reg, rm: Reg, rn: Reg },
    ///Store Register Halfword
    Strh { cond: Cond, rd: Reg, addr: AddrMiscLoad },
    #[cfg(feature = "arm")]
    ///Store Register with Translation
    Strt { cond: Cond, rd: Reg, addr: AddrLdrStrPost },
    ///Subtract
    Sub { s: bool, thumb: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
    ///Supervisor Call
    Svc { cond: Cond, imm: u32 },
    #[cfg(feature = "arm")]
    ///Swap
    Swp { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    #[cfg(feature = "arm")]
    ///Swap Byte
    Swpb { cond: Cond, rd: Reg, rd2: Reg, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Sign Extend and Add Byte
    Sxtab { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Sign Extend to 16 bits and Add Byte
    Sxtab16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Sign Extend and Add Halfword
    Sxtah { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Sign Extend Byte
    Sxtb { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Sign Extend Byte to 16 bits
    Sxtb16 { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Sign Extend Halfword
    Sxth { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(feature = "arm")]
    ///Test Equivalence
    Teq { cond: Cond, rn: Reg, op2: Op2 },
    ///Test
    Tst { cond: Cond, rn: Reg, op2: Op2 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Add two 16-bit values
    Uadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Add four 8-bit values
    Uadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Add and Subtract with Exchange
    Uasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
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
    ///Undefined Permanently
    Udf { imm: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Add two 16-bit values
    Uhadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Add four 8-bit values
    Uhadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Add and Subtract with Exchange
    Uhasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Subtract and Add with Exchange
    Uhsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Subtract two 16-bit values
    Uhsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Halving Subtract four 8-bit values
    Uhsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Multiply Accumulate Accumulate Long
    Umaal { cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    #[cfg(feature = "arm")]
    ///Unsigned Multiply Accumulate Long
    Umlal { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    #[cfg(feature = "arm")]
    ///Unsigned Multiply Long
    Umull { s: bool, cond: Cond, rd_lo: Reg, rd_hi: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Add two 16-bit values
    Uqadd16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Add four 8-bit values
    Uqadd8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Add and Subtract with Exchange
    Uqasx { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Subtract and Add with Exchange
    Uqsax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Subtract two 16-bit values
    Uqsub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturating Subtract four 8-bit values
    Uqsub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Sum of Absolute Differences for four 8-bit values
    Usad8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Sum of Absolute Differences and Accumulate four 8-bit values
    Usada8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, ra: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturate
    Usat { cond: Cond, rd: Reg, imm: u32, op2: ShiftImm },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Saturate two 16-bit values
    Usat16 { cond: Cond, rd: Reg, imm: u32, rn: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Subtract and Add with Exchange
    Usax { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Subtract two 16-bit values
    Usub16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Subtract four 8-bit values
    Usub8 { cond: Cond, rd: Reg, rn: Reg, rm: Reg },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Extend and Add Byte
    Uxtab { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Extend to 16 bits and Add Byte
    Uxtab16 { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Extend and Add Halfword
    Uxtah { cond: Cond, rd: Reg, rn: Reg, rm: Reg, rotate: u32 },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Unsigned Extend Byte
    Uxtb { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
    ///Unsigned Extend Byte to 16 bits
    Uxtb16 { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(any(feature = "v6", feature = "v6k"))]
    ///Unsigned Extend Halfword
    Uxth { cond: Cond, rd: Reg, rm: Reg, rotate: u32 },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Absolute 32-bit
    VabsF32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Absolute 64-bit
    VabsF64 { cond: Cond, dd: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Add 32-bit
    VaddF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Add 64-bit
    VaddF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Compare 32-bit
    VcmpF32 { nan_exc: bool, cond: Cond, sd: Sreg, op2: VcmpF32Op2 },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Compare 64-bit
    VcmpF64 { nan_exc: bool, cond: Cond, dd: Dreg, op2: VcmpF64Op2 },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 32-bit from Floating-point 64-bit
    VcvtF32F64 { cond: Cond, sd: Sreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 32-bit from Signed 32-bit integer
    VcvtF32S32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 32-bit from Unsigned 32-bit integer
    VcvtF32U32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 64-bit from Floating-point 32-bit
    VcvtF64F32 { cond: Cond, dd: Dreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 64-bit from Signed 32-bit integer
    VcvtF64S32 { cond: Cond, dd: Dreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Floating-point 64-bit from Unsigned 32-bit integer
    VcvtF64U32 { cond: Cond, dd: Dreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Signed 32-bit integer from Floating-point 32-bit
    VcvtS32F32 { round_zero: bool, cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Signed 32-bit integer from Floating-point 64-bit
    VcvtS32F64 { round_zero: bool, cond: Cond, sd: Sreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Unsigned 32-bit integer from Floating-point 32-bit
    VcvtU32F32 { round_zero: bool, cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Convert to Unsigned 32-bit integer from Floating-point 64-bit
    VcvtU32F64 { round_zero: bool, cond: Cond, sd: Sreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Divide Floating-point 32-bit
    VdivF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Divide Floating-point 64-bit
    VdivF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Load Multiple Floating-point 32-bit
    VldmF32 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: SregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Load Multiple Floating-point 64-bit
    VldmF64 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: DregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Load Register Floating-point 32-bit
    VldrF32 { cond: Cond, sd: Sreg, addr: AddrLdrStr },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Load Register Floating-point 64-bit
    VldrF64 { cond: Cond, dd: Dreg, addr: AddrLdrStr },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Accumulate Floating-point 32-bit
    VmlaF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Accumulate Floating-point 64-bit
    VmlaF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Subtract Floating-point 32-bit
    VmlsF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Subtract Floating-point 64-bit
    VmlsF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to 32-bit scalar from register
    Vmov32Reg { cond: Cond, dd: DregIndex, rt: Reg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move Floating-point 32-bit
    VmovF32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to Floating-point 32-bit from register
    VmovF32Reg { cond: Cond, sn: Sreg, rt: Reg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move Floating-point 64-bit
    VmovF64 { cond: Cond, dd: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to register from 32-bit scalar
    VmovReg32 { cond: Cond, rt: Reg, dn: DregIndex },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to register from floating-point 32-bit
    VmovRegF32 { cond: Cond, rt: Reg, sn: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to two registers from two floating-point 32-bit
    VmovRegF32Dual { cond: Cond, rt: Reg, rt2: Reg, sm: Sreg, sm2: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to two floating-point 32-bit from two registers
    VmovF32RegDual { cond: Cond, sm: Sreg, sm2: Sreg, rt: Reg, rt2: Reg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to two registers from one floating-point 64-bit
    VmovRegF64 { cond: Cond, rt: Reg, rt2: Reg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to one floating-point 64-bit from two registers
    VmovF64Reg { cond: Cond, dm: Dreg, rt: Reg, rt2: Reg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to Register from Status register
    Vmrs { cond: Cond, rd: Reg, fpscr: Fpscr },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Move to Status register from Register
    Vmsr { cond: Cond, fpscr: Fpscr, rd: Reg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Floating-point 32-bit
    VmulF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Multiply Floating-point 64-bit
    VmulF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Floating-point 32-bit
    VnegF32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Floating-point 64-bit
    VnegF64 { cond: Cond, dd: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Accumulate Floating-point 32-bit
    VnmlaF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Accumulate Floating-point 64-bit
    VnmlaF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Subtract Floating-point 32-bit
    VnmlsF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Subtract Floating-point 64-bit
    VnmlsF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Floating-point 32-bit
    VnmulF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Negate Multiply Floating-point 64-bit
    VnmulF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Pop Floating-point 32-bit
    VpopF32 { cond: Cond, regs: SregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Pop Floating-point 64-bit
    VpopF64 { cond: Cond, regs: DregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Push Floating-point 32-bit
    VpushF32 { cond: Cond, regs: SregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Push Floating-point 64-bit
    VpushF64 { cond: Cond, regs: DregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Square Root Floating-point 32-bit
    VsqrtF32 { cond: Cond, sd: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Square Root Floating-point 64-bit
    VsqrtF64 { cond: Cond, dd: Dreg, dm: Dreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Store Multiple Floating-point 32-bit
    VstmF32 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: SregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Store Multiple Floating-point 64-bit
    VstmF64 { mode: VldmVstmMode, cond: Cond, rn: Reg, writeback: bool, regs: DregList },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Store Register Floating-point 32-bit
    VstrF32 { cond: Cond, sd: Sreg, addr: AddrLdrStr },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Store Register Floating-point 64-bit
    VstrF64 { cond: Cond, dd: Dreg, addr: AddrLdrStr },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Subtract 32-bit
    VsubF32 { cond: Cond, sd: Sreg, sn: Sreg, sm: Sreg },
    #[cfg(
        all(
            feature = "arm",
            feature = "vfp_v2",
            any(feature = "v5te", feature = "v5tej", feature = "v6", feature = "v6k")
        )
    )]
    ///Vector Subtract 64-bit
    VsubF64 { cond: Cond, dd: Dreg, dn: Dreg, dm: Dreg },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Wait For Event
    Wfe { cond: Cond },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Wait For Interrupt
    Wfi { cond: Cond },
    #[cfg(all(feature = "arm", feature = "v6k"))]
    ///Yield
    Yield { cond: Cond },
    Word(u32),
    HalfWord(u16),
    Byte(u8),
    Illegal,
}
impl Ins {
    ///Returns the condition code of this instruction.
    pub fn cond(&self) -> Cond {
        match self {
            Ins::Adc { cond, .. } => *cond,
            Ins::Add { cond, .. } => *cond,
            Ins::And { cond, .. } => *cond,
            Ins::Asr { cond, .. } => *cond,
            Ins::B { cond, .. } => *cond,
            Ins::Bic { cond, .. } => *cond,
            Ins::Bl { cond, .. } => *cond,
            #[cfg(
                any(
                    feature = "v5t",
                    feature = "v5te",
                    feature = "v5tej",
                    feature = "v6",
                    feature = "v6k"
                )
            )]
            Ins::Blx { cond, .. } => *cond,
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
            Ins::Bx { cond, .. } => *cond,
            #[cfg(
                all(
                    feature = "arm",
                    any(feature = "v5tej", feature = "v6", feature = "v6k")
                )
            )]
            Ins::Bxj { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Cdp { cond, .. } => *cond,
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
            Ins::Clz { cond, .. } => *cond,
            Ins::Cmn { cond, .. } => *cond,
            Ins::Cmp { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Csdb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Dbg { cond, .. } => *cond,
            Ins::Eor { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Ldc { cond, .. } => *cond,
            Ins::Ldm { cond, .. } => *cond,
            Ins::Ldr { cond, .. } => *cond,
            Ins::Ldrb { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Ldrbt { cond, .. } => *cond,
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
            Ins::Ldrd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ldrex { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexh { cond, .. } => *cond,
            Ins::Ldrh { cond, .. } => *cond,
            Ins::Ldrsb { cond, .. } => *cond,
            Ins::Ldrsh { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Ldrt { cond, .. } => *cond,
            Ins::Lsl { cond, .. } => *cond,
            Ins::Lsr { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Mcr { cond, .. } => *cond,
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
            Ins::Mcrr { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Mla { cond, .. } => *cond,
            Ins::Mov { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Mrc { cond, .. } => *cond,
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
            Ins::Mrrc { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Mrs { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Msr { cond, .. } => *cond,
            Ins::Mul { cond, .. } => *cond,
            Ins::Mvn { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Nop { cond, .. } => *cond,
            Ins::Orr { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhbt { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhtb { cond, .. } => *cond,
            Ins::Pop { cond, .. } => *cond,
            Ins::Push { cond, .. } => *cond,
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
            Ins::Qadd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qasx { cond, .. } => *cond,
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
            Ins::Qdadd { cond, .. } => *cond,
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
            Ins::Qdsub { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsax { cond, .. } => *cond,
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
            Ins::Qsub { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub8 { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev16 { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Revsh { cond, .. } => *cond,
            Ins::Ror { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Rrx { cond, .. } => *cond,
            Ins::Rsb { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Rsc { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sasx { cond, .. } => *cond,
            Ins::Sbc { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sel { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Sev { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shasx { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsax { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub8 { cond, .. } => *cond,
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
            Ins::Smla { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlad { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Smlal { cond, .. } => *cond,
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
            Ins::SmlalHalf { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlald { cond, .. } => *cond,
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
            Ins::Smlaw { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsld { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmla { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmls { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmul { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smuad { cond, .. } => *cond,
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
            Ins::Smul { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Smull { cond, .. } => *cond,
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
            Ins::Smulw { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smusd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssax { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub8 { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Stc { cond, .. } => *cond,
            Ins::Stm { cond, .. } => *cond,
            Ins::Str { cond, .. } => *cond,
            Ins::Strb { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Strbt { cond, .. } => *cond,
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
            Ins::Strd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Strex { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexd { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexh { cond, .. } => *cond,
            Ins::Strh { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Strt { cond, .. } => *cond,
            Ins::Sub { cond, .. } => *cond,
            Ins::Svc { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Swp { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Swpb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtah { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxtb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtb16 { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxth { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Teq { cond, .. } => *cond,
            Ins::Tst { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uasx { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhasx { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsax { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Umaal { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Umlal { cond, .. } => *cond,
            #[cfg(feature = "arm")]
            Ins::Umull { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqasx { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsax { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usad8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usada8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usax { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub8 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab16 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtah { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxtb { cond, .. } => *cond,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtb16 { cond, .. } => *cond,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxth { cond, .. } => *cond,
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
            Ins::VabsF32 { cond, .. } => *cond,
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
            Ins::VabsF64 { cond, .. } => *cond,
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
            Ins::VaddF32 { cond, .. } => *cond,
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
            Ins::VaddF64 { cond, .. } => *cond,
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
            Ins::VcmpF32 { cond, .. } => *cond,
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
            Ins::VcmpF64 { cond, .. } => *cond,
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
            Ins::VcvtF32F64 { cond, .. } => *cond,
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
            Ins::VcvtF32S32 { cond, .. } => *cond,
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
            Ins::VcvtF32U32 { cond, .. } => *cond,
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
            Ins::VcvtF64F32 { cond, .. } => *cond,
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
            Ins::VcvtF64S32 { cond, .. } => *cond,
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
            Ins::VcvtF64U32 { cond, .. } => *cond,
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
            Ins::VcvtS32F32 { cond, .. } => *cond,
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
            Ins::VcvtS32F64 { cond, .. } => *cond,
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
            Ins::VcvtU32F32 { cond, .. } => *cond,
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
            Ins::VcvtU32F64 { cond, .. } => *cond,
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
            Ins::VdivF32 { cond, .. } => *cond,
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
            Ins::VdivF64 { cond, .. } => *cond,
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
            Ins::VldmF32 { cond, .. } => *cond,
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
            Ins::VldmF64 { cond, .. } => *cond,
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
            Ins::VldrF32 { cond, .. } => *cond,
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
            Ins::VldrF64 { cond, .. } => *cond,
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
            Ins::VmlaF32 { cond, .. } => *cond,
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
            Ins::VmlaF64 { cond, .. } => *cond,
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
            Ins::VmlsF32 { cond, .. } => *cond,
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
            Ins::VmlsF64 { cond, .. } => *cond,
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
            Ins::Vmov32Reg { cond, .. } => *cond,
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
            Ins::VmovF32 { cond, .. } => *cond,
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
            Ins::VmovF32Reg { cond, .. } => *cond,
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
            Ins::VmovF64 { cond, .. } => *cond,
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
            Ins::VmovReg32 { cond, .. } => *cond,
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
            Ins::VmovRegF32 { cond, .. } => *cond,
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
            Ins::VmovRegF32Dual { cond, .. } => *cond,
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
            Ins::VmovF32RegDual { cond, .. } => *cond,
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
            Ins::VmovRegF64 { cond, .. } => *cond,
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
            Ins::VmovF64Reg { cond, .. } => *cond,
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
            Ins::Vmrs { cond, .. } => *cond,
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
            Ins::Vmsr { cond, .. } => *cond,
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
            Ins::VmulF32 { cond, .. } => *cond,
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
            Ins::VmulF64 { cond, .. } => *cond,
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
            Ins::VnegF32 { cond, .. } => *cond,
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
            Ins::VnegF64 { cond, .. } => *cond,
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
            Ins::VnmlaF32 { cond, .. } => *cond,
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
            Ins::VnmlaF64 { cond, .. } => *cond,
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
            Ins::VnmlsF32 { cond, .. } => *cond,
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
            Ins::VnmlsF64 { cond, .. } => *cond,
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
            Ins::VnmulF32 { cond, .. } => *cond,
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
            Ins::VnmulF64 { cond, .. } => *cond,
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
            Ins::VpopF32 { cond, .. } => *cond,
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
            Ins::VpopF64 { cond, .. } => *cond,
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
            Ins::VpushF32 { cond, .. } => *cond,
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
            Ins::VpushF64 { cond, .. } => *cond,
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
            Ins::VsqrtF32 { cond, .. } => *cond,
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
            Ins::VsqrtF64 { cond, .. } => *cond,
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
            Ins::VstmF32 { cond, .. } => *cond,
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
            Ins::VstmF64 { cond, .. } => *cond,
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
            Ins::VstrF32 { cond, .. } => *cond,
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
            Ins::VstrF64 { cond, .. } => *cond,
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
            Ins::VsubF32 { cond, .. } => *cond,
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
            Ins::VsubF64 { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Wfe { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Wfi { cond, .. } => *cond,
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Yield { cond, .. } => *cond,
            _ => Cond::default(),
        }
    }
}
impl Ins {
    ///Returns the S flag of this instruction.
    pub fn s(&self) -> bool {
        match self {
            Ins::Adc { s, .. } => *s,
            Ins::Add { s, .. } => *s,
            Ins::And { s, .. } => *s,
            Ins::Asr { s, .. } => *s,
            Ins::Bic { s, .. } => *s,
            Ins::Eor { s, .. } => *s,
            Ins::Lsl { s, .. } => *s,
            Ins::Lsr { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Mla { s, .. } => *s,
            Ins::Mov { s, .. } => *s,
            Ins::Mul { s, .. } => *s,
            Ins::Mvn { s, .. } => *s,
            Ins::Orr { s, .. } => *s,
            Ins::Ror { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Rrx { s, .. } => *s,
            Ins::Rsb { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Rsc { s, .. } => *s,
            Ins::Sbc { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Smlal { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Smull { s, .. } => *s,
            Ins::Sub { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Umlal { s, .. } => *s,
            #[cfg(feature = "arm")]
            Ins::Umull { s, .. } => *s,
            _ => false,
        }
    }
}
#[cfg(feature = "arm")]
impl AddrLdcStc {
    ///Returns the "rn" source register of this instruction.
    pub fn rn(&self) -> Reg {
        match self {
            Self::Pre { rn, .. } => *rn,
            Self::Post { rn, .. } => *rn,
            Self::Unidx { rn, .. } => *rn,
        }
    }
}
impl AddrLdrStr {
    ///Returns the "rn" source register of this instruction.
    pub fn rn(&self) -> Reg {
        match self {
            Self::Pre { rn, .. } => *rn,
            Self::Post(addr_ldr_str_post) => addr_ldr_str_post.rn(),
        }
    }
}
impl AddrLdrStrPost {
    ///Returns the "rn" source register of this instruction.
    pub fn rn(&self) -> Reg {
        self.rn
    }
}
impl AddrMiscLoad {
    ///Returns the "rn" source register of this instruction.
    pub fn rn(&self) -> Reg {
        match self {
            Self::Pre { rn, .. } => *rn,
            Self::Post { rn, .. } => *rn,
        }
    }
}
impl Ins {
    ///Returns the "rn" source register of this instruction.
    pub fn rn(&self) -> Option<Reg> {
        match self {
            Ins::Adc { rn, .. } => Some(*rn),
            Ins::Add { rn, .. } => Some(*rn),
            Ins::And { rn, .. } => Some(*rn),
            Ins::Asr { rn, .. } => Some(*rn),
            Ins::Bic { rn, .. } => Some(*rn),
            Ins::Cmn { rn, .. } => Some(*rn),
            Ins::Cmp { rn, .. } => Some(*rn),
            Ins::Eor { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Ldc { dest, .. } => Some(dest.rn()),
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
            Ins::Ldc2 { dest, .. } => Some(dest.rn()),
            Ins::Ldm { rn, .. } => Some(*rn),
            Ins::Ldr { addr, .. } => Some(addr.rn()),
            Ins::Ldrb { addr, .. } => Some(addr.rn()),
            #[cfg(feature = "arm")]
            Ins::Ldrbt { addr, .. } => Some(addr.rn()),
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
            Ins::Ldrd { addr, .. } => Some(addr.rn()),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ldrex { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexb { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexd { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Ldrexh { rn, .. } => Some(*rn),
            Ins::Ldrh { addr, .. } => Some(addr.rn()),
            Ins::Ldrsb { addr, .. } => Some(addr.rn()),
            Ins::Ldrsh { addr, .. } => Some(addr.rn()),
            #[cfg(feature = "arm")]
            Ins::Ldrt { addr, .. } => Some(addr.rn()),
            Ins::Lsl { rn, .. } => Some(*rn),
            Ins::Lsr { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Mla { rn, .. } => Some(*rn),
            Ins::Mul { rn, .. } => Some(*rn),
            Ins::Orr { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhbt { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhtb { rn, .. } => Some(*rn),
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
            Ins::Pld { addr, .. } => Some(addr.rn()),
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
            Ins::Qadd { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qasx { rn, .. } => Some(*rn),
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
            Ins::Qdadd { rn, .. } => Some(*rn),
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
            Ins::Qdsub { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsax { rn, .. } => Some(*rn),
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
            Ins::Qsub { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Rfe { rn, .. } => Some(*rn),
            Ins::Ror { rn, .. } => Some(*rn),
            Ins::Rsb { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Rsc { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sasx { rn, .. } => Some(*rn),
            Ins::Sbc { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sel { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shasx { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsax { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub8 { rn, .. } => Some(*rn),
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
            Ins::Smla { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlad { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Smlal { rn, .. } => Some(*rn),
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
            Ins::SmlalHalf { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlald { rn, .. } => Some(*rn),
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
            Ins::Smlaw { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsd { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsld { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmla { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmls { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmul { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smuad { rn, .. } => Some(*rn),
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
            Ins::Smul { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Smull { rn, .. } => Some(*rn),
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
            Ins::Smulw { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smusd { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Srs { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssax { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub8 { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Stc { dest, .. } => Some(dest.rn()),
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
            Ins::Stc2 { dest, .. } => Some(dest.rn()),
            Ins::Stm { rn, .. } => Some(*rn),
            Ins::Str { addr, .. } => Some(addr.rn()),
            Ins::Strb { addr, .. } => Some(addr.rn()),
            #[cfg(feature = "arm")]
            Ins::Strbt { addr, .. } => Some(addr.rn()),
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
            Ins::Strd { addr, .. } => Some(addr.rn()),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Strex { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexb { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexd { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", feature = "v6k"))]
            Ins::Strexh { rn, .. } => Some(*rn),
            Ins::Strh { addr, .. } => Some(addr.rn()),
            #[cfg(feature = "arm")]
            Ins::Strt { addr, .. } => Some(addr.rn()),
            Ins::Sub { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Swp { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Swpb { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtah { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Teq { rn, .. } => Some(*rn),
            Ins::Tst { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uasx { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhasx { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsax { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Umaal { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Umlal { rn, .. } => Some(*rn),
            #[cfg(feature = "arm")]
            Ins::Umull { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqasx { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsax { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usad8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usada8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usax { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub8 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab16 { rn, .. } => Some(*rn),
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtah { rn, .. } => Some(*rn),
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
            Ins::VldmF32 { rn, .. } => Some(*rn),
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
            Ins::VldmF64 { rn, .. } => Some(*rn),
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
            Ins::VldrF32 { addr, .. } => Some(addr.rn()),
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
            Ins::VldrF64 { addr, .. } => Some(addr.rn()),
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
            Ins::VstmF32 { rn, .. } => Some(*rn),
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
            Ins::VstmF64 { rn, .. } => Some(*rn),
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
            Ins::VstrF32 { addr, .. } => Some(addr.rn()),
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
            Ins::VstrF64 { addr, .. } => Some(addr.rn()),
            _ => None,
        }
    }
}
impl Ins {
    ///Returns whether this instruction is a comparison. Does not include VCMP.
    pub fn is_comparison(&self) -> bool {
        match self {
            Ins::Cmn { .. } => true,
            Ins::Cmp { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Teq { .. } => true,
            Ins::Tst { .. } => true,
            _ => false,
        }
    }
    ///Returns whether this instruction is a data operation storing its result in a register. Does not include VFP instructions.
    pub fn is_data_operation(&self) -> bool {
        match self {
            Ins::Adc { .. } => true,
            Ins::Add { .. } => true,
            Ins::And { .. } => true,
            Ins::Asr { .. } => true,
            Ins::Bic { .. } => true,
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
            Ins::Clz { .. } => true,
            Ins::Eor { .. } => true,
            Ins::Lsl { .. } => true,
            Ins::Lsr { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Mla { .. } => true,
            Ins::Mov { .. } => true,
            Ins::Mul { .. } => true,
            Ins::Mvn { .. } => true,
            #[cfg(feature = "thumb")]
            Ins::Neg { .. } => true,
            Ins::Orr { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhbt { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Pkhtb { .. } => true,
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
            Ins::Qadd { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qasx { .. } => true,
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
            Ins::Qdadd { .. } => true,
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
            Ins::Qdsub { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsax { .. } => true,
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
            Ins::Qsub { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Qsub8 { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Rev16 { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Revsh { .. } => true,
            Ins::Ror { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Rrx { .. } => true,
            Ins::Rsb { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Rsc { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sasx { .. } => true,
            Ins::Sbc { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sel { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shasx { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsax { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Shsub8 { .. } => true,
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
            Ins::Smla { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlad { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Smlal { .. } => true,
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
            Ins::SmlalHalf { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlald { .. } => true,
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
            Ins::Smlaw { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsd { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smlsld { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmla { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmls { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smmul { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smuad { .. } => true,
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
            Ins::Smul { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Smull { .. } => true,
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
            Ins::Smulw { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Smusd { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssat16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssax { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Ssub8 { .. } => true,
            Ins::Sub { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtab16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtah { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxtb { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Sxtb16 { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Sxth { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uasx { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhasx { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsax { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uhsub8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Umaal { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Umlal { .. } => true,
            #[cfg(feature = "arm")]
            Ins::Umull { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqadd8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqasx { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsax { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uqsub8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usad8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usada8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usat16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usax { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Usub8 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtab16 { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtah { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxtb { .. } => true,
            #[cfg(all(feature = "arm", any(feature = "v6", feature = "v6k")))]
            Ins::Uxtb16 { .. } => true,
            #[cfg(any(feature = "v6", feature = "v6k"))]
            Ins::Uxth { .. } => true,
            _ => false,
        }
    }
}

use core::fmt::{self, Display, Formatter};

use crate::{
    args::{
        Argument, CoReg, CpsrFlags, CpsrMode, Endian, OffsetImm, OffsetReg, Reg, Register, Shift, ShiftImm, ShiftReg,
        StatusMask, StatusReg,
    },
    parse::ParsedIns,
};

impl ParsedIns {
    pub fn display(&self, options: DisplayOptions) -> ParsedInsDisplay<'_> {
        ParsedInsDisplay {
            ins: self,
            options,
            symbols: None,
        }
    }

    pub fn display_with_symbols<'a>(&'a self, options: DisplayOptions, symbols: Symbols<'a>) -> ParsedInsDisplay<'a> {
        ParsedInsDisplay {
            ins: self,
            options,
            symbols: Some(symbols),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DisplayOptions {
    pub reg_names: RegNames,
}

pub trait LookupSymbol {
    fn lookup_symbol_name(&self, source: u32, destination: u32) -> Option<&str>;
}

#[derive(Clone, Copy)]
pub struct Symbols<'a> {
    pub lookup: &'a dyn LookupSymbol,
    pub program_counter: u32,
    pub pc_load_offset: i32,
}

pub struct ParsedInsDisplay<'a> {
    ins: &'a ParsedIns,
    options: DisplayOptions,
    symbols: Option<Symbols<'a>>,
}

impl<'a> ParsedInsDisplay<'a> {
    fn write_arg<I>(&self, f: &mut Formatter<'_>, iter: &mut I, arg: &Argument) -> fmt::Result
    where
        I: Iterator<Item = &'a Argument>,
    {
        match arg {
            Argument::Reg(Reg {
                deref: true,
                reg,
                writeback,
            }) => {
                let next = iter.next();

                // replace PC-relative dereferences with symbol name
                if let Some(symbols) = self.symbols {
                    if *reg == Register::Pc {
                        if let Some(Argument::OffsetImm(OffsetImm {
                            post_indexed: false,
                            value,
                        })) = next
                        {
                            let destination = (symbols.program_counter as i32 + value + symbols.pc_load_offset) as u32 & !3;
                            if let Some(name) = symbols.lookup.lookup_symbol_name(symbols.program_counter, destination) {
                                return write!(f, "{name}");
                            }
                        }
                    }
                }

                // display reg instead of arg to avoid writing "!" too soon
                write!(f, "[{}", reg.display(self.options.reg_names))?;

                if let Some(next) = next {
                    if next.ends_deref() {
                        return write!(f, "], {}", next.display(self.options, self.symbols));
                    } else {
                        write!(f, ", {}", next.display(self.options, self.symbols))?;
                        for more in iter {
                            write!(f, ", {}", more.display(self.options, self.symbols))?;
                        }
                    }
                }

                write!(f, "]")?;
                if *writeback {
                    write!(f, "!")?;
                }
            }
            _ => {
                write!(f, "{}", arg.display(self.options, self.symbols))?;
            }
        }
        Ok(())
    }
}

impl Display for ParsedInsDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ins.mnemonic)?;
        let mut iter = self.ins.args_iter();
        if let Some(arg) = iter.next() {
            write!(f, " ")?;
            self.write_arg(f, &mut iter, arg)?;
        }
        while let Some(arg) = iter.next() {
            write!(f, ", ")?;
            self.write_arg(f, &mut iter, arg)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SignedHex(pub i32);

impl Display for SignedHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#")?;
        if self.0.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x{:x}", self.0.abs())
    }
}

impl Argument {
    fn ends_deref(&self) -> bool {
        matches!(
            self,
            Argument::OffsetImm(OffsetImm {
                post_indexed: true,
                value: _,
            }) | Argument::OffsetReg(OffsetReg {
                add: _,
                post_indexed: true,
                reg: _,
            }) | Argument::CoOption(_)
        )
    }

    pub fn display<'a>(&'a self, options: DisplayOptions, symbols: Option<Symbols<'a>>) -> DisplayArgument<'a> {
        DisplayArgument {
            arg: self,
            options,
            symbols,
        }
    }
}

pub struct DisplayArgument<'a> {
    arg: &'a Argument,
    options: DisplayOptions,
    symbols: Option<Symbols<'a>>,
}

impl Display for DisplayArgument<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.arg {
            Argument::None => Ok(()),
            Argument::Reg(reg) => {
                write!(f, "{}", reg.reg.display(self.options.reg_names))?;
                if reg.writeback {
                    write!(f, "!")?;
                }
                Ok(())
            }
            Argument::RegList(list) => {
                write!(f, "{{")?;
                let mut first = true;
                for i in 0..16 {
                    if (list.regs & (1 << i)) != 0 {
                        if !first {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", Register::parse(i).display(self.options.reg_names))?;
                        first = false;
                    }
                }
                write!(f, "}}")?;
                if list.user_mode {
                    write!(f, "^")?;
                }
                Ok(())
            }
            Argument::CoReg(x) => write!(f, "{}", x),
            Argument::StatusReg(x) => write!(f, "{}", x),
            Argument::UImm(x) => write!(f, "#0x{:x}", x),
            Argument::SImm(x) => write!(f, "{}", SignedHex(*x)),
            Argument::OffsetImm(x) => write!(f, "{}", SignedHex(x.value)),
            Argument::CoOption(x) => write!(f, "{{0x{:x}}}", x),
            Argument::CoOpcode(x) => write!(f, "#{}", x),
            Argument::CoprocNum(x) => write!(f, "p{}", x),
            Argument::ShiftImm(x) => write!(f, "{}", x),
            Argument::ShiftReg(x) => write!(f, "{}", x.display(self.options.reg_names)),
            Argument::OffsetReg(x) => write!(f, "{}", x.display(self.options.reg_names)),
            Argument::BranchDest(dest) => {
                if let Some(symbols) = &self.symbols {
                    let destination = ((symbols.program_counter as i32) + *dest) as u32 & !1;
                    if let Some(name) = symbols.lookup.lookup_symbol_name(symbols.program_counter, destination) {
                        return write!(f, "{name}");
                    }
                }
                write!(f, "{}", SignedHex(*dest))
            }
            Argument::StatusMask(x) => write!(f, "{}", x),
            Argument::Shift(x) => write!(f, "{}", x),
            Argument::SatImm(x) => write!(f, "#0x{:x}", x),
            Argument::CpsrMode(x) => write!(f, "{}", x),
            Argument::CpsrFlags(x) => write!(f, "{}", x),
            Argument::Endian(x) => write!(f, "{}", x),
        }
    }
}

/// How R9 should be displayed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum R9Use {
    /// R9 or V6.
    #[default]
    GeneralPurpose,
    /// Position-independent data (PID). If true, R9 will display as SB (static base).
    Pid,
    /// Thread-local storage (TLS). If true, R9 will display as TR (TLS register).
    Tls,
}

/// Customizes the display of register names.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RegNames {
    /// If true, R0-R3 and R4-R11 will display as A1-A4 and V1-V8.
    pub av_registers: bool,
    /// How R9 should be displayed.
    pub r9_use: R9Use,
    /// If true, R10 will display as SL (stack limit).
    pub explicit_stack_limit: bool,
    /// If true, R11 will display as FP (frame pointer).
    pub frame_pointer: bool,
    /// If true, R12 will display as IP (intra procedure call scratch register). Used for interworking and long branches.
    pub ip: bool,
}

impl Register {
    pub fn display(self, names: RegNames) -> RegDisplay {
        RegDisplay(self, names)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RegDisplay(Register, RegNames);

impl RegDisplay {
    pub fn as_str(self) -> &'static str {
        #[rustfmt::skip]
        let str = match self.0 {
            Register::Illegal => "<illegal>",
            Register::R0 => if self.1.av_registers { "a1" } else { "r0" },
            Register::R1 => if self.1.av_registers { "a2" } else { "r1" },
            Register::R2 => if self.1.av_registers { "a3" } else { "r2" },
            Register::R3 => if self.1.av_registers { "a4" } else { "r3" },
            Register::R4 => if self.1.av_registers { "v1" } else { "r4" },
            Register::R5 => if self.1.av_registers { "v2" } else { "r5" },
            Register::R6 => if self.1.av_registers { "v3" } else { "r6" },
            Register::R7 => if self.1.av_registers { "v4" } else { "r7" },
            Register::R8 => if self.1.av_registers { "v5" } else { "r8" },
            Register::R9 => match self.1.r9_use {
                R9Use::GeneralPurpose => if self.1.av_registers { "v6" } else { "r9" },
                R9Use::Pid => "sb",
                R9Use::Tls => "tr",
            },
            Register::R10 => if self.1.explicit_stack_limit { "sl" } else if self.1.av_registers { "v7" } else { "r10" },
            Register::R11 => if self.1.frame_pointer { "fp" } else if self.1.av_registers { "v8" } else { "r11" },
            Register::R12 => if self.1.ip { "ip" } else { "r12" },
            Register::Sp => "sp",
            Register::Lr => "lr",
            Register::Pc => "pc",
        };
        str
    }
}

impl Display for RegDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl CoReg {
    pub fn as_str(self) -> &'static str {
        match self {
            CoReg::Illegal => "<illegal>",
            CoReg::C0 => "c0",
            CoReg::C1 => "c1",
            CoReg::C2 => "c2",
            CoReg::C3 => "c3",
            CoReg::C4 => "c4",
            CoReg::C5 => "c5",
            CoReg::C6 => "c6",
            CoReg::C7 => "c7",
            CoReg::C8 => "c8",
            CoReg::C9 => "c9",
            CoReg::C10 => "c10",
            CoReg::C11 => "c11",
            CoReg::C12 => "c12",
            CoReg::C13 => "c13",
            CoReg::C14 => "c14",
            CoReg::C15 => "c15",
        }
    }
}

impl Display for CoReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StatusReg {
    pub fn as_str(self) -> &'static str {
        match self {
            StatusReg::Illegal => "<illegal>",
            StatusReg::Cpsr => "cpsr",
            StatusReg::Spsr => "spsr",
        }
    }
}

impl Display for StatusReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for StatusMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reg)?;
        if self.flags || self.status || self.extension || self.control {
            write!(f, "_")?;
        }
        if self.flags {
            write!(f, "f")?;
        }
        if self.status {
            write!(f, "s")?;
        }
        if self.extension {
            write!(f, "x")?;
        }
        if self.control {
            write!(f, "c")?;
        }
        Ok(())
    }
}

impl Shift {
    pub fn as_str(self) -> &'static str {
        match self {
            Shift::Illegal => "<illegal>",
            Shift::Lsl => "lsl",
            Shift::Lsr => "lsr",
            Shift::Asr => "asr",
            Shift::Ror => "ror",
            Shift::Rrx => "rrx",
        }
    }
}

impl Display for Shift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for ShiftImm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} #0x{:x}", self.op, self.imm)
    }
}

impl ShiftReg {
    pub fn display(self, names: RegNames) -> DisplayShiftReg {
        DisplayShiftReg(self, names)
    }
}

pub struct DisplayShiftReg(ShiftReg, RegNames);

impl Display for DisplayShiftReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0.op, self.0.reg.display(self.1))
    }
}

impl OffsetReg {
    pub fn display(self, names: RegNames) -> DisplayOffsetReg {
        DisplayOffsetReg(self, names)
    }
}

pub struct DisplayOffsetReg(OffsetReg, RegNames);

impl Display for DisplayOffsetReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.0.add {
            write!(f, "-")?;
        }
        write!(f, "{}", self.0.reg.display(self.1))
    }
}

impl Display for CpsrMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#0x{:x}", self.mode)?;
        if self.writeback {
            write!(f, "!")?;
        }
        Ok(())
    }
}

impl Display for CpsrFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.a {
            write!(f, "a")?;
        }
        if self.i {
            write!(f, "i")?;
        }
        if self.f {
            write!(f, "f")?;
        }
        if !self.a && !self.i && !self.f {
            write!(f, "none")?;
        }
        Ok(())
    }
}

impl Endian {
    pub fn as_str(self) -> &'static str {
        match self {
            Endian::Illegal => "<illegal>",
            Endian::Le => "le",
            Endian::Be => "be",
        }
    }
}

impl Display for Endian {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

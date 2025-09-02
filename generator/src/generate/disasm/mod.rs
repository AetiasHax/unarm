use proc_macro2::TokenStream;
use quote::quote;

use crate::isa::Isa;

pub struct DisassemblerGenerator {
    pub isa: Isa,
}

impl DisassemblerGenerator {
    pub fn generate(&self) -> TokenStream {
        let options_struct = self.isa.options().struct_tokens();
        let version_enum = self.isa.versions().enum_tokens();
        let internal_option_types = self.isa.options().internal_types_tokens();

        let data_types = self.isa.types().types_tokens();
        let data_parse_impls = self.isa.types().parse_impls_tokens();
        let data_default_impls = self.isa.types().default_impls_tokens();
        let data_display_impls = self.isa.types().display_impls_tokens(&self.isa);

        let ins_enum = self.isa.opcodes().ins_enum_tokens(&self.isa);
        let opcode_parse_fns = self.isa.opcodes().parse_fns_tokens(&self.isa);

        quote! {
            #options_struct
            #version_enum
            #internal_option_types

            #data_types
            #data_parse_impls
            #data_default_impls
            #data_display_impls

            #ins_enum
            #opcode_parse_fns
        }
    }
}

pub struct Options {
    av: bool,
    r9_use: R9Use,
    sl: bool,
    fp: bool,
    ip: bool,
    ual: bool,
}

pub enum R9Use {
    R9,
    Sb,
    Tr,
}

pub enum Cond {
    Eq,
    Al,
    // ...
}

impl From<u32> for Cond {
    fn from(value: u32) -> Self {
        match value {
            0x0 => Self::Eq,
            0xe => Self::Al,
            // ...
            _ => panic!(),
        }
    }
}

impl Default for Cond {
    fn default() -> Self {
        Self::Al
    }
}

impl Cond {
    pub fn display(&self, options: &Options, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cond::Eq => f.write_str("eq"),
            Cond::Al => Ok(()),
        }
    }
}

pub enum Reg {
    R0,
    // ...
}

impl From<u32> for Reg {
    fn from(value: u32) -> Self {
        match value {
            0 => Reg::R0,
            // ...
            _ => panic!(),
        }
    }
}

impl Reg {
    pub fn display(&self, options: &Options, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::R0 => {
                if options.av {
                    f.write_str("a1")
                } else {
                    f.write_str("r0")
                }
            }
        }
    }
}

pub enum Op2 {
    Imm(u32),
    ShiftReg { rm: Reg, shift_op: ShiftOp, rs: Reg },
    ShiftImm { rm: Reg, shift_op: ShiftOp, imm: u32 },
}

impl From<u32> for Op2 {
    fn from(value: u32) -> Self {
        if (value & 0x02000000) == 0x02000000 {
            Self::Imm(value & 0xfff)
        } else if (value & 0x02000010) == 0x00000010 {
            Self::ShiftReg {
                rm: Reg::from(value & 0xf),
                shift_op: ShiftOp::from((value >> 5) & 0x3),
                rs: Reg::from((value >> 4) & 0xf),
            }
        } else {
            Self::ShiftImm {
                rm: Reg::from(value & 0xf),
                shift_op: ShiftOp::from((value >> 5) & 0x3),
                imm: (value >> 3) & 0x1f,
            }
        }
    }
}

impl Op2 {
    pub fn display(&self, options: &Options, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op2::Imm(imm) => {
                f.write_str("#")?;
                write!(f, "{imm:#x}")?;
                Ok(())
            }
            Op2::ShiftReg { rm, shift_op, rs } => {
                rm.display(options, f)?;
                f.write_str(", ")?;
                shift_op.display(options, f)?;
                f.write_str(" ")?;
                rs.display(options, f)?;
                Ok(())
            }
            Op2::ShiftImm { rm, shift_op, imm } => {
                if *imm == 0 && *shift_op == ShiftOp::Lsl {
                    rm.display(options, f)?;
                    Ok(())
                } else {
                    rm.display(options, f)?;
                    f.write_str(", ")?;
                    shift_op.display(options, f)?;
                    f.write_str(" ")?;
                    write!(f, "{imm:#x}")?;
                    Ok(())
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ShiftOp {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl From<u32> for ShiftOp {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Lsl,
            1 => Self::Lsr,
            2 => Self::Asr,
            3 => Self::Ror,
            _ => panic!(),
        }
    }
}

impl Default for ShiftOp {
    fn default() -> Self {
        Self::Lsl
    }
}

impl ShiftOp {
    pub fn display(&self, options: &Options, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShiftOp::Lsl => {
                f.write_str("lsl")?;
                Ok(())
            }
            ShiftOp::Lsr => {
                f.write_str("lsr")?;
                Ok(())
            }
            ShiftOp::Asr => {
                f.write_str("asr")?;
                Ok(())
            }
            ShiftOp::Ror => {
                f.write_str("ror")?;
                Ok(())
            }
        }
    }
}

pub enum Ins {
    Adc { s: bool, cond: Cond, rd: Reg, rn: Reg, op2: Op2 },
}

impl Ins {
    pub fn display(&self, options: &Options, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ins::Adc { s, cond, rd, rn, op2 } => {
                f.write_str("adc")?;
                if *s {
                    f.write_str("s")?;
                }
                cond.display(options, f)?;
                rd.display(options, f)?;
                f.write_str(", ")?;
                rn.display(options, f)?;
                f.write_str(", ")?;
                op2.display(options, f)?;
                Ok(())
            }
        }
    }
}

fn parse_arm_adc_0(ins: u32) -> Ins {
    let s = ((ins >> 20) & 1) != 0;
    let cond = Cond::from((ins >> 28) & 0xf);
    let rd = Reg::from((ins >> 12) & 0xf);
    let rn = Reg::from((ins >> 16) & 0xf);
    let op2 = Op2::from(ins);
    Ins::Adc { s, cond, rd, rn, op2 }
}

fn parse_thumb_adc_0(ins: u32) -> Ins {
    let s = false;
    let cond = Cond::default();
    let rd = Reg::from(ins & 0x7);
    let rn = Reg::from(ins & 0x7);
    let op2 = Op2::ShiftImm {
        rm: Reg::from((ins >> 3) & 0x7),
        shift_op: ShiftOp::default(),
        imm: u32::default(),
    };
    Ins::Adc { s, cond, rd, rn, op2 }
}

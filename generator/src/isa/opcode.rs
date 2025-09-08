use std::{cmp::Reverse, collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{
        Arch, BitRange, DataExpr, DataType, DataTypeEnumVariantName, DataTypeKind, DataTypeName,
        Format, FormatParams, Isa, IsaVersionPattern, OpcodePattern,
    },
    util::{hex_literal::HexLiteral, str::capitalize},
};

#[derive(Deserialize, Debug)]
pub struct Opcodes(Vec<Opcode>);

impl Opcodes {
    pub fn iter(&self) -> impl Iterator<Item = &Opcode> {
        self.0.iter()
    }

    pub fn ins_enum_tokens(&self, isa: &Isa) -> TokenStream {
        let opcodes = self.iter().map(|o| o.ins_variant_tokens(isa));
        quote! {
            pub enum Ins {
                #(#opcodes),*,
                Illegal,
            }
        }
    }

    pub fn parse_fns_tokens(&self, isa: &Isa) -> TokenStream {
        let parse_fns = self.iter().map(|o| o.parse_fns_tokens(isa));
        quote!(#(#parse_fns)*)
    }

    pub fn write_impl_tokens(&self, isa: &Isa) -> TokenStream {
        let opcodes = self.0.iter().map(|o| o.write_opcode_tokens(isa));
        let params = self.0.iter().map(|o| o.write_params_tokens(isa));
        quote! {
            impl Ins {
                pub fn write_opcode<F>(&self, formatter: &mut F) -> core::fmt::Result
                where
                    F: Write + ?Sized
                {
                    let options = formatter.options();
                    match self {
                        #(#opcodes)*
                        Ins::Illegal => {
                            formatter.write_str("<illegal>")?;
                        }
                    }
                    Ok(())
                }

                pub fn write_params<F>(&self, formatter: &mut F) -> core::fmt::Result
                where
                    F: Write + ?Sized
                {
                    let options = formatter.options();
                    match self {
                        #(#params)*
                        Ins::Illegal => {
                            formatter.write_str("<illegal>")?;
                        }
                    }
                    Ok(())
                }
            }
        }
    }

    pub fn parse_arm_ifchain_fn_tokens(&self) -> TokenStream {
        let mut encodings = self
            .0
            .iter()
            .flat_map(|o| o.parse_arm_ifchain_tokens())
            .map(Reverse)
            .collect::<Vec<_>>();
        encodings.sort_unstable();
        let encodings_tokens = encodings.iter().map(|e| e.0.parse_ifchain_tokens());
        quote! {
            pub fn parse_arm(ins: u32, pc: u32) -> Ins {
                #(#encodings_tokens)else*
                else {
                    Ins::Illegal
                }
            }
        }
    }

    pub fn parse_thumb_ifchain_fn_tokens(&self) -> TokenStream {
        let mut encodings = self
            .0
            .iter()
            .flat_map(|o| o.parse_thumb_ifchain_tokens())
            .map(Reverse)
            .collect::<Vec<_>>();
        encodings.sort_unstable();
        let encodings_tokens = encodings.iter().map(|e| e.0.parse_ifchain_tokens());
        quote! {
            pub fn parse_thumb(ins: u16, next: Option<u16>, pc: u32) -> Ins {
                #(#encodings_tokens)else*
                else {
                    Ins::Illegal
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Opcode {
    mnemonic: String,
    description: String,
    params: IndexMap<OpcodeParamName, DataTypeName>,
    format: OpcodeFormat,
    arm: Option<Vec<OpcodeEncoding>>,
    thumb: Option<Vec<OpcodeEncoding>>,
    // a64: Option<OpcodeEncodingArm>, // TODO: AArch64 support
}

impl Opcode {
    pub fn mnemonic(&self) -> &str {
        &self.mnemonic
    }

    pub fn params(&self) -> &IndexMap<OpcodeParamName, DataTypeName> {
        &self.params
    }

    pub fn validate(&self, isa: &Isa) -> Result<()> {
        for (param, type_name) in self.params.iter() {
            isa.types().get(type_name).ok_or_else(|| {
                anyhow!(
                    "Data type '{type_name}' not found for parameter '{param}' of opcode '{}'",
                    self.mnemonic
                )
            })?;
        }
        Ok(())
    }

    fn ins_variant_tokens(&self, isa: &Isa) -> TokenStream {
        let variant_ident = Ident::new(&capitalize(&self.mnemonic), Span::call_site());
        let params = self.params.iter().map(|(name, type_name)| {
            let Some(data_type) = isa.types().get(type_name) else {
                panic!();
            };
            let name_ident = Ident::new(&name.0, Span::call_site());
            let type_ident = data_type.type_tokens(isa);
            quote!(#name_ident: #type_ident)
        });
        let description = &self.description;
        quote! {
            #[doc = #description]
            #variant_ident { #(#params),* }
        }
    }

    fn parse_fn_name(&self, arch: Arch, index: usize) -> String {
        match arch {
            Arch::Arm => format!("parse_arm_{}_{}", self.mnemonic, index),
            Arch::Thumb => format!("parse_thumb_{}_{}", self.mnemonic, index),
        }
    }

    fn parse_fns_tokens(&self, isa: &Isa) -> TokenStream {
        let arm_parse_fns = self.arm.iter().flatten().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(self.parse_fn_name(Arch::Arm, i), self, isa)
        });
        let thumb_parse_fns = self.thumb.iter().flatten().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(self.parse_fn_name(Arch::Thumb, i), self, isa)
        });
        quote! {
            #(#arm_parse_fns)*
            #(#thumb_parse_fns)*
        }
    }

    fn format_params(&self, isa: &Isa) -> FormatParams {
        let mut params: FormatParams = HashMap::new();
        for (param_name, type_name) in &self.params {
            let Some(data_type) = isa.types().get(type_name) else {
                panic!();
            };
            params.insert(param_name.0.clone(), data_type.clone());
        }
        params
    }

    fn write_opcode_tokens(&self, isa: &Isa) -> TokenStream {
        let params = self.format_params(isa);
        let display_expr = self.format.opcode.fmt_expr_tokens(isa, &params);
        self.write_variant_tokens(display_expr)
    }

    fn write_params_tokens(&self, isa: &Isa) -> TokenStream {
        let params = self.format_params(isa);
        let display_expr = if !self.format.params.is_empty() {
            let display_expr = self.format.params.fmt_expr_tokens(isa, &params);
            quote! {
                formatter.write_space()?;
                #display_expr
            }
        } else {
            quote!()
        };
        self.write_variant_tokens(display_expr)
    }

    fn write_variant_tokens(&self, expr: TokenStream) -> TokenStream {
        let variant_ident = Ident::new(&capitalize(&self.mnemonic), Span::call_site());
        let param_names = self.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

        quote! {
            Ins::#variant_ident { #(#param_names),* } => {
                #expr
            }
        }
    }

    fn parse_arm_ifchain_tokens(&self) -> impl Iterator<Item = ParseOpcodeEncoding> {
        self.arm.iter().flatten().enumerate().map(|(i, encoding)| {
            let parse_fn_name = self.parse_fn_name(Arch::Arm, i);
            encoding.as_parse_encoding(parse_fn_name)
        })
    }

    fn parse_thumb_ifchain_tokens(&self) -> impl Iterator<Item = ParseOpcodeEncoding> {
        self.thumb.iter().flatten().enumerate().map(|(i, encoding)| {
            let parse_fn_name = self.parse_fn_name(Arch::Thumb, i);
            encoding.as_parse_encoding(parse_fn_name)
        })
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct OpcodeParamName(pub String);

impl Display for OpcodeParamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Deserialize, Debug)]
pub struct OpcodeFormat {
    opcode: Format,
    params: Format,
}

#[derive(Deserialize, Debug)]
pub struct OpcodeEncoding {
    version: Vec<IsaVersionPattern>,
    pattern: OpcodePattern,
    params: IndexMap<OpcodeParamName, OpcodeParamValue>,
}
impl OpcodeEncoding {
    pub fn get_param(&self, name: &OpcodeParamName) -> Option<&OpcodeParamValue> {
        self.params.get(name)
    }

    fn parse_fn_tokens(&self, name: String, opcode: &Opcode, isa: &Isa) -> TokenStream {
        let fn_ident = Ident::new(&name, Span::call_site());

        let params = opcode.params().iter().map(|(name, type_name)| {
            let data_type = isa.types().get(type_name).unwrap();
            let parse_expr = if let Some(value) = self.get_param(name) {
                value.parse_expr_tokens(isa, data_type)
            } else {
                data_type.default_expr_tokens(isa)
            };

            let name_ident = Ident::new(&name.0, Span::call_site());
            quote!(let #name_ident = #parse_expr;)
        });
        let variant_ident = Ident::new(&capitalize(opcode.mnemonic()), Span::call_site());
        let param_names = opcode.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

        quote! {
            fn #fn_ident(value: u32, pc: u32) -> Ins {
                #(#params)*
                Ins::#variant_ident { #(#param_names),* }
            }
        }
    }

    fn as_parse_encoding(&self, parse_fn_name: String) -> ParseOpcodeEncoding {
        ParseOpcodeEncoding { pattern: self.pattern.clone(), parse_fn_name }
    }
}

#[derive(PartialEq, Eq)]
struct ParseOpcodeEncoding {
    pattern: OpcodePattern,
    parse_fn_name: String,
}

impl PartialOrd for ParseOpcodeEncoding {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParseOpcodeEncoding {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pattern.num_bits().cmp(&other.pattern.num_bits())
    }
}

impl ParseOpcodeEncoding {
    fn parse_ifchain_tokens(&self) -> TokenStream {
        let parse_fn_ident = Ident::new(&self.parse_fn_name, Span::call_site());

        let first_bitmask = HexLiteral(self.pattern.first().bitmask());
        let first_pattern = HexLiteral(self.pattern.first().pattern());
        if let Some(second) = self.pattern.second() {
            let second_bitmask = HexLiteral(second.bitmask());
            let second_pattern = HexLiteral(second.pattern());
            quote! {
                if let Some(next) = next
                    && (ins & #first_bitmask) == #first_pattern
                    && (next & #second_bitmask) == #second_pattern
                {
                    #parse_fn_ident(((ins as u32) << 16) | (next as u32), pc)
                }
            }
        } else {
            quote! {
                if (ins & #first_bitmask) == #first_pattern {
                    #parse_fn_ident(ins as u32, pc)
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum OpcodeParamValue {
    #[serde(rename = "bits")]
    Bits(BitRange),
    #[serde(rename = "const")]
    Const(u32),
    #[serde(rename = "expr")]
    Expr(DataExpr),
    #[serde(rename = "enum")]
    Enum(DataTypeEnumVariantName, Box<OpcodeParamValue>),
    #[serde(rename = "struct")]
    Struct(IndexMap<String, OpcodeParamValue>),
}
impl OpcodeParamValue {
    pub fn parse_expr_tokens(&self, isa: &Isa, data_type: &DataType) -> TokenStream {
        match self {
            OpcodeParamValue::Bits(bit_range) => {
                data_type.parse_expr_tokens(isa, Some(bit_range.shift_mask_tokens(None)))
            }
            OpcodeParamValue::Const(value) => {
                let literal = Literal::u32_unsuffixed(*value).into_token_stream();
                data_type.parse_expr_tokens(isa, Some(literal))
            }
            OpcodeParamValue::Expr(expr) => {
                let expr = expr.as_tokens(Ident::new("value", Span::call_site()));
                data_type.parse_expr_tokens(isa, Some(expr))
            }
            OpcodeParamValue::Enum(variant, value) => {
                let inner_type = match data_type.kind() {
                    DataTypeKind::Type(data_type_name, _) => {
                        let Some(inner_type) = isa.types().get(data_type_name) else {
                            panic!();
                        };
                        inner_type
                    }
                    _ => data_type,
                };
                let DataTypeKind::Enum(data_type_enum) = inner_type.kind() else {
                    panic!("Data type '{}' is not an enum", data_type.name().0);
                };
                let Some((_, variant)) = data_type_enum.get_variant(variant) else {
                    panic!();
                };
                variant.param_expr_tokens(isa, inner_type.name(), value)
            }
            OpcodeParamValue::Struct(params) => {
                let DataTypeKind::Struct(data_type_struct) = data_type.kind() else {
                    panic!();
                };
                data_type_struct.param_tokens(isa, data_type.name(), params)
            }
        }
    }
}

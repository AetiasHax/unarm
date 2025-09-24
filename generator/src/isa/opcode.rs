use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{
        Arch, BitRange, DataExpr, DataType, DataTypeEnumVariantName, DataTypeKind, DataTypeName,
        Format, FormatCond, FormatParams, IllegalChecks, Isa, IsaExtensionPatterns,
        IsaVersionPatterns, OpcodeLookupTable, OpcodePattern,
    },
    util::str::snake_to_pascal_case,
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
                Illegal,
                #(#opcodes),*,
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
                    match self {
                        Ins::Illegal => formatter.write_str("<illegal>")?,
                        #(#opcodes)*
                    }
                    Ok(())
                }

                pub fn write_params<F>(&self, formatter: &mut F) -> core::fmt::Result
                where
                    F: Write + ?Sized
                {
                    match self {
                        Ins::Illegal => {},
                        #(#params)*
                    }
                    Ok(())
                }
            }
        }
    }

    pub fn parse_arm_lookup_match_tokens(&self, isa: &Isa) -> TokenStream {
        let lookup_table = OpcodeLookupTable::new_arm(isa);
        let parse_fn_body = lookup_table.parse_match_fn_body_tokens();
        quote! {
            pub fn parse_arm(ins: u32, pc: u32, options: &Options) -> Ins {
                #parse_fn_body
            }
        }
    }

    pub fn parse_thumb_lookup_match_tokens(&self, isa: &Isa) -> TokenStream {
        let lookup_table = OpcodeLookupTable::new_thumb(isa);
        let parse_fn_body = lookup_table.parse_match_fn_body_tokens();
        quote! {
            pub fn parse_thumb(ins: u32, pc: u32, options: &Options) -> Ins {
                #parse_fn_body
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
        let variant_ident = Ident::new(&snake_to_pascal_case(&self.mnemonic), Span::call_site());
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

    pub fn parse_fn_name(&self, arch: Arch, index: usize) -> String {
        format!("parse_{}_{}_{}", arch, self.mnemonic, index)
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
        let display_expr = self.format.opcode.fmt_expr_tokens(isa, &params, None);
        self.write_variant_tokens(display_expr)
    }

    fn write_params_tokens(&self, isa: &Isa) -> TokenStream {
        let params = self.format_params(isa);
        let display_expr = if !self.format.params.is_empty() {
            let display_expr = self.format.params.fmt_expr_tokens(isa, &params, None);
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
        let variant_ident = Ident::new(&snake_to_pascal_case(&self.mnemonic), Span::call_site());
        let param_names = self.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

        quote! {
            Ins::#variant_ident { #(#param_names),* } => {
                #expr
            }
        }
    }

    pub fn arm_encodings(&self) -> Option<&[OpcodeEncoding]> {
        self.arm.as_deref()
    }

    pub fn thumb_encodings(&self) -> Option<&[OpcodeEncoding]> {
        self.thumb.as_deref()
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
    version: IsaVersionPatterns,
    #[serde(default)]
    extensions: IsaExtensionPatterns,
    pattern: OpcodePattern,
    ignore: Option<FormatCond>,
    #[serde(default)]
    illegal: IllegalChecks,
    params: IndexMap<OpcodeParamName, OpcodeParamValue>,
}
impl OpcodeEncoding {
    pub fn get_param(&self, name: &OpcodeParamName) -> Option<&OpcodeParamValue> {
        self.params.get(name)
    }

    fn parse_fn_tokens(&self, name: String, opcode: &Opcode, isa: &Isa) -> TokenStream {
        let fn_ident = Ident::new(&name, Span::call_site());

        let params = opcode.params().iter().map(|(param_name, type_name)| {
            let data_type = isa.types().get(type_name).unwrap();
            let value = self.get_param(param_name);
            let parse_expr = if let Some(value) = value {
                value.parse_expr_tokens(isa, data_type)
            } else {
                data_type.default_expr_tokens(isa).unwrap_or_else(|| {
                    panic!(
                        "Missing value for parameter '{}' in opcode encoding '{}'",
                        param_name.0, name
                    )
                })
            };

            let name_ident = Ident::new(&param_name.0, Span::call_site());
            if data_type.can_be_illegal(isa)
                && matches!(
                    value,
                    Some(
                        OpcodeParamValue::Bits(_)
                            | OpcodeParamValue::Const(_)
                            | OpcodeParamValue::Expr(_)
                    )
                )
            {
                let x = quote! {
                    let Some(#name_ident) = #parse_expr else {
                        return Some(Ins::Illegal);
                    };
                };
                // println!("{x}");
                x
            } else {
                quote!(let #name_ident = #parse_expr;)
            }
        });
        let variant_ident = Ident::new(&snake_to_pascal_case(opcode.mnemonic()), Span::call_site());
        let param_names = opcode.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

        let illegal_checks = self.illegal.checks_tokens(None);

        let version_check = if isa.versions().matches_all(&self.version) {
            quote!()
        } else {
            let versions = self.version.versions(isa);
            let versions = versions.iter().map(|version| {
                let ident = version.as_ident();
                quote!(Version::#ident)
            });
            quote! {
                const VERSIONS: Versions = Versions::of(&[#(#versions),*]);
                if !VERSIONS.has(options.version) {
                    return None;
                }
            }
        };

        let extensions_check = if self.extensions.is_empty() {
            quote!()
        } else {
            let extensions = self.extensions.extensions(isa);
            let extensions = extensions.iter().map(|extension| {
                let ident = extension.as_ident();
                quote!(Extension::#ident)
            });
            quote! {
                const EXTENSIONS: Extensions = Extensions::of(&[#(#extensions),*]);
                if !EXTENSIONS.has_all(options.extensions) {
                    return None;
                }
            }
        };

        let ignore_check = self.ignore.as_ref().map(|ignore| {
            let ignore_expr = ignore.as_tokens(Some(quote!(options)));
            quote! {
                if #ignore_expr {
                    return None;
                }
            }
        });

        quote! {
            fn #fn_ident(value: u32, pc: u32, options: &Options) -> Option<Ins> {
                #version_check
                #extensions_check
                #ignore_check
                #illegal_checks
                #(#params)*
                Some(Ins::#variant_ident { #(#param_names),* })
            }
        }
    }

    pub fn pattern(&self) -> &OpcodePattern {
        &self.pattern
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
                let inner_type = data_type.canonical(isa);
                let variant = match inner_type.kind() {
                    DataTypeKind::Enum(data_type_enum) => {
                        data_type_enum.get_variant(variant).unwrap()
                    }
                    DataTypeKind::Union(data_type_union) => {
                        data_type_union.get_variant(variant).unwrap().1
                    }
                    _ => {
                        panic!("Data type '{}' is not an enum", data_type.name().0);
                    }
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

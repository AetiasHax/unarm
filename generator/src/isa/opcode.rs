use std::{collections::HashMap, fmt::Display};

use anyhow::{Context, Result, anyhow};
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
            #[repr(u16)]
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum Ins {
                #(#opcodes),*,
                Word(u32),
                HalfWord(u16),
                Byte(u8),
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
                    match self {
                        #(#opcodes)*
                        Ins::Illegal => formatter.write_str("<illegal>")?,
                        Ins::Word(value) => formatter.write_str(".word")?,
                        Ins::HalfWord(value) => formatter.write_str(".hword")?,
                        Ins::Byte(value) => formatter.write_str(".byte")?,
                    }
                    Ok(())
                }

                pub fn write_params<F>(&self, formatter: &mut F) -> core::fmt::Result
                where
                    F: Write + ?Sized
                {
                    match self {
                        #(#params)*
                        Ins::Illegal => {},
                        Ins::Word(value) => {
                            formatter.write_space()?;
                            formatter.write_uimm(*value)?;
                        },
                        Ins::HalfWord(value) => {
                            formatter.write_space()?;
                            formatter.write_uimm(*value as u32)?;
                        },
                        Ins::Byte(value) => {
                            formatter.write_space()?;
                            formatter.write_uimm(*value as u32)?;
                        },
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
            pub fn parse_thumb(ins: u32, pc: u32, options: &Options) -> (Ins, u32) {
                #parse_fn_body
            }
        }
    }

    pub fn parse_with_discriminant_tokens(&self, arch: Arch) -> TokenStream {
        let num_opcodes = self.0.len();

        // See Opcodes::ins_enum_tokens
        let id_byte = Literal::usize_unsuffixed(num_opcodes);
        let id_halfword = Literal::usize_unsuffixed(num_opcodes + 1);
        let id_word = Literal::usize_unsuffixed(num_opcodes + 2);

        let cases = self.0.iter().enumerate().filter_map(|(discriminant, opcode)| {
            opcode.parse_with_discriminant_case(discriminant as u16, arch)
        });

        let fn_name = format!("parse_{arch}_with_discriminant");
        let fn_ident = Ident::new(&fn_name, Span::call_site());

        quote! {
            pub fn #fn_ident(ins: u32, discriminant: u16, pc: u32, options: &Options) -> Ins {
                match discriminant {
                    #(#cases),*,
                    #id_byte => Ins::Byte(ins as u8),
                    #id_halfword => Ins::HalfWord(ins as u16),
                    #id_word => Ins::Word(ins),
                    _ => Ins::Illegal,
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
    #[serde(default)]
    arm: Vec<OpcodeEncoding>,
    #[serde(default)]
    thumb: Vec<OpcodeEncoding>,
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
        for (i, encoding) in self.arm.iter().enumerate() {
            encoding.validate(false).with_context(|| {
                format!("Invalid ARM encoding {i} for opcode '{}'", self.mnemonic)
            })?;
        }
        for (i, encoding) in self.thumb.iter().enumerate() {
            encoding.validate(true).with_context(|| {
                format!("Invalid Thumb encoding {i} for opcode '{}'", self.mnemonic)
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

    pub fn parse_fn_ident(&self, arch: Arch, index: usize) -> Ident {
        let name = format!("parse_{}_{}_{}", arch, self.mnemonic, index);
        Ident::new(&name, Span::call_site())
    }

    fn parse_fns_tokens(&self, isa: &Isa) -> TokenStream {
        let arm_parse_fns = self.arm.iter().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(self.parse_fn_ident(Arch::Arm, i), self, isa, Arch::Arm)
        });
        let thumb_parse_fns = self.thumb.iter().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(self.parse_fn_ident(Arch::Thumb, i), self, isa, Arch::Thumb)
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

    pub fn arm_encodings(&self) -> &[OpcodeEncoding] {
        &self.arm
    }

    pub fn thumb_encodings(&self) -> &[OpcodeEncoding] {
        &self.thumb
    }

    fn parse_with_discriminant_case(&self, discriminant: u16, arch: Arch) -> Option<TokenStream> {
        let encodings = match arch {
            Arch::Arm => &self.arm,
            Arch::Thumb => &self.thumb,
        };

        let parse_encoding = match encodings.len() {
            0 => return None,
            1 => self.with_discriminant_encoding_parse_fn(arch, 0),
            _ => {
                let encoding_ifs = encodings.iter().enumerate().map(|(index, encoding)| {
                    let condition = encoding.pattern.condition_tokens(quote!(ins));
                    let parse_fn = self.with_discriminant_encoding_parse_fn(arch, index);
                    quote! {
                        if #condition {
                            #parse_fn
                        }
                    }
                });
                quote! {
                    #(#encoding_ifs)else*
                    else {
                        Ins::Illegal
                    }
                }
            }
        };

        let discriminant_lit = Literal::u16_unsuffixed(discriminant);
        Some(quote!(#discriminant_lit => #parse_encoding))
    }

    fn with_discriminant_encoding_parse_fn(
        &self,
        arch: Arch,
        encoding_index: usize,
    ) -> TokenStream {
        let parse_fn_ident = self.parse_fn_ident(arch, encoding_index);
        match arch {
            Arch::Arm => quote!(#parse_fn_ident(ins, pc, options).unwrap_or(Ins::Illegal)),
            Arch::Thumb => {
                quote!(#parse_fn_ident(ins, pc, options).map(|(ins, _size)| ins).unwrap_or(Ins::Illegal))
            }
        }
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
    fn validate(&self, thumb: bool) -> Result<()> {
        self.pattern.validate(thumb)
    }

    pub fn get_param(&self, name: &OpcodeParamName) -> Option<&OpcodeParamValue> {
        self.params.get(name)
    }

    fn parse_fn_tokens(
        &self,
        fn_ident: Ident,
        opcode: &Opcode,
        isa: &Isa,
        arch: Arch,
    ) -> TokenStream {
        let ins_size = Literal::u32_unsuffixed(self.pattern.combined().size() / 8);
        let illegal_value = match arch {
            Arch::Arm => quote!(Some(Ins::Illegal)),
            Arch::Thumb => quote!(Some((Ins::Illegal, #ins_size))),
        };
        let illegal_checks = self.illegal.checks_tokens(Some(illegal_value.clone()));

        let params = opcode.params().iter().map(|(param_name, type_name)| {
            let data_type = isa.types().get(type_name).unwrap();
            let value = self.get_param(param_name);
            let parse_expr = if let Some(value) = value {
                value.parse_expr_tokens(isa, data_type)
            } else {
                data_type.default_expr_tokens(isa).unwrap_or_else(|| {
                    panic!(
                        "Missing value for parameter '{}' in opcode encoding '{}'",
                        param_name.0, fn_ident
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
                quote! {
                    let Some(#name_ident) = #parse_expr else {
                        return #illegal_value;
                    };
                }
            } else {
                quote!(let #name_ident = #parse_expr;)
            }
        });
        let variant_ident = Ident::new(&snake_to_pascal_case(opcode.mnemonic()), Span::call_site());
        let param_names = opcode.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

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

        let return_value = match arch {
            Arch::Arm => quote!(Ins::#variant_ident { #(#param_names),* }),
            Arch::Thumb => {
                quote!((Ins::#variant_ident { #(#param_names),* }, #ins_size))
            }
        };
        let return_type = match arch {
            Arch::Arm => quote!(Option<Ins>),
            Arch::Thumb => quote!(Option<(Ins, u32)>),
        };

        quote! {
            fn #fn_ident(value: u32, pc: u32, options: &Options) -> #return_type {
                #version_check
                #extensions_check
                #ignore_check
                #illegal_checks
                #(#params)*
                Some(#return_value)
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

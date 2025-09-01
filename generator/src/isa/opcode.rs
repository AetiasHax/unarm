use std::fmt::Display;

use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{
        BitRange, DataType, DataTypeEnumVariantName, DataTypeName, Format, Isa, IsaVersionPattern,
        OpcodePattern,
    },
    util::str::capitalize,
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
                #(#opcodes),*
            }
        }
    }

    pub fn parse_fns_tokens(&self, isa: &Isa) -> TokenStream {
        let parse_fns = self.iter().map(|o| o.parse_fns_tokens(isa));
        quote!(#(#parse_fns)*)
    }
}

#[derive(Deserialize, Debug)]
pub struct Opcode {
    mnemonic: String,
    params: IndexMap<OpcodeParamName, DataTypeName>,
    format: Format,
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
            let type_ident = data_type.type_tokens(&type_name.0);
            quote!(#name_ident: #type_ident)
        });
        quote!(#variant_ident { #(#params),* })
    }

    fn parse_fns_tokens(&self, isa: &Isa) -> TokenStream {
        let arm_parse_fns = self.arm.iter().flatten().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(format!("parse_arm_{}_{}", self.mnemonic, i), self, isa)
        });
        let thumb_parse_fns = self.thumb.iter().flatten().enumerate().map(|(i, encoding)| {
            encoding.parse_fn_tokens(format!("parse_thumb_{}_{}", self.mnemonic, i), self, isa)
        });
        quote! {
            #(#arm_parse_fns)*
            #(#thumb_parse_fns)*
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
                value.parse_expr_tokens(Some(type_name), data_type)
            } else {
                data_type.default_expr_tokens(Some(type_name))
            };

            let name_ident = Ident::new(&name.0, Span::call_site());
            quote!(let #name_ident = #parse_expr;)
        });
        let variant_ident = Ident::new(&capitalize(opcode.mnemonic()), Span::call_site());
        let param_names = opcode.params.keys().map(|k| Ident::new(&k.0, Span::call_site()));

        quote! {
            fn #fn_ident(value: u32) -> Ins {
                #(#params)*
                Ins::#variant_ident { #(#param_names),* }
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
    #[serde(rename = "enum")]
    Enum(DataTypeEnumVariantName, Box<OpcodeParamValue>),
    #[serde(rename = "struct")]
    Struct(IndexMap<String, OpcodeParamValue>),
}
impl OpcodeParamValue {
    pub fn parse_expr_tokens(
        &self,
        type_name: Option<&DataTypeName>,
        data_type: &DataType,
    ) -> TokenStream {
        match self {
            OpcodeParamValue::Bits(bit_range) => {
                data_type.parse_expr_tokens(type_name, Some(bit_range.shift_mask_tokens()))
            }
            OpcodeParamValue::Const(value) => {
                let type_name = type_name.unwrap();
                let literal = Literal::u32_unsuffixed(*value).into_token_stream();
                data_type.parse_expr_tokens(Some(type_name), Some(literal))
            }
            OpcodeParamValue::Enum(variant, value) => {
                let type_name = type_name.unwrap();
                let DataType::Enum(data_type_enum) = data_type else {
                    panic!();
                };
                let Some((_, variant)) = data_type_enum.get_variant(variant) else {
                    panic!();
                };
                variant.param_expr_tokens(type_name, value)
            }
            OpcodeParamValue::Struct(params) => {
                let type_name = type_name.unwrap();
                let DataType::Struct(data_type_struct) = data_type else {
                    panic!();
                };
                data_type_struct.param_tokens(type_name, params)
            }
        }
    }
}

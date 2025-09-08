use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::{Ident, visit_mut::VisitMut};

use crate::{
    isa::{BitRange, Format, FormatParams, Isa, OpcodeParamValue, Pattern, SynExpr},
    util::{hex_literal::HexLiteral, str::snake_to_pascal_case},
};

#[derive(Deserialize, Debug)]
pub struct DataTypes(Vec<DataType>);

impl DataTypes {
    pub fn get(&self, name: &DataTypeName) -> Option<&DataType> {
        self.0.iter().find(|dt| &dt.name == name)
    }

    pub fn post_process(&mut self) {
        for data_type in self.0.iter_mut() {
            data_type.top_level = true;
        }
    }

    pub fn validate(&self) -> Result<()> {
        for data_type in self.0.iter() {
            data_type.validate()?;
        }
        Ok(())
    }

    pub fn types_tokens(&self, isa: &Isa) -> TokenStream {
        let types = self.0.iter().filter_map(|dt| dt.type_decl_tokens(isa));
        quote!(#(#types)*)
    }

    pub fn parse_impls_tokens(&self, isa: &Isa) -> TokenStream {
        let parse_impls = self.0.iter().filter_map(|dt| dt.parse_impl_tokens(isa));
        quote!(#(#parse_impls)*)
    }

    pub fn default_impls_tokens(&self) -> TokenStream {
        let default_impls = self.0.iter().filter_map(|dt| dt.default_impl_tokens());
        quote!(#(#default_impls)*)
    }

    pub fn fmt_impls_tokens(&self, isa: &Isa) -> TokenStream {
        let fmt_impls = self.0.iter().filter_map(|dt| dt.write_impl_tokens(isa));
        quote!(#(#fmt_impls)*)
    }

    pub fn write_trait_tokens(&self, isa: &Isa) -> TokenStream {
        let data_types = self.0.iter().map(|dt| dt.trait_write_fn_tokens(isa));

        quote! {
            pub trait Write: core::fmt::Write {
                fn options(&self) -> &Options;

                fn write_space(&mut self) -> core::fmt::Result {
                    self.write_str(" ")
                }

                fn write_separator(&mut self) -> core::fmt::Result {
                    self.write_str(", ")
                }

                #(#data_types)*

                fn write_ins(&mut self, ins: &Ins) -> core::fmt::Result {
                    ins.write_opcode(self)?;
                    ins.write_params(self)?;
                    Ok(())
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DataTypeName(pub String);

impl DataTypeName {
    // TODO: Use this method more
    fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }

    fn as_pascal_ident(&self) -> Ident {
        Ident::new(&snake_to_pascal_case(&self.0), Span::call_site())
    }
}

impl Display for DataTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataType {
    name: DataTypeName,
    kind: DataTypeKind,
    #[serde(skip)]
    top_level: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub enum DataTypeKind {
    #[serde(rename = "bool")]
    Bool {
        #[serde(default)]
        bits: BitRange,
        write: Option<String>,
    },
    #[serde(rename = "uint")]
    UInt(DataExpr),
    #[serde(rename = "int")]
    Int(DataExpr),
    #[serde(rename = "enum")]
    Enum(DataTypeEnum),
    #[serde(rename = "struct")]
    Struct(DataTypeStruct),
    #[serde(rename = "type")]
    Type(DataTypeName, DataExpr),
    #[serde(rename = "custom")]
    Custom,
}

impl DataType {
    pub fn name(&self) -> &DataTypeName {
        &self.name
    }

    pub fn kind(&self) -> &DataTypeKind {
        &self.kind
    }

    pub fn validate(&self) -> Result<()> {
        match &self.kind {
            DataTypeKind::Bool { .. } => Ok(()),
            DataTypeKind::UInt(_) => Ok(()),
            DataTypeKind::Int(_) => Ok(()),
            DataTypeKind::Enum(data_type_enum) => data_type_enum.validate(&self.name),
            DataTypeKind::Struct(_) => Ok(()),
            DataTypeKind::Type(_, _) => Ok(()),
            DataTypeKind::Custom => Ok(()),
        }
    }

    fn type_decl_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(data_type_enum) => Some(data_type_enum.enum_tokens(isa, &self.name)),
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.struct_tokens(isa, &self.name))
            }
            DataTypeKind::Type(_, _) => None,
            DataTypeKind::Custom => None,
        }
    }

    fn parse_impl_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(data_type_enum) => {
                Some(data_type_enum.parse_impl_tokens(isa, &self.name))
            }
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.parse_impl_tokens(isa, &self.name))
            }
            DataTypeKind::Type(_, _) => None,
            DataTypeKind::Custom => None,
        }
    }

    pub fn type_tokens(&self, isa: &Isa) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool { .. } => quote!(bool),
            DataTypeKind::UInt(_) => quote!(u32),
            DataTypeKind::Int(_) => quote!(i32),
            DataTypeKind::Enum(_) => {
                let name_ident = self.name.as_pascal_ident();
                quote!(#name_ident)
            }
            DataTypeKind::Struct(_) => {
                let name_ident = self.name.as_pascal_ident();
                quote!(#name_ident)
            }
            DataTypeKind::Type(name, _) => {
                let Some(inner_type) = isa.types().get(name) else {
                    panic!();
                };
                inner_type.type_tokens(isa)
            }
            DataTypeKind::Custom => {
                let name_ident = self.name.as_pascal_ident();
                quote!(#name_ident)
            }
        }
    }

    fn field_tokens(&self, isa: &Isa, is_pub: bool) -> TokenStream {
        let name_ident = Ident::new(&self.name.0, Span::call_site());
        let type_tokens = self.type_tokens(isa);
        if is_pub {
            quote!(pub #name_ident: #type_tokens)
        } else {
            quote!(#name_ident: #type_tokens)
        }
    }

    pub fn parse_expr_tokens(&self, isa: &Isa, value: Option<TokenStream>) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool { bits, .. } => {
                let value = value.unwrap_or_else(|| bits.shift_mask_tokens(None));
                quote!((#value) != 0)
            }
            DataTypeKind::UInt(data_expr) => {
                value.unwrap_or_else(|| data_expr.as_tokens(Ident::new("value", Span::call_site())))
            }
            DataTypeKind::Int(data_expr) => {
                let expr = value
                    .unwrap_or_else(|| data_expr.as_tokens(Ident::new("value", Span::call_site())));
                quote!((#expr) as i32)
            }
            DataTypeKind::Enum(data_type_enum) => {
                data_type_enum.parse_expr_tokens(&self.name, value)
            }
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.parse_expr_tokens(&self.name, value)
            }
            DataTypeKind::Type(data_type_name, data_expr) => {
                let Some(inner_type) = isa.types().get(data_type_name) else {
                    panic!();
                };
                let value = value
                    .unwrap_or_else(|| data_expr.as_tokens(Ident::new("value", Span::call_site())));
                inner_type.parse_expr_tokens(isa, Some(value))
            }
            DataTypeKind::Custom => {
                let name_ident = self.name.as_pascal_ident();
                let value = value.unwrap_or_else(|| quote!(value));
                quote!(#name_ident::parse(#value))
            }
        }
    }

    pub fn default_expr_tokens(&self) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool { .. } => quote!(false),
            DataTypeKind::UInt(_) => quote!(0),
            DataTypeKind::Int(_) => quote!(0),
            DataTypeKind::Enum(data_type_enum) => data_type_enum.default_expr_tokens(&self.name),
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.default_expr_tokens(&self.name)
            }
            DataTypeKind::Type(data_type_name, _) => {
                let type_ident = data_type_name.as_pascal_ident();
                quote!(#type_ident::default())
            }
            DataTypeKind::Custom => {
                let type_ident = self.name.as_pascal_ident();
                quote!(#type_ident::default())
            }
        }
    }

    fn default_impl_tokens(&self) -> Option<TokenStream> {
        let default_expr = match &self.kind {
            DataTypeKind::Bool { .. } => return None,
            DataTypeKind::UInt(_) => return None,
            DataTypeKind::Int(_) => return None,
            DataTypeKind::Enum(data_type_enum) => data_type_enum.default_impl_body_tokens()?,
            DataTypeKind::Struct(data_type_struct) => data_type_struct.default_impl_body_tokens(),
            DataTypeKind::Type(_, _) => return None,
            DataTypeKind::Custom => return None,
        };
        let name_ident = self.name.as_pascal_ident();
        Some(quote! {
            impl Default for #name_ident {
                fn default() -> Self {
                    #default_expr
                }
            }
        })
    }

    pub fn write_impl_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(data_type_enum) => Some(data_type_enum.write_impl_body_tokens(isa)),
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.write_impl_body_tokens(isa))
            }
            DataTypeKind::Type(_, _) => None,
            DataTypeKind::Custom => None,
        }
    }

    fn write_impl_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool { .. } => return None,
            DataTypeKind::UInt(_) => return None,
            DataTypeKind::Int(_) => return None,
            DataTypeKind::Enum(_) => {}
            DataTypeKind::Struct(_) => {}
            DataTypeKind::Type(_, _) => return None,
            DataTypeKind::Custom => return None,
        };
        let display_expr = self.write_impl_body_tokens(isa);
        let name_ident = self.name().as_pascal_ident();
        Some(quote! {
            impl #name_ident {
                pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
                where
                    F: Write + ?Sized
                {
                    let options = formatter.options();
                    #display_expr
                    Ok(())
                }
            }
        })
    }

    pub fn fmt_expr_tokens(&self, value: TokenStream) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool { .. } if !self.top_level => {
                self.write_expr_tokens(quote!(*#value), quote!(formatter))
            }
            _ => {
                let write_fn_ident = self.trait_write_fn_ident();
                quote!(formatter.#write_fn_ident(*#value)?;)
            }
        }
    }

    pub fn fmt_expr_in_enum_variant_tokens(&self, isa: &Isa, value: TokenStream) -> TokenStream {
        if let DataTypeKind::Struct(data_type_struct) = &self.kind {
            data_type_struct.fmt_expr_tokens(isa)
        } else {
            self.fmt_expr_tokens(value)
        }
    }

    fn trait_write_fn_ident(&self) -> Ident {
        let base_name = match &self.kind {
            DataTypeKind::Bool { .. } => &self.name.0,
            DataTypeKind::UInt(_) => "uimm",
            DataTypeKind::Int(_) => "simm",
            DataTypeKind::Enum(_) => &self.name.0,
            DataTypeKind::Struct(_) => &self.name.0,
            DataTypeKind::Type(name, _) => &name.0,
            DataTypeKind::Custom => &self.name.0,
        };
        let fn_name = format!("write_{}", base_name);
        Ident::new(&fn_name, Span::call_site())
    }

    fn write_expr_tokens(&self, value: TokenStream, formatter: TokenStream) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool { write, .. } => {
                let write = write.as_ref().unwrap_or(&self.name.0);
                quote! {
                    if #value {
                        #formatter.write_str(#write)?;
                    }
                }
            }
            DataTypeKind::UInt(_) => {
                quote!(write!(#formatter, "{:#x}", #value)?;)
            }
            DataTypeKind::Int(_) => {
                quote! {
                    if #value < 0 {
                        write!(#formatter, "-{:#x}", -#value)?;
                    } else {
                        write!(#formatter, "{:#x}", #value)?;
                    }
                }
            }
            DataTypeKind::Enum(_) => quote!(#value.write(#formatter)?;),
            DataTypeKind::Struct(_) => quote!(#value.write(#formatter)?;),
            DataTypeKind::Type(_, _) => quote!(#value.write(#formatter)?;),
            DataTypeKind::Custom => quote!(#value.write(#formatter)?;),
        }
    }

    fn trait_write_fn_tokens(&self, isa: &Isa) -> TokenStream {
        let fn_ident = self.trait_write_fn_ident();
        let value = self.name.as_ident();
        let type_tokens = self.type_tokens(isa);

        let write_expr = self.write_expr_tokens(value.to_token_stream(), quote!(self));

        quote! {
            fn #fn_ident(&mut self, #value: #type_tokens) -> core::fmt::Result {
                #write_expr
                Ok(())
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataTypeEnum {
    bits: BitRange,
    default: Option<DataTypeEnumVariantName>,
    variants: IndexMap<Pattern, DataTypeEnumVariant>,
}

impl DataTypeEnum {
    pub fn get_variant(
        &self,
        name: &DataTypeEnumVariantName,
    ) -> Option<(&Pattern, &DataTypeEnumVariant)> {
        self.variants.iter().find(|(_, v)| v.name() == name)
    }

    pub fn validate(&self, name: &DataTypeName) -> Result<()> {
        if let Some(default) = &self.default {
            self.get_variant(default)
                .ok_or_else(|| anyhow!("Default variant '{default}' of enum '{name}' not found"))?;
        }
        Ok(())
    }

    fn enum_tokens(&self, isa: &Isa, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        let variants = self.variants.values().map(|v| v.variant_tokens(isa));
        quote! {
            #[derive(PartialEq, Eq, Clone, Copy)]
            pub enum #name_ident {
                #(#variants),*
            }
        }
    }

    /// Returns `true` if any enum variant has any optional bits, e.g. the "x" in `00x1`
    fn has_optional_bits(&self) -> bool {
        let expected_bitmask = self.bits.mask();
        self.variants.keys().any(|pattern| pattern.bitmask() != expected_bitmask)
    }

    fn parse_impl_tokens(&self, isa: &Isa, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();

        let fn_body = if self.has_optional_bits() {
            let variants = self
                .variants
                .iter()
                .map(|(pattern, variant)| variant.parse_if_tokens(isa, pattern));
            quote! {
                #(#variants)else*
                else {
                    panic!();
                }
            }
        } else {
            let variants = self
                .variants
                .iter()
                .map(|(pattern, variant)| variant.parse_match_tokens(isa, pattern));
            quote! {
                match value {
                    #(#variants),*,
                    _ => panic!(),
                }
            }
        };

        quote! {
            impl #name_ident {
                pub(crate) fn parse(value: u32, pc: u32) -> Self {
                    #fn_body
                }
            }
        }
    }

    fn parse_expr_tokens(&self, name: &DataTypeName, value: Option<TokenStream>) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        let value = value.unwrap_or_else(|| self.bits.shift_mask_tokens(None));
        quote!(#name_ident::parse(#value, pc))
    }

    fn default_impl_body_tokens(&self) -> Option<TokenStream> {
        let default = self.default.as_ref()?;
        let Some((_, default_variant)) = self.get_variant(default) else {
            panic!();
        };
        Some(default_variant.default_expr_tokens())
    }

    fn default_expr_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        quote!(#name_ident::default())
    }

    fn write_impl_body_tokens(&self, isa: &Isa) -> TokenStream {
        let variants = self.variants.iter().map(|(_, variant)| variant.write_expr_tokens(isa));
        quote! {
            match self {
                #(#variants),*
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataTypeEnumVariant {
    name: DataTypeEnumVariantName,
    description: Option<String>,
    format: Option<Format>,
    data: Option<DataType>,
}

impl DataTypeEnumVariant {
    pub fn name(&self) -> &DataTypeEnumVariantName {
        &self.name
    }

    fn as_ident(&self) -> Ident {
        Ident::new(&snake_to_pascal_case(&self.name.0), Span::call_site())
    }

    fn variant_tokens(&self, isa: &Isa) -> TokenStream {
        let variant_ident = self.as_ident();
        let variant = if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let record = data_type_struct.record_tokens(isa, false);
                quote!(#variant_ident #record)
            } else {
                let variant_data = data.type_tokens(isa);
                quote!(#variant_ident(#variant_data))
            }
        } else {
            quote!(#variant_ident)
        };

        if let Some(description) = &self.description {
            quote! {
                #[doc = #description]
                #variant
            }
        } else {
            variant
        }
    }

    fn parse_match_tokens(&self, isa: &Isa, pattern: &Pattern) -> TokenStream {
        let pattern = HexLiteral(pattern.pattern());
        let parse_expr = self.parse_expr_tokens(isa);
        quote!(#pattern => #parse_expr)
    }

    fn parse_if_tokens(&self, isa: &Isa, pattern: &Pattern) -> TokenStream {
        let bitmask = HexLiteral(pattern.bitmask());
        let pattern = HexLiteral(pattern.pattern());
        let parse_expr = self.parse_expr_tokens(isa);
        quote! {
            if (value & #bitmask) == #pattern {
                #parse_expr
            }
        }
    }

    fn parse_expr_tokens(&self, isa: &Isa) -> TokenStream {
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let record = data_type_struct.parse_record_tokens(isa);
                quote!(Self::#variant_ident #record)
            } else {
                let parse_expr = data.parse_expr_tokens(isa, None);
                quote!(Self::#variant_ident(#parse_expr))
            }
        } else {
            quote!(Self::#variant_ident)
        }
    }

    fn default_expr_tokens(&self) -> TokenStream {
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let record = data_type_struct.default_record_tokens();
                quote!(Self::#variant_ident #record)
            } else {
                let default_expr = data.default_expr_tokens();
                quote!(Self::#variant_ident(#default_expr))
            }
        } else {
            quote!(Self::#variant_ident)
        }
    }

    pub fn param_expr_tokens(
        &self,
        isa: &Isa,
        enum_name: &DataTypeName,
        value: &OpcodeParamValue,
    ) -> TokenStream {
        let enum_ident = enum_name.as_pascal_ident();
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let OpcodeParamValue::Struct(struct_params) = value else {
                    panic!();
                };
                let record = data_type_struct.param_record_tokens(isa, struct_params);
                quote!(#enum_ident::#variant_ident #record)
            } else {
                let value_tokens = match value {
                    OpcodeParamValue::Bits(bit_range) => bit_range.shift_mask_tokens(None),
                    OpcodeParamValue::Const(imm) => {
                        let lit = Literal::u32_unsuffixed(*imm);
                        quote!(#lit)
                    }
                    OpcodeParamValue::Expr(data_expr) => {
                        data_expr.as_tokens(Ident::new("value", Span::call_site()))
                    }
                    OpcodeParamValue::Enum(_, _) => {
                        panic!()
                    }
                    OpcodeParamValue::Struct(_) => panic!(),
                };
                let parse_expr = data.parse_expr_tokens(isa, Some(value_tokens));
                quote!(#enum_ident::#variant_ident(#parse_expr))
            }
        } else {
            quote!(#enum_ident::#variant_ident)
        }
    }

    fn pattern_destructure_tokens(&self) -> TokenStream {
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let field_names = data_type_struct.fields.iter().map(|f| f.name.as_ident());
                quote!(Self::#variant_ident { #(#field_names),* })
            } else {
                let data_ident = data.name.as_ident();
                quote!(Self::#variant_ident(#data_ident))
            }
        } else {
            quote!(Self::#variant_ident)
        }
    }

    fn write_expr_tokens(&self, isa: &Isa) -> TokenStream {
        let mut params: FormatParams = HashMap::new();
        if let Some(data) = &self.data {
            params.insert(data.name.0.clone(), data.clone());
        };
        let fmt_expr = if let Some(format) = &self.format {
            format.fmt_expr_tokens(isa, &params)
        } else if let Some(data) = &self.data {
            let value = Ident::new(&data.name.0, Span::call_site()).into_token_stream();
            data.fmt_expr_in_enum_variant_tokens(isa, value)
        } else {
            let variant_name = &self.name.0;
            quote!(formatter.write_str(#variant_name)?;)
        };
        let case_pattern = self.pattern_destructure_tokens();
        quote! {
            #case_pattern => {
                #fmt_expr
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DataTypeEnumVariantName(pub String);

impl DataTypeEnumVariantName {
    fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }
}

impl Display for DataTypeEnumVariantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataTypeStruct {
    format: Format,
    fields: Vec<DataType>,
}

impl DataTypeStruct {
    fn struct_tokens(&self, isa: &Isa, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        let record = self.record_tokens(isa, true);
        quote! {
            #[derive(PartialEq, Eq, Clone, Copy)]
            pub struct #name_ident #record
        }
    }

    fn record_tokens(&self, isa: &Isa, is_pub: bool) -> TokenStream {
        let fields = self.fields.iter().map(|field| field.field_tokens(isa, is_pub));
        quote!({ #(#fields),* })
    }

    fn parse_impl_tokens(&self, isa: &Isa, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        let record = self.parse_record_tokens(isa);
        quote! {
            impl #name_ident {
                pub(crate) fn parse(value: u32, pc: u32) -> Self {
                    Self #record
                }
            }
        }
    }

    fn parse_record_tokens(&self, isa: &Isa) -> TokenStream {
        let fields = self.fields.iter().map(|field| {
            let field_ident = Ident::new(&field.name.0, Span::call_site());
            let parse_expr = field.parse_expr_tokens(isa, None);
            quote!(#field_ident: #parse_expr)
        });
        quote!({ #(#fields),* })
    }

    fn parse_expr_tokens(&self, name: &DataTypeName, value: Option<TokenStream>) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        let value = value.unwrap_or_else(|| quote!(value));
        quote!(#name_ident::parse(#value, pc))
    }

    fn default_record_tokens(&self) -> TokenStream {
        let fields = self.fields.iter().map(|field| {
            let field_ident = Ident::new(&field.name.0, Span::call_site());
            let default_expr = field.default_expr_tokens();
            quote!(#field_ident: #default_expr)
        });
        quote!({ #(#fields),* })
    }

    fn default_expr_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = name.as_pascal_ident();
        quote!(#name_ident::default())
    }

    fn default_impl_body_tokens(&self) -> TokenStream {
        let record = self.default_record_tokens();
        quote!(Self #record)
    }

    fn param_record_tokens(
        &self,
        isa: &Isa,
        params: &IndexMap<String, OpcodeParamValue>,
    ) -> TokenStream {
        let fields = self.fields.iter().map(|field| {
            let name = &field.name.0;
            let field_ident = Ident::new(name, Span::call_site());
            let param_expr = if let Some(value) = params.get(name) {
                value.parse_expr_tokens(isa, field)
            } else {
                field.default_expr_tokens()
            };
            quote!(#field_ident: #param_expr)
        });
        quote!({ #(#fields),* })
    }

    pub fn param_tokens(
        &self,
        isa: &Isa,
        type_name: &DataTypeName,
        params: &IndexMap<String, OpcodeParamValue>,
    ) -> TokenStream {
        let type_name_ident = type_name.as_pascal_ident();
        let record = self.param_record_tokens(isa, params);
        quote! {
            #type_name_ident #record
        }
    }

    fn write_impl_body_tokens(&self, isa: &Isa) -> TokenStream {
        let fields = self.fields.iter().map(|f| f.name.as_ident());
        let fmt_expr = self.fmt_expr_tokens(isa);
        quote! {
            let Self { #(#fields),* } = self;
            #fmt_expr
        }
    }

    fn fmt_expr_tokens(&self, isa: &Isa) -> TokenStream {
        let mut params: FormatParams = HashMap::new();
        for field in &self.fields {
            params.insert(field.name.0.clone(), field.clone());
        }
        self.format.fmt_expr_tokens(isa, &params)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataExpr(SynExpr);

impl DataExpr {
    pub fn as_tokens(&self, input_ident: Ident) -> TokenStream {
        let mut replace = DataExprReplace { input_ident };
        let mut expr = self.0.0.clone();
        replace.visit_expr_mut(&mut expr);
        expr.into_token_stream()
    }
}

struct DataExprReplace {
    input_ident: Ident,
}

impl VisitMut for DataExprReplace {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match node {
            syn::Expr::Call(call) => {
                let fn_name = call.func.to_token_stream().to_string();
                match fn_name.as_str() {
                    "bits" => {
                        if call.args.len() != 1 {
                            panic!("bits function takes one argument");
                        }
                        let arg = &call.args[0];
                        let syn::Expr::Range(range) = arg else {
                            panic!("bits argument must be a range");
                        };
                        let Some(start) = &range.start else {
                            panic!("bits range must have a start");
                        };
                        let Some(end) = &range.end else {
                            panic!("bits range must have a start");
                        };
                        let start: u8 = start.to_token_stream().to_string().parse().unwrap();
                        let end: u8 = end.to_token_stream().to_string().parse().unwrap();
                        let range = BitRange(start..end);
                        let result = range.shift_mask_tokens(Some(self.input_ident.clone()));
                        *node = syn::parse2(quote!((#result))).unwrap();
                    }
                    "bit" => {
                        if call.args.len() != 1 {
                            panic!("bit function takes one argument");
                        }
                        let arg = &call.args[0];
                        let bit: u8 = arg.to_token_stream().to_string().parse().unwrap();
                        let range = BitRange(bit..bit + 1);
                        let result = range.shift_mask_tokens(Some(self.input_ident.clone()));
                        *node = syn::parse2(quote!((#result))).unwrap();
                    }
                    _ => panic!("Unknown data expression function {fn_name}"),
                }
            }
            syn::Expr::MethodCall(call) => {
                let fn_name = call.method.to_string();
                match fn_name.as_str() {
                    // These already exist in Rust
                    "rotate_right" | "wrapping_add" | "wrapping_sub" => {}

                    "sign_extend" => {
                        if call.args.len() != 1 {
                            panic!("sign_extend function takes one argument");
                        }
                        let arg = &call.args[0];
                        let bits: u32 = arg.to_token_stream().to_string().parse().unwrap();
                        let bits = Literal::u32_unsuffixed(bits);
                        let receiver = &call.receiver;
                        let result = quote!(((#receiver as i32) << #bits >> #bits) as u32);
                        *node = syn::parse2(quote!((#result))).unwrap();
                    }

                    "negate_if" => {
                        if call.args.len() != 1 {
                            panic!("negate_if function takes one argument");
                        }
                        let arg = &call.args[0];
                        let receiver = &call.receiver;
                        let result = quote! {
                            if #arg {
                                -(#receiver as i32)
                            } else {
                                #receiver as i32
                            }
                        };
                        *node = syn::parse2(quote!((#result))).unwrap();
                    }

                    _ => panic!("Unknown data expression method {fn_name}"),
                }
            }
            syn::Expr::Binary(binary) => {
                let left = &binary.left;
                let right = &binary.right;
                match binary.op {
                    syn::BinOp::Add(_) => {
                        let result = quote!(#left.wrapping_add(#right));
                        *node = syn::parse2(result).unwrap();
                    }
                    syn::BinOp::Sub(_) => {
                        let result = quote!(#left.wrapping_sub(#right));
                        *node = syn::parse2(result).unwrap();
                    }
                    _ => {}
                }
            }
            _ => (),
        }
        syn::visit_mut::visit_expr_mut(self, node);
    }
}

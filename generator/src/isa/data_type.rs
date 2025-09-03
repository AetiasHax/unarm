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

    pub fn validate(&self) -> Result<()> {
        for data_type in self.0.iter() {
            data_type.validate()?;
        }
        Ok(())
    }

    pub fn types_tokens(&self) -> TokenStream {
        let types = self.0.iter().filter_map(|dt| dt.type_decl_tokens());
        quote!(#(#types)*)
    }

    pub fn parse_impls_tokens(&self) -> TokenStream {
        let parse_impls = self.0.iter().filter_map(|dt| dt.parse_impl_tokens());
        quote!(#(#parse_impls)*)
    }

    pub fn default_impls_tokens(&self) -> TokenStream {
        let default_impls = self.0.iter().filter_map(|dt| dt.default_impl_tokens());
        quote!(#(#default_impls)*)
    }

    pub fn fmt_impls_tokens(&self, isa: &Isa) -> TokenStream {
        let fmt_impls = self.0.iter().filter_map(|dt| dt.fmt_impl_tokens(isa));
        quote!(#(#fmt_impls)*)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DataTypeName(pub String);

impl DataTypeName {
    // TODO: Use this method more
    fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
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
}

#[derive(Deserialize, Debug, Clone)]
pub enum DataTypeKind {
    #[serde(rename = "bool")]
    Bool(BitRange),
    #[serde(rename = "uint")]
    UInt(DataExpr),
    #[serde(rename = "enum")]
    Enum(DataTypeEnum),
    #[serde(rename = "struct")]
    Struct(DataTypeStruct),
    #[serde(rename = "type")]
    Type(DataTypeName, BitRange),
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
            DataTypeKind::Bool(_bit_range) => Ok(()),
            DataTypeKind::UInt(_bit_range) => Ok(()),
            DataTypeKind::Enum(data_type_enum) => data_type_enum.validate(&self.name),
            DataTypeKind::Struct(_data_type_struct) => Ok(()),
            DataTypeKind::Type(_data_type_name, _bit_range) => Ok(()),
        }
    }

    fn type_decl_tokens(&self) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool(_) => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Enum(data_type_enum) => Some(data_type_enum.enum_tokens(&self.name)),
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.struct_tokens(&self.name))
            }
            DataTypeKind::Type(_, _) => None,
        }
    }

    fn parse_impl_tokens(&self) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool(_) => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Enum(data_type_enum) => {
                Some(data_type_enum.parse_impl_tokens(&self.name))
            }
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.parse_impl_tokens(&self.name))
            }
            DataTypeKind::Type(_, _) => None,
        }
    }

    pub fn type_tokens(&self) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool(_) => quote!(bool),
            DataTypeKind::UInt(_) => quote!(u32),
            DataTypeKind::Enum(_) => {
                let name_ident = Ident::new(&snake_to_pascal_case(&self.name.0), Span::call_site());
                quote!(#name_ident)
            }
            DataTypeKind::Struct(_) => {
                let name_ident = Ident::new(&snake_to_pascal_case(&self.name.0), Span::call_site());
                quote!(#name_ident)
            }
            DataTypeKind::Type(name, _) => {
                let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
                quote!(#name_ident)
            }
        }
    }

    fn field_tokens(&self, is_pub: bool) -> TokenStream {
        let name_ident = Ident::new(&self.name.0, Span::call_site());
        let type_tokens = self.type_tokens();
        if is_pub {
            quote!(pub #name_ident: #type_tokens)
        } else {
            quote!(#name_ident: #type_tokens)
        }
    }

    pub fn parse_expr_tokens(&self, value: Option<TokenStream>) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool(bit_range) => {
                let value = value.unwrap_or_else(|| bit_range.shift_mask_tokens(None));
                quote!((#value) != 0)
            }
            DataTypeKind::UInt(data_expr) => {
                value.unwrap_or_else(|| data_expr.as_tokens(Ident::new("value", Span::call_site())))
            }
            DataTypeKind::Enum(data_type_enum) => {
                data_type_enum.parse_expr_tokens(&self.name, value)
            }
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.parse_expr_tokens(&self.name, value)
            }
            DataTypeKind::Type(data_type_name, bit_range) => {
                let type_ident =
                    Ident::new(&snake_to_pascal_case(&data_type_name.0), Span::call_site());
                let value = value.unwrap_or_else(|| bit_range.shift_mask_tokens(None));
                quote!(#type_ident::from(#value))
            }
        }
    }

    pub fn default_expr_tokens(&self) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool(_) => quote!(false),
            DataTypeKind::UInt(_) => quote!(0),
            DataTypeKind::Enum(data_type_enum) => data_type_enum.default_expr_tokens(&self.name),
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.default_expr_tokens(&self.name)
            }
            DataTypeKind::Type(data_type_name, _) => {
                let type_ident =
                    Ident::new(&snake_to_pascal_case(&data_type_name.0), Span::call_site());
                quote!(#type_ident::default())
            }
        }
    }

    fn default_impl_tokens(&self) -> Option<TokenStream> {
        let default_expr = match &self.kind {
            DataTypeKind::Bool(_) => return None,
            DataTypeKind::UInt(_) => return None,
            DataTypeKind::Enum(data_type_enum) => data_type_enum.default_impl_body_tokens()?,
            DataTypeKind::Struct(data_type_struct) => data_type_struct.default_impl_body_tokens(),
            DataTypeKind::Type(_, _) => return None,
        };
        let name_ident = Ident::new(&snake_to_pascal_case(&self.name.0), Span::call_site());
        Some(quote! {
            impl Default for #name_ident {
                fn default() -> Self {
                    #default_expr
                }
            }
        })
    }

    pub fn fmt_impl_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool(_) => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Enum(data_type_enum) => Some(data_type_enum.fmt_impl_body_tokens(isa)),
            DataTypeKind::Struct(data_type_struct) => {
                Some(data_type_struct.fmt_impl_body_tokens(isa))
            }
            DataTypeKind::Type(_, _) => None,
        }
    }

    fn fmt_impl_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match &self.kind {
            DataTypeKind::Bool(_) => return None,
            DataTypeKind::UInt(_) => return None,
            DataTypeKind::Enum(_) => {}
            DataTypeKind::Struct(_) => {}
            DataTypeKind::Type(_, _) => return None,
        };
        let display_expr = self.fmt_impl_body_tokens(isa);
        let name_ident = Ident::new(&snake_to_pascal_case(&self.name.0), Span::call_site());
        Some(quote! {
            impl #name_ident {
                pub fn fmt(&self, options: &Options, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    #display_expr
                }
            }
        })
    }

    pub fn display_expr_tokens(&self, isa: &Isa, value: TokenStream) -> TokenStream {
        match &self.kind {
            DataTypeKind::Bool(_) => {
                let name = &self.name.0;
                quote! {
                    if *#value {
                        f.write_str(#name)?;
                    }
                }
            }
            DataTypeKind::UInt(_) => {
                quote!(write!(f, "{:#x}", #value)?;)
            }
            DataTypeKind::Enum(_) => {
                quote!(#value.fmt(options, f)?;)
            }
            DataTypeKind::Struct(data_type_struct) => data_type_struct.fmt_impl_body_tokens(isa),
            DataTypeKind::Type(_, _) => {
                quote!(#value.fmt(options, f)?;)
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

    fn enum_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        let variants = self.variants.values().map(|v| v.variant_tokens());
        quote! {
            #[derive(PartialEq, Eq)]
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

    fn parse_impl_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());

        let fn_body = if self.has_optional_bits() {
            let variants =
                self.variants.iter().map(|(pattern, variant)| variant.parse_if_tokens(pattern));
            quote! {
                #(#variants)else*
                else {
                    panic!();
                }
            }
        } else {
            let variants =
                self.variants.iter().map(|(pattern, variant)| variant.parse_match_tokens(pattern));
            quote! {
                match value {
                    #(#variants),*,
                    _ => panic!(),
                }
            }
        };

        quote! {
            impl From<u32> for #name_ident {
                fn from(value: u32) -> Self {
                    #fn_body
                }
            }
        }
    }

    fn parse_expr_tokens(&self, name: &DataTypeName, value: Option<TokenStream>) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        let value = value.unwrap_or_else(|| self.bits.shift_mask_tokens(None));
        quote!(#name_ident::from(#value))
    }

    fn default_impl_body_tokens(&self) -> Option<TokenStream> {
        let default = self.default.as_ref()?;
        let Some((_, default_variant)) = self.get_variant(default) else {
            panic!();
        };
        Some(default_variant.default_expr_tokens())
    }

    fn default_expr_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        quote!(#name_ident::default())
    }

    fn fmt_impl_body_tokens(&self, isa: &Isa) -> TokenStream {
        let variants = self.variants.iter().map(|(_, variant)| variant.display_expr_tokens(isa));
        quote! {
            match self {
                #(#variants),*
            }
            Ok(())
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

    fn variant_tokens(&self) -> TokenStream {
        let variant_ident = self.as_ident();
        let variant = if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let record = data_type_struct.record_tokens(false);
                quote!(#variant_ident #record)
            } else {
                let variant_data = data.type_tokens();
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

    fn parse_match_tokens(&self, pattern: &Pattern) -> TokenStream {
        let pattern = HexLiteral(pattern.pattern());
        let parse_expr = self.parse_expr_tokens();
        quote!(#pattern => #parse_expr)
    }

    fn parse_if_tokens(&self, pattern: &Pattern) -> TokenStream {
        let bitmask = HexLiteral(pattern.bitmask());
        let pattern = HexLiteral(pattern.pattern());
        let parse_expr = self.parse_expr_tokens();
        quote! {
            if (value & #bitmask) == #pattern {
                #parse_expr
            }
        }
    }

    fn parse_expr_tokens(&self) -> TokenStream {
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let record = data_type_struct.parse_record_tokens();
                quote!(Self::#variant_ident #record)
            } else {
                let parse_expr = data.parse_expr_tokens(None);
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
        enum_name: &DataTypeName,
        value: &OpcodeParamValue,
    ) -> TokenStream {
        let enum_ident = Ident::new(&snake_to_pascal_case(&enum_name.0), Span::call_site());
        let variant_ident = self.as_ident();
        if let Some(data) = &self.data {
            if let DataTypeKind::Struct(data_type_struct) = &data.kind {
                let OpcodeParamValue::Struct(struct_params) = value else {
                    panic!();
                };
                let record = data_type_struct.param_record_tokens(struct_params);
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
                    OpcodeParamValue::Enum(data_type_enum_variant_name, opcode_param_value) => {
                        panic!()
                    }
                    OpcodeParamValue::Struct(index_map) => panic!(),
                };
                let parse_expr = data.parse_expr_tokens(Some(value_tokens));
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
                quote!(Self::#variant_ident(data))
            }
        } else {
            quote!(Self::#variant_ident)
        }
    }

    fn display_expr_tokens(&self, isa: &Isa) -> TokenStream {
        let mut params: FormatParams = HashMap::new();
        if let Some(data) = &self.data {
            params.insert("data".into(), data.clone());
        };
        let display_expr = if let Some(format) = &self.format {
            format.display_expr_tokens(isa, &params)
        } else if let Some(data) = &self.data {
            let value = Ident::new(&data.name.0, Span::call_site()).into_token_stream();
            data.display_expr_tokens(isa, value)
        } else {
            let variant_name = &self.name.0;
            quote!(f.write_str(#variant_name)?;)
        };
        let case_pattern = self.pattern_destructure_tokens();
        quote! {
            #case_pattern => {
                #display_expr
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DataTypeEnumVariantName(pub String);

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
    fn struct_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        let record = self.record_tokens(true);
        quote!(pub struct #name_ident #record)
    }

    fn record_tokens(&self, is_pub: bool) -> TokenStream {
        let fields = self.fields.iter().map(|field| field.field_tokens(is_pub));
        quote!({ #(#fields),* })
    }

    fn parse_impl_tokens(&self, name: &DataTypeName) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        let record = self.parse_record_tokens();
        quote! {
            impl From<u32> for #name_ident {
                fn from(value: u32) -> Self {
                    Self #record
                }
            }
        }
    }

    fn parse_record_tokens(&self) -> TokenStream {
        let fields = self.fields.iter().map(|field| {
            let field_ident = Ident::new(&field.name.0, Span::call_site());
            let parse_expr = field.parse_expr_tokens(None);
            quote!(#field_ident: #parse_expr)
        });
        quote!({ #(#fields),* })
    }

    fn parse_expr_tokens(&self, name: &DataTypeName, value: Option<TokenStream>) -> TokenStream {
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        let value = value.unwrap_or_else(|| quote!(value));
        quote!(#name_ident::from(#value))
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
        let name_ident = Ident::new(&snake_to_pascal_case(&name.0), Span::call_site());
        quote!(#name_ident::default())
    }

    fn default_impl_body_tokens(&self) -> TokenStream {
        let record = self.default_record_tokens();
        quote!(Self #record)
    }

    fn param_record_tokens(&self, params: &IndexMap<String, OpcodeParamValue>) -> TokenStream {
        let fields = self.fields.iter().map(|field| {
            let name = &field.name.0;
            let field_ident = Ident::new(name, Span::call_site());
            let param_expr = if let Some(value) = params.get(name) {
                value.parse_expr_tokens(field)
            } else {
                field.default_expr_tokens()
            };
            quote!(#field_ident: #param_expr)
        });
        quote!({ #(#fields),* })
    }

    pub fn param_tokens(
        &self,
        type_name: &DataTypeName,
        params: &IndexMap<String, OpcodeParamValue>,
    ) -> TokenStream {
        let type_name_ident = Ident::new(&snake_to_pascal_case(&type_name.0), Span::call_site());
        let record = self.param_record_tokens(params);
        quote! {
            #type_name_ident #record
        }
    }

    fn fmt_impl_body_tokens(&self, isa: &Isa) -> TokenStream {
        let mut params: FormatParams = HashMap::new();
        for field in &self.fields {
            params.insert(field.name.0.clone(), field.clone());
        }
        self.format.display_expr_tokens(isa, &params)
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
        if let syn::Expr::Call(call) = node {
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
        syn::visit_mut::visit_expr_mut(self, node);
    }
}

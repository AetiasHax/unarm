use std::{collections::HashMap, fmt::Debug, str::FromStr};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use serde::Deserialize;
use syn::{Ident, visit_mut::VisitMut};

use crate::{
    isa::{DataType, Isa, SynExpr},
    util::str::snake_to_pascal_case,
};

#[derive(Deserialize, Debug, Clone)]
pub enum Format {
    #[serde(rename = "if")]
    If(IfFormat),
    #[serde(rename = "fmt")]
    Fragments(FragmentsFormat),
}

pub type FormatParams = HashMap<String, DataType>;

impl Format {
    pub fn fmt_expr_tokens(&self, isa: &Isa, params: &FormatParams) -> TokenStream {
        match self {
            Format::If(if_format) => if_format.fmt_expr_tokens(isa, params),
            Format::Fragments(fragments_format) => fragments_format.fmt_expr_tokens(params),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct IfFormat {
    cond: FormatCond,
    #[serde(rename = "then")]
    if_true: Box<Format>,
    #[serde(rename = "else")]
    if_false: Box<Format>,
}

impl IfFormat {
    fn fmt_expr_tokens(&self, isa: &Isa, params: &FormatParams) -> TokenStream {
        let condition = self.cond.as_tokens();
        let fmt_true = self.if_true.fmt_expr_tokens(isa, params);
        let fmt_false = self.if_false.fmt_expr_tokens(isa, params);
        quote! {
            if #condition {
                #fmt_true
            } else {
                #fmt_false
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct FormatCond(SynExpr);

impl FormatCond {
    fn as_tokens(&self) -> TokenStream {
        let mut replace = FormatCondReplace;
        let mut expr = self.0.0.clone();
        replace.visit_expr_mut(&mut expr);
        expr.into_token_stream()
    }
}

struct FormatCondReplace;

impl VisitMut for FormatCondReplace {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        if let syn::Expr::Call(call) = node {
            let fn_name = call.func.to_token_stream().to_string();
            match fn_name.as_str() {
                "option" => {
                    if call.args.len() != 1 {
                        panic!("option function takes one argument");
                    }
                    let option = &call.args[0];
                    *node = syn::parse2(quote!(options.#option)).unwrap();
                }
                "field" => {
                    if call.args.len() != 1 {
                        panic!("field function takes one argument");
                    }
                    let field = &call.args[0];
                    *node = syn::parse2(quote!(*#field)).unwrap();
                }
                "enum_variant" => {
                    if call.args.len() != 2 {
                        panic!("enum_variant function takes two arguments");
                    }
                    let type_name = call.args[0].to_token_stream().to_string();
                    let variant_name = call.args[1].to_token_stream().to_string();
                    let type_ident =
                        Ident::new(&snake_to_pascal_case(&type_name), Span::call_site());
                    let variant_ident =
                        Ident::new(&snake_to_pascal_case(&variant_name), Span::call_site());
                    *node = syn::parse2(quote!(#type_ident::#variant_ident)).unwrap();
                }
                _ => panic!("Unknown format condition function {fn_name}"),
            }
        }
        syn::visit_mut::visit_expr_mut(self, node);
    }
}

#[derive(Debug, Clone)]
pub struct FragmentsFormat {
    fragments: Vec<FormatFragment>,
}

impl FragmentsFormat {
    fn fmt_expr_tokens(&self, params: &FormatParams) -> TokenStream {
        let fragments = self.fragments.iter().map(|f| f.fmt_expr_tokens(params));
        quote!(#(#fragments)*)
    }
}

#[derive(Debug, Clone)]
enum FormatFragment {
    Text(String),
    Space,
    Separator,
    Param(String),
}

impl FormatFragment {
    fn fmt_expr_tokens(&self, params: &FormatParams) -> TokenStream {
        match self {
            FormatFragment::Text(text) => quote!(f.write_str(#text)?;),
            FormatFragment::Space => quote!(f.write_space()?;),
            FormatFragment::Separator => quote!(f.write_separator()?;),
            FormatFragment::Param(param_name) => {
                let Some(param) = params.get(param_name) else {
                    panic!();
                };
                let param_ident = Ident::new(param_name, Span::call_site());
                param.fmt_expr_tokens(quote!(#param_ident))
            }
        }
    }
}

impl FromStr for FragmentsFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fragments = Vec::new();
        let mut token = String::new();
        let mut parsing_param = false;

        fn push_str_fragments(fragments: &mut Vec<FormatFragment>, mut s: &str) {
            while !s.is_empty() {
                let (fragment, rest) = {
                    if let Some(rest) = s.strip_prefix(" ") {
                        (FormatFragment::Space, rest)
                    } else if let Some(rest) = s.strip_prefix(", ") {
                        (FormatFragment::Separator, rest)
                    } else {
                        (FormatFragment::Text(s.to_string()), "")
                    }
                };
                fragments.push(fragment);
                s = rest;
            }
        }

        for ch in s.chars() {
            match ch {
                '(' => {
                    if !token.is_empty() {
                        push_str_fragments(&mut fragments, &token);
                        token = String::new();
                    }
                    parsing_param = true;
                }
                ')' => {
                    if parsing_param {
                        fragments.push(FormatFragment::Param(token));
                        token = String::new();
                        parsing_param = false;
                    } else {
                        return Err("Unmatched closing parenthesis".into());
                    }
                }
                _ => {
                    token.push(ch);
                }
            }
        }
        if !token.is_empty() {
            if parsing_param {
                fragments.push(FormatFragment::Param(token));
            } else {
                push_str_fragments(&mut fragments, &token);
            }
        }
        Ok(Self { fragments })
    }
}

impl<'de> Deserialize<'de> for FragmentsFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::util::str::snake_to_pascal_case;

#[derive(Deserialize, Debug)]
pub struct IsaOptions(IndexMap<String, IsaOption>);

impl IsaOptions {
    pub fn struct_tokens(&self) -> TokenStream {
        let fields = self.0.iter().map(|(name, option)| option.struct_field_tokens(name));
        quote! {
            pub struct Options {
                #(#fields),*
            }
        }
    }

    pub fn internal_types_tokens(&self) -> TokenStream {
        let types = self.0.iter().filter_map(|(name, option)| option.internal_type_tokens(name));
        quote!(#(#types)*)
    }
}

#[derive(Deserialize, Debug)]
pub struct IsaOption {
    description: String,
    kind: OptionKind,
}

impl IsaOption {
    fn struct_field_tokens(&self, name: &str) -> TokenStream {
        let name_ident = Ident::new(name, Span::call_site());
        let option_type = self.kind.type_tokens(name);
        let field_tokens = quote!(pub #name_ident: #option_type);
        let description = &self.description;
        quote! {
            #[doc = #description]
            #field_tokens
        }
    }

    fn internal_type_tokens(&self, name: &str) -> Option<TokenStream> {
        self.kind.internal_type_tokens(name)
    }
}

#[derive(Deserialize, Debug)]
pub enum OptionKind {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "enum")]
    Enum(Vec<OptionEnumVariant>),
    #[serde(rename = "version")]
    Version,
}

impl OptionKind {
    fn type_tokens(&self, name: &str) -> TokenStream {
        match self {
            OptionKind::Bool => quote!(bool),
            OptionKind::Enum(_) => {
                let type_name = Ident::new(&snake_to_pascal_case(name), Span::call_site());
                quote!(#type_name)
            }
            OptionKind::Version => quote!(Version),
        }
    }

    fn internal_type_tokens(&self, name: &str) -> Option<TokenStream> {
        match self {
            OptionKind::Bool => None,
            OptionKind::Enum(variants) => {
                let type_name = Ident::new(&snake_to_pascal_case(name), Span::call_site());
                let variants = variants.iter().map(|v| v.variant_tokens());
                Some(quote! {
                    #[derive(PartialEq, Eq)]
                    pub enum #type_name {
                        #(#variants),*
                    }
                })
            }
            OptionKind::Version => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct OptionEnumVariant {
    name: String,
    description: String,
}

impl OptionEnumVariant {
    fn variant_tokens(&self) -> TokenStream {
        let variant_ident = Ident::new(&snake_to_pascal_case(&self.name), Span::call_site());
        let description = &self.description;
        quote! {
            #[doc = #description]
            #variant_ident
        }
    }
}

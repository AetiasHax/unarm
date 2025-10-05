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
            #[derive(Debug, Clone)]
            pub struct Options {
                #(#fields),*
            }
        }
    }

    pub fn internal_types_tokens(&self) -> TokenStream {
        let types = self.0.iter().filter_map(|(name, option)| option.internal_type_tokens(name));
        quote!(#(#types)*)
    }

    pub fn default_impls_tokens(&self) -> TokenStream {
        let fields = self.0.iter().map(|(name, option)| {
            let ident = Ident::new(name, Span::call_site());
            let value = option.kind.default_value_tokens(name);
            quote!(#ident: #value)
        });
        quote! {
            impl Default for Options {
                fn default() -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    Bool(OptionBool),
    #[serde(rename = "enum")]
    Enum(Vec<OptionEnumVariant>),
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "extensions")]
    Extensions,
}

impl OptionKind {
    fn type_tokens(&self, name: &str) -> TokenStream {
        match self {
            OptionKind::Bool(_) => quote!(bool),
            OptionKind::Enum(_) => {
                let type_name = Ident::new(&snake_to_pascal_case(name), Span::call_site());
                quote!(#type_name)
            }
            OptionKind::Version => quote!(Version),
            OptionKind::Extensions => quote!(Extensions),
        }
    }

    fn internal_type_tokens(&self, name: &str) -> Option<TokenStream> {
        match self {
            OptionKind::Bool(_) => None,
            OptionKind::Enum(variants) => {
                let type_name = Ident::new(&snake_to_pascal_case(name), Span::call_site());
                let mut variants = variants.iter().map(|v| v.variant_tokens());
                let first_variant = variants.next();
                Some(quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
                    pub enum #type_name {
                        #[default]
                        #first_variant,
                        #(#variants),*
                    }
                })
            }
            OptionKind::Version => None,
            OptionKind::Extensions => None,
        }
    }

    fn default_value_tokens(&self, name: &str) -> TokenStream {
        match self {
            OptionKind::Bool(option_bool) => {
                if option_bool.default_value {
                    quote!(true)
                } else {
                    quote!(false)
                }
            }
            OptionKind::Enum(_) => {
                let type_name = Ident::new(&snake_to_pascal_case(name), Span::call_site());
                quote!(#type_name::default())
            }
            OptionKind::Version => quote!(Version::default()),
            OptionKind::Extensions => quote!(Extensions::default()),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OptionBool {
    #[serde(rename = "default", default)]
    default_value: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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

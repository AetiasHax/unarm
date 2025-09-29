use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de};
use syn::Ident;

use crate::{isa::Isa, util::str::snake_to_pascal_case};

#[derive(Deserialize, Debug)]
pub struct IsaExtensions(Vec<IsaExtension>);

impl IsaExtensions {
    pub fn iter(&self) -> impl Iterator<Item = &IsaExtension> {
        self.0.iter()
    }

    fn struct_inner_type(&self) -> TokenStream {
        match self.0.len() {
            0..8 => quote!(u8),
            8..16 => quote!(u16),
            16..32 => quote!(u32),
            32..64 => quote!(u64),
            _ => panic!("Too many extensions"),
        }
    }

    pub fn enum_tokens(&self) -> TokenStream {
        let extensions = self.0.iter().map(|e| {
            let extension = &e.0;
            let ident = e.as_ident();
            quote! {
                #[cfg(feature = #extension)]
                #ident
            }
        });
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
            pub enum Extension {
                #(#extensions),*
            }

            impl Extension {
                pub const fn bit(self) -> #inner_type {
                    1 << self as #inner_type
                }
            }
        }
    }

    pub fn struct_tokens(&self) -> TokenStream {
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub struct Extensions(#inner_type);

            impl Extensions {
                pub const fn none() -> Self {
                    Self(0)
                }

                pub const fn all() -> Self {
                    Self(#inner_type::MAX)
                }

                pub fn with(self, extension: Extension) -> Self {
                    Self(self.0 | extension.bit())
                }

                pub const fn of(extensions: &[Extension]) -> Self {
                    let mut mask = 0;
                    let mut i = 0;
                    loop {
                        if i >= extensions.len() {
                            break;
                        }
                        mask |= extensions[i].bit();
                        i += 1;
                    }
                    Self(mask)
                }

                pub fn has_all(self, extensions: Extensions) -> bool {
                    (self.0 & extensions.0) == self.0
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsaExtension(String);

impl IsaExtension {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn as_ident(&self) -> Ident {
        Ident::new(&snake_to_pascal_case(&self.0), Span::call_site())
    }
}

#[derive(Debug)]
pub struct IsaExtensionPattern {
    prefix: String,
    wildcard: bool,
}

impl IsaExtensionPattern {
    pub fn matches(&self, extension: &IsaExtension) -> bool {
        if self.wildcard {
            extension.name().starts_with(&self.prefix)
        } else {
            extension.name() == self.prefix
        }
    }
}

impl<'de> Deserialize<'de> for IsaExtensionPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(prefix) = s.strip_suffix('*') {
            Ok(IsaExtensionPattern { prefix: prefix.into(), wildcard: true })
        } else if s.contains('*') {
            Err(de::Error::custom("Wildcard '*' only allowed at the end"))
        } else {
            Ok(IsaExtensionPattern { prefix: s, wildcard: false })
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct IsaExtensionPatterns(Vec<IsaExtensionPattern>);

impl IsaExtensionPatterns {
    pub fn extensions<'a>(&self, isa: &'a Isa) -> Vec<&'a IsaExtension> {
        isa.extensions()
            .iter()
            .filter(|&version| self.0.iter().any(|pattern| pattern.matches(version)))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de};
use syn::Ident;

use crate::{isa::Isa, util::str::snake_to_pascal_case};

#[derive(Deserialize, Debug)]
pub struct IsaVersions(Vec<IsaVersion>);

impl IsaVersions {
    pub fn iter(&self) -> impl Iterator<Item = &IsaVersion> {
        self.0.iter()
    }

    pub fn get_matches(&self, pattern: &IsaVersionPattern) -> impl Iterator<Item = &IsaVersion> {
        self.0.iter().filter(|v| pattern.matches(v))
    }

    fn struct_inner_type(&self) -> TokenStream {
        match self.0.len() {
            0..8 => quote!(u8),
            8..16 => quote!(u16),
            16..32 => quote!(u32),
            32..64 => quote!(u64),
            _ => panic!("Too many versions"),
        }
    }

    pub fn enum_tokens(&self) -> TokenStream {
        let versions = self.0.iter().map(|v| v.as_ident());
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Clone, Copy)]
            pub enum Version {
                #(#versions),*
            }

            impl Version {
                pub const fn bit(self) -> #inner_type {
                    1 << self as #inner_type
                }
            }
        }
    }

    pub fn struct_tokens(&self) -> TokenStream {
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Clone, Copy)]
            pub struct Versions(#inner_type);

            impl Versions {
                pub const fn none() -> Self {
                    Self(0)
                }

                pub const fn all() -> Self {
                    Self(#inner_type::MAX)
                }

                pub fn with(self, version: Version) -> Self {
                    Self(self.0 | version.bit())
                }

                pub const fn of(versions: &[Version]) -> Self {
                    let mut mask = 0;
                    let mut i = 0;
                    loop {
                        if i >= versions.len() {
                            break;
                        }
                        mask |= versions[i].bit();
                        i += 1;
                    }
                    Self(mask)
                }

                pub fn has(self, version: Version) -> bool {
                    (self.0 & version.bit()) != 0
                }
            }
        }
    }

    pub fn matches_all(&self, versions: &IsaVersionPatterns) -> bool {
        self.0.iter().all(|v| versions.iter().any(|pattern| pattern.matches(v)))
    }
}

#[derive(Deserialize, Debug)]
pub struct IsaVersion(String);

impl IsaVersion {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn as_ident(&self) -> Ident {
        Ident::new(&snake_to_pascal_case(&self.0), Span::call_site())
    }
}

#[derive(Debug)]
pub struct IsaVersionPattern {
    prefix: String,
    wildcard: bool,
}

impl IsaVersionPattern {
    pub fn matches(&self, version: &IsaVersion) -> bool {
        if self.wildcard {
            version.name().starts_with(&self.prefix)
        } else {
            version.name() == self.prefix
        }
    }
}

impl<'de> Deserialize<'de> for IsaVersionPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(prefix) = s.strip_suffix('*') {
            Ok(IsaVersionPattern { prefix: prefix.into(), wildcard: true })
        } else if s.contains('*') {
            Err(de::Error::custom("Wildcard '*' only allowed at the end"))
        } else {
            Ok(IsaVersionPattern { prefix: s, wildcard: false })
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IsaVersionPatterns(Vec<IsaVersionPattern>);

impl IsaVersionPatterns {
    pub fn iter(&self) -> impl Iterator<Item = &IsaVersionPattern> {
        self.0.iter()
    }

    pub fn versions<'a>(&self, isa: &'a Isa) -> Vec<&'a IsaVersion> {
        isa.versions()
            .iter()
            .filter(|&version| self.0.iter().any(|pattern| pattern.matches(version)))
            .collect()
    }
}

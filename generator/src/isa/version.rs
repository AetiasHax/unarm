use indexmap::IndexSet;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de};
use syn::Ident;

use crate::{
    isa::{Arch, Isa},
    util::str::snake_to_pascal_case,
};

#[derive(Deserialize, Debug)]
pub struct IsaVersions(Vec<IsaVersion>);

impl IsaVersions {
    pub fn iter(&self) -> impl Iterator<Item = &IsaVersion> {
        self.0.iter()
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
        let versions = self.0.iter().map(|v| {
            let version = v.name();
            let ident = v.as_ident();
            quote! {
                #[cfg(feature = #version)]
                #ident
            }
        });
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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
            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    pub fn has_all(&self, versions: &IsaVersionSet, arch: Option<Arch>) -> bool {
        self.0
            .iter()
            .filter(|v| arch != Some(Arch::Thumb) || v.thumb)
            .all(|v| versions.0.contains(v))
    }

    pub fn default_impl_tokens(&self) -> TokenStream {
        let versions = self.0.iter().rev().map(|v| {
            let version = v.name();
            let ident = v.as_ident();
            quote! {
                #[cfg(feature = #version)]
                return Self::#ident;
            }
        });
        quote! {
            impl Default for Version {
                fn default() -> Self {
                    #(#versions)*
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsaVersion {
    name: String,
    #[serde(default = "isa_version_thumb_default")]
    thumb: bool,
}

fn isa_version_thumb_default() -> bool {
    true
}

impl IsaVersion {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn as_ident(&self) -> Ident {
        Ident::new(&snake_to_pascal_case(&self.name), Span::call_site())
    }
}

#[derive(Clone)]
pub struct IsaVersionSet(pub IndexSet<IsaVersion>);

impl IsaVersionSet {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    pub fn is_same(&self, thumb_versions: &IsaVersionSet) -> bool {
        let mut versions = self.clone();
        versions.0.retain(|v| v.thumb);
        versions.0 == thumb_versions.0
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

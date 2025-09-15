use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de};
use syn::Ident;

use crate::util::str::snake_to_pascal_case;

#[derive(Deserialize, Debug)]
pub struct IsaVersions(Vec<IsaVersion>);

impl IsaVersions {
    pub fn get_matches(&self, pattern: &IsaVersionPattern) -> impl Iterator<Item = &IsaVersion> {
        self.0.iter().filter(|v| pattern.matches(v))
    }

    pub fn enum_tokens(&self) -> TokenStream {
        let versions = self.0.iter().map(|v| v.as_ident());
        quote! {
            #[derive(Clone, Copy)]
            pub enum Version {
                #(#versions),*
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IsaVersion(String);

impl IsaVersion {
    pub fn name(&self) -> &str {
        &self.0
    }

    fn as_ident(&self) -> Ident {
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

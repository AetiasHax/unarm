use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de};
use syn::Ident;

#[derive(Deserialize, Debug)]
pub struct IsaExtensions(Vec<IsaExtension>);

impl IsaExtensions {
    pub fn get_matches(
        &self,
        pattern: &IsaExtensionPattern,
    ) -> impl Iterator<Item = &IsaExtension> {
        self.0.iter().filter(|e| pattern.matches(e))
    }

    fn struct_inner_type(&self) -> TokenStream {
        match self.0.len() {
            0 => quote!(()),
            1..=8 => quote!(u8),
            9..=16 => quote!(u16),
            17..=32 => quote!(u32),
            33..=64 => quote!(u64),
            _ => panic!("Too many extensions"),
        }
    }

    pub fn struct_tokens(&self) -> TokenStream {
        let inner_type = self.struct_inner_type();
        quote! {
            #[derive(Clone, Copy)]
            pub struct Extensions(#inner_type);
        }
    }

    pub fn struct_impl_tokens(&self) -> TokenStream {
        let inner_type = self.struct_inner_type();
        let with_fns = self.0.iter().enumerate().map(|(i, e)| e.with_fn_tokens(i));
        quote! {
            impl Extensions {
                pub fn none() -> Self {
                    Self(0)
                }

                pub fn all() -> Self {
                    Self(#inner_type::MAX)
                }

                #(#with_fns)*
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IsaExtension(String);

impl IsaExtension {
    pub fn name(&self) -> &str {
        &self.0
    }

    fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }

    fn with_fn_tokens(&self, index: usize) -> TokenStream {
        let fn_name = format!("with_{}", self.0);
        let fn_ident = Ident::new(&fn_name, Span::call_site());
        let flag = Literal::usize_unsuffixed(1 << index);
        quote! {
            pub fn #fn_ident(self) -> Self {
                Self(self.0 | #flag)
            }
        }
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

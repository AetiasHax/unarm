use indexmap::IndexSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::isa::{Isa, IsaExtension, IsaVersion};

pub fn cfg_condition_tokens(
    versions: &IndexSet<IsaVersion>,
    extensions: &IndexSet<IsaExtension>,
    isa: &Isa,
) -> Option<TokenStream> {
    let mut extension_features = extensions.iter().map(|ext| {
        let name = ext.name();
        quote!(feature = #name)
    });
    let mut version_features = versions.iter().map(|ver| {
        let name = ver.name();
        quote!(feature = #name)
    });

    match (isa.versions.has_all(versions), extensions.is_empty()) {
        (true, true) => None,
        (true, false) => {
            if extensions.len() == 1 {
                let feature = extension_features.next().unwrap();
                Some(quote!(#[cfg(#feature)]))
            } else {
                Some(quote!(#[cfg(all(#(#extension_features),*))]))
            }
        }
        (false, true) => {
            if versions.len() == 1 {
                let feature = version_features.next().unwrap();
                Some(quote!(#[cfg(#feature)]))
            } else {
                Some(quote!(#[cfg(any(#(#version_features),*))]))
            }
        }
        (false, false) => {
            if versions.len() == 1 {
                let version_feature = version_features.next().unwrap();
                Some(quote!(#[cfg(all(#(#extension_features),*, #version_feature))]))
            } else {
                Some(quote!(#[cfg(all(#(#extension_features),*, any(#(#version_features),*)))]))
            }
        }
    }
}

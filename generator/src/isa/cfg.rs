use indexmap::IndexSet;
use proc_macro2::TokenStream;
use quote::quote;

use crate::isa::{Arch, Isa, IsaExtension, IsaVersionSet};

pub fn cfg_condition_tokens(
    versions: &IsaVersionSet,
    extensions: &IndexSet<IsaExtension>,
    isa: &Isa,
    arch: Option<Arch>,
) -> Option<TokenStream> {
    let mut extension_features = extensions.iter().map(|ext| {
        let name = ext.name();
        quote!(feature = #name)
    });
    let extension_features = match extension_features.len() {
        0 => None,
        1 => {
            let feature = extension_features.next().unwrap();
            Some(feature)
        }
        _ => Some(quote!(all(#(#extension_features),*))),
    };

    let version_features = if isa.versions().has_all(versions, arch) {
        None
    } else {
        let version_features = versions
            .0
            .iter()
            .map(|ver| {
                let name = ver.name();
                quote!(feature = #name)
            })
            .collect::<Vec<_>>();
        match version_features.len() {
            0 => return None,
            1 => {
                let feature = &version_features[0];
                Some(quote!(#feature))
            }
            _ => Some(quote!(any(#(#version_features),*))),
        }
    };

    let arch_feature = arch.map(|arch| {
        let arch_name = arch.to_string();
        quote!(feature = #arch_name)
    });

    let all_features = arch_feature
        .iter()
        .chain(extension_features.iter())
        .chain(version_features.iter())
        .collect::<Vec<_>>();

    match all_features.len() {
        0 => None,
        1 => {
            let feature = all_features[0];
            Some(quote!(#feature))
        }
        _ => Some(quote!(all(#(#all_features),*))),
    }
}

pub fn cfg_attribute_tokens(
    arm_versions: &IsaVersionSet,
    arm_extensions: &IndexSet<IsaExtension>,
    thumb_versions: &IsaVersionSet,
    thumb_extensions: &IndexSet<IsaExtension>,
    isa: &Isa,
) -> Option<TokenStream> {
    if arm_versions.is_same(thumb_versions) && arm_extensions == thumb_extensions {
        cfg_condition_tokens(arm_versions, arm_extensions, isa, None)
            .map(|cond| quote!(#[cfg(#cond)]))
    } else {
        let arm_cond = cfg_condition_tokens(arm_versions, arm_extensions, isa, Some(Arch::Arm));
        let thumb_cond =
            cfg_condition_tokens(thumb_versions, thumb_extensions, isa, Some(Arch::Thumb));
        match (arm_cond, thumb_cond) {
            (None, None) => None,
            (None, Some(thumb)) => Some(quote!(#[cfg(#thumb)])),
            (Some(arm), None) => Some(quote!(#[cfg(#arm)])),
            (Some(arm), Some(thumb)) => Some(quote!(#[cfg(any(#arm, #thumb))])),
        }
    }
}

pub fn cfg_attribute_single_arch_tokens(
    versions: &IsaVersionSet,
    extensions: &IndexSet<IsaExtension>,
    isa: &Isa,
    arch: Arch,
) -> Option<TokenStream> {
    cfg_condition_tokens(versions, extensions, isa, Some(arch)).map(|cond| quote!(#[cfg(#cond)]))
}

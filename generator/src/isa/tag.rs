use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::{isa::Isa, util::str::snake_to_pascal_case};

#[derive(Deserialize, Debug)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub fn get(&self, name: &TagName) -> Option<&Tag> {
        self.0.iter().find(|t| &t.name == name)
    }

    pub fn impl_tokens(&self, isa: &Isa) -> TokenStream {
        let fns_tokens = self.0.iter().map(|t| t.fn_tokens(isa));
        quote! {
            impl Ins {
                #(#fns_tokens)*
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    name: TagName,
    description: String,
}

impl Tag {
    pub fn fn_tokens(&self, isa: &Isa) -> TokenStream {
        let cases = isa.opcodes().iter().filter_map(|o| {
            if o.tags().contains(&self.name) {
                let ident = Ident::new(&snake_to_pascal_case(o.mnemonic()), Span::call_site());
                let cfg = o.cfg_attribute_tokens(isa);
                Some(quote! {
                    #cfg
                    Ins::#ident { .. } => true
                })
            } else {
                None
            }
        });

        let tag_fn_ident = self.name.as_ident();
        let description = &self.description;

        quote! {
            #[doc = #description]
            pub fn #tag_fn_ident(&self) -> bool {
                match self {
                    #(#cases),*,
                    _ => false,
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct TagName(pub String);

impl TagName {
    pub fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }
}

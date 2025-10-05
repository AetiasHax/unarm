use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::isa::DataExpr;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct IllegalChecks(Vec<DataExpr>);

impl IllegalChecks {
    pub fn checks_tokens(&self, illegal_value: Option<TokenStream>) -> TokenStream {
        let illegal_value = illegal_value.unwrap_or_else(|| quote!(Some(Ins::Illegal)));
        let illegal_checks = self.0.iter().map(|illegal| {
            let ident = Ident::new("value", Span::call_site());
            let illegal_expr = illegal.as_tokens(ident);
            quote! {
                if #illegal_expr {
                    return #illegal_value;
                }
            }
        });
        quote!(#(#illegal_checks)*)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

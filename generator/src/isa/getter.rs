use anyhow::{Result, anyhow};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{DataTypeName, Isa},
    util::str::snake_to_pascal_case,
};

#[derive(Deserialize, Debug)]
pub struct Getters(Vec<Getter>);

impl Getters {
    pub fn validate(&self, isa: &Isa) -> Result<()> {
        for getter in self.0.iter() {
            getter.validate(isa)?;
        }
        Ok(())
    }

    pub fn impl_tokens(&self, isa: &Isa) -> TokenStream {
        let impls = self.0.iter().map(|g| g.impl_tokens(isa));
        quote!(#(#impls)*)
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Getter {
    name: GetterName,
    description: String,
    kind: GetterKind,
}

impl Getter {
    pub fn validate(&self, isa: &Isa) -> Result<()> {
        match &self.kind {
            GetterKind::InsParamOfType(type_name) => {
                let data_type = isa
                    .types()
                    .get(type_name)
                    .ok_or_else(|| anyhow!("Type name '{type_name}' not found"))?;
                data_type
                    .default_expr_tokens(isa)
                    .ok_or_else(|| anyhow!("Data type '{type_name}' has no default value"))?;
            }
        }
        Ok(())
    }

    pub fn impl_tokens(&self, isa: &Isa) -> TokenStream {
        match &self.kind {
            GetterKind::InsParamOfType(param_type_name) => {
                let getter_ident = self.name.as_ident();
                let data_type = isa.types.get(param_type_name).unwrap();
                let type_tokens = data_type.type_tokens(isa);

                let cases = isa.opcodes().iter().filter_map(|o| {
                    let ident = Ident::new(&snake_to_pascal_case(o.mnemonic()), Span::call_site());
                    let cfg = o.cfg_attribute_tokens(isa);
                    o.params().iter().find_map(|(name, type_name)| {
                        if type_name == param_type_name {
                            let param_ident = Ident::new(&name.0, Span::call_site());
                            Some(quote! {
                                #cfg
                                Ins::#ident { #param_ident, .. } => *#param_ident
                            })
                        } else {
                            None
                        }
                    })
                });

                let default_expr_tokens = data_type.default_expr_tokens(isa).unwrap();
                let description = &self.description;

                quote! {
                    impl Ins {
                        #[doc = #description]
                        pub fn #getter_ident(&self) -> #type_tokens {
                            match self {
                                #(#cases),*,
                                _ => #default_expr_tokens,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct GetterName(pub String);

impl GetterName {
    pub fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum GetterKind {
    #[serde(rename = "ins_param_of_type")]
    InsParamOfType(DataTypeName),
}

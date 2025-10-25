use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{
        DataType, DataTypeEnumVariant, DataTypeKind, DataTypeStruct, DataTypeUnion, DataTypes,
        FormatCond, Isa, Opcode, Opcodes,
    },
    util::str::snake_to_pascal_case,
};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DefsUses(#[serde(default)] Vec<DefUse>);

impl DefsUses {
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn defs_exprs_tokens(&self) -> Vec<TokenStream> {
        self.0.iter().map(|def| def.defs_expr_tokens()).collect()
    }

    pub fn uses_exprs_tokens(&self) -> Vec<TokenStream> {
        self.0.iter().map(|def| def.uses_expr_tokens()).collect()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum DefUse {
    #[serde(rename = "if")]
    If(FormatCond, FormatCond),
    #[serde(rename = "always")]
    Always(FormatCond),
}

impl DefUse {
    fn defs_uses_expr_tokens(&self, list_ident: Ident) -> TokenStream {
        match self {
            DefUse::If(format_cond, data_expr) => {
                let cond = format_cond.as_tokens(None);
                let data = data_expr.as_tokens(None);
                quote! {
                    if #cond {
                        #list_ident.push(#data);
                    }
                }
            }
            DefUse::Always(data_expr) => {
                let data = data_expr.as_tokens(None);
                quote!(#list_ident.push(#data);)
            }
        }
    }

    fn defs_expr_tokens(&self) -> TokenStream {
        self.defs_uses_expr_tokens(Ident::new("defs", Span::call_site()))
    }

    fn uses_expr_tokens(&self) -> TokenStream {
        self.defs_uses_expr_tokens(Ident::new("uses", Span::call_site()))
    }
}

impl Opcodes {
    pub fn defs_uses_impl_tokens(&self, isa: &Isa) -> TokenStream {
        let defs_cases = self.iter().filter_map(|op| op.defs_case_tokens(isa));
        let uses_cases = self.iter().filter_map(|op| op.uses_case_tokens(isa));

        quote! {
            impl Ins {
                /// Returns a [`DefsUses`] object containing all the registers this instruction
                /// defines, in no particular order.
                pub fn defs(&self) -> DefsUses {
                    let mut defs = DefsUses::new();
                    match self {
                        #(#defs_cases)*
                        _ => {}
                    }
                    defs
                }

                /// Returns a [`DefsUses`] object containing all the registers this instruction
                /// uses, in no particular order.
                pub fn uses(&self) -> DefsUses {
                    let mut uses = DefsUses::new();
                    match self {
                        #(#uses_cases)*
                        _ => {}
                    }
                    uses
                }
            }
        }
    }
}

impl Opcode {
    fn defs_case_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        if !self.has_defs(isa) {
            return None;
        }

        let variant_ident = Ident::new(&snake_to_pascal_case(self.mnemonic()), Span::call_site());
        let param_names = self.params().keys().map(|k| Ident::new(&k.0, Span::call_site()));

        let opcode_defs = self.defs().defs_exprs_tokens();
        let params_defs = self.params().iter().filter_map(|(param_name, type_name)| {
            let data_type = isa.types().get(type_name).unwrap();
            data_type.has_defs(isa).then(|| {
                let param_ident = param_name.as_ident();
                quote!(#param_ident.defs(&mut defs);)
            })
        });

        Some(quote! {
            Ins::#variant_ident { #(#param_names),* } => {
                #(#opcode_defs)*
                #(#params_defs)*
            }
        })
    }

    fn uses_case_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        if !self.has_uses(isa) {
            return None;
        }

        let variant_ident = Ident::new(&snake_to_pascal_case(self.mnemonic()), Span::call_site());
        let param_names = self.params().keys().map(|k| Ident::new(&k.0, Span::call_site()));

        let opcode_uses = self.uses().uses_exprs_tokens();
        let params_uses = self.params().iter().filter_map(|(param_name, type_name)| {
            let data_type = isa.types().get(type_name).unwrap();
            data_type.has_uses(isa).then(|| {
                let param_ident = param_name.as_ident();
                quote!(#param_ident.uses(&mut uses);)
            })
        });

        Some(quote! {
            Ins::#variant_ident { #(#param_names),* } => {
                #(#opcode_uses)*
                #(#params_uses)*
            }
        })
    }

    fn has_defs(&self, isa: &Isa) -> bool {
        if !self.defs().is_empty() {
            true
        } else {
            self.params().values().any(|type_name| {
                let data_type = isa.types().get(type_name).unwrap();
                data_type.has_defs(isa)
            })
        }
    }

    fn has_uses(&self, isa: &Isa) -> bool {
        if !self.uses().is_empty() {
            true
        } else {
            self.params().values().any(|type_name| {
                let data_type = isa.types().get(type_name).unwrap();
                data_type.has_uses(isa)
            })
        }
    }
}

impl DataTypes {
    pub fn defs_uses_impl_tokens(&self, isa: &Isa) -> TokenStream {
        let impls = self.iter().filter_map(|data_type| data_type.defs_uses_impl_tokens(isa));
        quote! {
            #(#impls)*
        }
    }
}

impl DataType {
    fn has_defs(&self, isa: &Isa) -> bool {
        match self.kind() {
            DataTypeKind::Bool { .. } => false,
            DataTypeKind::UInt(_) => false,
            DataTypeKind::Int(_) => false,
            DataTypeKind::Enum(_) => false,
            DataTypeKind::Union(data_type_union) => data_type_union.has_defs(isa),
            DataTypeKind::Struct(data_type_struct) => data_type_struct.has_defs(isa),
            DataTypeKind::Type(_, _) => self.canonical(isa).has_defs(isa),
            DataTypeKind::Custom(_) => false,
        }
    }

    fn has_uses(&self, isa: &Isa) -> bool {
        match self.kind() {
            DataTypeKind::Bool { .. } => false,
            DataTypeKind::UInt(_) => false,
            DataTypeKind::Int(_) => false,
            DataTypeKind::Enum(_) => false,
            DataTypeKind::Union(data_type_union) => data_type_union.has_uses(isa),
            DataTypeKind::Struct(data_type_struct) => data_type_struct.has_uses(isa),
            DataTypeKind::Type(_, _) => self.canonical(isa).has_uses(isa),
            DataTypeKind::Custom(_) => false,
        }
    }

    fn defs_uses_impl_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        let defs_fn = self.defs_fn_body_tokens(isa).map(|fn_body| {
            quote! {
                fn defs(&self, defs: &mut DefsUses) {
                    #fn_body
                }
            }
        });

        let uses_fn = self.uses_fn_body_tokens(isa).map(|fn_body| {
            quote! {
                fn uses(&self, uses: &mut DefsUses) {
                    #fn_body
                }
            }
        });

        let type_ident = self.name().as_pascal_ident();

        if defs_fn.is_none() && uses_fn.is_none() {
            None
        } else {
            Some(quote! {
                impl #type_ident {
                    #defs_fn
                    #uses_fn
                }
            })
        }
    }

    fn defs_fn_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match self.kind() {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(_) => None,
            DataTypeKind::Union(data_type_union) => data_type_union.defs_fn_body_tokens(isa),
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.defs_fn_body_tokens(isa, true)
            }
            DataTypeKind::Type(_, _) => self.canonical(isa).defs_fn_body_tokens(isa),
            DataTypeKind::Custom(_) => None,
        }
    }

    fn uses_fn_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match self.kind() {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(_) => None,
            DataTypeKind::Union(data_type_union) => data_type_union.uses_fn_body_tokens(isa),
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.uses_fn_body_tokens(isa, true)
            }
            DataTypeKind::Type(_, _) => self.canonical(isa).uses_fn_body_tokens(isa),
            DataTypeKind::Custom(_) => None,
        }
    }

    fn defs_expr_in_enum_variant_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match self.kind() {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(_) => None,
            DataTypeKind::Union(_) => {
                let ident = self.name().as_ident();
                Some(quote!(#ident.defs(defs);))
            }
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.defs_fn_body_tokens(isa, false)
            }
            DataTypeKind::Type(_, _) => {
                let canonical = self.canonical(isa);
                if let DataTypeKind::Struct(data_type_struct) = canonical.kind() {
                    data_type_struct.has_defs(isa).then(|| {
                        let ident = self.name().as_ident();
                        quote!(#ident.defs(defs);)
                    })
                } else {
                    canonical.defs_expr_in_enum_variant_tokens(isa)
                }
            }
            DataTypeKind::Custom(_) => None,
        }
    }

    fn uses_expr_in_enum_variant_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        match self.kind() {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(_) => None,
            DataTypeKind::Union(_) => {
                let ident = self.name().as_ident();
                Some(quote!(#ident.uses(uses);))
            }
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.uses_fn_body_tokens(isa, false)
            }
            DataTypeKind::Type(_, _) => {
                let canonical = self.canonical(isa);
                if let DataTypeKind::Struct(data_type_struct) = canonical.kind() {
                    data_type_struct.has_uses(isa).then(|| {
                        let ident = self.name().as_ident();
                        quote!(#ident.uses(uses);)
                    })
                } else {
                    canonical.uses_expr_in_enum_variant_tokens(isa)
                }
            }
            DataTypeKind::Custom(_) => None,
        }
    }
}

impl DataTypeUnion {
    fn has_defs(&self, isa: &Isa) -> bool {
        self.variants().values().any(|variant| variant.has_defs(isa))
    }

    fn has_uses(&self, isa: &Isa) -> bool {
        self.variants().values().any(|variant| variant.has_uses(isa))
    }

    fn defs_fn_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        let cases = self
            .variants()
            .values()
            .filter_map(|variant| {
                let defs_expr = variant.defs_expr_tokens(isa)?;
                let case_pattern = variant.pattern_destructure_tokens();
                Some(quote! {
                    #case_pattern => {
                        #defs_expr
                    }
                })
            })
            .collect::<Vec<_>>();
        if cases.is_empty() {
            None
        } else {
            Some(quote! {
                match self {
                    #(#cases)*
                    _ => {}
                }
            })
        }
    }

    fn uses_fn_body_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        let cases = self
            .variants()
            .values()
            .filter_map(|variant| {
                let uses_expr = variant.uses_expr_tokens(isa)?;
                let case_pattern = variant.pattern_destructure_tokens();
                Some(quote! {
                    #case_pattern => {
                        #uses_expr
                    }
                })
            })
            .collect::<Vec<_>>();
        if cases.is_empty() {
            None
        } else {
            Some(quote! {
                match self {
                    #(#cases)*
                    _ => {}
                }
            })
        }
    }
}

impl DataTypeEnumVariant {
    fn has_defs(&self, isa: &Isa) -> bool {
        if !self.defs().is_empty() {
            true
        } else if let Some(data) = self.data() {
            data.has_defs(isa)
        } else {
            false
        }
    }

    fn has_uses(&self, isa: &Isa) -> bool {
        if !self.uses().is_empty() {
            true
        } else if let Some(data) = self.data() {
            data.has_uses(isa)
        } else {
            false
        }
    }

    fn defs_expr_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        let variant_defs = self.defs().defs_exprs_tokens();
        let data_def = self.data().and_then(|data| data.defs_expr_in_enum_variant_tokens(isa));
        if variant_defs.is_empty() && data_def.is_none() {
            None
        } else {
            Some(quote! {
                #(#variant_defs)*
                #data_def
            })
        }
    }

    fn uses_expr_tokens(&self, isa: &Isa) -> Option<TokenStream> {
        let variant_uses = self.uses().uses_exprs_tokens();
        let data_use = self.data().and_then(|data| data.uses_expr_in_enum_variant_tokens(isa));
        if variant_uses.is_empty() && data_use.is_none() {
            None
        } else {
            Some(quote! {
                #(#variant_uses)*
                #data_use
            })
        }
    }
}

impl DataTypeStruct {
    fn has_defs(&self, isa: &Isa) -> bool {
        if !self.defs().is_empty() {
            true
        } else {
            self.fields().iter().any(|field| field.has_defs(isa))
        }
    }

    fn has_uses(&self, isa: &Isa) -> bool {
        if !self.uses().is_empty() {
            true
        } else {
            self.fields().iter().any(|field| field.has_uses(isa))
        }
    }

    fn defs_fn_body_tokens(&self, isa: &Isa, destructure: bool) -> Option<TokenStream> {
        let struct_defs = self.defs().defs_exprs_tokens();
        let data_defs = self
            .fields()
            .iter()
            .filter(|&field| field.has_defs(isa))
            .map(|field| {
                let ident = field.name().as_ident();
                quote!(#ident.defs(defs);)
            })
            .collect::<TokenStream>();

        let destructure = destructure.then(|| {
            let fields = self.fields().iter().map(|f| f.name().as_ident());
            quote!(let Self { #(#fields),* } = self;)
        });

        if struct_defs.is_empty() && data_defs.is_empty() {
            None
        } else {
            Some(quote! {
                #destructure
                #(#struct_defs)*
                #data_defs
            })
        }
    }

    fn uses_fn_body_tokens(&self, isa: &Isa, destructure: bool) -> Option<TokenStream> {
        let struct_uses = self.uses().uses_exprs_tokens();
        let data_uses = self
            .fields()
            .iter()
            .filter(|&field| field.has_uses(isa))
            .map(|field| {
                let ident = field.name().as_ident();
                quote!(#ident.uses(uses);)
            })
            .collect::<TokenStream>();

        let destructure = destructure.then(|| {
            let fields = self.fields().iter().map(|f| f.name().as_ident());
            quote!(let Self { #(#fields),* } = self;)
        });

        if struct_uses.is_empty() && data_uses.is_empty() {
            None
        } else {
            Some(quote! {
                #destructure
                #(#struct_uses)*
                #data_uses
            })
        }
    }
}

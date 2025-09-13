use proc_macro2::TokenStream;
use quote::quote;

use crate::isa::Isa;

impl Isa {
    pub fn generate_types(&self) -> TokenStream {
        let options_struct = self.options().struct_tokens();
        let version_enum = self.versions().enum_tokens();
        let internal_option_types = self.options().internal_types_tokens();

        let data_types = self.types().types_tokens(self);

        let ins_enum = self.opcodes().ins_enum_tokens(self);

        quote! {
            use crate::*;

            #options_struct
            #version_enum
            #internal_option_types

            #data_types

            #ins_enum
        }
    }

    pub fn generate_parser(&self) -> TokenStream {
        let data_parse_impls = self.types().parse_impls_tokens(self);
        let data_default_impls = self.types().default_impls_tokens(self);
        let parse_arm_ifchain_fn = self.opcodes().parse_arm_ifchain_fn_tokens();
        let parse_thumb_ifchain_fn = self.opcodes().parse_thumb_ifchain_fn_tokens();
        let opcode_parse_fns = self.opcodes().parse_fns_tokens(self);

        quote! {
            #![allow(clippy::eq_op)]
            #![allow(clippy::double_parens)]
            #![allow(clippy::unnecessary_cast)]
            #![allow(clippy::derivable_impls)]
            #![allow(unused_parens)]
            #![allow(unused_variables)]

            use crate::*;

            #data_parse_impls
            #data_default_impls

            #parse_arm_ifchain_fn
            #parse_thumb_ifchain_fn
            #opcode_parse_fns
        }
    }

    pub fn generate_display(&self) -> TokenStream {
        let write_trait = self.types().write_trait_tokens(self);

        let data_fmt_impls = self.types().fmt_impls_tokens(self);

        let ins_fmt_impl = self.opcodes().write_impl_tokens(self);

        quote! {
            #![allow(clippy::collapsible_else_if)]
            #![allow(unused_variables)]

            use crate::*;

            #write_trait

            #data_fmt_impls

            #ins_fmt_impl
        }
    }
}

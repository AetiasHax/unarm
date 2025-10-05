use proc_macro2::TokenStream;
use quote::quote;

use crate::isa::{Arch, Isa};

impl Isa {
    pub fn generate_types(&self) -> TokenStream {
        let options_struct = self.options().struct_tokens();
        let options_default_impl = self.options().default_impls_tokens();
        let version_enum = self.versions().enum_tokens();
        let versions_struct = self.versions().struct_tokens();
        let version_default_impl = self.versions().default_impl_tokens();
        let extension_enum = self.extensions().enum_tokens();
        let extensions_struct = self.extensions().struct_tokens();
        let extension_default_impl = self.extensions().default_impl_tokens();
        let internal_option_types = self.options().internal_types_tokens();

        let data_types = self.types().types_tokens(self);

        let ins_enum = self.opcodes().ins_enum_tokens(self);

        quote! {
            #![cfg_attr(rustfmt, rustfmt_skip)]

            use crate::*;

            #options_struct
            #options_default_impl
            #version_enum
            #versions_struct
            #version_default_impl
            #extension_enum
            #extensions_struct
            #extension_default_impl
            #internal_option_types

            #data_types

            #ins_enum
        }
    }

    pub fn generate_parser(&self) -> TokenStream {
        let data_parse_impls = self.types().parse_impls_tokens(self);
        let data_default_impls = self.types().default_impls_tokens(self);
        let parse_arm_fn = self.opcodes().parse_arm_lookup_match_tokens(self);
        let parse_thumb_fn = self.opcodes().parse_thumb_lookup_match_tokens(self);
        let parse_arm_with_discriminant_fn =
            self.opcodes().parse_with_discriminant_tokens(self, Arch::Arm);
        let parse_thumb_with_discriminant_fn =
            self.opcodes().parse_with_discriminant_tokens(self, Arch::Thumb);
        let opcode_parse_fns = self.opcodes().parse_fns_tokens(self);

        quote! {
            #![cfg_attr(rustfmt, rustfmt_skip)]

            #![allow(clippy::eq_op)]
            #![allow(clippy::double_parens)]
            #![allow(clippy::unnecessary_cast)]
            #![allow(clippy::derivable_impls)]
            #![allow(clippy::needless_else)]
            #![allow(clippy::manual_range_patterns)]
            #![allow(unused_parens)]
            #![allow(unused_variables)]

            use crate::*;

            #data_parse_impls
            #data_default_impls

            #parse_arm_fn
            #parse_thumb_fn
            #parse_arm_with_discriminant_fn
            #parse_thumb_with_discriminant_fn
            #opcode_parse_fns
        }
    }

    pub fn generate_display(&self) -> TokenStream {
        let write_trait = self.types().write_trait_tokens(self);

        let data_fmt_impls = self.types().fmt_impls_tokens(self);

        let ins_fmt_impl = self.opcodes().write_impl_tokens(self);

        quote! {
            #![cfg_attr(rustfmt, rustfmt_skip)]

            #![allow(clippy::collapsible_else_if)]
            #![allow(clippy::needless_else)]
            #![allow(clippy::explicit_auto_deref)]
            #![allow(unused_variables)]

            use crate::*;

            #write_trait

            #data_fmt_impls

            #ins_fmt_impl
        }
    }
}

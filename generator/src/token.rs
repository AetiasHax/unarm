use std::{fmt::LowerHex, str::FromStr};

use proc_macro2::TokenStream;
use quote::ToTokens;

// Credits to ppc750cl (MIT License):
// https://github.com/encounter/ppc750cl/blob/6cbd7d888c7082c2c860f66cbb9848d633f753ed/genisa/src/isa.rs#L370
pub struct HexLiteral<T>(pub T);

impl<T: LowerHex> LowerHex for HexLiteral<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl<T: LowerHex> ToTokens for HexLiteral<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = format!("0x{:08x}", self);
        tokens.extend(TokenStream::from_str(&s).unwrap());
    }
}

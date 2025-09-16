use std::{ops::Range, str::FromStr};

use anyhow::Context;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, de::Visitor};
use syn::Ident;

use crate::util::hex_literal::HexLiteral;

#[derive(Debug, Clone)]
pub struct BitRange(pub Range<u8>);

impl BitRange {
    pub fn mask(&self) -> u32 {
        1u32.unbounded_shl(self.0.len() as u32).wrapping_sub(1)
    }

    pub fn shift_mask_tokens(&self, value: Option<Ident>) -> TokenStream {
        let value = value.unwrap_or_else(|| Ident::new("value", Span::call_site()));
        let mask = HexLiteral(self.mask());
        if self.0.start == 0 {
            if self.0.end == 32 {
                quote!(#value)
            } else {
                quote!((#value) & #mask)
            }
        } else {
            let shift = Literal::u8_unsuffixed(self.0.start);
            quote!(((#value) >> #shift) & #mask)
        }
    }
}

impl Default for BitRange {
    fn default() -> Self {
        Self(0..1)
    }
}

impl FromStr for BitRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once("..") {
            let start: u8 = start.parse().context("Invalid start of bit range")?;
            let end: u8 = end.parse().context("Invalid end of bit range")?;
            Ok(Self(start..end))
        } else {
            let bit: u8 = s.parse().context("Invalid bit position")?;
            Ok(Self(bit..bit + 1))
        }
    }
}

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BitRangeVisitor)
    }
}

struct BitRangeVisitor;

impl<'de> Visitor<'de> for BitRangeVisitor {
    type Value = BitRange;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or range (e.g. 0..12)")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BitRange(v as u8..v as u8 + 1))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        BitRange::from_str(v).map_err(serde::de::Error::custom)
    }
}

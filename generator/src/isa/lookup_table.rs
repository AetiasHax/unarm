use std::collections::BTreeMap;

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    isa::{Arch, BitRanges, Isa, Opcode, OpcodeEncoding},
    util::hex_literal::HexLiteral,
};

pub struct OpcodeLookupTable<'a> {
    arch: Arch,
    bitmask: u32,
    buckets: Vec<Bucket<'a>>,
    /// Maps bucket index to the bucket index sharing the same parse function, for deduplication. Can be the same index.
    bucket_parse_map: BTreeMap<usize, usize>,
    /// Reverse mapping of `bucket_parse_map`
    bucket_parse_map_rev: BTreeMap<usize, Vec<usize>>,
    all_encodings: Vec<Encoding<'a>>,
}

#[derive(PartialEq, Eq)]
pub struct Bucket<'a> {
    encodings: Vec<Encoding<'a>>,
}

#[derive(Clone)]
pub struct Encoding<'a> {
    opcode: &'a Opcode,
    encoding: &'a OpcodeEncoding,
    index_opcode: usize,
    index_all: usize,
}

impl<'a> OpcodeLookupTable<'a> {
    pub fn new_arm(isa: &'a Isa) -> Self {
        let mut encodings = isa
            .opcodes()
            .iter()
            .filter_map(|opcode| {
                opcode.arm_encodings().map(|encodings| {
                    encodings
                        .iter()
                        .enumerate()
                        .map(|(index, encoding)| Encoding {
                            opcode,
                            encoding,
                            index_opcode: index,
                            index_all: 0,
                        })
                        .collect::<Vec<_>>()
                })
            })
            .flatten()
            .collect::<Vec<_>>();
        encodings.sort_by(|a, b| {
            let a_pattern = a.encoding.pattern().combined();
            let b_pattern = b.encoding.pattern().combined();
            b_pattern.bitmask().count_ones().cmp(&a_pattern.bitmask().count_ones())
        });
        for (index, encoding) in encodings.iter_mut().enumerate() {
            encoding.index_all = index;
        }
        Self::optimize(encodings, Arch::Arm, 0x0ff801f0)
    }

    pub fn optimize(encodings: Vec<Encoding<'a>>, arch: Arch, bitmask: u32) -> Self {
        let num_bits = bitmask.count_ones();
        let num_buckets = 1 << num_bits;
        let bit_positions = (0..32).filter(|i| (bitmask & (1 << i)) != 0).collect::<Vec<_>>();

        let buckets = (0..num_buckets)
            .map(|key| {
                let key = bit_positions
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, pos)| acc | (((key >> i) & 1) << pos));

                let encodings = encodings
                    .iter()
                    .filter(|encoding| {
                        let pattern = encoding.encoding.pattern().combined();
                        (pattern.pattern() & bitmask) == (key & pattern.bitmask())
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                Bucket { encodings }
            })
            .collect::<Vec<_>>();

        let bucket_parse_clones = buckets
            .iter()
            .enumerate()
            .map(|(i, bucket)| {
                buckets.iter().take(i).position(|b| b == bucket).map(|j| (i, j)).unwrap_or((i, i))
            })
            .collect::<BTreeMap<_, _>>();
        let bucket_parse_clones_rev = bucket_parse_clones.iter().map(|(k, v)| (*v, *k)).fold(
            BTreeMap::<usize, Vec<usize>>::new(),
            |mut acc, (k, v)| {
                acc.entry(k).or_default().push(v);
                acc
            },
        );

        OpcodeLookupTable {
            arch,
            bitmask,
            buckets,
            all_encodings: encodings,
            bucket_parse_map: bucket_parse_clones,
            bucket_parse_map_rev: bucket_parse_clones_rev,
        }
    }

    pub fn parse_match_fn_body_tokens(&self) -> TokenStream {
        let cases = self.bucket_parse_map_rev.iter().map(|(key, clones)| {
            let pattern_literals = clones.iter().map(|k| HexLiteral(*k));

            let mapped_key = self.bucket_parse_map.get(key).unwrap();
            let fn_name = format!("parse_{}_{mapped_key:x}", self.arch);
            let fn_ident = Ident::new(&fn_name, Span::call_site());
            quote! {
                #(#pattern_literals)|* => #fn_ident(ins, pc)
            }
        });
        let bit_ranges = BitRanges::from_mask(self.bitmask);
        let index_expr = bit_ranges.shift_mask_tokens(Some(Ident::new("ins", Span::call_site())));
        quote! {
            match #index_expr {
                #(#cases),*,
                _ => unreachable!(),
            }
        }
    }

    pub fn parse_table_fn_body_tokens(&self) -> TokenStream {
        let bit_ranges = BitRanges::from_mask(self.bitmask);
        let index_expr = bit_ranges.shift_mask_tokens(Some(Ident::new("ins", Span::call_site())));
        let lookup_table_ident = self.lookup_table_ident();
        let encoding_array_ident = self.encoding_array_ident();
        quote! {
            let index = #index_expr;
            let encodings = #lookup_table_ident[index as usize];
            for encoding_index in encodings {
                let (bitmask, pattern, parse_fn) = #encoding_array_ident[*encoding_index as usize];
                if (ins & bitmask) == pattern {
                    return parse_fn(ins, pc);
                }
            }
            None
        }
    }

    pub fn parse_buckets_tokens(&self) -> TokenStream {
        let parse_case_fns = self
            .buckets
            .iter()
            .enumerate()
            .filter(|(key, _)| self.bucket_parse_map.get(key) == Some(key))
            .map(|(key, bucket)| {
                let body_tokens = bucket.parse_bucket_tokens(self.arch);
                let fn_name = format!("parse_{}_{key:x}", self.arch);
                let fn_ident = Ident::new(&fn_name, Span::call_site());
                quote! {
                    fn #fn_ident(ins: u32, pc: u32) -> Option<Ins> {
                        #body_tokens
                    }
                }
            });
        quote!(#(#parse_case_fns)*)
    }

    pub fn buckets_table_array_tokens(&self) -> TokenStream {
        let ident = self.lookup_table_ident();
        let buckets = self.buckets.iter().map(|bucket| bucket.table_entry_tokens(self.arch));

        quote! {
            const #ident: &[&[u16]] = &[
                #(#buckets),*
            ];
        }
    }

    fn lookup_table_ident(&self) -> Ident {
        let name = match self.arch {
            Arch::Arm => "ARM_LOOKUP_TABLE",
            Arch::Thumb => "THUMB_LOOKUP_TABLE",
        };
        Ident::new(name, Span::call_site())
    }

    pub fn encoding_array_tokens(&self) -> TokenStream {
        let ident = self.encoding_array_ident();
        let encodings =
            self.all_encodings.iter().map(|encoding| encoding.table_tuple_tokens(self.arch));

        quote! {
            const #ident: &[(u32, u32, fn (u32, u32) -> Option<Ins>)] = &[
                #(#encodings),*
            ];
        }
    }

    fn encoding_array_ident(&self) -> Ident {
        let name = match self.arch {
            Arch::Arm => "ARM_ENCODINGS",
            Arch::Thumb => "THUMB_ENCODINGS",
        };
        Ident::new(name, Span::call_site())
    }
}

impl<'a> Bucket<'a> {
    pub fn parse_bucket_tokens(&self, arch: Arch) -> TokenStream {
        if self.encodings.is_empty() {
            quote!(None)
        } else if self.encodings.len() == 1 {
            let encoding = &self.encodings[0];
            let parse_fn_name = encoding.opcode.parse_fn_name(arch, encoding.index_opcode);
            let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
            quote!(#parse_fn_ident(ins, pc))
        } else {
            let parse_ifs = self.encodings.iter().map(|encoding| {
                let pattern = encoding.encoding.pattern().combined();
                let pattern_literal = HexLiteral(pattern.pattern());
                let bitmask_literal = HexLiteral(pattern.bitmask());
                let parse_fn_name = encoding.opcode.parse_fn_name(arch, encoding.index_opcode);
                let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
                quote! {
                    if (ins & #bitmask_literal) == #pattern_literal {
                        #parse_fn_ident(ins, pc)
                    }
                }
            });
            quote! {
                #(#parse_ifs)else*
                else {
                    None
                }
            }
        }
    }

    fn table_entry_tokens(&self, arch: Arch) -> TokenStream {
        // let encodings = self.encodings.iter().map(|encoding| encoding.table_tuple_tokens(arch));
        let encodings =
            self.encodings.iter().map(|encoding| Literal::usize_unsuffixed(encoding.index_all));
        quote!(&[#(#encodings),*])
    }
}

impl<'a> Encoding<'a> {
    fn table_tuple_tokens(&self, arch: Arch) -> TokenStream {
        let parse_fn_name = self.opcode.parse_fn_name(arch, self.index_opcode);
        let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
        let pattern = self.encoding.pattern().combined();
        let bitmask = HexLiteral(pattern.bitmask());
        let pattern = HexLiteral(pattern.pattern());
        quote!((#bitmask, #pattern, #parse_fn_ident))
    }
}

impl<'a> PartialEq for Encoding<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.encoding, other.encoding)
    }
}

impl<'a> Eq for Encoding<'a> {}

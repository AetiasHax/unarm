use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
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
    /// Contains all buckets which contain the same encodings as other buckets
    equivalent_buckets: BTreeMap<usize, Vec<usize>>,
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
        Self::new(isa, Arch::Arm, 0x0ff801f0)
    }

    pub fn new_thumb(isa: &'a Isa) -> Self {
        Self::new(isa, Arch::Thumb, 0xffc0)
    }

    fn new(isa: &'a Isa, arch: Arch, bitmask: u32) -> Self {
        let mut encodings = isa
            .opcodes()
            .iter()
            .filter_map(|opcode| {
                let encodings = match arch {
                    Arch::Arm => opcode.arm_encodings(),
                    Arch::Thumb => opcode.thumb_encodings(),
                }?;
                Some(
                    encodings
                        .iter()
                        .enumerate()
                        .map(|(index, encoding)| Encoding {
                            opcode,
                            encoding,
                            index_opcode: index,
                            index_all: 0,
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect::<Vec<_>>();
        encodings.sort_by(|a, b| {
            let a_pattern = a.encoding.pattern().combined();
            let b_pattern = b.encoding.pattern().combined();
            b_pattern.num_bits().cmp(&a_pattern.num_bits())
        });
        for (index, encoding) in encodings.iter_mut().enumerate() {
            encoding.index_all = index;
        }
        Self::optimize(encodings, arch, bitmask)
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
        let equivalent_buckets = bucket_parse_clones.iter().map(|(k, v)| (*v, *k)).fold(
            BTreeMap::<usize, Vec<usize>>::new(),
            |mut acc, (k, v)| {
                acc.entry(k).or_default().push(v);
                acc
            },
        );

        OpcodeLookupTable { arch, bitmask, buckets, equivalent_buckets }
    }

    pub fn parse_match_fn_body_tokens(&self) -> TokenStream {
        let cases = self.equivalent_buckets.iter().map(|(key, clones)| {
            let pattern_literals = clones.iter().map(|k| HexLiteral(*k));

            let bucket = self.buckets.get(*key).unwrap();
            let body_tokens = bucket.parse_bucket_tokens(self.arch);

            quote! {
                #(#pattern_literals)|* => #body_tokens
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
}

impl<'a> Bucket<'a> {
    pub fn parse_bucket_tokens(&self, arch: Arch) -> TokenStream {
        let ins_size = match arch {
            Arch::Arm => quote!(4),
            Arch::Thumb => quote!(2),
        };
        if self.encodings.is_empty() {
            match arch {
                Arch::Arm => quote!(Ins::Illegal),
                Arch::Thumb => quote!((Ins::Illegal, #ins_size)),
            }
        } else if self.encodings.len() == 1 {
            let encoding = &self.encodings[0];
            let parse_fn_name = encoding.opcode.parse_fn_name(arch, encoding.index_opcode);
            let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
            match arch {
                Arch::Arm => quote!(#parse_fn_ident(ins, pc, options).unwrap_or(Ins::Illegal)),
                Arch::Thumb => {
                    quote!(#parse_fn_ident(ins, pc, options).unwrap_or((Ins::Illegal, #ins_size)))
                }
            }
        } else {
            let parse_ifs = self.encodings.iter().map(|encoding| {
                let pattern = encoding.encoding.pattern().combined();
                let pattern_literal = HexLiteral(pattern.pattern());
                let bitmask_literal = HexLiteral(pattern.bitmask());
                let parse_fn_name = encoding.opcode.parse_fn_name(arch, encoding.index_opcode);
                let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
                quote! {
                    if (ins & #bitmask_literal) == #pattern_literal && let Some(ins) = #parse_fn_ident(ins, pc, options) {
                        ins
                    }
                }
            });
            let fallback = match arch {
                Arch::Arm => quote!(Ins::Illegal),
                Arch::Thumb => quote!((Ins::Illegal, #ins_size)),
            };
            quote! {
                #(#parse_ifs)else*
                else {
                    #fallback
                }
            }
        }
    }
}

impl<'a> PartialEq for Encoding<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.encoding, other.encoding)
    }
}

impl<'a> Eq for Encoding<'a> {}

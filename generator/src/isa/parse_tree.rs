use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    isa::{Arch, Isa, Opcode, OpcodeEncoding},
    util::hex_literal::HexLiteral,
};

pub enum OpcodeParseTree<'a> {
    Internal { bitmask: u32, children: Vec<OpcodeParseTree<'a>> },
    Leaf { encodings: Vec<Encoding<'a>>, arch: Arch },
}

#[derive(Clone)]
pub struct Encoding<'a> {
    opcode: &'a Opcode,
    encoding: &'a OpcodeEncoding,
    encoding_index: usize,
}

impl<'a> OpcodeParseTree<'a> {
    pub fn new_arm(isa: &'a Isa) -> Self {
        Self::new(isa, Arch::Arm, |opcode| opcode.arm_encodings())
    }

    pub fn new_thumb(isa: &'a Isa) -> Self {
        Self::new(isa, Arch::Thumb, |opcode| opcode.thumb_encodings())
    }

    fn new<EncodingsFn>(isa: &'a Isa, arch: Arch, encodings_fn: EncodingsFn) -> Self
    where
        EncodingsFn: Fn(&'a Opcode) -> Option<&[OpcodeEncoding]>,
    {
        let mut encodings: Vec<_> = isa
            .opcodes()
            .iter()
            .filter_map(|opcode| encodings_fn(opcode).map(|encodings| (opcode, encodings)))
            .flat_map(|(opcode, encodings)| {
                encodings.iter().enumerate().map(|(i, encoding)| Encoding {
                    opcode,
                    encoding,
                    encoding_index: i,
                })
            })
            .collect();
        encodings.sort_by(|a, b| {
            let a_key = a.encoding.pattern().combined().bitmask().count_ones();
            let b_key = b.encoding.pattern().combined().bitmask().count_ones();
            a_key.cmp(&b_key).reverse()
        });

        Self::optimize(encodings, arch, 0, 0)
    }

    fn optimize(encodings: Vec<Encoding<'a>>, arch: Arch, depth: u32, checked_bits: u32) -> Self {
        if encodings.len() <= 2 {
            return Self::Leaf { encodings, arch };
        }
        if depth > 32 {
            panic!("Exceeded maximum depth of 32");
        }

        let mut smallest_max_bucket_size = usize::MAX;
        let mut best_pattern = 0;
        for num_bits in 1..=1 {
            for start in 0..(32 - num_bits) {
                let pattern = ((1u32 << num_bits) - 1) << start;
                if pattern & checked_bits != 0 {
                    continue;
                }
                if pattern.count_ones() != num_bits {
                    continue;
                }
                let mut bucket_sizes = vec![0; 1 << num_bits];
                for encoding in encodings.iter() {
                    let combined_pattern = encoding.encoding.pattern().combined();
                    for key in 0..(1 << num_bits) {
                        let test_pattern = (key << start) & combined_pattern.bitmask();
                        if (combined_pattern.pattern() & pattern) == test_pattern {
                            bucket_sizes[key as usize] += 1;
                            break;
                        }
                    }
                }
                let max_bucket_size = bucket_sizes.iter().max().unwrap_or(&0);
                if *max_bucket_size < smallest_max_bucket_size {
                    smallest_max_bucket_size = *max_bucket_size;
                    best_pattern = pattern;
                }
            }
        }
        if best_pattern == 0 {
            panic!("Failed to find a pattern");
        }

        // let indent = "  ".repeat(depth as usize);
        // println!("{indent}Depth {depth}: {} encodings", encodings.len());
        // println!(
        //     "{indent}Best pattern: {best_pattern:032b} with max bucket size {smallest_max_bucket_size}"
        // );

        if smallest_max_bucket_size == encodings.len() {
            return Self::Leaf { encodings, arch };
        }

        let num_pattern_bits = best_pattern.count_ones();
        let num_children = 1 << num_pattern_bits;
        let bit_positions =
            (0..32u32).filter(|i| (best_pattern & (1 << i)) != 0).collect::<Vec<_>>();

        let children = (0..num_children)
            .map(|i| {
                let pattern = bit_positions.iter().enumerate().fold(0u32, |acc, (j, pos)| {
                    if (i & (1 << j)) != 0 { acc | (1 << pos) } else { acc }
                });
                let child_encodings = encodings
                    .iter()
                    .filter(|e| {
                        let combined_pattern = e.encoding.pattern().combined();
                        let test_pattern = pattern & combined_pattern.bitmask();
                        (combined_pattern.pattern() & best_pattern) == test_pattern
                    })
                    .cloned()
                    .collect::<Vec<_>>();

                OpcodeParseTree::optimize(
                    child_encodings,
                    arch,
                    depth + 1,
                    checked_bits | best_pattern,
                )
            })
            .collect::<Vec<_>>();

        Self::Internal { bitmask: best_pattern, children }
    }

    pub fn parse_expr_tokens(&self) -> TokenStream {
        match self {
            OpcodeParseTree::Internal { bitmask, children } => {
                let bitmask_literal = HexLiteral(*bitmask);
                if bitmask.count_ones() == 1 {
                    let zero = children[0].parse_expr_tokens();
                    let one = children[1].parse_expr_tokens();
                    quote! {
                        if (ins & #bitmask_literal) == 0 {
                            #zero
                        } else {
                            #one
                        }
                    }
                } else {
                    let shift = bitmask.trailing_zeros();
                    let children = children.iter().enumerate().map(|(pattern, child)| {
                        let pattern = HexLiteral(pattern as u32);
                        let child = child.parse_expr_tokens();
                        quote! {
                            #pattern => #child
                        }
                    });
                    quote! {
                        match (ins & #bitmask_literal) >> #shift {
                            #(#children),*,
                            _ => unreachable!(),
                        }
                    }
                }
            }
            OpcodeParseTree::Leaf { encodings, arch } => {
                if encodings.is_empty() {
                    quote! { None }
                } else if encodings.len() == 1 {
                    let e = &encodings[0];
                    let parse_fn_name = e.opcode.parse_fn_name(*arch, e.encoding_index);
                    let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
                    quote! {
                        #parse_fn_ident(ins, pc)
                    }
                } else {
                    let encoding_checks = encodings.iter().map(|e| {
                        let pattern = e.encoding.pattern().combined();
                        let bitmask = HexLiteral(pattern.bitmask());
                        let pattern = HexLiteral(pattern.pattern());
                        let parse_fn_name = e.opcode.parse_fn_name(*arch, e.encoding_index);
                        let parse_fn_ident = Ident::new(&parse_fn_name, Span::call_site());
                        quote! {
                            if (ins & #bitmask) == #pattern {
                                #parse_fn_ident(ins, pc)
                            }
                        }
                    });
                    quote! {
                        #(#encoding_checks)else* else {
                            None
                        }
                    }
                }
            }
        }
    }
}

mod disasm;
mod isa;
mod iter;
mod token;

use std::{fs, path::Path};

use anyhow::Result;
use isa::Isa;

use crate::disasm::generate_disasm;

/// Best bucket bitmask with 6 set bits, found by find_best_bucket_bitmask.
/// 35 of 64 non-empty buckets, maximum of 7 opcodes per bucket.
const BUCKET_BITMASK: u32 = 0x21f00000;

fn main() -> Result<()> {
    let arm = Isa::load(Path::new("arm.yaml"))?;
    arm.validate()?;
    let opcodes = arm.get_all_opcodes()?;
    println!("{} opcodes", arm.opcodes.len());
    println!("{} total opcodes", opcodes.len());

    let tokens = generate_disasm(&arm, BUCKET_BITMASK)?;
    let file = syn::parse2(tokens)?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/generated.rs", formatted)?;

    Ok(())
}

fn find_best_bucket_bitmask(isa: &Isa, size: u32) {
    let mut best_non_empty = 0;
    let mut best_max = u8::MAX;
    for i in 0..=u32::MAX {
        if i.count_ones() != size {
            continue;
        }
        let buckets = isa.count_opcode_buckets(i);
        let non_empty = buckets.iter().filter(|b| **b > 0).count();
        let max = *buckets.iter().max().unwrap_or(&0);
        if non_empty > best_non_empty {
            best_non_empty = non_empty;
            println!("Found new best mask 0x{:08x} with {} non-empty buckets", i, best_non_empty);
        }
        if max < best_max {
            best_max = max;
            println!("Found new best mask 0x{:08x} with max bucket size {}", i, best_max);
        }
    }
}

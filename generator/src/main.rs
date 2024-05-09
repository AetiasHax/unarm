mod disasm;
mod isa;
mod iter;
mod search;
mod token;

use std::{fs, path::Path};

use anyhow::{Context, Result};
use isa::Isa;

use crate::{disasm::generate_disasm, search::SearchTree};

/// These bits are always defined in every ARM mnemonic, and all other bits vary.
const BUCKET_BITMASK: u32 = 0x0c000000;

fn main() -> Result<()> {
    let arm = Isa::load(Path::new("arm.yaml"))?;
    arm.validate()?;
    println!("{} opcodes", arm.opcodes.len());

    // let tree = SearchTree::optimize(arm.opcodes.to_vec());

    let tokens = generate_disasm(&arm, BUCKET_BITMASK).context("While generating tokens for disassembler")?;
    let file = syn::parse2(tokens).context("While parsing tokens for disassembler")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/generated.rs", formatted)?;

    Ok(())
}

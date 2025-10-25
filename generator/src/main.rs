use std::fs::{self, File};

use anyhow::Context;
use proc_macro2::TokenStream;

use crate::isa::Isa;

mod generate;
mod isa;
mod util;

fn main() -> anyhow::Result<()> {
    let file = File::open("generator/assets/isa.yaml")?;
    let isa = Isa::parse(file)?;
    isa.validate()?;
    // println!("{:#?}", isa);

    generate_file("disasm/src/generated/types.rs", isa.generate_types())?;
    // println!("{}", isa.generate_parser());
    generate_file("disasm/src/generated/parse.rs", isa.generate_parser())?;
    // println!("{}", isa.generate_display());
    generate_file("disasm/src/generated/display.rs", isa.generate_display())?;
    // println!("{}", isa.generate_defs_uses());
    generate_file("disasm/src/generated/defs_uses.rs", isa.generate_defs_uses())?;

    Ok(())
}

fn generate_file(path: &str, tokens: TokenStream) -> anyhow::Result<()> {
    let file = syn::parse2(tokens).context("Failed to parse generated token stream")?;
    let formatted = prettyplease::unparse(&file);
    fs::write(path, formatted)?;
    Ok(())
}

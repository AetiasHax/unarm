use std::fs::{self, File};

use anyhow::Context;

use crate::{generate::DisassemblerGenerator, isa::Isa};

mod generate;
mod isa;
mod util;

fn main() -> anyhow::Result<()> {
    let file = File::open("generator/assets/isa.yaml")?;
    let isa: Isa = serde_yaml::from_reader(file)?;
    isa.validate()?;
    // println!("{:#?}", isa);

    let generator = DisassemblerGenerator { isa };
    let tokens = generator.generate();
    // println!("{tokens}");
    let file =
        syn::parse2(tokens).context("Failed to parse generated disassembler token stream")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/parse.rs", formatted)?;

    Ok(())
}

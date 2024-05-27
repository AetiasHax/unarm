mod args;
mod disasm;
mod isa;
mod iter;
mod search;
mod token;

use std::{fs, path::Path};

use anyhow::{Context, Result};
use args::IsaArgs;
use isa::Isa;

use crate::disasm::generate_disasm;

fn main() -> Result<()> {
    let args = IsaArgs::load(Path::new("args.yaml"))?;
    args.validate()?;

    let arm = Isa::load(Path::new("arm.yaml"))?;
    arm.validate()?;

    let thumb = Isa::load(Path::new("thumb.yaml"))?;
    thumb.validate()?;

    let tokens = generate_disasm(&arm, "arm").context("While generating tokens for ARM disassembler")?;
    let file = syn::parse2(tokens).context("While parsing tokens for ARM disassembler")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/arm/generated.rs", formatted)?;

    let tokens = generate_disasm(&thumb, "thumb").context("While generating tokens for Thumb disassembler")?;
    let file = syn::parse2(tokens).context("While parsing tokens for Thumb disassembler")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/thumb/generated.rs", formatted)?;

    Ok(())
}

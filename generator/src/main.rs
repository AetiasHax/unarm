mod args;
mod generate;
mod isa;
mod iter;
mod search;
mod token;
mod util;

use std::{fs, path::Path};

use anyhow::{Context, Result};
use args::IsaArgs;
use generate::{args::generate_args, disasm::generate_disasm};
use isa::Isa;

fn main() -> Result<()> {
    let args = IsaArgs::load(Path::new("args.yaml"))?;
    args.validate()?;

    let arm = Isa::load(Path::new("arm.yaml"))?;
    arm.validate(&args).context("While validating arm.yaml")?;

    let thumb = Isa::load(Path::new("thumb.yaml"))?;
    thumb.validate(&args).context("While validating thumb.yaml")?;

    let tokens = generate_args(&args).context("While generating tokens for arguments module")?;
    let file = syn::parse2(tokens).context("While parsing tokens for arguments module")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/args.rs", formatted)?;

    let tokens = generate_disasm(&arm, &args, "arm").context("While generating tokens for ARM disassembler")?;
    let file = syn::parse2(tokens).context("While parsing tokens for ARM disassembler")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/arm/generated.rs", formatted)?;

    let tokens = generate_disasm(&thumb, &args, "thumb").context("While generating tokens for Thumb disassembler")?;
    let file = syn::parse2(tokens).context("While parsing tokens for Thumb disassembler")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/thumb/generated.rs", formatted)?;

    Ok(())
}

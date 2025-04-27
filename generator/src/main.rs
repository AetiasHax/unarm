mod args;
mod generate;
mod isa;
mod iter;
mod search;
mod token;
mod util;

use std::fs;

use anyhow::{Context, Result};
use args::IsaArgs;
use generate::{args::generate_args, disasm::generate_disasm};
use isa::Isa;

fn main() -> Result<()> {
    let args = IsaArgs::load("specs/args.yaml")?;
    args.validate().context("While validation args.yaml")?;

    let arm_isa = Isa::load("specs/arm.yaml")?;
    arm_isa
        .validate(&args)
        .with_context(|| "While validating arm.yaml".to_string())?;

    let thumb_isa = Isa::load("specs/thumb.yaml")?;
    thumb_isa
        .validate(&args)
        .with_context(|| "While validating thumb.yaml".to_string())?;

    let max_args = arm_isa
        .get_max_args(false)?
        .max(arm_isa.get_max_args(true)?)
        .max(thumb_isa.get_max_args(false)?)
        .max(thumb_isa.get_max_args(true)?);

    let tokens = generate_args(&args, max_args).context("While generating tokens for arguments module")?;
    let file = syn::parse2(tokens).context("While parsing tokens for arguments module")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/args.rs", formatted)?;

    let tokens = generate_disasm(&arm_isa, &args, max_args)
        .with_context(|| "While generating disassembler for arm.yaml".to_string())?;
    let file = syn::parse2(tokens).with_context(|| "While parsing disassembler tokens for arm.yaml".to_string())?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/arm/generated.rs", formatted)?;

    let tokens = generate_disasm(&thumb_isa, &args, max_args)
        .with_context(|| "While generating disassembler for thumb.yaml".to_string())?;
    let file = syn::parse2(tokens).with_context(|| "While parsing disassembler tokens for thumb.yaml".to_string())?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/thumb/generated.rs", formatted)?;

    Ok(())
}

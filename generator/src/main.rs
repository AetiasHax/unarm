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
    let args = IsaArgs::load(Path::new("specs/args.yaml"))?;
    args.validate()?;

    let specs_path = Path::new("specs/");

    // Locate .yaml files in subdirs of `specs/`
    let mut isas = vec![];
    for dir in fs::read_dir(specs_path)? {
        let dir = dir?;
        if !dir.file_type()?.is_dir() {
            continue;
        }
        for file in fs::read_dir(dir.path())? {
            let file = file?;
            let path = file.path();
            let Some(ext) = path.extension() else {
                continue;
            };
            match ext.to_str() {
                Some("yaml") | Some("yml") => {}
                _ => continue,
            }

            let isa = Isa::load(&path)?;
            isa.validate(&args)
                .with_context(|| format!("While validating {}", path.display()))?;
            isas.push((path, isa));
        }
    }

    let max_args: Result<usize> = {
        let mut max_args = 0;
        for (_, isa) in &isas {
            max_args = max_args.max(isa.get_max_args(false)?).max(isa.get_max_args(true)?);
        }
        Ok(max_args)
    };
    let max_args = max_args?;

    let tokens = generate_args(&args, max_args).context("While generating tokens for arguments module")?;
    let file = syn::parse2(tokens).context("While parsing tokens for arguments module")?;
    let formatted = prettyplease::unparse(&file);
    fs::write("disasm/src/args.rs", formatted)?;

    for (path, isa) in &isas {
        let tokens = generate_disasm(isa, &args, max_args)
            .with_context(|| format!("While generating disassembler for {}", path.display()))?;
        let file = syn::parse2(tokens).with_context(|| format!("While parsing disassembler tokens for {}", path.display()))?;
        let formatted = prettyplease::unparse(&file);

        let module_name = path
            .file_stem()
            .context("ISA file has no file name")?
            .to_str()
            .context("Failed to convert ISA file name to string")?;

        let module_path = path
            .parent()
            .context("Output path has no parent")?
            .strip_prefix(specs_path)?
            .join(module_name);

        let out_path = format!("disasm/src/{}/generated.rs", module_path.display());
        println!("{}", out_path);
        fs::write(out_path, formatted)?;
    }

    Ok(())
}

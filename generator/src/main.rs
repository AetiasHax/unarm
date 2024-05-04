mod isa;

use std::path::Path;

use anyhow::Result;
use isa::Isa;

fn main() -> Result<()> {
    let arm = Isa::load(Path::new("arm.yaml"))?;
    arm.validate()?;

    Ok(())
}

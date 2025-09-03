mod bit_range;
mod data_type;
mod format;
mod opcode;
mod option;
mod pattern;
mod syn;
mod version;

use anyhow::Result;
use serde::Deserialize;

pub use bit_range::*;
pub use data_type::*;
pub use format::*;
pub use opcode::*;
pub use option::*;
pub use pattern::*;
pub use syn::*;
pub use version::*;

#[derive(Deserialize, Debug)]
pub struct Isa {
    options: IsaOptions,
    versions: IsaVersions,
    types: DataTypes,
    opcodes: Opcodes,
}

impl Isa {
    pub fn validate(&self) -> Result<()> {
        self.types.validate()?;
        for opcode in self.opcodes.iter() {
            opcode.validate(self)?;
        }
        Ok(())
    }

    pub fn options(&self) -> &IsaOptions {
        &self.options
    }

    pub fn types(&self) -> &DataTypes {
        &self.types
    }

    pub fn versions(&self) -> &IsaVersions {
        &self.versions
    }

    pub fn opcodes(&self) -> &Opcodes {
        &self.opcodes
    }
}

pub enum Arch {
    Arm,
    Thumb,
}

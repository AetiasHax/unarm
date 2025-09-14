mod bit_range;
mod data_type;
mod extension;
mod format;
mod opcode;
mod option;
mod pattern;
mod syn;
mod version;

use std::io::Read;

use anyhow::Result;
use serde::Deserialize;

pub use bit_range::*;
pub use data_type::*;
pub use extension::*;
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
    extensions: IsaExtensions,
    types: DataTypes,
    opcodes: Opcodes,
}

impl Isa {
    pub fn parse<R>(r: R) -> Result<Self>
    where
        R: Read,
    {
        let mut isa: Self = serde_yaml::from_reader(r)?;
        isa.types.post_process();
        Ok(isa)
    }

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

    pub fn extensions(&self) -> &IsaExtensions {
        &self.extensions
    }

    pub fn opcodes(&self) -> &Opcodes {
        &self.opcodes
    }
}

pub enum Arch {
    Arm,
    Thumb,
}

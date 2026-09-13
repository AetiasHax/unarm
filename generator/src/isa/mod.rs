mod bit_range;
mod cfg;
mod data_type;
mod defs_uses;
mod extension;
mod format;
mod getter;
mod illegal;
mod lookup_table;
mod opcode;
mod option;
mod pattern;
mod syn;
mod version;

use std::{fmt::Display, io::Read};

use anyhow::Result;
pub use bit_range::*;
pub use cfg::*;
pub use data_type::*;
pub use defs_uses::*;
pub use extension::*;
pub use format::*;
pub use getter::*;
pub use illegal::*;
pub use lookup_table::*;
pub use opcode::*;
pub use option::*;
pub use pattern::*;
use serde::Deserialize;
pub use syn::*;
pub use version::*;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Isa {
    options: IsaOptions,
    versions: IsaVersions,
    extensions: IsaExtensions,
    types: DataTypes,
    getters: Getters,
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
        self.types.validate(self)?;
        for opcode in self.opcodes.iter() {
            opcode.validate(self)?;
        }
        self.getters.validate(self)?;
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

    pub fn getters(&self) -> &Getters {
        &self.getters
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    Arm,
    Thumb,
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::Arm => f.write_str("arm"),
            Arch::Thumb => f.write_str("thumb"),
        }
    }
}

use std::{collections::HashMap, fs::File, path::Path};

use serde::Deserialize;

use anyhow::{bail, Context, Result};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsaArgs {
    pub types: Box<[Type]>,
    pub args: Box<[Arg]>,
}

impl IsaArgs {
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("Failed to open args file '{}'", path.display()))?;
        let args: Self =
            serde_yml::from_reader(file).with_context(|| format!("While parsing args file '{}'", path.display()))?;
        Ok(args)
    }

    pub fn validate(&self) -> Result<()> {
        for r#type in self.types.iter() {
            r#type.validate()?;
        }
        for arg in self.args.iter() {
            arg.validate(self)?;
        }
        Ok(())
    }

    pub fn get_type(&self, name: &str) -> Result<&Type> {
        self.types
            .iter()
            .find(|t| t.name == name)
            .with_context(|| format!("Failed to find type: '{}'", name))
    }

    pub fn get_arg(&self, name: &str) -> Result<&Arg> {
        self.args
            .iter()
            .find(|a| a.name == name)
            .with_context(|| format!("Failed to find argument: '{}'", name))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Type {
    pub name: String,
    pub desc: String,
    pub r#type: ArgType,
}

impl Type {
    pub fn validate(&self) -> Result<()> {
        if let ArgType::Custom(_) = self.r#type {
            bail!("Argument type '{}' can't be custom", self.name)
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arg {
    pub name: String,
    pub desc: String,
    pub r#type: ArgType,
}

impl Arg {
    pub fn validate(&self, args: &IsaArgs) -> Result<()> {
        if let ArgType::Custom(name) = &self.r#type {
            args.get_type(name).map(|_| ())?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ArgType {
    Struct(HashMap<String, StructMember>),
    Enum(HashMap<String, EnumValue>),
    U32,
    I32,
    Bool,
    Custom(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructMember {
    pub desc: String,
    pub r#type: ArgType,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumValue {
    pub desc: Option<String>,
    pub value: u32,
}

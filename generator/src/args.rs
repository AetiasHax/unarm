use std::{collections::BTreeMap, fs::File, path::Path};

use serde::Deserialize;

use anyhow::{Context, Result};

use crate::util::capitalize_with_delimiter;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsaArgs {
    pub types: Box<[Type]>,
    pub args: Box<[Arg]>,
}

impl IsaArgs {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).with_context(|| format!("Failed to open args file '{}'", path.display()))?;
        let args: Self =
            serde_yml::from_reader(file).with_context(|| format!("While parsing args file '{}'", path.display()))?;
        Ok(args)
    }

    pub fn validate(&self) -> Result<()> {
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
    pub r#type: TypeKind,
}

impl Type {
    pub fn pascal_case_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TypeKind {
    Struct(BTreeMap<String, StructMember>),
    Enum(Box<[EnumValue]>),
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

    pub fn pascal_case_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ArgType {
    Struct(BTreeMap<String, StructMember>),
    Enum(Box<[EnumValue]>),
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

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct EnumValue {
    pub name: String,
    pub desc: Option<String>,
    pub value: u32,
}

impl EnumValue {
    pub fn pascal_case_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }
}

pub fn is_continuous(values: &[EnumValue]) -> bool {
    let values = {
        let mut values = Vec::from(values);
        values.sort_unstable_by_key(|v| v.value);
        values
    };
    let mut prev_value = values[0].value;
    for value in values.iter().skip(1) {
        if value.value != prev_value + 1 {
            return false;
        }
        prev_value = value.value;
    }
    true
}

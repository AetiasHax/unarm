use std::{fs::File, ops::Range, path::Path};

use anyhow::{bail, Context, Result};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Isa {
    pub fields: Box<[Field]>,
    pub modifiers: Box<[Modifier]>,
    pub opcodes: Box<[Opcode]>,
}

impl Isa {
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("Failed to open ISA file '{}'", path.display()))?;
        let isa: Self =
            serde_yml::from_reader(file).with_context(|| format!("While parsing ISA file '{}'", path.display()))?;
        Ok(isa)
    }

    pub fn validate(&self) -> Result<()> {
        for modifier in self.modifiers.iter() {
            modifier.validate(self)?;
        }
        for opcode in self.opcodes.iter() {
            opcode.validate(self)?;
        }
        Ok(())
    }

    pub fn get_modifier(&self, name: &str) -> Result<&Modifier> {
        self.modifiers
            .iter()
            .find(|m| m.name == name)
            .with_context(|| format!("Failed to find modifier '{name}'"))
    }

    pub fn get_field(&self, name: &str) -> Result<&Field> {
        self.fields
            .iter()
            .find(|f| f.name == name)
            .with_context(|| format!("Failed to find field '{name}'"))
    }
}

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    pub desc: String,
    pub bits: BitRange,
}

impl Field {
    pub fn get_bitmask(&self) -> u32 {
        ((1 << self.bits.0.len()) - 1) << self.bits.0.start
    }

    pub fn validate(&self) -> Result<()> {
        if self.get_bitmask() == 0 {
            bail!("Field {} has no bitmask", self.name)
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Modifier {
    pub name: String,
    pub desc: String,
    pub bitmask: Option<u32>,
    pub pattern: Option<u32>,
    pub suffix: Option<String>,
    pub nsuffix: Option<String>,
    pub cases: Option<Box<[ModifierCase]>>,
}

impl Modifier {
    /// Gets the combined (bitwise OR) bitmask of all cases
    pub fn get_full_bitmask(&self, isa: &Isa) -> Result<u32> {
        let mod_bitmask = self.bitmask.unwrap_or(0);
        if let Some(cases) = &self.cases {
            let first_case = cases
                .first()
                .with_context(|| format!("Empty case list for modifier '{}'", self.name))?;
            let cases_bitmask = first_case
                .get_bitmask(isa, self)
                .with_context(|| format!("While getting first case bitmask for modifier '{}'", self.name))?;
            for case in cases.iter() {
                let bitmask = case
                    .get_bitmask(isa, self)
                    .with_context(|| format!("While getting bitmask for modifier '{}'", self.name))?;
                if bitmask != cases_bitmask {
                    bail!(
                        "Case '{}' with bitmask 0x{:08x} doesn't match other case bitmasks (0x{:08x}) in modifier '{}'",
                        case.name,
                        bitmask,
                        cases_bitmask,
                        self.name
                    )
                }
            }
            Ok(cases_bitmask)
        } else {
            Ok(mod_bitmask)
        }
    }

    pub fn validate(&self, isa: &Isa) -> Result<()> {
        if let Some(cases) = &self.cases {
            for case in cases.iter() {
                case.validate(isa, self)
                    .with_context(|| format!("In modifier '{}'", self.name))?;
            }
        } else {
            let bitmask = self
                .get_full_bitmask(isa)
                .with_context(|| format!("While validating modifier '{}'", self.name))?;
            if bitmask == 0 {
                bail!("Modifier '{}' has no bitmask", self.name)
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct ModifierCase {
    pub name: String,
    pub desc: String,
    pub suffix: Option<String>,
    pub bitmask: Option<u32>,
    pub pattern: u32,
    pub args: Option<Box<[String]>>,
    pub uses: Option<Box<[String]>>,
}

impl ModifierCase {
    pub fn get_bitmask(&self, isa: &Isa, parent: &Modifier) -> Result<u32> {
        let case_bitmask = self.bitmask.or(parent.bitmask).unwrap_or(0);
        if let Some(args) = &self.args {
            let mut arg_bitmask = 0;
            for arg in args.iter() {
                let arg = isa
                    .get_field(arg)
                    .with_context(|| format!("While getting bitmask for modifier case '{}'", self.name))?;
                arg_bitmask |= arg.get_bitmask();
            }
            Ok(arg_bitmask | case_bitmask)
        } else {
            Ok(case_bitmask)
        }
    }

    pub fn validate(&self, isa: &Isa, parent: &Modifier) -> Result<()> {
        if self.get_bitmask(isa, parent)? == 0 {
            bail!("Modifier case '{}' has no bitmask", self.name)
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Opcode {
    pub name: String,
    pub desc: String,
    pub bitmask: u32,
    pub pattern: u32,
    pub modifiers: Option<Box<[String]>>,
    pub args: Option<Box<[String]>>,
    pub defs: Option<Box<[String]>>,
    pub uses: Option<Box<[String]>>,
}

impl Opcode {
    pub fn validate(&self, isa: &Isa) -> Result<()> {
        if self.pattern & !self.bitmask != 0 {
            bail!(
                "Opcode '{}' has pattern bits 0x{:08x} outside its bitmask 0x{:08x}",
                self.name,
                self.pattern,
                self.bitmask
            )
        }

        let mut bitmask_acc = self.bitmask;
        if let Some(modifiers) = &self.modifiers {
            for modifier in modifiers.iter() {
                let modifier = isa
                    .get_modifier(modifier)
                    .with_context(|| format!("While validating opcode '{}'", self.name))?;
                let bitmask = modifier
                    .get_full_bitmask(isa)
                    .with_context(|| format!("While validating opcode '{}'", self.name))?;
                bitmask_acc |= bitmask;
            }
        }
        if let Some(args) = &self.args {
            for arg in args.iter() {
                let arg = isa
                    .get_field(arg)
                    .with_context(|| format!("While validating opcode '{}'", self.name))?;
                let bitmask = arg.get_bitmask();
                if bitmask_acc & bitmask != 0 {
                    bail!(
                        "Argument '{}' (0x{:08x}) collides with other bitmasks in opcode '{}' (0x{:08x})",
                        arg.name,
                        bitmask,
                        self.name,
                        bitmask_acc
                    )
                }
                bitmask_acc |= arg.get_bitmask();
            }
        }
        if bitmask_acc != u32::MAX {
            bail!("Opcode '{}' has an incomplete bitmask 0x{:08x}", self.name, bitmask_acc)
        }
        Ok(())
    }
}

pub struct BitRange(Range<u8>);

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Credits to ppc750cl (MIT License):
        // https://github.com/encounter/ppc750cl/blob/6cbd7d888c7082c2c860f66cbb9848d633f753ed/genisa/src/isa.rs#L229

        let range_str: String = Deserialize::deserialize(deserializer)?;
        if let Some((start_str, end_str)) = range_str.split_once("..") {
            let start = start_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            let end = end_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            Ok(Self(Range { start, end }))
        } else {
            let bit_idx = range_str.parse::<u8>().map_err(serde::de::Error::custom)?;
            Ok(Self(Range {
                start: bit_idx,
                end: bit_idx + 1,
            }))
        }
    }
}

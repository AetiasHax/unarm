use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{bail, Context, Result};

use regex::Regex;
use serde::Deserialize;
use syn::Expr;

use crate::{
    args::{Arg, ArgType, IsaArgs},
    iter::cartesian,
    util::{capitalize_with_delimiter, BitRange},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Isa {
    pub ins_size: u32,
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

    pub fn validate(&self, args: &IsaArgs) -> Result<()> {
        for field in self.fields.iter() {
            field.validate(args)?;
        }
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

    pub fn get_max_args(&self) -> Result<usize> {
        let mut max = 0;
        for opcode in self.opcodes.iter() {
            let args = opcode.get_max_args(self)?;
            if args > max {
                max = args;
            }
        }
        Ok(max)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    pub arg: String,
    pub desc: String,
    #[serde(default)]
    pub allow_collide: bool,
    #[serde(default)]
    pub no_bitmask: bool,
    pub value: FieldValue,
}

impl Field {
    pub fn get_bitmask(&self) -> Result<u32> {
        self.value.get_bitmask()
    }

    fn validate(&self, isa_args: &IsaArgs) -> Result<()> {
        if !self.no_bitmask && self.get_bitmask()? == 0 {
            bail!(
                "Field {} has no bitmask, please specify `no_bitmask: true` if this is intentional",
                self.name
            )
        }
        let arg = isa_args.get_arg(&self.arg)?;
        self.value.validate(self, arg)?;
        Ok(())
    }

    pub fn doc(&self) -> String {
        format!(" {}: {}", self.name, self.desc)
    }

    pub fn accessor_name(&self) -> String {
        format!("field_{}", self.name.to_lowercase())
    }

    pub fn struct_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub enum FieldValue {
    Bits(BitRange),
    Bool(bool),
    U32(u32),
    Struct(HashMap<String, FieldValue>),
    Expr(String),
}

impl FieldValue {
    pub fn get_bitmask(&self) -> Result<u32> {
        match self {
            FieldValue::Bits(range) => Ok(range.bitmask()),
            FieldValue::Bool(_) => Ok(0),
            FieldValue::U32(_) => Ok(0),
            FieldValue::Struct(members) => {
                let mut mask = 0;
                for value in members.values() {
                    mask |= value.get_bitmask()?;
                }
                Ok(mask)
            }
            FieldValue::Expr(expr) => {
                let bits_regex = Regex::new(r"code\.bits\((\d+),(\d+)\)")?;
                let bit_regex = Regex::new(r"code\.bit\((\d+)\)")?;

                let mut mask = 0;
                for (_, [min, max]) in bits_regex.captures_iter(expr).map(|c| c.extract()) {
                    let min = min.parse()?;
                    let max = max.parse()?;
                    let range = BitRange(min..max);
                    mask |= range.bitmask();
                }
                for (_, [bit]) in bit_regex.captures_iter(expr).map(|c| c.extract()) {
                    let bit: u32 = bit.parse()?;
                    mask |= 1 << bit;
                }

                Ok(mask)
            }
        }
    }

    pub fn validate(&self, field: &Field, arg: &Arg) -> Result<()> {
        match (self, &arg.r#type) {
            (FieldValue::Struct(values), ArgType::Struct(members)) => {
                let not_members: Vec<_> = values.keys().filter(|k| !members.contains_key(k.as_str())).cloned().collect();
                if !not_members.is_empty() {
                    bail!(
                        "The field values [{}] of field '{}' do not exist in the argument '{}'",
                        not_members.join(", "),
                        field.name,
                        arg.name
                    );
                }
                let missing_members: Vec<_> = members.keys().filter(|k| !values.contains_key(k.as_str())).cloned().collect();
                if !missing_members.is_empty() {
                    bail!(
                        "Missing values [{}] of field '{}' for the argument '{}'",
                        missing_members.join(", "),
                        field.name,
                        arg.name
                    );
                }
            }

            (_, ArgType::Struct(_)) => bail!("Expected value to be struct in field '{}'", field.name),
            (FieldValue::Struct(_), _) => bail!("Expected value to be numeric in field '{}'", field.name),

            _ => {}
        }
        if let Self::Expr(expr) = self {
            syn::parse_str::<Expr>(expr)?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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

    fn validate(&self, isa: &Isa) -> Result<()> {
        let bitmask = self
            .get_full_bitmask(isa)
            .with_context(|| format!("While validating modifier '{}'", self.name))?;
        if let Some(cases) = &self.cases {
            for case in cases.iter() {
                case.validate(isa, self)
                    .with_context(|| format!("In modifier '{}'", self.name))?;
            }
        } else if bitmask == 0 {
            bail!("Modifier '{}' has no bitmask", self.name)
        }
        if self.nsuffix.is_some() && self.cases.is_some() {
            bail!("Modifier '{}' has a negative suffix and modifier cases", self.name)
        }
        if self.nsuffix.is_some() && bitmask.count_ones() > 1 {
            bail!(
                "Modifier '{}' has a negative suffix but the bitmask 0x{:08x} has more than 1 bit",
                self.name,
                bitmask
            )
        }
        Ok(())
    }

    pub fn get_cases(&self) -> Result<Box<[ModifierCase]>> {
        if let Some(cases) = &self.cases {
            let cases_vec: Vec<_> = cases.iter().map(|c| c.inherit(self)).collect();
            Ok(cases_vec.into_boxed_slice())
        } else {
            Ok(Box::new([
                ModifierCase::from_modifier(self, false)?,
                ModifierCase::from_modifier(self, true)?,
            ]))
        }
    }

    pub fn doc(&self) -> String {
        format!(" {}: {}", self.name, self.desc)
    }

    pub fn accessor_name(&self) -> String {
        format!("modifier_{}", self.name.to_lowercase())
    }

    pub fn enum_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }
}

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct ModifierCase {
    pub name: String,
    pub desc: Option<String>,
    pub suffix: Option<String>,
    pub bitmask: Option<u32>,
    pub pattern: u32,
    pub args: Option<Box<[String]>>,
    pub defs: Option<Box<[String]>>,
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
                arg_bitmask |= arg.get_bitmask()?;
            }
            Ok(arg_bitmask | case_bitmask)
        } else {
            Ok(case_bitmask)
        }
    }

    fn validate(&self, isa: &Isa, parent: &Modifier) -> Result<()> {
        if self.get_bitmask(isa, parent)? == 0 {
            bail!("Modifier case '{}' has no bitmask", self.name)
        }
        Ok(())
    }

    pub fn from_modifier(modifier: &Modifier, negative: bool) -> Result<Self> {
        let (desc, suffix, pattern) = if !negative {
            (
                Some(modifier.desc.clone()),
                modifier.suffix.clone(),
                modifier.pattern.with_context(|| {
                    format!("No modifier case pattern to inherit from parent modifier '{}'", modifier.name)
                })?,
            )
        } else {
            (None, modifier.nsuffix.clone(), 0)
        };
        Ok(Self {
            name: modifier.name.clone(),
            desc,
            suffix,
            bitmask: modifier.bitmask,
            pattern,
            args: None,
            defs: None,
            uses: None,
        })
    }

    pub fn inherit(&self, parent: &Modifier) -> Self {
        Self {
            name: self.name.clone(),
            desc: self.desc.clone(),
            suffix: self.suffix.clone().or(parent.suffix.clone()),
            bitmask: self.bitmask.or(parent.bitmask),
            pattern: self.pattern,
            args: self.args.clone(),
            defs: self.defs.clone(),
            uses: self.uses.clone(),
        }
    }

    pub fn variant_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }

    pub fn doc(&self) -> String {
        if let Some(desc) = &self.desc {
            format!(" {}: {}", self.name, desc)
        } else {
            format!(" {}", self.name)
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Opcode {
    name: String,
    pub desc: String,
    #[serde(default)]
    pub suffix: String,
    pub bitmask: u32,
    pub pattern: u32,
    pub modifiers: Option<Box<[String]>>,
    pub args: Option<Box<[String]>>,
    pub defs: Option<Box<[String]>>,
    pub uses: Option<Box<[String]>>,
}

impl Opcode {
    fn validate(&self, isa: &Isa) -> Result<()> {
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
                let bitmask = arg.get_bitmask()?;
                if !arg.allow_collide && (bitmask_acc & bitmask) != 0 {
                    bail!(
                        "Argument '{}' (0x{:08x}) collides with other bitmasks in opcode '{}' (0x{:08x})",
                        arg.name,
                        bitmask,
                        self.name,
                        bitmask_acc
                    )
                }
                bitmask_acc |= bitmask;
            }
        }
        let complete_bitmask = ((1u64 << isa.ins_size) - 1).try_into().unwrap();
        if bitmask_acc != complete_bitmask {
            bail!("Opcode '{}' has an incomplete bitmask 0x{:08x}", self.name, bitmask_acc)
        }
        Ok(())
    }

    pub fn base_name(&self) -> &str {
        if let Some((name, _)) = self.name.split_once('$') {
            name
        } else {
            &self.name
        }
    }

    pub fn name(&self) -> String {
        self.base_name().to_owned() + &self.suffix
    }

    pub fn doc(&self) -> String {
        format!(" {}: {}", self.name().to_uppercase(), self.desc)
    }

    pub fn enum_name(&self) -> String {
        // Split by $ delimiter, capitalize all the words, then join them
        // e.g. smlal$xy => SmlalXy
        capitalize_with_delimiter(self.name.clone(), '$')
    }

    pub fn ident_name(&self) -> String {
        self.name.replace('$', "_")
    }

    pub fn parser_name(&self) -> String {
        format!("parse_{}", self.ident_name())
    }

    pub fn get_modifier_cases(&self, isa: &Isa) -> Result<Vec<Box<[ModifierCase]>>> {
        if let Some(modifiers) = &self.modifiers {
            let modifiers: Result<Vec<_>> = modifiers
                .iter()
                .map(|m| {
                    let modifier = isa.get_modifier(m)?;
                    modifier.get_cases()
                })
                .collect();
            modifiers
        } else {
            Ok(vec![])
        }
    }

    fn get_max_args(&self, isa: &Isa) -> Result<usize> {
        let base_args = self.args.as_ref().map(|args| args.len()).unwrap_or(0);
        let modifiers = self.get_modifier_cases(isa)?;
        let max_case_args = cartesian(&modifiers)
            .map(|modifiers| {
                modifiers
                    .iter()
                    .map(|case| case.args.as_ref().map(|args| args.len()).unwrap_or(0))
                    .sum()
            })
            .max()
            .unwrap_or(0);
        Ok(base_args + max_case_args)
    }
}

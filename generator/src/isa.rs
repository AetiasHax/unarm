use std::{cmp::Ordering, fmt::Display, fs::File, ops::Range, path::Path};

use anyhow::{bail, Context, Result};

use serde::Deserialize;

use crate::iter::cartesian;

#[derive(Deserialize)]
pub struct Isa {
    pub args: Box<[Arg]>,
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
        for arg in self.args.iter() {
            arg.validate()?;
        }
        for field in self.fields.iter() {
            field.validate()?;
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

    pub fn get_arg(&self, name: &str) -> Result<&Arg> {
        self.args
            .iter()
            .find(|a| a.name == name)
            .with_context(|| format!("Failed to find argument '{name}'"))
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

    pub fn get_all_opcodes(&self) -> Result<Box<[Opcode]>> {
        let opcodes: Result<Vec<_>> = self.opcodes.iter().map(|opcode| opcode.get_variants(self)).collect();
        let opcodes = opcodes?;
        let opcodes: Vec<_> = opcodes.iter().flat_map(|op| op.iter()).cloned().collect();
        Ok(opcodes.into_boxed_slice())
    }

    pub fn calc_opcode_buckets(&self, bitmask: u32) -> Vec<Vec<Opcode>> {
        let num_buckets = 1 << bitmask.count_ones();
        let mut buckets: Vec<Vec<Opcode>> = vec![vec![]; num_buckets];
        for opcode in self.opcodes.iter() {
            let bucket = opcode.bucket_index(bitmask);
            buckets[bucket].push(opcode.clone());
        }
        buckets
    }

    pub fn count_opcode_buckets(&self, bitmask: u32) -> Vec<u8> {
        let num_buckets = 1 << bitmask.count_ones();
        let mut buckets: Vec<u8> = vec![0; num_buckets];
        for opcode in self.opcodes.iter() {
            let bucket = opcode.bucket_index(bitmask);
            buckets[bucket] += 1;
        }
        buckets
    }
}

#[derive(Deserialize)]
pub struct Arg {
    pub name: String,
    pub desc: String,
    pub values: Option<Box<[ArgValue]>>,
    #[serde(default)]
    pub signed: bool,
    #[serde(default)]
    pub boolean: bool,
}

impl Arg {
    pub fn variant_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }

    pub fn doc(&self) -> String {
        format!(" {}: {}", self.name, self.desc)
    }

    pub fn validate(&self) -> Result<()> {
        match (&self.values, self.signed, self.boolean) {
            (None, true, true) => bail!("Arg '{}' is both signed and boolean", self.name),
            (Some(_), true, true) => bail!("Arg '{}' has values and is both signed and boolean", self.name),
            (Some(_), true, false) => bail!("Arg '{}' has values and is signed", self.name),
            (Some(_), false, true) => bail!("Arg '{}' has values and is boolean", self.name),
            _ => {}
        }
        if !self.is_continuous() {
            bail!("Arg '{}' is not continuous", self.name);
        }
        Ok(())
    }

    pub fn is_continuous(&self) -> bool {
        if let Some(values) = &self.values {
            let sorted_values = {
                let mut sorted_values: Vec<_> = values.iter().collect();
                sorted_values.sort_unstable_by_key(|v| v.value);
                sorted_values
            };
            for values in sorted_values.windows(2) {
                if values[0].value + 1 != values[1].value {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Deserialize)]
pub struct ArgValue {
    pub name: String,
    pub desc: Option<String>,
    pub value: u8,
}

impl ArgValue {
    pub fn variant_name(&self) -> String {
        capitalize_with_delimiter(self.name.clone(), '_')
    }

    pub fn doc(&self) -> String {
        if let Some(desc) = &self.desc {
            format!(" {}: {}", self.name, desc)
        } else {
            "".to_string()
        }
    }
}

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    pub arg: String,
    pub desc: String,
    pub bits: BitRange,
}

impl Field {
    pub fn get_bitmask(&self) -> u32 {
        ((1 << self.bits.0.len()) - 1) << self.bits.0.start
    }

    fn validate(&self) -> Result<()> {
        if self.get_bitmask() == 0 {
            bail!("Field {} has no bitmask", self.name)
        }
        Ok(())
    }

    pub fn doc(&self) -> String {
        format!(" {}: {}", self.name, self.desc)
    }

    pub fn accessor_name(&self) -> String {
        format!("field_{}", self.name.to_lowercase())
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
                arg_bitmask |= arg.get_bitmask();
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

    pub fn get_variants(&self, isa: &Isa) -> Result<Box<[Opcode]>> {
        let modifiers = self.get_modifier_cases(isa)?;
        if !modifiers.is_empty() {
            let variants: Vec<_> = cartesian(&modifiers)
                .map(|cases| cases.iter().fold(self.clone(), |acc, case| acc.apply_case(case)))
                .collect();
            Ok(variants.into_boxed_slice())
        } else {
            Ok(Box::new([self.clone()]))
        }
    }

    fn apply_case(self, case: &ModifierCase) -> Self {
        let args = join_strings(self.args, &case.args);
        let defs = join_strings(self.defs, &case.defs);
        let uses = join_strings(self.uses, &case.uses);

        let desc = match &case.desc {
            Some(case_desc) => self.desc + ", " + case_desc,
            None => self.desc,
        };

        Self {
            name: self.name + &case.suffix.clone().unwrap_or("".to_string()),
            desc,
            bitmask: self.bitmask | case.bitmask.unwrap_or(0),
            pattern: self.pattern | case.pattern,
            modifiers: None,
            args,
            defs,
            uses,
        }
    }

    pub fn bucket_index(&self, mut bitmask: u32) -> usize {
        let code = self.pattern;
        let mut index = 0;
        let mut total_shift = 0;
        let mut bucket_bits = 0;
        while bitmask != 0 {
            let zero_shift = bitmask.trailing_zeros();
            bitmask >>= zero_shift;
            let one_shift = bitmask.trailing_ones();
            let bits = (1 << one_shift) - 1;
            bitmask >>= one_shift;

            total_shift += zero_shift;
            let mask = bits << total_shift;
            let bucket_shift: i32 = total_shift as i32 - bucket_bits as i32;
            match bucket_shift.cmp(&0) {
                Ordering::Greater => index |= (code & mask) >> bucket_shift,
                Ordering::Less => index |= (code & mask) << bucket_shift,
                Ordering::Equal => index |= code & mask,
            };
            bucket_bits += one_shift;
            total_shift += one_shift;
        }
        index.try_into().unwrap()
    }

    pub fn name(&self) -> &str {
        if let Some((name, _)) = self.name.split_once('$') {
            name
        } else {
            &self.name
        }
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

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} - {}", self.name, self.desc)?;
        writeln!(f, "bitmask: 0x{:08x}", self.bitmask)?;
        writeln!(f, "pattern: 0x{:08x}", self.pattern)?;
        // writeln!(f, "args: {}", self.args.clone().unwrap_or(Box::new([])).join(", "))?;
        // writeln!(f, "defs: {}", self.defs.clone().unwrap_or(Box::new([])).join(", "))?;
        // writeln!(f, "uses: {}", self.uses.clone().unwrap_or(Box::new([])).join(", "))?;
        Ok(())
    }
}

pub struct BitRange(pub Range<u8>);

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

fn join_strings(a: Option<Box<[String]>>, b: &Option<Box<[String]>>) -> Option<Box<[String]>> {
    let slice = a
        .iter()
        .flat_map(|a| a.iter())
        .cloned()
        .chain(b.iter().flat_map(|b| b.iter()).cloned())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    if slice.is_empty() {
        None
    } else {
        Some(slice)
    }
}

fn capitalize_with_delimiter(s: String, delim: char) -> String {
    s.split(delim)
        .map(|s| {
            let mut chars = s.chars();
            let mut name = match chars.next() {
                None => return "".to_string(),
                Some(c) => c.to_uppercase().to_string(),
            };
            chars.for_each(|c| c.to_lowercase().for_each(|c| name.push(c)));
            name
        })
        .collect()
}

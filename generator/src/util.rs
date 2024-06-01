use std::ops::Range;

use serde::Deserialize;

pub struct BitRange(pub Range<u8>);

impl BitRange {
    pub fn bitmask(&self) -> u32 {
        ((1 << self.0.len()) - 1) << self.0.start
    }
}

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

pub fn capitalize_with_delimiter(s: String, delim: char) -> String {
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

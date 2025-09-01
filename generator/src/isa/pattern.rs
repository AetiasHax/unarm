use std::str::FromStr;

use anyhow::bail;
use serde::{
    Deserialize,
    de::{self, IgnoredAny, Visitor},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pattern {
    pattern: u64,
    bitmask: u64,
}

impl Pattern {
    pub fn pattern(&self) -> u64 {
        self.pattern
    }

    pub fn bitmask(&self) -> u64 {
        self.bitmask
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pattern = 0;
        let mut bitmask = 0;
        let mut chars = s.chars().peekable();
        while chars.peek().is_some() {
            pattern <<= 1;
            bitmask <<= 1;
            for c in chars.by_ref() {
                match c {
                    '1' => {
                        pattern |= 1;
                        bitmask |= 1;
                    }
                    '0' => {
                        bitmask |= 1;
                    }
                    'x' => {}
                    '_' => continue,
                    _ => bail!("Invalid character '{s}'"),
                }
                break;
            }
        }

        Ok(Self { pattern, bitmask })
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct OpcodePattern {
    first: Pattern,
    second: Option<Pattern>,
}

impl<'de> Deserialize<'de> for OpcodePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(OpcodePatternVisitor)
    }
}

struct OpcodePatternVisitor;

impl<'de> Visitor<'de> for OpcodePatternVisitor {
    type Value = OpcodePattern;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a pattern (e.g. 1111_0x01_010x_xxxx) or a sequence of two patterns")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(OpcodePattern {
            first: Pattern::from_str(v).map_err(serde::de::Error::custom)?,
            second: None,
        })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let first: String =
            seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let second: String =
            seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        if seq.next_element::<IgnoredAny>()?.is_some() {
            Err(serde::de::Error::invalid_length(3, &self))
        } else {
            Ok(OpcodePattern {
                first: Pattern::from_str(&first).map_err(serde::de::Error::custom)?,
                second: Some(Pattern::from_str(&second).map_err(serde::de::Error::custom)?),
            })
        }
    }
}

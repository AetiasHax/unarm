use std::str::FromStr;

use anyhow::{Result, bail};
use serde::{
    Deserialize,
    de::{self, IgnoredAny, Visitor},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pattern {
    pattern: u32,
    bitmask: u32,
    /// Number of bits in the pattern (including don't care bits)
    size: u32,
}

impl Pattern {
    pub fn pattern(&self) -> u32 {
        self.pattern
    }

    pub fn bitmask(&self) -> u32 {
        self.bitmask
    }

    pub fn num_bits(&self) -> u32 {
        self.bitmask.count_ones()
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pattern = 0;
        let mut bitmask = 0;
        let mut size = 0;
        let mut chars = s.chars().peekable();
        while chars.peek().is_some() {
            pattern <<= 1;
            bitmask <<= 1;
            size += 1;
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

        Ok(Self { pattern, bitmask, size })
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpcodePattern {
    first: Pattern,
    second: Option<Pattern>,
}

impl OpcodePattern {
    pub fn validate(&self, thumb: bool) -> Result<()> {
        if thumb {
            if self.first.size() != 16 {
                bail!("Thumb pattern must be 16 bits");
            }
            if let Some(second) = &self.second
                && second.size() != 16
            {
                bail!("Thumb pattern must be 16 bits");
            }
        } else {
            if self.first.size() != 32 {
                bail!("ARM pattern must be 32 bits");
            }
            if self.second.is_some() {
                bail!("ARM pattern cannot have a second part");
            }
        }
        Ok(())
    }

    pub fn combined(&self) -> Pattern {
        if let Some(second) = &self.second {
            Pattern {
                pattern: (second.pattern() << 16) | self.first.pattern(),
                bitmask: (second.bitmask() << 16) | self.first.bitmask(),
                size: self.first.size() + second.size(),
            }
        } else {
            self.first.clone()
        }
    }
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

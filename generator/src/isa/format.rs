use std::{fmt::Debug, str::FromStr};

use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Format {
    #[serde(rename = "if")]
    If(IfFormat),
    #[serde(rename = "match")]
    Match(MatchFormat),
    #[serde(rename = "fmt")]
    Fragments(FragmentsFormat),
}

impl Format {
    pub fn new_string(s: String) -> Self {
        Self::Fragments(FragmentsFormat { fragments: vec![FormatFragment::Text(s)] })
    }
}

#[derive(Deserialize, Debug, Clone)]
struct IfFormat {
    cond: SynExpr,
    #[serde(rename = "then")]
    if_true: Box<Format>,
    #[serde(rename = "else")]
    if_false: Box<Format>,
}

#[derive(Clone)]
struct SynExpr(syn::Expr);

impl<'de> Deserialize<'de> for SynExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        syn::parse_str(&s).map(SynExpr).map_err(serde::de::Error::custom)
    }
}

impl Debug for SynExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr = &self.0;
        write!(f, "{}", quote::quote! { #expr })
    }
}

#[derive(Deserialize, Debug, Clone)]
struct MatchFormat {
    value: String,
    cases: IndexMap<String, Format>,
}

#[derive(Debug, Clone)]
struct FragmentsFormat {
    fragments: Vec<FormatFragment>,
}

#[derive(Debug, Clone)]
enum FormatFragment {
    Text(String),
    Param(String),
}

impl FromStr for FragmentsFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fragments = Vec::new();
        let mut token = String::new();
        let mut parsing_param = false;
        for ch in s.chars() {
            match ch {
                '(' => {
                    if !token.is_empty() {
                        fragments.push(FormatFragment::Text(token));
                        token = String::new();
                    }
                    parsing_param = true;
                }
                ')' => {
                    if parsing_param {
                        fragments.push(FormatFragment::Param(token));
                        token = String::new();
                        parsing_param = false;
                    } else {
                        return Err("Unmatched closing parenthesis".into());
                    }
                }
                _ => {
                    token.push(ch);
                }
            }
        }
        if !token.is_empty() {
            if parsing_param {
                fragments.push(FormatFragment::Param(token));
            } else {
                fragments.push(FormatFragment::Text(token));
            }
        }
        Ok(Self { fragments })
    }
}

impl<'de> Deserialize<'de> for FragmentsFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

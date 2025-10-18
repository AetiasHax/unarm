use serde::Deserialize;

use crate::isa::{DataExpr, FormatCond};

#[derive(Debug, Deserialize, Clone)]
pub struct DefsUses(Vec<DefUse>);

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum DefUse {
    #[serde(rename = "if")]
    If(FormatCond, DataExpr),
    #[serde(rename = "always")]
    Always(DataExpr),
}

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SynExpr(pub syn::Expr);

impl<'de> Deserialize<'de> for SynExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        syn::parse_str(&s).map(SynExpr).map_err(serde::de::Error::custom)
    }
}

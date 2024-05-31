use crate::{
    parse::{ArmVersion, Parse},
    ParsedIns,
};

use super::Ins;

pub struct ParseThumbV5Te;

impl Parse for ParseThumbV5Te {
    fn version(&self) -> ArmVersion {
        ArmVersion::ARMv5TE
    }

    fn thumb(&self) -> bool {
        true
    }

    fn parse(&self, code: u32) -> ParsedIns {
        let ins = Ins::new(code);
        ParsedIns::parse_thumb_v5te(ins)
    }
}

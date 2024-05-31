use crate::{
    parse::{ArmVersion, Parse},
    ParsedIns,
};

use super::Ins;

pub struct ParseArmV5Te;

impl Parse for ParseArmV5Te {
    fn version(&self) -> ArmVersion {
        ArmVersion::ARMv5TE
    }

    fn thumb(&self) -> bool {
        false
    }

    fn parse(&self, code: u32) -> ParsedIns {
        let ins = Ins::new(code);
        ParsedIns::parse_arm_v5te(ins)
    }
}

use crate::{v6::arm::generated::Opcode, ParsedIns};

use super::parse;

#[derive(Clone, Copy)]
pub struct Ins {
    pub code: u32,
    pub op: Opcode,
}

impl Ins {
    pub fn new(code: u32) -> Self {
        let op = Opcode::find(code);
        Self { code, op }
    }

    pub fn parse(self) -> ParsedIns {
        let mut out = ParsedIns::default();
        parse(&mut out, self);
        out
    }
}

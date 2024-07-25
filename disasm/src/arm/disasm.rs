use crate::{arm::generated::Opcode, ParseFlags, ParsedIns};

use super::{parse, Cond};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ins {
    pub code: u32,
    pub op: Opcode,
}

impl Ins {
    pub fn new(code: u32, flags: &ParseFlags) -> Self {
        let op = Opcode::find(code, flags);
        Self { code, op }
    }

    pub fn parse(self, flags: &ParseFlags) -> ParsedIns {
        let mut out = ParsedIns::default();
        parse(&mut out, self, flags);
        out
    }

    pub fn is_conditional(&self) -> bool {
        self.has_cond() && self.modifier_cond() != Cond::Al
    }
}

use crate::{args::Arguments, arm::generated::Opcode, ParseFlags, ParsedIns};

use super::{defs, parse, uses, Cond};

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

    pub fn defs(self, flags: &ParseFlags) -> Arguments {
        let mut out = Arguments::default();
        defs(&mut out, self, flags);
        out
    }

    pub fn uses(self, flags: &ParseFlags) -> Arguments {
        let mut out = Arguments::default();
        uses(&mut out, self, flags);
        out
    }

    pub fn is_conditional(&self) -> bool {
        self.has_cond() && self.modifier_cond() != Cond::Al
    }

    pub fn updates_condition_flags(&self) -> bool {
        (self.has_s() && self.modifier_s()) || self.is_compare_op()
    }
}

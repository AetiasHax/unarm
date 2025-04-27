use crate::{args::Arguments, thumb::generated::Opcode, ParseFlags, ParsedIns};

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

    /// Returns whether this is a BL half-instruction and should be combined with the upcoming instruction
    pub fn is_half_bl(&self) -> bool {
        self.op == Opcode::BlH
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
}

use crate::{v5te::thumb::generated::Opcode, ParseFlags, ParsedIns};

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

    /// Returns whether this is a BL half-instruction and should be combined with the upcoming instruction
    pub fn is_half_bl(&self) -> bool {
        self.op == Opcode::BlH
    }

    pub fn parse(self, flags: &ParseFlags) -> ParsedIns {
        let mut out = ParsedIns::default();
        parse(&mut out, self, flags);
        out
    }
}

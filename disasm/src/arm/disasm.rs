use crate::{arm::generated::Opcode, ParsedIns};

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

pub struct InsIter<'a> {
    pub address: u32,
    pub data: &'a [u8],
}

impl<'a> InsIter<'a> {
    pub fn new(data: &'a [u8], address: u32) -> Self {
        Self { address, data }
    }
}

impl<'a> Iterator for InsIter<'a> {
    type Item = (u32, Ins);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 4 {
            return None;
        }
        let bytes = [self.data[0], self.data[1], self.data[2], self.data[3]];
        let ins = Ins::new(u32::from_le_bytes(bytes));
        let addr = self.address;
        self.address += 4;
        self.data = &self.data[4..];
        Some((addr, ins))
    }
}

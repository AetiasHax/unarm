use crate::thumb::generated::{parse, Argument, Arguments, Opcode};

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
}

#[derive(Default)]
pub struct ParsedIns {
    pub mnemonic: &'static str,
    pub args: Arguments,
}

impl ParsedIns {
    pub fn parse(ins: Ins) -> Self {
        let mut out = Self::default();
        parse(&mut out, ins);
        out
    }

    pub fn args_iter(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().take_while(|a| **a != Argument::None)
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
        if self.data.len() < 2 {
            return None;
        }
        let bytes = [self.data[0], self.data[1]];
        let ins = Ins::new(u16::from_le_bytes(bytes) as u32);
        let addr = self.address;
        self.address += 2;
        self.data = &self.data[2..];
        Some((addr, ins))
    }
}

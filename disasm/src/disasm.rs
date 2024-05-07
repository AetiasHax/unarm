use crate::generated::{Arguments, Opcode};

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

pub struct ParsedIns {
    pub mnemonic: &'static str,
    pub args: Arguments,
}

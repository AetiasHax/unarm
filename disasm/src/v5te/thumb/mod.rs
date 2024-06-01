mod disasm;
mod generated;

pub use disasm::{Ins, InsIter};

pub use generated::{parse, Cond, Opcode};

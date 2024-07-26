use crate::{
    args::{Argument, RegList, Register},
    Ins, ParsedIns,
};

impl Register {
    /// Bitmask for [`RegList`]s.
    pub fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

impl RegList {
    /// Returns whether this [`RegList`] contains a given [`Register`].
    pub fn contains(&self, register: Register) -> bool {
        self.regs & register.mask() != 0
    }
}

impl Ins {
    pub fn is_conditional(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.is_conditional(),
            Ins::Thumb(ins) => ins.is_conditional(),
            Ins::Data => false,
        }
    }

    pub fn mnemonic(&self) -> &str {
        match self {
            Ins::Arm(ins) => ins.op.mnemonic(),
            Ins::Thumb(ins) => ins.op.mnemonic(),
            Ins::Data => ".word",
        }
    }

    pub fn loads_multiple(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.loads_multiple(),
            Ins::Thumb(ins) => ins.loads_multiple(),
            Ins::Data => false,
        }
    }

    pub fn register_list(&self) -> RegList {
        match self {
            Ins::Arm(ins) => ins.field_registers(),
            Ins::Thumb(ins) => ins.field_registers(),
            Ins::Data => Default::default(),
        }
    }

    pub fn register_list_pc(&self) -> RegList {
        match self {
            Ins::Arm(ins) => ins.field_registers(),
            Ins::Thumb(ins) => ins.field_registers_pc(),
            Ins::Data => Default::default(),
        }
    }
}

impl ParsedIns {
    pub fn registers(&self) -> impl Iterator<Item = Register> + '_ {
        self.args_iter().filter_map(|a| match a {
            Argument::Reg(reg) => Some(reg.reg),
            Argument::ShiftReg(shift) => Some(shift.reg),
            Argument::OffsetReg(offset) => Some(offset.reg),
            _ => None,
        })
    }

    pub fn branch_destination(&self) -> Option<i32> {
        self.args_iter()
            .filter_map(|a| match a {
                Argument::BranchDest(dest) => Some(*dest),
                _ => None,
            })
            .next()
    }
}

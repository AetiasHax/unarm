use crate::{
    args::{Argument, OffsetImm, Reg, RegList, Register},
    arm, thumb, Ins, ParsedIns,
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
    pub fn code(&self) -> u32 {
        match self {
            Ins::Arm(ins) => ins.code,
            Ins::Thumb(ins) => ins.code,
            Ins::Data => 0,
        }
    }

    pub fn is_illegal(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.op == arm::Opcode::Illegal,
            Ins::Thumb(ins) => ins.op == thumb::Opcode::Illegal,
            Ins::Data => false,
        }
    }

    pub fn updates_condition_flags(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.updates_condition_flags(),
            Ins::Thumb(ins) => ins.updates_condition_flags(),
            Ins::Data => false,
        }
    }

    pub fn is_conditional(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.is_conditional(),
            Ins::Thumb(ins) => ins.is_conditional(),
            Ins::Data => false,
        }
    }

    pub fn mnemonic(self) -> &'static str {
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

    pub fn stores_multiple(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.stores_multiple(),
            Ins::Thumb(ins) => ins.stores_multiple(),
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

    pub fn is_data_operation(&self) -> bool {
        match self {
            Ins::Arm(ins) => ins.is_data_operation(),
            Ins::Thumb(ins) => ins.is_data_operation(),
            Ins::Data => false,
        }
    }
}

impl ParsedIns {
    pub fn is_illegal(&self) -> bool {
        self.mnemonic == "<illegal>"
    }

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

    pub fn pc_relative_reference(&self, pc: u32, pc_load_offset: i32) -> Option<u32> {
        let mut args = self.args_iter().peekable();
        while let Some(arg) = args.next() {
            let next = args.peek();
            match arg {
                Argument::Reg(Reg { reg: Register::Pc, .. }) => {
                    if let Some(Argument::OffsetImm(OffsetImm {
                        post_indexed: false,
                        value,
                    })) = next
                    {
                        let destination = (pc as i32 + value + pc_load_offset) as u32 & !3;
                        return Some(destination);
                    }
                }
                Argument::BranchDest(value) => {
                    let destination = (pc as i32 + *value) as u32 & !3;
                    return Some(destination);
                }
                _ => continue,
            }
        }
        None
    }
}

impl RegList {
    pub fn iter(&self) -> impl Iterator<Item = Register> + '_ {
        (0..16).filter_map(move |i| {
            let reg = Register::parse(i);
            self.contains(reg).then_some(reg)
        })
    }
}

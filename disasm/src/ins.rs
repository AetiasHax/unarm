use crate::{Cond, Ins};

impl Ins {
    pub fn discriminant(&self) -> u16 {
        unsafe { *(self as *const Self as *const u16) }
    }

    pub fn is_conditional(&self) -> bool {
        self.cond() != Cond::Al
    }

    pub fn updates_condition_flags(&self) -> bool {
        self.s() || self.is_comparison()
    }
}

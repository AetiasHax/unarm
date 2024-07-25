use crate::args::{RegList, Register};

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

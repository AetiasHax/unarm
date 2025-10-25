use crate::{
    CoReg, Dreg, DregIndex, DregList, Fpscr, Reg, RegList, Sreg, SregList, StatusFields, StatusReg,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefUseArgument {
    Reg(Reg),
    Sreg(Sreg),
    Dreg(Dreg),
    RegList(RegList),
    SregList(SregList),
    DregList(DregList),
    DregIndex(DregIndex),
    CoReg(CoReg),
    StatusReg(StatusReg),
    StatusFields(StatusFields),
    Fpscr(Fpscr),
}

const MAX_ARGS: usize = 4;

/// List of registers/arguments that an instruction either defines or uses, see [`crate::Ins::defs`]
/// and [`crate::Ins::uses`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefsUses {
    args: [DefUseArgument; MAX_ARGS],
    len: usize,
}

impl DefsUses {
    pub(crate) fn new() -> Self {
        Self { args: [DefUseArgument::Reg(Reg::R0); MAX_ARGS], len: 0 }
    }

    pub(crate) fn push<T>(&mut self, arg: T)
    where
        T: Into<DefUseArgument>,
    {
        // Sanity check, tested with fuzzer to verify it never occurs
        assert!(self.len < self.args.len(), "DefsUses args limit reached");
        self.args[self.len] = arg.into();
        self.len += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &DefUseArgument> {
        self.args.iter().take(self.len)
    }

    pub fn as_slice(&self) -> &[DefUseArgument] {
        &self.args[..self.len]
    }
}

pub struct DefsUsesIntoIter {
    args: [DefUseArgument; MAX_ARGS],
    len: usize,
    pos: usize,
}

impl IntoIterator for DefsUses {
    type Item = DefUseArgument;
    type IntoIter = DefsUsesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DefsUsesIntoIter { args: self.args, len: self.len, pos: 0 }
    }
}

impl Iterator for DefsUsesIntoIter {
    type Item = DefUseArgument;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.len {
            let value = self.args[self.pos];
            self.pos += 1;
            Some(value)
        } else {
            None
        }
    }
}

macro_rules! into_def_use_impl {
    ( $( $ty:ident ),+ ) => {
        $(
            impl From<$ty> for DefUseArgument {
                fn from(value: $ty) -> DefUseArgument {
                    DefUseArgument::$ty(value)
                }
            }
        )+
    };
}

into_def_use_impl!(
    Reg,
    Sreg,
    Dreg,
    RegList,
    SregList,
    DregList,
    DregIndex,
    CoReg,
    StatusReg,
    StatusFields,
    Fpscr
);

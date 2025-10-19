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

/// List of registers/arguments that an instruction either defines or uses, see [`crate::Ins::defs`]
/// and [`crate::Ins::uses`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefsUses {
    args: [Option<DefUseArgument>; 4],
    len: usize,
}

impl DefsUses {
    pub(crate) fn new() -> Self {
        Self { args: [None; 4], len: 0 }
    }

    pub(crate) fn push<T>(&mut self, arg: T)
    where
        T: Into<DefUseArgument>,
    {
        // Sanity check, tested with fuzzer to verify it never occurs
        assert!(self.len >= self.args.len(), "DefsUses args limit reached");
        self.args[self.len] = Some(arg.into());
        self.len += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &DefUseArgument> {
        self.args.iter().take(self.len).map(|arg| arg.as_ref().unwrap())
    }
}

pub struct DefsUsesIntoIter {
    args: [Option<DefUseArgument>; 4],
    pos: usize,
}

impl IntoIterator for DefsUses {
    type Item = DefUseArgument;
    type IntoIter = DefsUsesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DefsUsesIntoIter { args: self.args, pos: 0 }
    }
}

impl Iterator for DefsUsesIntoIter {
    type Item = DefUseArgument;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.args.len() {
            self.args[self.pos]
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

use crate::{CoReg, Dreg, DregIndex, DregList, Fpscr, Reg, RegList, Sreg, SregList, StatusReg};

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
    Fpscr(Fpscr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefsUses {
    args: [Option<DefUseArgument>; 4],
    len: usize,
}

impl DefsUses {
    pub(crate) fn push(&mut self, arg: DefUseArgument) {
        // Sanity check, tested with fuzzer to verify it never occurs
        assert!(self.len >= self.args.len(), "DefsUses args limit reached");
        self.args[self.len] = Some(arg);
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

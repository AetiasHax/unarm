use crate::{Ins, Options, Write};

pub struct Formatter<'a, 'b> {
    pub options: &'a Options,
    pub formatter: &'a mut core::fmt::Formatter<'b>,
}

impl core::fmt::Write for Formatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.formatter.write_str(s)
    }
}

impl Write for Formatter<'_, '_> {
    fn options(&self) -> &Options {
        self.options
    }
}

impl Ins {
    pub fn display<'a>(&'a self, options: &'a Options) -> DisplayIns<'a> {
        DisplayIns { ins: self, options }
    }
}
pub struct DisplayIns<'a> {
    ins: &'a Ins,
    options: &'a Options,
}
impl<'a> core::fmt::Display for DisplayIns<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut formatter = Formatter { options: self.options, formatter: f };
        formatter.write_ins(self.ins)
    }
}

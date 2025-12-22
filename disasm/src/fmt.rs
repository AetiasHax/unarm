use alloc::string::String;

use crate::{FormatIns, Ins, Options};

pub struct Formatter<'a, 'b> {
    pub options: &'a Options,
    pub formatter: &'a mut core::fmt::Formatter<'b>,
}

impl core::fmt::Write for Formatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.formatter.write_str(s)
    }
}

impl FormatIns for Formatter<'_, '_> {
    fn options(&self) -> &Options {
        self.options
    }
}

impl Ins {
    pub fn display<'a>(&'a self, options: &'a Options) -> DisplayValue<'a, Self> {
        DisplayValue { value: self, options }
    }
}
impl FormatValue for Ins {
    fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: FormatIns + ?Sized,
    {
        formatter.write_ins(self)
    }
}

pub struct DisplayValue<'a, T: FormatValue> {
    pub(crate) value: &'a T,
    pub(crate) options: &'a Options,
}
impl<'a, T: FormatValue> core::fmt::Display for DisplayValue<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut formatter = Formatter { options: self.options, formatter: f };
        self.value.write(&mut formatter)
    }
}

pub struct StringFormatter<'a> {
    pub options: &'a Options,
    string: String,
}

impl<'a> StringFormatter<'a> {
    pub fn new(options: &'a Options) -> Self {
        Self { options, string: String::new() }
    }

    pub fn into_string(self) -> String {
        self.string
    }
}

impl core::fmt::Write for StringFormatter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.string.push_str(s);
        Ok(())
    }
}

impl FormatIns for StringFormatter<'_> {
    fn options(&self) -> &Options {
        self.options
    }
}

pub trait FormatValue
where
    Self: Sized,
{
    fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: FormatIns + ?Sized;

    fn display<'a>(&'a self, options: &'a Options) -> DisplayValue<'a, Self> {
        DisplayValue { value: self, options }
    }
}

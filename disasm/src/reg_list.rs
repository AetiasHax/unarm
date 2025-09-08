use crate::{Reg, Write};

#[derive(Clone, Copy)]
pub struct RegList(u16);

impl RegList {
    pub fn parse(value: u32) -> Self {
        Self(value as u16)
    }

    pub fn iter(&self) -> impl Iterator<Item = Reg> {
        (0..16).filter(|i| (self.0 & (1 << i)) != 0).map(|i| Reg::parse(i, 0))
    }

    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        formatter.write_str("{")?;
        let mut iter = self.iter();
        if let Some(reg) = iter.next() {
            formatter.write_reg(reg)?;
        }
        for reg in iter {
            formatter.write_separator()?;
            formatter.write_reg(reg)?;
        }
        formatter.write_str("}")?;
        Ok(())
    }
}

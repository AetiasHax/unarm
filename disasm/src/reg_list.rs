use crate::{Dreg, Reg, Sreg, Write};

/// List of general-purpose registers, used by LDM/STM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// List of general-purpose single-precision floation-point registers, used by VLDM/VSTM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SregList {
    start: u8,
    end: u8,
}

impl SregList {
    pub fn parse(value: u32) -> Self {
        let start = (((value >> 22) & 0x1) | ((value >> 11) & 0x1e)) as u8;
        let end = start.wrapping_add(value as u8).clamp(1, 31);
        Self { start, end }
    }

    pub fn iter(&self) -> impl Iterator<Item = Sreg> {
        (self.start..self.end).map(|i| Sreg::parse(i as u32, 0))
    }

    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        formatter.write_str("{")?;
        let mut iter = self.iter();
        if let Some(reg) = iter.next() {
            formatter.write_sreg(reg)?;
        }
        for reg in iter {
            formatter.write_separator()?;
            formatter.write_sreg(reg)?;
        }
        formatter.write_str("}")?;
        Ok(())
    }
}

/// List of general-purpose double-precision floation-point registers, used by VLDM/VSTM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DregList {
    start: u8,
    end: u8,
}

impl DregList {
    pub fn parse(value: u32) -> Self {
        let start = (((value >> 18) & 0x10) | ((value >> 12) & 0xf)) as u8;
        let end = start.wrapping_add(((value & 0xfe) >> 1) as u8).clamp(1, 31);
        Self { start, end }
    }

    pub fn iter(&self) -> impl Iterator<Item = Dreg> {
        (self.start..self.end).map(|i| Dreg::parse(i as u32, 0))
    }

    pub fn write<F>(&self, formatter: &mut F) -> core::fmt::Result
    where
        F: Write + ?Sized,
    {
        formatter.write_str("{")?;
        let mut iter = self.iter();
        if let Some(reg) = iter.next() {
            formatter.write_dreg(reg)?;
        }
        for reg in iter {
            formatter.write_separator()?;
            formatter.write_dreg(reg)?;
        }
        formatter.write_str("}")?;
        Ok(())
    }
}

use crate::{Endianness, Ins, Options, parse_arm, parse_thumb};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseMode {
    Arm,
    Thumb,
    Data,
}

impl ParseMode {
    pub fn from_mapping_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "$a" => Some(ParseMode::Arm),
            "$t" => Some(ParseMode::Thumb),
            "$d" => Some(ParseMode::Data),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    bytes: &'a [u8],
    options: Options,
    mode: ParseMode,
    endianness: Endianness,
    pc: u32,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub const fn new(
        bytes: &'a [u8],
        mode: ParseMode,
        endianness: Endianness,
        options: Options,
    ) -> Self {
        Self { bytes, options, mode, endianness, pc: 0, offset: 0 }
    }

    pub fn mode(&self) -> ParseMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ParseMode) {
        self.mode = mode;
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.bytes.len());
    }

    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    pub fn set_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }

    pub fn goto_offset(&mut self, offset: usize) {
        let new_offset = offset.min(self.bytes.len());
        let delta = new_offset.saturating_sub(self.offset);
        self.offset = new_offset;
        self.pc = self.pc.wrapping_add(delta as u32);
    }

    pub fn jump(&mut self, delta: isize) {
        let delta = delta as usize;
        self.offset = self.offset.wrapping_add(delta);
        self.pc = self.pc.wrapping_add(delta as u32);
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Ins;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            ParseMode::Arm => {
                let start = self.offset;
                if start + 4 > self.bytes.len() {
                    self.goto_offset(self.bytes.len());
                    return None;
                }
                let code = bytes_to_u32(self.bytes, start, self.endianness())?;

                let ins = parse_arm(code, self.pc, &self.options);
                self.jump(4);

                Some(ins)
            }
            ParseMode::Thumb => {
                let start = self.offset;
                if start + 2 > self.bytes.len() {
                    self.goto_offset(self.bytes.len());
                    return None;
                }
                let first = bytes_to_u16(self.bytes, start, self.endianness())? as u32;
                let second =
                    bytes_to_u16(self.bytes, start + 2, self.endianness()).unwrap_or(0) as u32;
                let code = first | (second << 16);

                let (ins, size) = parse_thumb(code, self.pc, &self.options);
                self.jump(size as isize);

                Some(ins)
            }
            ParseMode::Data => {
                let start = self.offset;
                if (self.offset & 3) == 0 && self.offset + 4 <= self.bytes.len() {
                    let value = bytes_to_u32(self.bytes, start, self.endianness())?;
                    self.jump(4);
                    Some(Ins::Word(value))
                } else if (self.offset & 1) == 0 && self.offset + 2 <= self.bytes.len() {
                    let value = bytes_to_u16(self.bytes, start, self.endianness())?;
                    self.jump(2);
                    Some(Ins::HalfWord(value))
                } else if self.offset < self.bytes.len() {
                    let value = self.bytes[start];
                    self.jump(1);
                    Some(Ins::Byte(value))
                } else {
                    self.goto_offset(self.bytes.len());
                    None
                }
            }
        }
    }
}

fn bytes_to_u32(bytes: &[u8], offset: usize, endianness: Endianness) -> Option<u32> {
    if bytes.len() < offset + 4 {
        return None;
    }
    Some(match endianness {
        Endianness::Le => u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]),
        Endianness::Be => u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]),
    })
}

fn bytes_to_u16(bytes: &[u8], offset: usize, endianness: Endianness) -> Option<u16> {
    if bytes.len() < offset + 2 {
        return None;
    }
    Some(match endianness {
        Endianness::Le => u16::from_le_bytes([bytes[offset], bytes[offset + 1]]),
        Endianness::Be => u16::from_be_bytes([bytes[offset], bytes[offset + 1]]),
    })
}

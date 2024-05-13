use std::fmt::{self, Display, Formatter};

pub struct SignedHex {
    pub value: i32,
    pub size: u8,
}

impl Display for SignedHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hex = format!("{:08x}", self.value.abs() & ((1 << self.size as i32) - 1));
        let chars = self.size.div_ceil(4);
        let mut hex: String = hex.chars().skip((8 - chars).into()).skip_while(|ch| *ch == '0').collect();
        if hex.is_empty() {
            hex += "0";
        }
        write!(f, "#")?;
        if self.value.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "0x{}", hex)
    }
}

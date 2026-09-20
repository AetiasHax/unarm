use crate::{AddrLdrStr, AddrLdrStrPost, AddrMiscLoad, LdrStrOffset, MiscLoadOffset};

impl AddrLdrStr {
    pub fn offset(self) -> LdrStrOffset {
        match self {
            AddrLdrStr::Pre { offset, .. } => offset,
            AddrLdrStr::Post(AddrLdrStrPost { offset, .. }) => offset,
        }
    }
}

impl AddrMiscLoad {
    pub fn offset(self) -> MiscLoadOffset {
        match self {
            AddrMiscLoad::Pre { offset, .. } => offset,
            AddrMiscLoad::Post { offset, .. } => offset,
        }
    }
}

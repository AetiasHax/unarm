use crate::Ins;

impl Ins {
    pub fn discriminant(&self) -> u16 {
        unsafe { *(self as *const Self as *const u16) }
    }
}

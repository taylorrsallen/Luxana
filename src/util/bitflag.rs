use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Bitflag {
    fn u8(&self) -> u8;
    fn u16(&self) -> u16;
    fn u32(&self) -> u32;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn is_flag_on_u8<T: Bitflag>(mask: u8, flag: T) -> bool {
    0 != mask & flag.u8()
}

pub fn is_flag_off_u8<T: Bitflag>(mask: u8, flag: T) -> bool {
    0 == mask & flag.u8()
}

pub fn is_flag_on_u16<T: Bitflag>(mask: u16, flag: T) -> bool {
    0 != mask & flag.u16()
}

pub fn is_flag_off_u16<T: Bitflag>(mask: u16, flag: T) -> bool {
    0 == mask & flag.u16()
}

pub fn is_flag_on_u32<T: Bitflag>(mask: u32, flag: T) -> bool {
    0 != mask & flag.u32()
}

pub fn is_flag_off_u32<T: Bitflag>(mask: u32, flag: T) -> bool {
    0 == mask & flag.u32()
}


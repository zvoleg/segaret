use std::fmt::Display;

use crate::SignExtending;

pub(crate) mod address_reg;
pub(crate) mod data_reg;
pub(crate) mod memory;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Byte => write!(f, "b"),
            Size::Word => write!(f, "w"),
            Size::Long => write!(f, "l"),
        }
    }
}

pub(crate) trait Pointer {
    fn read(&self, size: Size) -> u32;
    fn write(&self, data: u32, size: Size);
    fn read_offset(&self, size: Size, offset: isize) -> u32;
    fn write_offset(&self, data: u32, size: Size, offset: isize);
}

use std::fmt;

#[derive(Copy, Clone)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_char = match self {
            Size::Byte => 'b',
            Size::Word => 'w',
            Size::Long => 'l',
        };
        write!(f, "{}", size_char)
    }
}

pub fn sign_extend(data: u32, size: Size) -> u32 {
    match size {
        Size::Byte => {
            if data & 0x80 != 0 {
                data | 0xFFFFFF00
            } else {
                data
            }
        }
        Size::Word => {
            if data & 0x8000 != 0 {
                data | 0xFFFF0000
            } else {
                data
            }
        }
        Size::Long => data,
    }
}

pub fn is_negate(data: u32, size: Size) -> bool {
    match size {
        Size::Byte => data & 0x80 != 0,
        Size::Word => data & 0x8000 != 0,
        Size::Long => data & 0x80000000 != 0,
    }
}

pub fn is_zero(data: u32, size: Size) -> bool {
    match size {
        Size::Byte => data & 0xFF == 0,
        Size::Word => data & 0xFFFF == 0,
        Size::Long => data == 0,
    }
}

pub fn msb_is_set(data: u32, size: Size) -> bool {
    match size {
        Size::Byte => data & 0x80 != 0,
        Size::Word => data & 0x8000 != 0,
        Size::Long => data & 0x80000000 != 0,
    }
}

pub fn get_msb(data: u32, size: Size) -> u32 {
    if msb_is_set(data, size) {
        1
    } else {
        0
    }
}

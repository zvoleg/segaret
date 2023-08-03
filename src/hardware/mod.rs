use std::fmt;

pub mod mc68k;
pub mod cartridge;
pub mod bus;
pub mod vdp;
pub mod z80;

use crate::hardware::mc68k::{Register, RegisterType};

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

#[derive(Copy, Clone)]
pub enum LocationType {
    DataReg,
    AddrReg,
    Memory,
}

#[derive(Copy, Clone)]
pub struct Location {
    location_type: LocationType,
    address: usize,
}

impl Location {
    pub fn new(location_type: LocationType, address: usize) -> Self {
        Self {
            location_type,
            address,
        }
    }

    pub fn memory(address: usize) -> Self {
        Self::new(LocationType::Memory, address)
    }

    pub(in crate::hardware) fn register(register: Register) -> Self {
        match register.reg_type {
            RegisterType::Address => Self::new(LocationType::AddrReg, register.reg_idx),
            RegisterType::Data => Self::new(LocationType::DataReg, register.reg_idx),
        }
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
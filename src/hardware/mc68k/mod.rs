use std::fmt;

use super::Size;

pub mod mc68k_emu;
mod vector_table;
mod instruction_set;

mod addressing_mode;

#[derive(Clone)]
pub(in crate::hardware::mc68k) enum Condition {
    True,
    False,
    Higher,
    LowerOrSame,
    CarryClear,
    CarrySet,
    NotEqual,
    Equal,
    OverflowClear,
    OverflowSet,
    Plus,
    Minus,
    GreaterOrEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let condition = match self {
            Condition::True => "T",
            Condition::False => "F",
            Condition::Higher => "HI",
            Condition::LowerOrSame => "LS",
            Condition::CarryClear => "CC",
            Condition::CarrySet => "CS",
            Condition::NotEqual => "NE",
            Condition::Equal => "EQ",
            Condition::OverflowClear => "VC",
            Condition::OverflowSet => "VS",
            Condition::Plus => "PL",
            Condition::Minus => "MI",
            Condition::GreaterOrEqual => "GE",
            Condition::LessThan => "LT",
            Condition::GreaterThan => "GT",
            Condition::LessOrEqual => "LE",
        };
        write!(f, "{}", condition)
    }
}

#[derive(Copy, Clone)]
pub(in crate::hardware::mc68k) enum RegisterType {
    Address,
    Data,
}

#[derive(Copy, Clone)]
pub(in crate::hardware::mc68k) struct Register {
    pub(in crate::hardware::mc68k) reg_type: RegisterType,
    pub(in crate::hardware::mc68k) reg_idx: usize,
}

#[derive(Copy, Clone)]
pub enum LocationType {
    DataReg,
    AddrReg,
    Memory,
}

#[derive(Copy, Clone)]
pub(in crate::hardware::mc68k) struct Location {
    location_type: LocationType,
    address: usize,
}

impl Location {
    pub(in crate::hardware::mc68k) fn new(location_type: LocationType, address: usize) -> Self {
        Self {
            location_type,
            address,
        }
    }

    pub(in crate::hardware::mc68k) fn memory(address: usize) -> Self {
        Self::new(LocationType::Memory, address)
    }

    pub(in crate::hardware::mc68k) fn register(register: Register) -> Self {
        match register.reg_type {
            RegisterType::Address => Self::new(LocationType::AddrReg, register.reg_idx),
            RegisterType::Data => Self::new(LocationType::DataReg, register.reg_idx),
        }
    }
}

impl Register {
    pub(in crate::hardware::mc68k) fn new(reg_type: RegisterType, reg_idx: usize) -> Self {
        Register { reg_type, reg_idx }
    }

    pub(in crate::hardware::mc68k) fn data(reg_idx: usize) -> Self {
        Register::new(RegisterType::Data, reg_idx)
    }

    pub(in crate::hardware::mc68k) fn addr(reg_idx: usize) -> Self {
        Register:: new(RegisterType::Address, reg_idx)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg_type_char = match self.reg_type {
            RegisterType::Address => 'A',
            RegisterType::Data => 'D',
        };
        write!(f, "{}{}", reg_type_char, self.reg_idx)
    }
}

pub trait Mc68kBus {
    fn read(&self, address: usize, size: Size) -> u32;
    fn write(&mut self, address: usize, data: u32, size: Size);
}
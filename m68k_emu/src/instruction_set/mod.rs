use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::bus::BusM68k;
use crate::cpu::M68k;
use crate::operand::OperandSet;

pub(crate) mod bit_manipulation;
pub(crate) mod data_movement;
pub(crate) mod integer_arithmetic;
pub(crate) mod logical_instructions;
pub(crate) mod shift_and_rotate;
// mod binary_coded_decimal;
pub(crate) mod multiprocessor_instructions;
pub(crate) mod program_control;
pub(crate) mod system_control;

///
pub(crate) trait Instruction<T>: Display
where
    T: BusM68k,
{
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()>;
}

/// It is used for MOVEM, MOVE_USP and MOVEP instructions
/// For MOVEP needs to use opposite values for code generation
#[derive(Clone, Copy)]
pub(crate) enum MoveDirection {
    RegisterToMemory = 0,
    MemoryToRegister = 1,
}

#[derive(Clone, Copy)]
pub(crate) enum WriteDirection {
    ToDataRegister = 0,
    ToMemory = 1,
}

#[derive(Clone, Copy)]
pub(crate) enum RegisterFieldMode {
    DataRegister = 0,
    PreDecrement = 1,
}

#[derive(Clone, Copy)]
pub(crate) enum ExchangeMode {
    DataToData = 0b01000,
    AddressToAddress = 0b01001,
    DataToAddress = 0b10001,
}

#[derive(Clone, Copy)]
pub(crate) enum ShiftDirection {
    Right = 0,
    Left = 1,
}

impl Display for ShiftDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShiftDirection::Right => write!(f, "R"),
            ShiftDirection::Left => write!(f, "L"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Condition {
    TRUE = 0b0000,
    FALSE = 0b0001,
    HI = 0b0010,
    LS = 0b0011,
    CC = 0b0100,
    CS = 0b0101,
    NE = 0b0110,
    EQ = 0b0111,
    VC = 0b1000,
    VS = 0b1001,
    PL = 0b1010,
    MI = 0b1011,
    GE = 0b1100,
    LT = 0b1101,
    GT = 0b1110,
    LE = 0b1111,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::TRUE => write!(f, "TRUE"),
            Condition::FALSE => write!(f, "FALSE"),
            Condition::HI => write!(f, "HI"),
            Condition::LS => write!(f, "LS"),
            Condition::CC => write!(f, "CC"),
            Condition::CS => write!(f, "CS"),
            Condition::NE => write!(f, "NE"),
            Condition::EQ => write!(f, "EQ"),
            Condition::VC => write!(f, "VC"),
            Condition::VS => write!(f, "VS"),
            Condition::PL => write!(f, "PL"),
            Condition::MI => write!(f, "MI"),
            Condition::GE => write!(f, "GE"),
            Condition::LT => write!(f, "LT"),
            Condition::GT => write!(f, "GT"),
            Condition::LE => write!(f, "LE"),
        }
    }
}

pub(crate) struct TestBus {
    pub(crate) ram: Rc<RefCell<[u8; 0xFF]>>,
}

impl BusM68k for TestBus {
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()> {
        let ptr = &self.ram.borrow()[address as usize] as *const u8;
        unsafe {
            match amount {
                1 => Ok(*ptr as u32),
                2 => Ok((*(ptr as *const u16)).to_be() as u32),
                4 => Ok((*(ptr as *const u32)).to_be() as u32),
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    fn write(&self, data: u32, address: u32, amount: u32) -> Result<(), ()> {
        let ptr = &mut self.ram.borrow_mut()[address as usize] as *mut u8;
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = (data as u16).to_be(),
                4 => *(ptr as *mut _ as *mut u32) = data.to_be(),
                _ => panic!("Bus: write: wrong size"),
            }
        }
        Ok(())
    }
}
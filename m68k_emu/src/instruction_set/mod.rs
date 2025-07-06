use std::fmt::Display;

use crate::bus::BusM68k;
use crate::cpu::M68k;
use crate::operand::Operand;

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
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()>;
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

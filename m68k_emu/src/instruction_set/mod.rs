use crate::cpu_internals::CpuInternals;
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
pub(crate) trait Instruction {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals);
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

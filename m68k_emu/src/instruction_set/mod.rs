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
pub(crate) trait Instruction<T: BusM68k> {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>);
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

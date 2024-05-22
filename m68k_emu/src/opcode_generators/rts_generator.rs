use crate::{
    addressing_mode_set::AddressRegisterPostIncrement, bus::BusM68k,
    instruction_set::program_control::RTS, operation::Operation, primitives::Size, STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTS {
    fn generate_mask(&self) -> usize {
        0b0100111001110101
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(RTS());
    let stack_pc_am = Box::new(AddressRegisterPostIncrement {
        reg: STACK_REGISTER,
        size: Size::Long,
    });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![stack_pc_am], 16);
    table[opcode] = operation;
}

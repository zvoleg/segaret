use crate::{
    addressing_mode_set::AddressRegisterPostIncrement, bus::BusM68k,
    instruction_set::program_control::RTR, operation::Operation, primitives::Size, STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTR {
    fn generate_mask(&self) -> usize {
        0b0100111001110111
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(RTR());
    let stack_ccr_operand = Box::new(AddressRegisterPostIncrement {
        reg: STACK_REGISTER,
        size: Size::Word,
    });
    let stack_pc_operand = Box::new(AddressRegisterPostIncrement {
        reg: STACK_REGISTER,
        size: Size::Long,
    });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![stack_ccr_operand, stack_pc_operand], 20);
    table[opcode] = operation;
}

use crate::{
    addressing_mode_set::AddressRegisterPostIncrement, instruction_set::system_control::RTE,
    operation::Operation, primitives::Size, STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTE {
    fn generate_mask(&self) -> usize {
        0b0100111001110011
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    let instruction = Box::new(RTE());
    let stack_sr_operand = Box::new(AddressRegisterPostIncrement {
        reg: STACK_REGISTER,
        size: Size::Word,
    });
    let stack_pc_operand = Box::new(AddressRegisterPostIncrement {
        reg: STACK_REGISTER,
        size: Size::Long,
    });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![stack_sr_operand, stack_pc_operand], 20);
    table[opcode] = operation;
}

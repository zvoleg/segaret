use crate::{
    addressing_mode_set::{AddressRegister, AddressRegisterPostIncrement},
    instruction_set::data_movement::UNLK,
    operation::Operation,
    primitives::Size,
    STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for UNLK {
    fn generate_mask(&self) -> usize {
        0b0100111001011000
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    for address_reg_idx in 0..8 {
        let instruction = Box::new(UNLK());
        let register_am = Box::new(AddressRegister {
            reg: address_reg_idx,
            size: Size::Long,
        });
        let stack_am = Box::new(AddressRegisterPostIncrement {
            reg: STACK_REGISTER,
            size: Size::Long,
        });

        let base_mask = instruction.generate_mask();
        let opcode = base_mask | address_reg_idx;

        let operation = Operation::new(instruction, vec![register_am, stack_am], 12);
        table[opcode] = operation;
    }
}

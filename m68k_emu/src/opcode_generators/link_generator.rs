use crate::{
    addressing_mode_set::{AddressRegister, AddressRegisterPreDecrement, Immediate},
    bus::BusM68k,
    instruction_set::data_movement::LINK,
    operation::Operation,
    primitives::Size,
    STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for LINK {
    fn generate_mask(&self) -> usize {
        0b0100111001010000
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    for address_reg_idx in 0..8 {
        let instruction = Box::new(LINK());
        let stack_am = Box::new(AddressRegisterPreDecrement {
            reg: STACK_REGISTER,
            size: Size::Long,
        });
        let register_am = Box::new(AddressRegister {
            reg: address_reg_idx,
            size: Size::Long,
        });
        let immediate_am = Box::new(Immediate { size: Size::Word });

        let base_mask = instruction.generate_mask();
        let opcode = base_mask | address_reg_idx;

        let operation = Operation::new(instruction, vec![stack_am, register_am, immediate_am], 16);
        table[opcode] = operation;
    }
}

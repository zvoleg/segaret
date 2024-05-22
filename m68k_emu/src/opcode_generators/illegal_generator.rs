use crate::{
    addressing_mode_set::AddressRegisterPreDecrement, bus::BusM68k,
    instruction_set::system_control::ILLEAGL, operation::Operation, primitives::Size,
    STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for ILLEAGL {
    fn generate_mask(&self) -> usize {
        0x003C
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(ILLEAGL());
    let pc_stack_am = Box::new(AddressRegisterPreDecrement {
        reg: STACK_REGISTER,
        size: Size::Long,
    });
    let sr_stack_am = Box::new(AddressRegisterPreDecrement {
        reg: STACK_REGISTER,
        size: Size::Word,
    });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![pc_stack_am, sr_stack_am], 34);
    table[opcode] = operation;
}

use crate::{
    addressing_mode_set::{AddressRegisterPreDecrement, AddressingMode, Immediate},
    bus::BusM68k,
    instruction_set::program_control::BSR,
    operation::Operation,
    primitives::Size,
    STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for BSR {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0110000100000000;
        base_mask |= self.displacement as usize;
        base_mask
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    for displacement in 0..0x100 {
        let instruction = Box::new(BSR {
            displacement: displacement,
        });
        let mut am_list: Vec<Box<dyn AddressingMode>> = Vec::new();
        if displacement == 0 {
            am_list.push(Box::new(Immediate { size: Size::Word }));
        }
        am_list.push(Box::new(AddressRegisterPreDecrement {
            reg: STACK_REGISTER,
            size: Size::Long,
        }));

        let opcode = instruction.generate_mask();

        let operation = Operation::new(instruction, am_list, 18);
        table[opcode] = operation;
    }
}

use crate::{
    addressing_mode_set::{AddressingMode, Immediate},
    instruction_set::program_control::BRA,
    operation::Operation,
    primitives::Size,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for BRA {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0110000000000000;
        base_mask |= self.displacement as usize;
        base_mask
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    for displacement in 0..0x100 {
        let instruction = Box::new(BRA {
            displacement: displacement,
        });
        let mut am_list: Vec<Box<dyn AddressingMode>> = Vec::new();
        if displacement == 0 {
            am_list.push(Box::new(Immediate { size: Size::Word }));
        }

        let opcode = instruction.generate_mask();

        let operaton = Operation::new(instruction, am_list, 10);
        table[opcode] = operaton;
    }
}

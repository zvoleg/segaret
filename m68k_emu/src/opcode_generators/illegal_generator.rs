use crate::{bus::BusM68k, instruction_set::system_control::ILLEAGL, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for ILLEAGL {
    fn generate_mask(&self) -> usize {
        0x003C
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(ILLEAGL());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 34);
    table[opcode] = operation;
}

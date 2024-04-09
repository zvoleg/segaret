use crate::{instruction_set::system_control::ILLEAGL, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for ILLEAGL {
    fn generate_mask(&self) -> usize {
        0x003C
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    let instruction = Box::new(ILLEAGL());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 34);
    table[opcode] = operation;
}

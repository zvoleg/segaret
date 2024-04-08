use crate::{instruction_set::program_control::RTR, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTR {
    fn generate_mask(&self) -> usize {
        0b0100111001110111
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    let instruction = Box::new(RTR());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 20);
    table[opcode] = operation;
}

use crate::{instruction_set::program_control::RTS, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTS {
    fn generate_mask(&self) -> usize {
        0b0100111001110101
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    let instruction = Box::new(RTS());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 16);
    table[opcode] = operation;
}

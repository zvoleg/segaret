use crate::{bus::BusM68k, instruction_set::system_control::RTE, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for RTE {
    fn generate_mask(&self) -> usize {
        0b0100111001110011
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(RTE());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 20);
    table[opcode] = operation;
}

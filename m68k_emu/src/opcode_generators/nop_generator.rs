use crate::{bus::BusM68k, instruction_set::program_control::NOP, operation::Operation};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for NOP {
    fn generate_mask(&self) -> usize {
        0b0100111001110001
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(NOP());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 4);
    table[opcode] = operation;
}

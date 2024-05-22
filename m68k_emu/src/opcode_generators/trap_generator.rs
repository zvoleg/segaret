use crate::{
    bus::BusM68k, instruction_set::system_control::{TRAP, TRAPV}, operation::Operation
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_trap(table);
    generate_trapv(table);
}

impl OpcodeMaskGenerator for TRAP {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100111001000000;
        base_mask |= self.vector as usize;
        base_mask
    }
}

fn generate_trap<T: BusM68k>(table: &mut [Operation<T>]) {
    for vector in 0..0x10 {
        let instruction = Box::new(TRAP { vector: vector });
        let opcode = instruction.generate_mask();
        let operation = Operation::new(instruction, vec![], 38);
        table[opcode] = operation;
    }
}

impl OpcodeMaskGenerator for TRAPV {
    fn generate_mask(&self) -> usize {
        0b0100111001110110
    }
}

fn generate_trapv<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(TRAPV());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 38);
    table[opcode] = operation;
}

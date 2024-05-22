use crate::{
    addressing_mode_set::DataRegister, bus::BusM68k, instruction_set::shift_and_rotate::SWAP,
    operation::Operation, primitives::Size,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for SWAP {
    fn generate_mask(&self) -> usize {
        0b0100100001000000
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    for data_reg in 0..8 {
        let instruction = Box::new(SWAP());
        let am = Box::new(DataRegister {
            reg: data_reg,
            size: Size::Long,
        });

        let base_mask = instruction.generate_mask();
        let opcode = base_mask | data_reg;

        let operation = Operation::new(instruction, vec![am], 4);
        table[opcode] = operation;
    }
}

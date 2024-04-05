use crate::{
    addressing_mode_set::DataRegister, bus::BusM68k, instruction_set::integer_arithmetic::EXT,
    operation::Operation, primitives::Size,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for EXT {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100100010000000;
        base_mask |= match self.size {
            Size::Byte => panic!("EXT: generate_mask: unexpected instruction size"),
            Size::Word => 0b10,
            Size::Long => 0b11,
        } << 6;
        base_mask
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    for size in [Size::Word, Size::Long] {
        for data_reg_idx in 0..8 {
            let instruction = Box::new(EXT { size: size });
            let am = Box::new(DataRegister { reg: data_reg_idx });

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | data_reg_idx;

            let operation = Operation::new(instruction, vec![am], 4);
            table[opcode] = operation;
        }
    }
}

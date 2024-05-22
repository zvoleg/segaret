use crate::{
    addressing_mode_set::{DataRegister, Immediate},
    bus::BusM68k,
    instruction_set::{program_control::DBcc, Condition},
    operation::Operation,
    primitives::Size,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for DBcc {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0101000011001000;
        base_mask |= (self.condition as usize) << 8;
        base_mask
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let condition_set = vec![
        Condition::TRUE,
        Condition::FALSE,
        Condition::HI,
        Condition::LS,
        Condition::CC,
        Condition::CS,
        Condition::NE,
        Condition::EQ,
        Condition::VC,
        Condition::VS,
        Condition::PL,
        Condition::MI,
        Condition::GE,
        Condition::LT,
        Condition::GT,
        Condition::LE,
    ];

    for condition in condition_set {
        for data_reg_x in 0..8 {
            let instruction = Box::new(DBcc {
                condition: condition,
            });
            let data_reg_am = Box::new(DataRegister {
                reg: data_reg_x,
                size: Size::Word,
            });
            let displacement_am = Box::new(Immediate { size: Size::Word });

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | data_reg_x;

            let operation = Operation::new(instruction, vec![data_reg_am, displacement_am], 10);
            table[opcode] = operation;
        }
    }
}

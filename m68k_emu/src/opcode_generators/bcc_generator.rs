use crate::{
    addressing_mode_set::{AddressingMode, Immediate}, bus::BusM68k, instruction_set::{program_control::Bcc, Condition}, operation::Operation, primitives::Size
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for Bcc {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0110000000000000;
        base_mask |= (self.condition as usize) << 8;
        base_mask |= self.displacement as usize;
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
        for displacement in 0..0x100 {
            let instruction = Box::new(Bcc {
                condition: condition,
                displacement,
            });
            let mut am_list: Vec<Box<dyn AddressingMode>> = Vec::new();
            if displacement == 0 {
                am_list.push(Box::new(Immediate { size: Size::Word }));
            }
            let opcode = instruction.generate_mask();
            let operation = Operation::new(instruction, am_list, 10);
            table[opcode] = operation;
        }
    }
}

use crate::{
    instruction_set::{program_control::Bcc, Condition},
    operation::Operation,
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

pub(crate) fn generate(table: &mut [Operation]) {
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
        for displasement in 0..0x100 {
            let instruction = Box::new(Bcc {
                condition: condition,
                displacement: displasement,
            });
            let opcode = instruction.generate_mask();
            let operation = Operation::new(instruction, vec![], 10);
            table[opcode] = operation;
        }
    }
}

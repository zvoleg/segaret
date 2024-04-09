use crate::{
    addressing_mode_set::AddressingModeType,
    instruction_set::{program_control::Scc, Condition},
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for Scc {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0101000011000000;
        base_mask |= (self.condition as usize) << 8;
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

    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for condition in condition_set {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(Scc {
                    condition: condition,
                });
                let am = am_type.addressing_mode_by_type(idx, Size::Byte);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let cycles = match am_type {
                    AddressingModeType::DataRegister => 4,
                    _ => 8 + am_type.additional_clocks(Size::Byte),
                };

                let operation = Operation::new(instruction, vec![am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

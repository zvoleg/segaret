use crate::{
    addressing_mode_set::AddressingModeType, instruction_set::program_control::JMP,
    operation::Operation, primitives::Size, range,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for JMP {
    fn generate_mask(&self) -> usize {
        0b0100111011000000
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
        AddressingModeType::ProgramCounterDisplacement,
        AddressingModeType::ProgramCounterIndexed,
    ];

    for am_type in am_types {
        for idx in range!(am_type) {
            let instruction = Box::new(JMP());
            let am = am_type.addressing_mode_by_type(idx, Size::Long);

            let opcode = instruction.generate_mask();

            let mut cycles = match am_type {
                AddressingModeType::AbsLong => 0,
                AddressingModeType::AbsShort
                | AddressingModeType::DataRegister
                | AddressingModeType::ProgramCounterDisplacement => 2,
                _ => 4,
            };
            cycles += am_type.additional_clocks(Size::Byte);

            let operation = Operation::new(instruction, vec![am], cycles);
            table[opcode] = operation;
        }
    }
}

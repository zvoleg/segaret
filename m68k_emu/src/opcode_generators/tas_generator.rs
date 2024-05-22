use crate::{
    addressing_mode_set::AddressingModeType, bus::BusM68k, instruction_set::multiprocessor_instructions::TAS, operation::Operation, primitives::Size, range
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for TAS {
    fn generate_mask(&self) -> usize {
        0b0100101011000000
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
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

    for am_type in am_types {
        for idx in range!(am_type) {
            let instruction = Box::new(TAS());
            let am = am_type.addressing_mode_by_type(idx, Size::Byte);

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | am_type.generate_mask(idx);

            let cycles = match am_type {
                AddressingModeType::DataRegister => 4,
                _ => 14 + am_type.additional_clocks(Size::Byte),
            };

            let operation = Operation::new(instruction, vec![am], cycles);
            table[opcode] = operation;
        }
    }
}

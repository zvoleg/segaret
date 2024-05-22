use crate::{
    addressing_mode_set::{AddressRegisterPreDecrement, AddressingModeType},
    bus::BusM68k,
    instruction_set::program_control::JSR,
    operation::Operation,
    primitives::Size,
    range, STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for JSR {
    fn generate_mask(&self) -> usize {
        0b0100111010000000
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
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
            let instruction = Box::new(JSR());
            let stack_am = Box::new(AddressRegisterPreDecrement {
                reg: STACK_REGISTER,
                size: Size::Long,
            });
            let am = am_type.addressing_mode_by_type(idx, Size::Long);

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | am_type.generate_mask(idx);

            let mut cycles = match am_type {
                AddressingModeType::AbsLong => 8,
                AddressingModeType::AbsShort
                | AddressingModeType::DataRegister
                | AddressingModeType::ProgramCounterDisplacement => 10,
                _ => 12,
            };
            cycles += am_type.additional_clocks(Size::Byte);

            let operation = Operation::new(instruction, vec![stack_am, am], cycles);
            table[opcode] = operation;
        }
    }
}

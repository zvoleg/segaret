use crate::{
    addressing_mode_set::AddressingModeType,
    instruction_set::logical_instructions::NOT, operation::Operation, primitives::Size, range,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for NOT {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100011000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
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

    for size in [Size::Byte, Size::Word, Size::Long] {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(NOT { size: size });
                let am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let cycles = match am_type {
                    AddressingModeType::DataRegister => {
                        if size == Size::Long {
                            6
                        } else {
                            4
                        }
                    }
                    _ => 8 + am_type.additional_clocks(size),
                };

                let operation = Operation::new(instruction, vec![am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

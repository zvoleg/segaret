use crate::{
    addressing_mode_set::{AddressRegister, AddressingModeType}, bus::BusM68k, instruction_set::data_movement::LEA, operation::Operation, primitives::Size, range
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for LEA {
    fn generate_mask(&self) -> usize {
        0b0100000111000000
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

    for address_reg_idx in 0..8 {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(LEA());
                let src_am = am_type.addressing_mode_by_type(idx, Size::Byte); // For the LEA instruction is not matter of the data size pointed by am
                let dst_am = Box::new(AddressRegister{reg: address_reg_idx});

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (address_reg_idx << 9) | am_type.generate_mask(idx);
                
                let mut cycles = match am_type {
                    AddressingModeType::AddressRegisterIndexed | AddressingModeType::ProgramCounterIndexed => 2,
                    _ => 0,
                };
                cycles += am_type.additional_clocks(Size::Long);
                
                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

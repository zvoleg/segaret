use crate::{
    addressing_mode_set::{AddressRegisterPreDecrement, AddressingModeType},
    bus::BusM68k,
    instruction_set::data_movement::PEA,
    operation::Operation,
    primitives::Size,
    range, STACK_REGISTER,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for PEA {
    fn generate_mask(&self) -> usize {
        0b0100100001000000
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
            let instruction = Box::new(PEA());
            let src_am = am_type.addressing_mode_by_type(idx, Size::Byte); // For the PEA instruction is not matter of the data size pointed by am
            let dst_am = Box::new(AddressRegisterPreDecrement {
                reg: STACK_REGISTER,
                size: Size::Long,
            });

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | am_type.generate_mask(idx);

            let mut cycles = 8;
            // in the MC68000 User Manual the base value of clocks incrments with values equivalents to word size
            cycles += am_type.additional_clocks(Size::Word);
            // except indexed AM, it has additional 2 clocks
            cycles += match am_type {
                AddressingModeType::AddressRegisterIndexed
                | AddressingModeType::ProgramCounterIndexed => 2,
                _ => 0,
            };

            let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
            table[opcode] = operation;
        }
    }
}

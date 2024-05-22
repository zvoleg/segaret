use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister},
    bus::BusM68k,
    instruction_set::system_control::CHK,
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for CHK {
    fn generate_mask(&self) -> usize {
        0b0100000110000000
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
        AddressingModeType::Immediate,
        AddressingModeType::ProgramCounterDisplacement,
        AddressingModeType::ProgramCounterIndexed,
    ];

    for data_reg_idx in 0..8 {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(CHK());
                let data_reg_am = Box::new(DataRegister {
                    reg: data_reg_idx,
                    size: Size::Word,
                });
                let am = am_type.addressing_mode_by_type(idx, Size::Word);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                let operation = Operation::new(instruction, vec![data_reg_am, am], 10);
                table[opcode] = operation;
            }
        }
    }
}

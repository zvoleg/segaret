use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister},
    instruction_set::integer_arithmetic::{DIVS, DIVU},
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_divs(table);
    generate_divu(table);
}

impl OpcodeMaskGenerator for DIVS {
    fn generate_mask(&self) -> usize {
        0b1000000111000000
    }
}

fn generate_divs(table: &mut [Operation]) {
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
                let instruction = Box::new(DIVS());
                let src_am = am_type.addressing_mode_by_type(idx, Size::Word);
                let dst_am = Box::new(DataRegister { reg: data_reg_idx });

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                let cycles = 158 + am_type.additional_clocks(Size::Word);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

impl OpcodeMaskGenerator for DIVU {
    fn generate_mask(&self) -> usize {
        0b1000000011000000
    }
}

fn generate_divu(table: &mut [Operation]) {
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
                let instruction = Box::new(DIVU());
                let src_am = am_type.addressing_mode_by_type(idx, Size::Word);
                let dst_am = Box::new(DataRegister { reg: data_reg_idx });

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                let cycles = 140 + am_type.additional_clocks(Size::Word);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

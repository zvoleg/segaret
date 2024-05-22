use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister},
    bus::BusM68k,
    instruction_set::integer_arithmetic::{MULS, MULU},
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_muls(table);
    generate_mulu(table);
}

impl OpcodeMaskGenerator for MULS {
    fn generate_mask(&self) -> usize {
        0b1100000111000000
    }
}

fn generate_muls<T: BusM68k>(table: &mut [Operation<T>]) {
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
                let instruction = Box::new(MULS());
                let src_am = am_type.addressing_mode_by_type(idx, Size::Word);
                let dst_am = Box::new(DataRegister {
                    reg: data_reg_idx,
                    size: Size::Word,
                });

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                let cycles = 70 + am_type.additional_clocks(Size::Word);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

impl OpcodeMaskGenerator for MULU {
    fn generate_mask(&self) -> usize {
        0b1100000011000000
    }
}

fn generate_mulu<T: BusM68k>(table: &mut [Operation<T>]) {
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
                let instruction = Box::new(MULU());
                let src_am = am_type.addressing_mode_by_type(idx, Size::Word);
                let dst_am = Box::new(DataRegister {
                    reg: data_reg_idx,
                    size: Size::Word,
                });

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                let cycles = 70 + am_type.additional_clocks(Size::Word);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

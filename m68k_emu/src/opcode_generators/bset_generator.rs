use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister, Immediate},
    bus::BusM68k,
    instruction_set::bit_manipulation::BSET,
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_bset_reg(table);
    generate_bset_i(table);
}

impl OpcodeMaskGenerator for BSET {
    fn generate_mask(&self) -> usize {
        0b000000011000000
    }
}

fn generate_bset_reg<T: BusM68k>(table: &mut [Operation<T>]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for data_reg_idx in 0..8 {
        for am_type in am_types {
            for idx in range!(am_type) {
                let size = match am_type {
                    AddressingModeType::DataRegister => Size::Long,
                    _ => Size::Byte,
                };

                let instruction = Box::new(BSET { size: size });
                let src_am = Box::new(DataRegister {
                    reg: data_reg_idx,
                    size: Size::Long,
                });
                let dst_am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode =
                    base_mask | (data_reg_idx << 9) | (1 << 8) | am_type.generate_mask(idx);

                let cycles = 8 + am_type.additional_clocks(size);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

fn generate_bset_i<T: BusM68k>(table: &mut [Operation<T>]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for am_type in am_types {
        for idx in range!(am_type) {
            let size = match am_type {
                AddressingModeType::DataRegister => Size::Long,
                _ => Size::Byte,
            };

            let instruction = Box::new(BSET { size: size });
            let src_am = Box::new(Immediate { size: Size::Byte });
            let dst_am = am_type.addressing_mode_by_type(idx, size);

            let base_mask = instruction.generate_mask();
            let opcode = base_mask | (1 << 11) | am_type.generate_mask(idx);

            let cycles = 12 + am_type.additional_clocks(size);

            let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
            table[opcode] = operation;
        }
    }
}

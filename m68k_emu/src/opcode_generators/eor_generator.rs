use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister, Immediate},
    instruction_set::{
        logical_instructions::{EOR, EORI},
        system_control::{EORItoCCR, EORItoSR},
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_eor(table);
    generate_eori(table);
    generate_eori_to_ccr(table);
    generate_eori_to_sr(table);
}

impl OpcodeMaskGenerator for EOR {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1011000100000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_eor(table: &mut [Operation]) {
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

    for data_reg_idx in 0..8 {
        for size in [Size::Byte, Size::Word, Size::Long] {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(EOR { size: size });
                    let src_am = Box::new(DataRegister {
                        reg: data_reg_idx,
                        size,
                    });
                    let dst_am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                    let mut cycles = if size == Size::Byte || size == Size::Word {
                        match am_type {
                            AddressingModeType::DataRegister => 4,
                            _ => 8,
                        }
                    } else {
                        match am_type {
                            AddressingModeType::DataRegister => 8,
                            _ => 12,
                        }
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for EORI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000101000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_eori(table: &mut [Operation]) {
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
                let instruction = Box::new(EORI { size: size });
                let src_am = Box::new(Immediate { size: size });
                let dst_am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let mut cycles = if size == Size::Byte || size == Size::Word {
                    match am_type {
                        AddressingModeType::DataRegister => 8,
                        _ => 12,
                    }
                } else {
                    match am_type {
                        AddressingModeType::DataRegister => 16,
                        _ => 20,
                    }
                };
                cycles += am_type.additional_clocks(size);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

impl OpcodeMaskGenerator for EORItoCCR {
    fn generate_mask(&self) -> usize {
        0b0000101000111100
    }
}

fn generate_eori_to_ccr(table: &mut [Operation]) {
    let instruction = Box::new(EORItoCCR());
    let src_am = Box::new(Immediate { size: Size::Byte });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

impl OpcodeMaskGenerator for EORItoSR {
    fn generate_mask(&self) -> usize {
        0b0000101001111100
    }
}

fn generate_eori_to_sr(table: &mut [Operation]) {
    let instruction = Box::new(EORItoSR());
    let src_am = Box::new(Immediate { size: Size::Word });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

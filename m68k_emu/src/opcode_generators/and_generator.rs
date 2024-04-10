use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister, Immediate},
    instruction_set::{
        logical_instructions::{AND, ANDI},
        system_control::{ANDI_to_CCR, ANDI_to_SR},
        WriteDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_and_mem_to_reg(table);
    generate_and_reg_to_mem(table);
    generate_andi(table);
    generate_andi_ccr(table);
    generate_andi_sr(table);
}

impl OpcodeMaskGenerator for AND {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1100000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_and_mem_to_reg(table: &mut [Operation]) {
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
        for size in [Size::Byte, Size::Word, Size::Long] {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(AND {
                        size: size,
                    });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(DataRegister { reg: data_reg_idx });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | ((WriteDirection::ToDataRegister as usize) << 8) | am_type.generate_mask(idx);

                    let mut cycles = if size == Size::Byte || size == Size::Word {
                        4
                    } else {
                        match am_type {
                            AddressingModeType::DataRegister
                            | AddressingModeType::AddressRegister
                            | AddressingModeType::Immediate => 8,
                            _ => 6,
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

fn generate_and_reg_to_mem(table: &mut [Operation]) {
    let am_types = [
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
                    let instruction = Box::new(AND {
                        size: size,
                    });
                    let src_am = Box::new(DataRegister { reg: data_reg_idx });
                    let dst_am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | ((WriteDirection::ToMemory as usize) << 8) | am_type.generate_mask(idx);

                    let mut cycles = if size == Size::Byte || size == Size::Word {
                        8
                    } else {
                        12
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ANDI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000001000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_andi(table: &mut [Operation]) {
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
                let instruction = Box::new(ANDI { size: size });
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
                        AddressingModeType::DataRegister => 14,
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

impl OpcodeMaskGenerator for ANDI_to_CCR {
    fn generate_mask(&self) -> usize {
        0b0000001000111100
    }
}

fn generate_andi_ccr(table: &mut [Operation]) {
    let instruction = Box::new(ANDI_to_CCR());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 20);
    table[opcode] = operation;
}

impl OpcodeMaskGenerator for ANDI_to_SR {
    fn generate_mask(&self) -> usize {
        0b0000001001111100
    }
}

fn generate_andi_sr(table: &mut [Operation]) {
    let instruction = Box::new(ANDI_to_SR());
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![], 20);
    table[opcode] = operation;
}

use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister, Immediate},
    instruction_set::{
        logical_instructions::{OR, ORI},
        system_control::{ORItoCCR, ORItoSR},
        WriteDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_or_mem_to_reg(table);
    generate_or_reg_to_mem(table);
    generate_ori(table);
    generate_ori_to_ccr(table);
    generate_ori_to_sr(table);
}

impl OpcodeMaskGenerator for OR {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1000000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_or_mem_to_reg(table: &mut [Operation]) {
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
                    let instruction = Box::new(OR { size: size });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(DataRegister {
                        reg: data_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask
                        | (data_reg_idx << 9)
                        | ((WriteDirection::ToDataRegister as usize) << 8)
                        | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Byte | Size::Word => 4,
                        Size::Long => match am_type {
                            AddressingModeType::DataRegister | AddressingModeType::Immediate => 8,
                            _ => 6,
                        },
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

fn generate_or_reg_to_mem(table: &mut [Operation]) {
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
                    let instruction = Box::new(OR { size: size });
                    let src_am = Box::new(DataRegister {
                        reg: data_reg_idx,
                        size,
                    });
                    let dst_am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask
                        | (data_reg_idx << 9)
                        | ((WriteDirection::ToMemory as usize) << 8)
                        | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Byte | Size::Word => 4,
                        Size::Long => match am_type {
                            AddressingModeType::DataRegister | AddressingModeType::Immediate => 8,
                            _ => 6,
                        },
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ORI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_ori(table: &mut [Operation]) {
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
                let instruction = Box::new(ORI { size: size });
                let src_am = Box::new(Immediate { size: size });
                let dst_am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let mut cycles = match am_type {
                    AddressingModeType::DataRegister => match size {
                        Size::Byte | Size::Word => 8,
                        Size::Long => 16,
                    },
                    _ => match size {
                        Size::Byte | Size::Word => 12,
                        Size::Long => 20,
                    },
                };
                cycles += am_type.additional_clocks(size);

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

impl OpcodeMaskGenerator for ORItoCCR {
    fn generate_mask(&self) -> usize {
        0b0000000000111100
    }
}

fn generate_ori_to_ccr(table: &mut [Operation]) {
    let instruction = Box::new(ORItoCCR());
    let src_am = Box::new(Immediate { size: Size::Byte });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

impl OpcodeMaskGenerator for ORItoSR {
    fn generate_mask(&self) -> usize {
        0b0000000001111100
    }
}

fn generate_ori_to_sr(table: &mut [Operation]) {
    let instruction = Box::new(ORItoSR());
    let src_am = Box::new(Immediate { size: Size::Word });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

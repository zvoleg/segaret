use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister, Immediate},
    bus::BusM68k,
    instruction_set::{
        logical_instructions::{OR, ORI},
        system_control::{ORI_to_CCR, ORI_to_SR},
        WriteDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
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
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

fn generate_or_mem_to_reg<T: BusM68k>(table: &mut [Operation<T>]) {
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
                    let instruction = Box::new(OR {
                        size: size,
                        direction: WriteDirection::ToDataRegister,
                    });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(DataRegister { reg: data_reg_idx });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

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

fn generate_or_reg_to_mem<T: BusM68k>(table: &mut [Operation<T>]) {
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
                    let instruction = Box::new(OR {
                        size: size,
                        direction: WriteDirection::ToMemory,
                    });
                    let src_am = Box::new(DataRegister { reg: data_reg_idx });
                    let dst_am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

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

fn generate_ori<T: BusM68k>(table: &mut [Operation<T>]) {
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

impl OpcodeMaskGenerator for ORI_to_CCR {
    fn generate_mask(&self) -> usize {
        0b0000000000111100
    }
}

fn generate_ori_to_ccr<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(ORI_to_CCR());
    let src_am = Box::new(Immediate { size: Size::Byte });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

impl OpcodeMaskGenerator for ORI_to_SR {
    fn generate_mask(&self) -> usize {
        0b0000000001111100
    }
}

fn generate_ori_to_sr<T: BusM68k>(table: &mut [Operation<T>]) {
    let instruction = Box::new(ORI_to_SR());
    let src_am = Box::new(Immediate { size: Size::Byte });
    let opcode = instruction.generate_mask();
    let operation = Operation::new(instruction, vec![src_am], 20);
    table[opcode] = operation;
}

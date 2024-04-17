use crate::{
    addressing_mode_set::{
        AddressRegister, AddressRegisterPostIncrement, AddressingModeType, DataRegister, Immediate,
    },
    instruction_set::integer_arithmetic::{CMP, CMPA, CMPI, CMPM},
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_cmp(table);
    generate_cmpa(table);
    generate_cmpi(table);
    generate_cmpm(table);
}

impl OpcodeMaskGenerator for CMP {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1011000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_cmp(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegister, // Word and Long only
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
                    match am_type {
                        AddressingModeType::AddressRegister => {
                            if size == Size::Byte {
                                continue;
                            }
                        }
                        _ => (),
                    };
                    let instruction = Box::new(CMP { size: size });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(DataRegister {
                        reg: data_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                    let mut cycles = if size == Size::Byte || size == Size::Word {
                        4
                    } else {
                        6
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for CMPA {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1011000011000000;
        base_mask |= match self.size {
            Size::Byte => panic!("CMPA: generate_mask: unexpected instruction size"),
            Size::Word => 0,
            Size::Long => 1,
        } << 8;
        base_mask
    }
}

fn generate_cmpa(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegister,
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

    for address_reg_idx in 0..8 {
        for size in [Size::Word, Size::Long] {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(CMPA { size: size });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(AddressRegister {
                        reg: address_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (address_reg_idx << 9) | am_type.generate_mask(idx);

                    let cycles = 6 + am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for CMPI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000110000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_cmpi(table: &mut [Operation]) {
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
                let instruction = Box::new(CMPI { size: size });
                let src_am = Box::new(Immediate { size: size });
                let dst_am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let mut cycles = if size == Size::Byte || size == Size::Word {
                    8
                } else {
                    match am_type {
                        AddressingModeType::DataRegister => 14,
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

impl OpcodeMaskGenerator for CMPM {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1011000100001000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_cmpm(table: &mut [Operation]) {
    for reg_y in 0..8 {
        for reg_x in 0..8 {
            for size in [Size::Byte, Size::Word, Size::Long] {
                let instruction = Box::new(CMPM { size: size });
                let src_am = Box::new(AddressRegisterPostIncrement {
                    reg: reg_y,
                    size: size,
                });
                let dst_am = Box::new(AddressRegisterPostIncrement {
                    reg: reg_x,
                    size: size,
                });

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (reg_y << 9) | reg_x;

                let cycles = if size == Size::Byte || size == Size::Word {
                    12
                } else {
                    20
                };

                let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

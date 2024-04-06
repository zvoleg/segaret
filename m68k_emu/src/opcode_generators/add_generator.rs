use crate::{
    addressing_mode_set::{
        AddressRegister, AddressRegisterPreDecrement, AddressingMode, AddressingModeType,
        DataRegister, Immediate,
    },
    instruction_set::{
        integer_arithmetic::{ADD, ADDA, ADDI, ADDQ, ADDX},
        RegisterFieldMode, WriteDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_add_mem_to_reg(table);
    generate_add_reg_to_mem(table);
    generate_adda(table);
    generate_addi(table);
    generate_addq(table);
    generate_addx(table);
}

impl OpcodeMaskGenerator for ADD {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1101000000000000;
        base_mask |= (self.direction as usize) << 8;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_add_mem_to_reg(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegister, // Word and Long only
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::ProgramCounterDisplacement,
        AddressingModeType::ProgramCounterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
        AddressingModeType::Immediate,
    ];

    for size in [Size::Byte, Size::Word, Size::Long] {
        for data_reg_idx in 0..8 {
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
                    let instruction = Box::new(ADD {
                        size: size,
                        direction: WriteDirection::ToDataRegister,
                    });
                    let data_reg_am = Box::new(DataRegister { reg: data_reg_idx });
                    let am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

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

                    let operation = Operation::new(instruction, vec![am, data_reg_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

fn generate_add_reg_to_mem(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for size in [Size::Byte, Size::Word, Size::Long] {
        for data_reg_idx in 0..8 {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(ADD {
                        size: size,
                        direction: WriteDirection::ToMemory,
                    });
                    let data_reg_am = Box::new(DataRegister { reg: data_reg_idx });
                    let am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg_idx << 9) | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Byte | Size::Word => 8,
                        Size::Long => 12,
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![data_reg_am, am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ADDA {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1101000011000000;
        base_mask |= match self.size {
            Size::Byte => panic!("ADDA: generate_mask: unexpected instruction size"),
            Size::Word => 0,
            Size::Long => 1,
        } << 8;
        base_mask
    }
}

fn generate_adda(table: &mut [Operation]) {
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

    for size in [Size::Word, Size::Long] {
        for address_reg_idx in 0..8 {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(ADDA { size: size });
                    let addressregister_am = Box::new(AddressRegister {
                        reg: address_reg_idx,
                    });
                    let am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (address_reg_idx << 9) | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Word => 8,
                        Size::Long => 6,
                        Size::Byte => {
                            panic!("generate_adda: adda can't has the operand size equals to Byte")
                        }
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation =
                        Operation::new(instruction, vec![am, addressregister_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ADDI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000011000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_addi(table: &mut [Operation]) {
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
                let instruction = Box::new(ADDI { size: size });
                let immediate_am = Box::new(Immediate { size: size });
                let am = am_type.addressing_mode_by_type(idx, size);

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

                let operation = Operation::new(instruction, vec![immediate_am, am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

impl OpcodeMaskGenerator for ADDQ {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0101000000000000;
        base_mask |= (self.data as usize) << 9;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_addq(table: &mut [Operation]) {
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
    ];

    for data in 0..=0b111 {
        for size in [Size::Byte, Size::Word, Size::Long] {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(ADDQ {
                        size: size,
                        data: data,
                    });
                    let am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | am_type.generate_mask(idx);

                    let mut cycles = if size == Size::Byte || size == Size::Word {
                        match am_type {
                            AddressingModeType::DataRegister
                            | AddressingModeType::AddressRegister => 4,
                            _ => 8,
                        }
                    } else {
                        match am_type {
                            AddressingModeType::DataRegister
                            | AddressingModeType::AddressRegister => 8,
                            _ => 12,
                        }
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ADDX {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1101000100000000;
        base_mask |= (self.register_field_mode as usize) << 3;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_addx(table: &mut [Operation]) {
    for mode in [
        RegisterFieldMode::DataRegister,
        RegisterFieldMode::PreDecrement,
    ] {
        for reg_x in 0..8 {
            for size in [Size::Byte, Size::Word, Size::Long] {
                for reg_y in 0..8 {
                    let instruction = Box::new(ADDX {
                        size: size,
                        register_field_mode: mode,
                    });
                    let src_am: Box<dyn AddressingMode>;
                    let dst_am: Box<dyn AddressingMode>;
                    match mode {
                        RegisterFieldMode::DataRegister => {
                            src_am = Box::new(DataRegister { reg: reg_y });
                            dst_am = Box::new(DataRegister { reg: reg_x });
                        }
                        RegisterFieldMode::PreDecrement => {
                            src_am = Box::new(AddressRegisterPreDecrement {
                                reg: reg_y,
                                size: size,
                            });
                            dst_am = Box::new(AddressRegisterPreDecrement {
                                reg: reg_x,
                                size: size,
                            });
                        }
                    };

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (reg_x << 9) | reg_y;

                    let cycles = match mode {
                        RegisterFieldMode::DataRegister => {
                            4 + AddressingModeType::DataRegister.additional_clocks(size)
                        }
                        RegisterFieldMode::PreDecrement => {
                            let base = if size == Size::Long { 30 } else { 18 };
                            base + AddressingModeType::AddressRegisterPreDecrement
                                .additional_clocks(size)
                        }
                    };

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

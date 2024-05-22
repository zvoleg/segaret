use crate::{
    addressing_mode_set::{
        AddressRegister, AddressRegisterPreDecrement, AddressingMode, AddressingModeType,
        DataRegister, Immediate,
    }, bus::BusM68k, instruction_set::{
        integer_arithmetic::{SUB, SUBA, SUBI, SUBQ, SUBX},
        RegisterFieldMode, WriteDirection,
    }, operation::Operation, primitives::Size, range
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_sub_mem_to_reg(table);
    generate_sub_reg_to_mem(table);
    generate_suba(table);
    generate_subi(table);
    generate_subq(table);
    generate_subx(table);
}

impl OpcodeMaskGenerator for SUB {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1001000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_sub_mem_to_reg<T: BusM68k>(table: &mut [Operation<T>]) {
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

    for size in [Size::Byte, Size::Word, Size::Long] {
        for data_register_idx in 0..8 {
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

                    let instruction = Box::new(SUB { size: size });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(DataRegister {
                        reg: data_register_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask
                        | (data_register_idx << 9)
                        | ((WriteDirection::ToDataRegister as usize) << 8)
                        | am_type.generate_mask(idx);

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

fn generate_sub_reg_to_mem<T: BusM68k>(table: &mut [Operation<T>]) {
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
        for data_register_idx in 0..8 {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction = Box::new(SUB { size: size });
                    let src_am = Box::new(DataRegister {
                        reg: data_register_idx,
                        size,
                    });
                    let dst_am = am_type.addressing_mode_by_type(idx, size);

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask
                        | (data_register_idx << 9)
                        | ((WriteDirection::ToMemory as usize) << 8)
                        | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Byte | Size::Word => 8,
                        Size::Long => 12,
                    };
                    cycles += am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for SUBA {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1001000011000000;
        base_mask |= match self.size {
            Size::Byte => panic!("SUBA: generate_mask: unexpected instruction size"),
            Size::Word => 0,
            Size::Long => 1,
        } << 8;
        base_mask
    }
}

fn generate_suba<T: BusM68k>(table: &mut [Operation<T>]) {
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
                    let instruction = Box::new(SUBA { size: size });
                    let src_am = am_type.addressing_mode_by_type(idx, size);
                    let dst_am = Box::new(AddressRegister {
                        reg: address_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (address_reg_idx << 9) | am_type.generate_mask(idx);

                    let mut cycles = match size {
                        Size::Word => 8,
                        Size::Long => 6,
                        Size::Byte => {
                            panic!("generate_suba: suba can't has the operand size equals to Byte")
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

impl OpcodeMaskGenerator for SUBI {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000010000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_subi<T: BusM68k>(table: &mut [Operation<T>]) {
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
                let instruction = Box::new(SUBI { size: size });
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

impl OpcodeMaskGenerator for SUBQ {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0101000100000000;
        base_mask |= (self.data as usize) << 9;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_subq<T: BusM68k>(table: &mut [Operation<T>]) {
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
                    let instruction = Box::new(SUBQ {
                        size: size,
                        data: data,
                        to_address_reg: am_type == AddressingModeType::AddressRegister,
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

impl OpcodeMaskGenerator for SUBX {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1001000100000000;
        base_mask |= (self.register_field_mode as usize) << 3;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

fn generate_subx<T: BusM68k>(table: &mut [Operation<T>]) {
    for reg_x in 0..8 {
        for size in [Size::Byte, Size::Word, Size::Long] {
            for mode in [
                RegisterFieldMode::DataRegister,
                RegisterFieldMode::PreDecrement,
            ] {
                for reg_y in 0..8 {
                    let instruction = Box::new(SUBX {
                        size: size,
                        register_field_mode: mode,
                    });
                    let src_am: Box<dyn AddressingMode>;
                    let dst_am: Box<dyn AddressingMode>;
                    match mode {
                        RegisterFieldMode::DataRegister => {
                            src_am = Box::new(DataRegister { reg: reg_y, size });
                            dst_am = Box::new(DataRegister { reg: reg_x, size });
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

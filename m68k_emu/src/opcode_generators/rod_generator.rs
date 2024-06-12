use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister},
    bus::BusM68k,
    instruction_set::{
        shift_and_rotate::{
            ROXdDataReg, ROXdImplied, ROXdMemory, ROdDataReg, ROdImplied, ROdMemory,
        },
        Instruction, ShiftDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_rod_data_reg(table);
    generate_rod_implied(table);
    generate_rod_mem(table);
}

impl OpcodeMaskGenerator for ROdDataReg {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000111000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

impl OpcodeMaskGenerator for ROXdDataReg {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000110000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

fn generate_rod_data_reg<T: BusM68k>(table: &mut [Operation<T>]) {
    for extended in [true, false] {
        for data_reg_x_idx in 0..8 {
            for direction in [ShiftDirection::Right, ShiftDirection::Left] {
                for size in [Size::Byte, Size::Word, Size::Long] {
                    for data_reg_y_idx in 0..8 {
                        let instruction: Box<dyn Instruction<T>>;
                        let base_mask: usize;
                        if extended {
                            let roxd = Box::new(ROXdDataReg {
                                size: size,
                                direction: direction,
                            });
                            base_mask = roxd.generate_mask();
                            instruction = roxd;
                        } else {
                            let rod = Box::new(ROdDataReg {
                                size: size,
                                direction: direction,
                            });
                            base_mask = rod.generate_mask();
                            instruction = rod;
                        };
                        let src_am = Box::new(DataRegister {
                            reg: data_reg_x_idx,
                            size: Size::Long,
                        });
                        let dst_am = Box::new(DataRegister {
                            reg: data_reg_y_idx,
                            size,
                        });

                        let opcode = base_mask | (data_reg_x_idx << 9) | data_reg_y_idx;

                        let cycles = match size {
                            Size::Byte | Size::Word => 6,
                            Size::Long => 8,
                        };

                        let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                        table[opcode] = operation;
                    }
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ROdImplied {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000011000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask |= (self.direction as usize) << 8;
        base_mask |= (self.count as usize) << 9;
        base_mask
    }
}

impl OpcodeMaskGenerator for ROXdImplied {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000010000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask |= (self.direction as usize) << 8;
        // for the count value eight there is using 000 mask
        if self.count < 8 {
            base_mask |= (self.count as usize) << 9;
        }
        base_mask
    }
}

fn generate_rod_implied<T: BusM68k>(table: &mut [Operation<T>]) {
    for extended in [true, false] {
        for count in 0..8 {
            for direction in [ShiftDirection::Right, ShiftDirection::Left] {
                for size in [Size::Byte, Size::Word, Size::Long] {
                    for data_reg_idx in 0..8 {
                        let instruction: Box<dyn Instruction<T>>;
                        let base_mask: usize;
                        let count = if count == 0 { 8 } else { count };
                        if extended {
                            let roxd = Box::new(ROXdImplied {
                                size: size,
                                direction: direction,
                                count: count,
                            });
                            base_mask = roxd.generate_mask();
                            instruction = roxd;
                        } else {
                            let rod = Box::new(ROdImplied {
                                size: size,
                                direction: direction,
                                count: count,
                            });
                            base_mask = rod.generate_mask();
                            instruction = rod;
                        };
                        let am = Box::new(DataRegister {
                            reg: data_reg_idx,
                            size,
                        });

                        let opcode = base_mask | data_reg_idx;

                        let cycles = match size {
                            Size::Byte | Size::Word => 6,
                            Size::Long => 8,
                        };

                        let operation = Operation::new(instruction, vec![am], cycles);
                        table[opcode] = operation;
                    }
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for ROdMemory {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110011011000000;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

impl OpcodeMaskGenerator for ROXdMemory {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110010011000000;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

fn generate_rod_mem<T: BusM68k>(table: &mut [Operation<T>]) {
    let am_types = [
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for extened in [true, false] {
        for direction in [ShiftDirection::Right, ShiftDirection::Left] {
            for am_type in am_types {
                for idx in range!(am_type) {
                    let instruction: Box<dyn Instruction<T>>;
                    let base_mask: usize;
                    if extened {
                        let roxd = Box::new(ROXdMemory {
                            direction: direction,
                        });
                        base_mask = roxd.generate_mask();
                        instruction = roxd;
                    } else {
                        let rod = Box::new(ROdMemory {
                            direction: direction,
                        });
                        base_mask = rod.generate_mask();
                        instruction = rod;
                    }
                    let am = am_type.addressing_mode_by_type(idx, Size::Word);

                    let opcode = base_mask | am_type.generate_mask(idx);

                    let cycles = 8 + am_type.additional_clocks(Size::Word);

                    let operation = Operation::new(instruction, vec![am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

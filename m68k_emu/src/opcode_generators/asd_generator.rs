use crate::{
    addressing_mode_set::{AddressingModeType, DataRegister},
    instruction_set::{
        shift_and_rotate::{ASdDataReg, ASdImplied, ASdMemory},
        ShiftDirection,
    },
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate(table: &mut [Operation]) {
    generate_asd_data_reg(table);
    generate_asd_implied(table);
    generate_asd_mem(table);
}

impl OpcodeMaskGenerator for ASdDataReg {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000100000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

fn generate_asd_data_reg(table: &mut [Operation]) {
    for data_reg_x_idx in 0..8 {
        for direction in [ShiftDirection::Right, ShiftDirection::Left] {
            for size in [Size::Byte, Size::Word, Size::Long] {
                for data_reg_y_idx in 0..8 {
                    let instruction = Box::new(ASdDataReg {
                        size: size,
                        direction: direction,
                    });
                    let src_am = Box::new(DataRegister {
                        reg: data_reg_x_idx,
                        size: Size::Long,
                    });
                    let dst_am = Box::new(DataRegister {
                        reg: data_reg_y_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
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

impl OpcodeMaskGenerator for ASdImplied {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000000000000;
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

fn generate_asd_implied(table: &mut [Operation]) {
    for count in 0..8 {
        for direction in [ShiftDirection::Right, ShiftDirection::Left] {
            for size in [Size::Byte, Size::Word, Size::Long] {
                for data_reg_idx in 0..8 {
                    let count = if count == 0 { 8 } else { count };
                    let instruction = Box::new(ASdImplied {
                        size: size,
                        direction: direction,
                        count: count,
                    });
                    let am = Box::new(DataRegister {
                        reg: data_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
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

impl OpcodeMaskGenerator for ASdMemory {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1110000011000000;
        base_mask |= (self.direction as usize) << 8;
        base_mask
    }
}

fn generate_asd_mem(table: &mut [Operation]) {
    let am_types = [
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for direction in [ShiftDirection::Right, ShiftDirection::Left] {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(ASdMemory {
                    direction: direction,
                });
                let am = am_type.addressing_mode_by_type(idx, Size::Word);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let cycles = 8 + am_type.additional_clocks(Size::Word);

                let operation = Operation::new(instruction, vec![am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

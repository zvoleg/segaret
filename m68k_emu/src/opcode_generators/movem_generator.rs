use crate::{
    addressing_mode_set::{AddressingMode, AddressingModeType, Immediate},
    instruction_set::{data_movement::MOVEM, MoveDirection},
    operation::Operation,
    primitives::Size,
    range,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for MOVEM {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100100010000000;
        base_mask |= (self.direction as usize) << 10;
        base_mask |= match self.size {
            Size::Byte => panic!("MOVEM: generate_mask: unexpected size value"),
            Size::Word => 0,
            Size::Long => 1,
        } << 6;
        base_mask
    }
}

pub(crate) fn generate(table: &mut [Operation]) {
    generate_movem_reg_to_mem(table);
    generate_movem_mem_to_reg(table);
}

fn generate_movem_reg_to_mem(table: &mut [Operation]) {
    for size in [Size::Word, Size::Long] {
        for am_type in [
            AddressingModeType::AddressRegisterIndirect,
            AddressingModeType::AddressRegisterPreDecrement,
            AddressingModeType::AddressRegisterDisplacement,
            AddressingModeType::AddressRegisterIndexed,
            AddressingModeType::AbsShort,
            AddressingModeType::AbsLong,
        ] {
            for idx in range!(am_type) {
                let instruction = Box::new(MOVEM {
                    direction: MoveDirection::RegisterToMemory,
                    size: size,
                    addressing_mode_type: am_type,
                    am_register_idx: idx as isize,
                });
                let am = am_type.addressing_mode_by_type(idx, size);
                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);
                let mut clocks = 4;
                clocks += am_type.additional_clocks(size);
                match am_type {
                    AddressingModeType::AddressRegisterPreDecrement => clocks -= 2,
                    _ => (),
                }
                let mut addressing_mode_list: Vec<Box<dyn AddressingMode>> = Vec::new();
                addressing_mode_list.push(Box::new(Immediate { size: Size::Word }));
                addressing_mode_list.push(am);
                let operation = Operation::new(instruction, addressing_mode_list, clocks);
                table[opcode] = operation;
            }
        }
    }
}

fn generate_movem_mem_to_reg(table: &mut [Operation]) {
    for size in [Size::Word, Size::Long] {
        for am_type in [
            AddressingModeType::AddressRegisterIndirect,
            AddressingModeType::AddressRegisterPostIncrement,
            AddressingModeType::AddressRegisterDisplacement,
            AddressingModeType::AddressRegisterIndexed,
            AddressingModeType::ProgramCounterDisplacement,
            AddressingModeType::ProgramCounterIndexed,
            AddressingModeType::AbsShort,
            AddressingModeType::AbsLong,
        ] {
            for idx in range!(am_type) {
                let instruction = Box::new(MOVEM {
                    direction: MoveDirection::MemoryToRegister,
                    size: size,
                    addressing_mode_type: am_type,
                    am_register_idx: idx as isize,
                });
                let am = am_type.addressing_mode_by_type(idx, size);
                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let mut clocks = 8;
                clocks += am_type.additional_clocks(size);

                let mut addressing_mode_list: Vec<Box<dyn AddressingMode>> = Vec::new();
                addressing_mode_list.push(Box::new(Immediate { size: Size::Word }));
                addressing_mode_list.push(am);
                let operation = Operation::new(instruction, addressing_mode_list, clocks);
                table[opcode] = operation;
            }
        }
    }
}

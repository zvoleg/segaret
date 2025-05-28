use crate::{
    addressing_mode_set::{
        AddressRegister, AddressRegisterDisplacement, AddressingMode, AddressingModeType,
        DataRegister,
    },
    bus::BusM68k,
    instruction_set::{
        data_movement::{MOVE, MOVEA, MOVEP, MOVEQ},
        system_control::{MOVEfromSR, MOVEtoCCR, MOVEtoSR, MOVEUSP},
        MoveDirection,
    },
    operation::Operation,
    range, Size, USP_REGISTER,
};

use super::OpcodeMaskGenerator;

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    generate_move(table);
    generate_movea(table);
    generate_moveq(table);
    generate_movep(table);
    generate_move_to_ccr(table);
    generate_move_from_sr(table);
    generate_move_to_sr(table);
    generate_move_usp(table);
}

impl OpcodeMaskGenerator for MOVE {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000000000000000;
        base_mask |= match self.size {
            Size::Byte => 0b01,
            Size::Word => 0b11,
            Size::Long => 0b10,
        } << 12;
        base_mask
    }
}

fn swap_addressing_mod_mask(mask: usize) -> usize {
    let l = mask >> 3;
    let h = mask & 0x7;
    (h << 3) | l
}

fn generate_move<T: BusM68k>(table: &mut [Operation<T>]) {
    let dst_am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    let src_am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegister,
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
        for src_am_type in src_am_types {
            for dst_am_type in dst_am_types {
                for src_idx in range!(src_am_type) {
                    for dst_idx in range!(dst_am_type) {
                        let src_am = src_am_type.addressing_mode_by_type(src_idx, size);
                        let dst_am = dst_am_type.addressing_mode_by_type(dst_idx, size);
                        let instruction = Box::new(MOVE { size: size });

                        let base_mask = instruction.generate_mask();
                        let src_am_mask = src_am_type.generate_mask(src_idx);
                        let dst_am_mask =
                            swap_addressing_mod_mask(dst_am_type.generate_mask(dst_idx));
                        let opcode = base_mask | (dst_am_mask << 6) | src_am_mask;

                        let mut cycles = 4;
                        cycles += src_am_type.additional_clocks(size)
                            + dst_am_type.additional_clocks(size);
                        match dst_am_type {
                            AddressingModeType::AddressRegisterPostIncrement => cycles -= 2,
                            _ => (),
                        }
                        let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                        table[opcode] = operation;
                    }
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for MOVEA {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000000001000000;
        base_mask |= match self.size {
            Size::Word => 0b11,
            Size::Long => 0b10,
            _ => panic!("MOVEA: generate_mask: unexpected instruction size"),
        } << 12;
        base_mask
    }
}

fn generate_movea<T: BusM68k>(table: &mut [Operation<T>]) {
    let src_am_tyeps = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegister,
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

    for size in [Size::Word, Size::Long] {
        for address_reg_idx in 0..8 {
            for src_am_type in src_am_tyeps {
                for src_idx in range!(src_am_type) {
                    let instruction = Box::new(MOVEA { size: size });
                    let src_am = src_am_type.addressing_mode_by_type(src_idx, size);
                    let dst_am = Box::new(AddressRegister {
                        reg: address_reg_idx,
                        size,
                    });

                    let base_mask = instruction.generate_mask();
                    let src_am_mask = src_am_type.generate_mask(src_idx);
                    let opcode = base_mask | (address_reg_idx << 9) | src_am_mask;

                    let mut cycles = 4;
                    cycles += src_am_type.additional_clocks(size);

                    let operation = Operation::new(instruction, vec![src_am, dst_am], cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for MOVEQ {
    fn generate_mask(&self) -> usize {
        let base_mask = 0b0111000000000000;
        base_mask | self.data as usize
    }
}

fn generate_moveq<T: BusM68k>(table: &mut [Operation<T>]) {
    for reg in 0..8 {
        for data in 0..=0xFF {
            let instruction = Box::new(MOVEQ { data: data });
            let base_mask = instruction.generate_mask();
            let opcode = base_mask | reg << 9;
            let dst_am = Box::new(DataRegister {
                reg: reg,
                size: Size::Long,
            });
            let operation = Operation::new(instruction, vec![dst_am], 4);
            table[opcode] = operation;
        }
    }
}

impl OpcodeMaskGenerator for MOVEP {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0000000100001000;
        base_mask |= match self.size {
            Size::Byte => panic!("MOVEP: generate_mask: unexpected instruction size"),
            Size::Word => 0b100,
            Size::Long => 0b101,
        } << 6;
        base_mask |= match self.direction {
            MoveDirection::RegisterToMemory => 0b1,
            MoveDirection::MemoryToRegister => 0b0,
        } << 7;
        base_mask
    }
}

fn generate_movep<T: BusM68k>(table: &mut [Operation<T>]) {
    for data_reg in 0..8 {
        for size in [Size::Word, Size::Long] {
            for direction in [
                MoveDirection::MemoryToRegister,
                MoveDirection::RegisterToMemory,
            ] {
                for adr_reg in 0..8 {
                    let instruction = Box::new(MOVEP {
                        size: size,
                        direction: direction,
                    });
                    let data_register_am = Box::new(DataRegister {
                        reg: data_reg,
                        size,
                    });
                    let address_indireact_am =
                        Box::new(AddressRegisterDisplacement { reg: adr_reg, size });
                    let am_list: Vec<Box<dyn AddressingMode>> = match direction {
                        MoveDirection::RegisterToMemory => {
                            vec![data_register_am, address_indireact_am]
                        }

                        MoveDirection::MemoryToRegister => {
                            vec![address_indireact_am, data_register_am]
                        }
                    };

                    let base_mask = instruction.generate_mask();
                    let opcode = base_mask | (data_reg << 9) | adr_reg;

                    let cycles = match size {
                        Size::Byte => panic!("movep_generate: unexpected size 'Byte'"),
                        Size::Word => 16,
                        Size::Long => 24,
                    };

                    let operation = Operation::new(instruction, am_list, cycles);
                    table[opcode] = operation;
                }
            }
        }
    }
}

impl OpcodeMaskGenerator for MOVEtoCCR {
    fn generate_mask(&self) -> usize {
        0b0100010011000000
    }
}

fn generate_move_to_ccr<T: BusM68k>(table: &mut [Operation<T>]) {
    let src_am_types = [
        AddressingModeType::DataRegister,
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

    for src_am_type in src_am_types {
        for reg_idx in range!(src_am_type) {
            let instruction = Box::new(MOVEtoCCR());
            let src_am = src_am_type.addressing_mode_by_type(reg_idx, Size::Word);

            let base_mask = instruction.generate_mask();
            let am_mask = src_am_type.generate_mask(reg_idx);
            let opcode = base_mask | am_mask;

            let mut cycles = 12;
            cycles += src_am_type.additional_clocks(Size::Word);

            let operation = Operation::new(instruction, vec![src_am], cycles);
            table[opcode] = operation;
        }
    }
}

impl OpcodeMaskGenerator for MOVEfromSR {
    fn generate_mask(&self) -> usize {
        0b0100000011000000
    }
}

fn generate_move_from_sr<T: BusM68k>(table: &mut [Operation<T>]) {
    let dst_am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for dst_am_type in dst_am_types {
        for reg_idx in range!(dst_am_type) {
            let instruction = Box::new(MOVEfromSR());
            let dst_am = dst_am_type.addressing_mode_by_type(reg_idx, Size::Word);

            let base_mask = instruction.generate_mask();
            let am_mask = dst_am_type.generate_mask(reg_idx);
            let opcode = base_mask | am_mask;

            let mut cycles = 4;
            cycles += dst_am_type.additional_clocks(Size::Word);
            match dst_am_type {
                AddressingModeType::DataRegister => (),
                _ => cycles += 4,
            }

            let operation = Operation::new(instruction, vec![dst_am], cycles);
            table[opcode] = operation;
        }
    }
}

impl OpcodeMaskGenerator for MOVEtoSR {
    fn generate_mask(&self) -> usize {
        0b0100011011000000
    }
}

fn generate_move_to_sr<T: BusM68k>(table: &mut [Operation<T>]) {
    let src_am_types = [
        AddressingModeType::DataRegister,
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

    for src_am_type in src_am_types {
        for reg_idx in range!(src_am_type) {
            let instruction = Box::new(MOVEtoSR());
            let src_am = src_am_type.addressing_mode_by_type(reg_idx, Size::Word);

            let base_mask = instruction.generate_mask();
            let am_mask = src_am_type.generate_mask(reg_idx);
            let opcode = base_mask | am_mask;

            let mut cycles = 12;
            cycles += src_am_type.additional_clocks(Size::Word);

            let operation = Operation::new(instruction, vec![src_am], cycles);
            table[opcode] = operation;
        }
    }
}

impl OpcodeMaskGenerator for MOVEUSP {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100111001100000;
        base_mask |= match self.direction {
            MoveDirection::RegisterToMemory => 1, // USP to Memory(addr reg)
            MoveDirection::MemoryToRegister => 0, // Memory(addr reg) to USP
        } << 3;
        base_mask
    }
}

fn generate_move_usp<T: BusM68k>(table: &mut [Operation<T>]) {
    for direction in [
        MoveDirection::RegisterToMemory,
        MoveDirection::MemoryToRegister,
    ] {
        for reg in 0..8 {
            let instruction = Box::new(MOVEUSP {
                direction: direction,
            });
            let src_am: Box<dyn AddressingMode>;
            let dst_am: Box<dyn AddressingMode>;
            match direction {
                MoveDirection::RegisterToMemory => {
                    src_am = Box::new(AddressRegister {
                        reg: USP_REGISTER,
                        size: Size::Long,
                    });
                    dst_am = Box::new(AddressRegister {
                        reg: reg,
                        size: Size::Long,
                    });
                }
                MoveDirection::MemoryToRegister => {
                    src_am = Box::new(AddressRegister {
                        reg: reg,
                        size: Size::Long,
                    });
                    dst_am = Box::new(AddressRegister {
                        reg: USP_REGISTER,
                        size: Size::Long,
                    });
                }
            }
            let opcode = instruction.generate_mask() | reg;

            let operation = Operation::new(instruction, vec![src_am, dst_am], 6);
            table[opcode] = operation;
        }
    }
}

use crate::{decoder::{Operation, Direction, BitShiftingInfo, InstructionData, InstructionType}, Size, addressing_mode::AdrMode, adr_mode};

pub(crate) fn generate(table: &mut [Operation]) {
    generate_rod_reg(table);
    generate_rod_mem(table);
}

fn generate_rod_reg(table: &mut [Operation]) {
    let base_mask: usize = 0b1110000000011000;

    for counter in 0..=0b111 {
        for direction in 0..=1 {
            for size in [Size::Byte, Size::Word, Size::Long] {
                for mode in 0..=1 {
                    for extended_bit in 0..=1 {
                        for reg in 0..=0b111 {
                            let mask = counter << 9 | direction << 8 | usize::from(&size) << 6 | mode << 5 | extended_bit << 3 | reg;
                            let opcode = base_mask | mask;
                            let direction = Direction::from(direction);
                            let rotation_info = match mode {
                                0 => BitShiftingInfo::new(
                                    Some(counter as u32),
                                    None,
                                    direction,
                                ),
                                1 => BitShiftingInfo::new(
                                    None,
                                    Some(AdrMode::DataReg(counter)),
                                    direction
                                ),
                                _ => panic!("generated_rod_reg: unexpected mode bit {}", mode),
                            };
                            let inst_data = InstructionData::BitShiftingReg(rotation_info);
                            let clocks = match size {
                                Size::Byte | Size::Word => 6,
                                Size::Long => 8,
                            };
                            let inst = Operation::new(
                                opcode as u16,
                                inst_name(direction, extended_bit == 0),
                                InstructionType::ROd,
                                inst_data,
                                size,
                                extended_bit == 0,
                                clocks,
                            );
                            table[opcode] = inst;
                        }
                    }
                }
            }
        }
    }
}

fn generate_rod_mem(table: &mut [Operation]) {
    let base_mask: usize = 0b1110010011000000;

    let am_types = [
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,

    for direction_bit in 0..=1 {
        for extend_bit in 0..=1 {
            for am_type in am_types {
                let mask = extend_bit << 9 | direction_bit << 8 | usize::from(am);
                let opcode = base_mask | mask;
                let direction = Direction::from(direction_bit);
                let inst_data = InstructionData::BitShiftingMem(direction, *am);
                let clocks = 8 + am_type.additional_clocks(Size::Word);
                let inst = Operation::new(
                    opcode as u16,
                    inst_name(direction, extend_bit == 0),
                    InstructionType::ROd,
                    inst_data,
                    Size::Word,
                    extend_bit == 0,
                    clocks,
                );
                table[opcode] = inst;
            }
        }
    }
}

fn inst_name(direction: Direction, extended: bool) -> &'static str {
    if extended {
        match direction {
            Direction::Right => "ROXR",
            Direction::Left => "ROXL",
        }
    } else {
        match direction {
            Direction::Right => "ROR",
            Direction::Left => "ROL",
        }
    }
}

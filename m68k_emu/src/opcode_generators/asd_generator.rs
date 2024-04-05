use crate::{
    decoder::{Operation, InstructionData, BitShiftingInfo, Direction, InstructionType}, Size,
    adr_mode,
    addressing_mode::AdrMode
};

pub(crate) fn generate(table: &mut [Operation]) {
    generate_asd_reg(table);
    generate_asd_mem(table);
}

fn generate_asd_reg(table: &mut [Operation]) {
    let base_mask = 0b1110000000000000;

    for counter in 0..=0b111 {
        for direction in 0..=1 {
            for size in [Size::Byte, Size::Word, Size::Long] {
                for mode in 0..=1 {
                    for reg in 0..=0b111 {
                        let mask = counter << 9 | direction << 8 | usize::from(&size) << 6 | mode << 5 | reg;
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
                            _ => panic!("generated_asd_reg: unexpected mode bit {}", mode),
                        };
                        let inst_data = InstructionData::BitShiftingReg(rotation_info);
                        let clocks = match size {
                                Size::Byte | Size::Word => 6,
                                Size::Long => 8,
                        };
                        let inst = Operation::new(
                            opcode as u16,
                            inst_name(direction),
                            InstructionType::ASd,
                            inst_data,
                            size,
                            false,
                            clocks,
                        );
                        table[opcode] = inst;
                    }
                }
            }
        }
    }
}

fn generate_asd_mem(table: &mut [Operation]) {
    let base_mask = 0b1110000011000000;

    let am_types = [
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,

    for direction_bit in 0..=1 {
        for am_type in am_types {
            let mask = direction_bit << 8 | usize::from(am);
            let opcode = base_mask | mask;
            let direction = Direction::from(direction_bit);
            let inst_data = InstructionData::BitShiftingMem(direction, *am);
            let clocks = 8 + am_type.additional_clocks(Size::Word);
            let inst = Operation::new(
                opcode as u16,
                inst_name(direction),
                InstructionType::ASd,
                inst_data,
                Size::Word,
                false,
                clocks,
            );
            table[opcode] = inst;
        }
    }
}

fn inst_name(direction: Direction) -> &'static str {
    match direction {
        Direction::Right => "ASR",
        Direction::Left => "ASL",
    }
}

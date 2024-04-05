use crate::{decoder::{Operation, InstructionData, InstructionType}, adr_mode, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    generate_bclr_reg(table);
    generate_bclr_i(table);
}

fn generate_bclr_reg(table: &mut [Operation]) {
    let base_mask = 0b0000000110000000;

    let am_types = [
    AddressingModeType::DataRegister,
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,
    ];

    for reg in 0..8 {
        for am_type in am_types {
            let mask = reg << 9 | usize::from(am);
            let opcode = base_mask | mask;
            let inst_data = InstructionData::SrcDstAm(AdrMode::DataReg(reg), *am);
            let size = match am {
                AddressingModeType::DataRegister => Size::Long,
                _ => Size::Byte,
            };
            let mut clocks = match size {
                Size::Byte => 8,
                Size::Long => 10,
                _ => 0,
            };
            clocks += am_type.additional_clocks(size);
            let inst = Operation::new(
                opcode as u16,
                "BCLR",
                InstructionType::BCLR,
                inst_data,
                size,
                false,
                clocks,
            );
            table[opcode] = inst;
        }
    }
}

fn generate_bclr_i(table: &mut [Operation]) {
    let base_mask = 0b0000100010000000;

    let am_types = [
    AddressingModeType::DataRegister,
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,
    ];

    for am_type in am_types {
        let mask = usize::from(am);
        let opcode = base_mask | mask;
        let inst_data = InstructionData::SrcDstAm(AdrMode::Immediate, *am);
        let size = match am {
            AddressingModeType::DataRegister => Size::Long,
            _ => Size::Byte,
        };
        let mut clocks = match size {
            Size::Byte => 12,
            Size::Long => 14,
            _ => 0,
        };
        clocks += am_type.additional_clocks(size);
        let inst = Operation::new(
            opcode as u16,
            "BCLR",
            InstructionType::BCLR,
            inst_data,
            size,
            false,
            clocks,
        );
        table[opcode] = inst;
    }
}

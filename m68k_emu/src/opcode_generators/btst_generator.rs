use crate::{decoder::{Operation, InstructionData, InstructionType}, adr_mode, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    generate_btst_reg(table);
    generate_btst_i(table);
}


fn generate_btst_reg(table: &mut [Operation]) {
    let base_mask = 0b0000000100000000;

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
                Size::Byte => 4,
                Size::Long => 6,
                _ => 0,
            };
            clocks += am_type.additional_clocks(size);
            let inst = Operation::new(
                opcode as u16,
                "BTST",
                InstructionType::BTST,
                inst_data,
                size,
                false,
                clocks,
            );
            table[opcode] = inst;
        }
    }
}

fn generate_btst_i(table: &mut [Operation]) {
    let base_mask = 0b0000100000000000;

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
            Size::Byte => 8,
            Size::Long => 10,
            _ => 0,
        };
        clocks += am_type.additional_clocks(size);
        let inst = Operation::new(
            opcode as u16,
            "BTST",
            InstructionType::BTST,
            inst_data,
            size,
            false,
            clocks,
        );
        table[opcode] = inst;
    }
}
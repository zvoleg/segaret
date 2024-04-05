use crate::{
    addressing_mode::AdrMode,
    adr_mode,
    decoder::{InstructionData, InstructionType, Operation},
    Size,
};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100101011000000;

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

    for am_type in am_types {
        let mask = usize::from(am);
        let opcode = base_mask | mask;
        let inst_data = InstructionData::DstAm(*am);
        let clocks = match am {
            AddressingModeType::DataRegister => 4,
            _ => 14 + am_type.additional_clocks(Size::Byte),
        };
        let inst = Operation::new(
            opcode as u16,
            "TAS",
            InstructionType::TAS,
            inst_data,
            Size::Byte,
            false,
            clocks,
        );
        table[opcode] = inst;
    }
}

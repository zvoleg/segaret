use crate::{decoder::{Operation, InstructionData, InstructionType}, adr_mode, addressing_mode::AdrMode, Size};

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let base_mask = 0b0100100000000000;

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
            AddressingModeType::DataRegister => 6,
            _ => 8,
        };
        let inst = Operation::new(
            opcode as u16,
            "NBCD",
            InstructionType::NBCD,
            inst_data,
            Size::Byte,
            true,
            clocks,
        );
        table[opcode] = inst;
    }
}

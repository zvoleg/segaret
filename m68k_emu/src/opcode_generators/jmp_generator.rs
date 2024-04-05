use crate::{adr_mode, decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100111011000000;

    let am_types = [
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,
    AddressingModeType::ProgramCounterDisplacement,
    AddressingModeType::ProgramCounterIndexed,

    for am_type in am_types {
        let mask = usize::from(am);
        let opcode = base_mask | mask;
        let inst_data = InstructionData::DstAm(*am);
        let mut clocks = match am {
            AdrMode::AbsLong => 0,
            AdrMode::AbsShort | AddressingModeType::DataRegister | AdrMode::PcIndDisp => 2,
            _ => 4,
        };
        clocks += am_type.additional_clocks(Size::Byte);
        let inst = Operation::new(
            opcode as u16,
            "JMP",
            InstructionType::JMP,
            inst_data,
            Size::Byte,
            false,
            clocks,
        );
        table[opcode] = inst;
    }
}

use crate::{decoder::{Operation, InstructionData, InstructionType}, Size, adr_mode, addressing_mode::AdrMode};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100101000000000;

    let am_types = [
    AddressingModeType::DataRegister,
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,

    for size in [Size::Byte, Size::Word, Size::Long] {
        for am_type in am_types {
            let mask = usize::from(&size) << 6 | usize::from(am);
            let opcode = base_mask | mask;
            let inst_data = InstructionData::DstAm(*am);
            let clocks = 4 + am_type.additional_clocks(size);
            let inst = Operation::new(
                opcode as u16,
                "TST",
                InstructionType::TST,
                inst_data,
                size,
                false,
                clocks,
            );
            table[opcode] = inst;
        }
    }
}

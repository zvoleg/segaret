use crate::{decoder::{Operation, InstructionData, InstructionType}, Size, adr_mode, addressing_mode::AdrMode};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100000000000000;

    let am_types = [
    AddressingModeType::DataRegister,
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,
    AddressingModeType::Immediate,
    AddressingModeType::ProgramCounterDisplacement,
    AddressingModeType::ProgramCounterIndexed,

    for reg in 0..8 {
        for size in [Size::Word] {
            for am_type in am_types {
                let mask = reg << 9 | usize::from(&size) << 7 | usize::from(am);
                let opcode = base_mask | mask;
                let inst_data = InstructionData::SrcDstAm(*am, AdrMode::DataReg(reg));
                let inst = Operation::new(
                    opcode as u16,
                    "CHK",
                    InstructionType::CHK,
                    inst_data,
                    Size::Word,
                    false,
                    10,
                );
                table[opcode] = inst;
            }
        }
    }
}

use crate::{decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100111001011000;

    for reg in 0..8 {
        let opcode = base_mask | reg;
        let inst_data = InstructionData::DstAm(AdrMode::AdrReg(reg));
        let inst = Operation::new(
            opcode as u16,
            "UNLK",
            InstructionType::UNLK,
            inst_data,
            Size::Byte,
            false,
            12,
        );
        table[opcode] = inst;
    }
}

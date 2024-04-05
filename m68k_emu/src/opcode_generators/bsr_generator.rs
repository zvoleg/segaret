use crate::{decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0110000100000000;

    for displacement in 0..=0xFF {
        let opcode = base_mask | displacement;
        let inst_data = match displacement {
            0x00 => InstructionData::DstAm(AdrMode::Immediate),
            _ => InstructionData::DstAm(AdrMode::Implied(displacement)),
        };
        let inst = Operation::new(
            opcode as u16,
            "BSR",
            InstructionType::BSR,
            inst_data,
            Size::Word,
            false,
            18,
        );
        table[opcode] = inst;
    }
}

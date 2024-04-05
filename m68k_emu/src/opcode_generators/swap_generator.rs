use crate::{decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100100001000000;
    
    for reg in 0..8 {
        let opcode = base_mask | reg;
        let inst_data = InstructionData::DstAm(AdrMode::DataReg(reg));
        let inst = Operation::new(
            opcode as u16,
            "SWAP",
            InstructionType::SWAP,
            inst_data,
            Size::Word,
            false,
            4,
        );
        table[opcode] = inst;
    }
}

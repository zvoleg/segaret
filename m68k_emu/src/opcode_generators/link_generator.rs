use crate::{decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0100111001010000;

    for reg in 0..8 {
        let opcode = base_mask | reg;
        let inst_data = InstructionData::ExtDstAm(AdrMode::AdrReg(reg));
        let clocks = 16 + AdrMode::AdrReg(reg).additional_clocks(Size::Word);
        let inst = Operation::new(
            opcode as u16,
            "LINK",
            InstructionType::LINK,
            inst_data,
            Size::Word,
            false,
            clocks,
        );
        table[opcode] = inst;
    }
}

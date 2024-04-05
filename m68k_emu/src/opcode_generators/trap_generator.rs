use crate::{decoder::{Operation, InstructionData, InstructionType, Condition}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    generate_trap(table);
    generate_trapv(table);
}

fn generate_trap(table: &mut [Operation]) {
    let base_mask: usize = 0b0100111001000000;

    for vector in 0..=0xF {
        let opcode = base_mask | vector;
        let inst_data = InstructionData::ConditionAm(Condition::TRUE, AdrMode::Implied(vector));
        let inst = Operation::new(
            opcode as u16,
            "TRAPTRUE",
            InstructionType::TRAPcc,
            inst_data,
            Size::Byte,
            false,
            38,
        );
        table[opcode] = inst;
    }
}

fn generate_trapv(table: &mut [Operation]) {
    let opcode: usize = 0b0100111001110110;
    let inst_data = InstructionData::ConditionAm(Condition::VS, AdrMode::Implied(7));
    let inst = Operation::new(
        opcode as u16,
        "TRAPV",
        InstructionType::TRAPcc,
        inst_data,
        Size::Byte,
        false,
        38,
    );
    table[opcode] = inst;
}

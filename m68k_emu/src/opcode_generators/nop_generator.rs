use crate::{decoder::{Operation, InstructionData, InstructionType}, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let opcode = 0b0100111001110001;
    let inst = Operation::new(
        opcode as u16,
        "NOP",
        InstructionType::NOP,
        InstructionData::None,
        Size::Byte,
        false,
        4,
    );
    table[opcode] = inst;
}

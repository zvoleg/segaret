use crate::{decoder::{Operation, InstructionType, InstructionData}, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let opcode = 0b0100111001110111;
    let instruction = Operation::new(
        opcode as u16,
        "RTR",
        InstructionType::RTR,
        InstructionData::None,
        Size::Byte,
        false,
        20,
    );
    table[opcode] = instruction;
}

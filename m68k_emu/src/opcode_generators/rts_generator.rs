use crate::{decoder::{Operation, InstructionType, InstructionData}, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let opcode = 0b0100111001110101;
    let instruction = Operation::new(
        opcode as u16,
        "RTS",
        InstructionType::RTS,
        InstructionData::None,
        Size::Byte,
        false,
        16,
    );
    table[opcode] = instruction;
}

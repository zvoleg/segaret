use crate::{decoder::{Operation, InstructionType, InstructionData}, Size};

pub(crate) fn generate(table: &mut[Operation]) {
    let inst = Operation::new( 
        0x003C,
        "ILLEGAL",
        InstructionType::ILLEGAL,
        InstructionData::None,
        Size::Byte,
        false,
        34,
    );
    table[0x003C] = inst;
}

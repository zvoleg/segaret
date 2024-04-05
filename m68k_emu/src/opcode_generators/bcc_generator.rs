use crate::{decoder::{Operation, Condition, InstructionData, InstructionType}, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0110000000000000;

    let condition_set = vec![
        Condition::TRUE,
        Condition::FALSE,
        Condition::HI,
        Condition::LS,
        Condition::CC,
        Condition::CS,
        Condition::NE,
        Condition::EQ,
        Condition::VC,
        Condition::VS,
        Condition::PL,
        Condition::MI,
        Condition::GE,
        Condition::LT,
        Condition::GT,
        Condition::LE,
    ];

    for condition in condition_set {
        for displasement in 0..=0xFF {
            let mask = usize::from(condition) << 8 | displasement;
            let opcode = base_mask | mask;
            let inst_data = InstructionData::ConditionDisp(condition, displasement as u32);
            let inst = Operation::new(
                opcode as u16,
                inst_name(condition),
                InstructionType::Bcc,
                inst_data,
                Size::Byte,
                false,
                10,
            );
            table[opcode] = inst;
        }
    }
}

fn inst_name(condition: Condition) -> &'static str {
    match condition {
        Condition::TRUE => "BTRUE",
        Condition::FALSE => "BFALSE",
        Condition::HI => "BHI",
        Condition::LS => "BLS",
        Condition::CC => "BCC",
        Condition::CS => "BCS",
        Condition::NE => "BNE",
        Condition::EQ => "BEQ",
        Condition::VC => "BVC",
        Condition::VS => "BVS",
        Condition::PL => "BPL",
        Condition::MI => "BMI",
        Condition::GE => "BGE",
        Condition::LT => "BLT",
        Condition::GT => "BGT",
        Condition::LE => "BLE",
    }
}

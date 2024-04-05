use crate::{decoder::{Operation, Condition, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0101000011001000;

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
        for reg in 0..8 {
            let mask = usize::from(condition) << 8 | reg;
            let opcode = base_mask | mask;
            let inst_data = InstructionData::ConditionRegAm(condition, AdrMode::DataReg(reg), AdrMode::Immediate);
            let inst = Operation::new(
                opcode as u16,
                inst_name(condition),
                InstructionType::DBcc,
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
        Condition::TRUE => "DBTRUE",
        Condition::FALSE => "DBFALSE",
        Condition::HI => "DBHI",
        Condition::LS => "DBLS",
        Condition::CC => "DBCC",
        Condition::CS => "DBCS",
        Condition::NE => "DBNE",
        Condition::EQ => "DBEQ",
        Condition::VC => "DBVC",
        Condition::VS => "DBVS",
        Condition::PL => "DBPL",
        Condition::MI => "DBMI",
        Condition::GE => "DBGE",
        Condition::LT => "DBLT",
        Condition::GT => "DBGT",
        Condition::LE => "DBLE",
    }
}

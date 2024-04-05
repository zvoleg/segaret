use crate::{decoder::{Operation, Condition, InstructionData, InstructionType}, Size, adr_mode, addressing_mode::AdrMode};

pub(crate) fn generate(table: &mut [Operation]) {
    let base_mask = 0b0101000011000000;

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

    let am_types = [
    AddressingModeType::DataRegister,
    AddressingModeType::AddressRegisterIndirect,
    AddressingModeType::AddressRegisterPostIncrement,
    AddressingModeType::AddressRegisterPreDecrement,
    AddressingModeType::AddressRegisterDisplacement,
    AddressingModeType::AddressRegisterIndexed,
    AddressingModeType::AbsShort,
    AddressingModeType::AbsLong,

    for condition in condition_set {
        for am_type in am_types {
            let mask = usize::from(condition) << 8 | usize::from(am);
            let opcode = base_mask | mask;
            let inst_data = InstructionData::ConditionAm(condition, *am);
            let clocks = match am {
                AddressingModeType::DataRegister => 4,
                _ => 8 + am_type.additional_clocks(Size::Byte),
            };
            let inst = Operation::new(
                opcode as u16,
                inst_name(condition),
                InstructionType::Scc,
                inst_data,
                Size::Byte,
                false,
                clocks,
            );
            table[opcode] = inst;
        }
    }
}

fn inst_name(condition: Condition) -> &'static str {
    match condition {
        Condition::TRUE => "STRUE",
        Condition::FALSE => "SFALSE",
        Condition::HI => "SHI",
        Condition::LS => "SLS",
        Condition::CC => "SCC",
        Condition::CS => "SCS",
        Condition::NE => "SNE",
        Condition::EQ => "SEQ",
        Condition::VC => "SVC",
        Condition::VS => "SVS",
        Condition::PL => "SPL",
        Condition::MI => "SMI",
        Condition::GE => "SGE",
        Condition::LT => "SLT",
        Condition::GT => "SGT",
        Condition::LE => "SLE",
    }
}

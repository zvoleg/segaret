use std::fmt;

pub mod mc68k_emu;
mod vector_table;
mod instruction_set;

mod addressing_mode;

#[derive(Clone)]
pub(in crate::hardware) enum Condition {
    True,
    False,
    Higher,
    LowerOrSame,
    CarryClear,
    CarrySet,
    NotEqual,
    Equal,
    OverflowClear,
    OverflowSet,
    Plus,
    Minus,
    GreaterOrEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let condition = match self {
            Condition::True => "T",
            Condition::False => "F",
            Condition::Higher => "HI",
            Condition::LowerOrSame => "LS",
            Condition::CarryClear => "CC",
            Condition::CarrySet => "CS",
            Condition::NotEqual => "NE",
            Condition::Equal => "EQ",
            Condition::OverflowClear => "VC",
            Condition::OverflowSet => "VS",
            Condition::Plus => "PL",
            Condition::Minus => "MI",
            Condition::GreaterOrEqual => "GE",
            Condition::LessThan => "LT",
            Condition::GreaterThan => "GT",
            Condition::LessOrEqual => "LE",
        };
        write!(f, "{}", condition)
    }
}

#[derive(Copy, Clone)]
pub(in crate::hardware) enum RegisterType {
    Address,
    Data,
}

#[derive(Copy, Clone)]
pub(in crate::hardware) struct Register {
    pub(in crate::hardware) reg_type: RegisterType,
    pub(in crate::hardware) reg_idx: usize,
}

impl Register {
    pub(in crate::hardware) fn new(reg_type: RegisterType, reg_idx: usize) -> Self {
        Register { reg_type, reg_idx }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg_type_char = match self.reg_type {
            RegisterType::Address => 'A',
            RegisterType::Data => 'D',
        };
        write!(f, "{}{}", reg_type_char, self.reg_idx)
    }
}
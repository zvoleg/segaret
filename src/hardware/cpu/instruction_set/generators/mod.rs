use crate::hardware::cpu::Condition;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::RegisterType;

pub(in crate::hardware) mod addr_mode_generator;
pub(in crate::hardware) mod addr_mode_ext_word_generator;
pub(in crate::hardware) mod addr_mode_immediate_generator;
pub(in crate::hardware) mod addr_mode_data_generator;
pub(in crate::hardware) mod move_generator;
pub(in crate::hardware) mod rx_addr_mode_generator;
pub(in crate::hardware) mod rx_data_generator;
pub(in crate::hardware) mod rx_ry_generator;
pub(in crate::hardware) mod rx_ry_spec_addr_mode_generator;
pub(in crate::hardware) mod ry_generator;
pub(in crate::hardware) mod ry_ext_word_generator;
pub(in crate::hardware) mod condition_displ_generator;

fn register_type_by_char(c: char) -> RegisterType {
    match c {
        'd' => RegisterType::Data,
        'a' => RegisterType::Address,
        _ => panic!("register_type_by_char: unexpected alias for register type (char: {})", c)
    }
}

pub fn addr_mode_type_by_char(c: char) -> AddrModeType {
    match c {
        'D' => AddrModeType::Data,
        'A' => AddrModeType::Addr,
        'a' => AddrModeType::AddrInd,
        '+' => AddrModeType::AddrIndPostInc,
        '-' => AddrModeType::AddrIndPreDec,
        'd' => AddrModeType::AddrIndDips,
        'x' => AddrModeType::AddrIndIdx,
        'P' => AddrModeType::PcDisp,
        'X' => AddrModeType::PcIdx,
        'W' => AddrModeType::AbsShort,
        'L' => AddrModeType::AbsLong,
        'i' => AddrModeType::Immediate,
        _ => panic!("addr_mode_type_by_char: unexpected char allias for addr mode (char = '{}')", c),
    }
}


fn condition_by_bits(bits: u32) -> Condition {
    match bits {
        0b0000 => Condition::True,
        0b0001 => Condition::False,
        0b0010 => Condition::Higher,
        0b0011 => Condition::LowerOrSame,
        0b0100 => Condition::CarryClear,
        0b0101 => Condition::CarrySet,
        0b0110 => Condition::NotEqual,
        0b0111 => Condition::Equal,
        0b1000 => Condition::OverflowClear,
        0b1001 => Condition::OverflowSet,
        0b1010 => Condition::Plus,
        0b1011 => Condition::Minus,
        0b1100 => Condition::GreaterOrEqual,
        0b1101 => Condition::LessThan,
        0b1110 => Condition::GreaterThan,
        0b1111 => Condition::LessOrEqual,
        _ => panic!("Undefined bits for condition"),
    }
}
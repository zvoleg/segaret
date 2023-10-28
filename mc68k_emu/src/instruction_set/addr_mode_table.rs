use lazy_static::lazy_static;

use crate::addressing_mode::{AddrModeType, AddrMode};

lazy_static! {
    pub(in crate) static ref DATA: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::Data, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::Addr, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR_IND: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrInd, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR_IND_POST_INC: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndPostInc, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR_IND_PRE_DECR: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndPreDec, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR_IND_DISPL: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndDips, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ADDR_IND_IDX : Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndIdx, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref PC_DISPL : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::PcDisp, 0b010))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref PC_IDX : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::PcIdx, 0b011))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ABS_SHORT : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::AbsShort, 0b000))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref ABS_LONG: Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::AbsLong, 0b001))
        .collect::<Vec<AddrMode>>();
    pub(in crate) static ref IMMEDIATE: Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::Immediate, 0b100))
        .collect::<Vec<AddrMode>>();
}

pub(in crate) fn get_addr_mode_table(addr_mode_type: AddrModeType) -> &'static [AddrMode] {
    match addr_mode_type {
        AddrModeType::Data => &DATA,
        AddrModeType::Addr => &ADDR,
        AddrModeType::AddrInd => &ADDR_IND,
        AddrModeType::AddrIndPostInc => &ADDR_IND_POST_INC,
        AddrModeType::AddrIndPreDec => &ADDR_IND_PRE_DECR,
        AddrModeType::AddrIndDips => &ADDR_IND_DISPL,
        AddrModeType::AddrIndIdx => &ADDR_IND_IDX,
        AddrModeType::PcDisp => &PC_DISPL,
        AddrModeType::PcIdx => &PC_IDX,
        AddrModeType::AbsShort => &ABS_SHORT,
        AddrModeType::AbsLong => &ABS_LONG,
        AddrModeType::Immediate => &IMMEDIATE,
    }
}

pub(in crate) fn get_am_bits(addr_mode_type: AddrModeType) -> u16 {
    match addr_mode_type {
        AddrModeType::Data => 0b000,
        AddrModeType::Addr => 0b001,
        AddrModeType::AddrInd => 0b010,
        AddrModeType::AddrIndPostInc => 0b011,
        AddrModeType::AddrIndPreDec => 0b100,
        AddrModeType::AddrIndDips => 0b101,
        AddrModeType::AddrIndIdx => 0b110,
        AddrModeType::PcDisp => 0b111,
        AddrModeType::PcIdx => 0b111,
        AddrModeType::AbsShort => 0b111,
        AddrModeType::AbsLong => 0b111,
        AddrModeType::Immediate => 0b111,
    }    
}
use lazy_static::lazy_static;

use crate::hardware::cpu::addressing_mode::{AddrModeType, AddrMode};

lazy_static! {
    pub(in crate::hardware) static ref DATA: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::Data, 0b000, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::Addr, 0b001, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR_IND: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrInd, 0b010, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR_IND_POST_INC: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndPostInc, 0b011, i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR_IND_PRE_DECR: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndPreDec , 0b100 , i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR_IND_DISPL: Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndDips , 0b101 , i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ADDR_IND_IDX : Vec<AddrMode> = (0..8)
        .map(|i| AddrMode::new(AddrModeType::AddrIndIdx , 0b110 , i))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref PC_DISPL : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::PcDisp , 0b111 , 0b010))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref PC_IDX : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::PcIdx , 0b111 , 0b011))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ABS_SHORT : Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::AbsShort , 0b111 , 0b000))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref ABS_LONG: Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::AbsLong , 0b111 , 0b001))
        .collect::<Vec<AddrMode>>();
    pub(in crate::hardware) static ref IMMEDIATE: Vec<AddrMode> = (0..1)
        .map(|_| AddrMode::new(AddrModeType::Immediate , 0b111 , 0b100))
        .collect::<Vec<AddrMode>>();
}

pub(in crate::hardware) fn get_addr_mode_table(addr_mode_type: AddrModeType) -> Vec<AddrMode> {
    match addr_mode_type {
        AddrModeType::Data => (*DATA).clone(),
        AddrModeType::Addr => (*ADDR).clone(),
        AddrModeType::AddrInd => (*ADDR_IND).clone(),
        AddrModeType::AddrIndPostInc => (*ADDR_IND_POST_INC).clone(),
        AddrModeType::AddrIndPreDec => (*ADDR_IND_PRE_DECR).clone(),
        AddrModeType::AddrIndDips => (*ADDR_IND_DISPL).clone(),
        AddrModeType::AddrIndIdx => (*ADDR_IND_IDX).clone(),
        AddrModeType::PcDisp => (*PC_DISPL).clone(),
        AddrModeType::PcIdx => (*PC_IDX).clone(),
        AddrModeType::AbsShort => (*ABS_SHORT).clone(),
        AddrModeType::AbsLong => (*ABS_LONG).clone(),
        AddrModeType::Immediate => (*IMMEDIATE).clone(),
    }
}
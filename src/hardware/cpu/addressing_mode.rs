use crate::hardware::{Location, LocationType};
use crate::Mc68k;
use std::fmt;

use crate::hardware::Size;

use super::{RegisterType, Register};

#[derive(Copy, Clone, PartialEq)]
pub enum AddrModeType {
    Data,
    Addr,
    AddrInd,
    AddrIndPostInc,
    AddrIndPreDec,
    AddrIndDips,
    AddrIndIdx,
    PcDisp,
    PcIdx,
    AbsShort,
    AbsLong,
    Immediate,
}

#[derive(Copy, Clone)]
pub(in crate::hardware) struct BriefExtWord {
    pub register: Register,
    pub size: Size,
    pub displacement: u32,
}

impl BriefExtWord {
    pub fn new(ext_word: u32) -> Self {
        let reg_type = if ext_word & 0x8000 != 0 {
            RegisterType::Address
        } else {
            RegisterType::Data
        };
        let reg_idx = (ext_word >> 11) & 0x07;
        let size = if ext_word & 0x0800 != 0 {
            Size::Long
        } else {
            Size::Word
        };
        let displacement = ext_word & 0xFF;
        Self {
            register: Register::new(reg_type, reg_idx as usize),
            size: size,
            displacement: displacement,
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::hardware) struct AddrMode {
    pub(in crate::hardware) am_type: AddrModeType,
    pub(in crate::hardware) mode_bits: usize, 
    pub(in crate::hardware) reg_idx: usize,
    pub(in crate::hardware) ext_word: Option<u32>,
    pub(in crate::hardware) brief_ext_word: Option<BriefExtWord>
}

impl AddrMode {
    pub(in crate::hardware) fn new(am_type: AddrModeType, mode_bits: usize, reg_idx: usize) -> Self {
        Self {
            am_type: am_type,
            mode_bits: mode_bits,
            reg_idx: reg_idx,
            ext_word: None,
            brief_ext_word: None,
        }
    }

    pub(in crate::hardware) fn fetch_ext_word(&mut self, cpu: &mut Mc68k) {
        match self.am_type {
            AddrModeType::AddrIndDips => {
                let location = Location::new(LocationType::Memory, cpu.pc as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                self.ext_word = Some(data); 
            },
            AddrModeType::AddrIndIdx => {
                let location = Location::new(LocationType::Memory, cpu.pc as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                let brief_ext_word = BriefExtWord::new(data);
                self.brief_ext_word = Some(brief_ext_word);
            },
            AddrModeType::PcDisp => {
                let location = Location::new(LocationType::Memory, (cpu.pc) as usize);
                let data = cpu.read(location, Size::Word);

                self.ext_word = Some(data);
            },
            AddrModeType::PcIdx => {
                let location = Location::new(LocationType::Memory, (cpu.pc) as usize);
                let data = cpu.read(location, Size::Word);

                let brief_ext_word = BriefExtWord::new(data);
                self.brief_ext_word = Some(brief_ext_word);
            }
            AddrModeType::AbsShort => {
                let location = Location::new(LocationType::Memory, cpu.pc as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();

                self.ext_word = Some(data); 
            },
            AddrModeType::AbsLong => {
                let location = Location::new(LocationType::Memory, cpu.pc as usize);
                let data = cpu.read(location, Size::Long);
                cpu.increment_pc();
                cpu.increment_pc();

                self.ext_word = Some(data); 
            }
            _ => (),
        }
    }
}

impl fmt::Display for AddrMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let disassembly = match self.am_type {
            AddrModeType::Data => format!("D{}", self.reg_idx),
            AddrModeType::Addr => format!("A{}", self.reg_idx),
            AddrModeType::AddrInd => format!("(A{})", self.reg_idx),
            AddrModeType::AddrIndPostInc => format!("(A{})+", self.reg_idx),
            AddrModeType::AddrIndPreDec => format!("-(A{})", self.reg_idx),
            AddrModeType::AddrIndDips => format!("({}, A{})", self.ext_word.unwrap(), self.reg_idx),
            AddrModeType::AddrIndIdx => {
                let displacement = self.brief_ext_word.unwrap().displacement;
                let register = self.brief_ext_word.unwrap().register;
                let size = self.brief_ext_word.unwrap().size;
                format!("({}, A{}, {}, {})", displacement, self.reg_idx, register, size)
            },
            AddrModeType::PcDisp => format!("({},PC)", self.ext_word.unwrap()),
            AddrModeType::PcIdx => {
                let displacement = self.brief_ext_word.unwrap().displacement;
                let register = self.brief_ext_word.unwrap().register;
                let size = self.brief_ext_word.unwrap().size;
                format!("({}, PC, {}, {})", displacement, register, size)
            },
            AddrModeType::AbsShort => format!("{:04X}", self.ext_word.unwrap()),
            AddrModeType::AbsLong => format!("{:08X}", self.ext_word.unwrap()),
            AddrModeType::Immediate => format!(""),
        };
        write!(f, "{}", disassembly)
    }
}
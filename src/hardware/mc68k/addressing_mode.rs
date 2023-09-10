use crate::hardware::sign_extend;
use crate::hardware::Location;
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

impl AddrModeType {
    pub(in crate::hardware) fn get_clock_periods_short(&self) -> u32 {
        match self {
            AddrModeType::Data => 0,
            AddrModeType::Addr => 0,
            AddrModeType::AddrInd => 4,
            AddrModeType::AddrIndPostInc => 4,
            AddrModeType::AddrIndPreDec => 6,
            AddrModeType::AddrIndDips => 8,
            AddrModeType::AddrIndIdx => 10,
            AddrModeType::PcDisp => 8,
            AddrModeType::PcIdx => 10,
            AddrModeType::AbsShort => 8,
            AddrModeType::AbsLong => 12,
            AddrModeType::Immediate => 4,
        }
    }

    pub(in crate::hardware) fn get_clock_periods_long(&self) -> u32 {
        match self {
            AddrModeType::Data => 0,
            AddrModeType::Addr => 0,
            AddrModeType::AddrInd => 8,
            AddrModeType::AddrIndPostInc => 8,
            AddrModeType::AddrIndPreDec => 10,
            AddrModeType::AddrIndDips => 12,
            AddrModeType::AddrIndIdx => 14,
            AddrModeType::PcDisp => 12,
            AddrModeType::PcIdx => 14,
            AddrModeType::AbsShort => 12,
            AddrModeType::AbsLong => 16,
            AddrModeType::Immediate => 8,
        }
    }
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
        let mut displacement = ext_word & 0xFF;
        displacement = sign_extend(displacement, Size::Byte);
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
    pub(in crate::hardware) reg_idx: usize,
    pub(in crate::hardware) ext_word: Option<u32>,
    pub(in crate::hardware) brief_ext_word: Option<BriefExtWord>,
    pub(in crate::hardware) ext_word_addr: u32,
}

impl AddrMode {
    pub(in crate::hardware) fn new(am_type: AddrModeType, reg_idx: usize) -> Self {
        Self {
            am_type: am_type,
            reg_idx: reg_idx,
            ext_word: None,
            brief_ext_word: None,
            ext_word_addr: 0,
        }
    }

    pub(in crate::hardware) fn fetch_ext_word(&mut self, cpu: &mut Mc68k, size: Size) {
        match self.am_type {
            AddrModeType::AddrIndDips => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory(cpu.pc as usize);
                let mut data = cpu.read(location, Size::Word);
                data = sign_extend(data, Size::Word);
                cpu.increment_pc();
                self.ext_word = Some(data); 
            },
            AddrModeType::AddrIndIdx => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory(cpu.pc as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                let brief_ext_word = BriefExtWord::new(data);
                self.brief_ext_word = Some(brief_ext_word);
            },
            AddrModeType::PcDisp => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory((cpu.pc) as usize);
                let mut data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                data = sign_extend(data, Size::Word);
                self.ext_word = Some(data);
            },
            AddrModeType::PcIdx => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory((cpu.pc) as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                let brief_ext_word = BriefExtWord::new(data);
                self.brief_ext_word = Some(brief_ext_word);
            }
            AddrModeType::AbsShort => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory(cpu.pc as usize);
                let data = cpu.read(location, Size::Word);
                cpu.increment_pc();
                self.ext_word = Some(data); 
            },
            AddrModeType::AbsLong => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory(cpu.pc as usize);
                let data = cpu.read(location, Size::Long);
                cpu.increment_pc();
                cpu.increment_pc();

                self.ext_word = Some(data); 
            },
            AddrModeType::Immediate => {
                self.ext_word_addr = cpu.pc;
                let location = Location::memory(cpu.pc as usize);
                match size {
                    Size::Byte => {
                        let data = cpu.read(location, Size::Word) & 0xFF;
                        self.ext_word = Some(data);
                        cpu.increment_pc();
                    }
                    Size::Word => {
                        self.ext_word = Some(cpu.read(location, size));
                        cpu.increment_pc();
                    }
                    Size::Long => {
                        self.ext_word = Some(cpu.read(location, size));
                        (0..2).for_each(|_| cpu.increment_pc());
                    }
                };
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
            AddrModeType::AddrIndDips => format!("(0x{:04X}, A{})", self.ext_word.unwrap(), self.reg_idx),
            AddrModeType::AddrIndIdx => {
                let displacement = self.brief_ext_word.unwrap().displacement;
                let register = self.brief_ext_word.unwrap().register;
                let size = self.brief_ext_word.unwrap().size;
                format!("(0x{:02X}, A{}, {}, {})", displacement, self.reg_idx, register, size)
            },
            AddrModeType::PcDisp => format!("(0x{:04X},PC) -> [0x{:08X}]", self.ext_word.unwrap(), (self.ext_word.unwrap().wrapping_add(self.ext_word_addr))),
            AddrModeType::PcIdx => {
                let displacement = self.brief_ext_word.unwrap().displacement;
                let register = self.brief_ext_word.unwrap().register;
                let size = self.brief_ext_word.unwrap().size;
                format!("(0x{:02X}, PC, {}, {}) -> [0x{:08X} + {}]", displacement, register, size, (displacement.wrapping_add(self.ext_word_addr)), register)
            },
            AddrModeType::AbsShort => format!("0x{:04X}", self.ext_word.unwrap()),
            AddrModeType::AbsLong => format!("0x{:08X}", self.ext_word.unwrap()),
            AddrModeType::Immediate => format!("#0x{:X}", self.ext_word.unwrap()),
        };
        write!(f, "{}", disassembly)
    }
}
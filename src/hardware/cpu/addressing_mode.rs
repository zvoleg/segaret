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

impl AddrModeType {
    fn get_mode_bits(&self) -> usize {
        match self {
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
    pub(in crate::hardware) fn new(am_type: AddrModeType, reg_idx: usize) -> Self {
        Self {
            am_type: am_type,
            mode_bits: am_type.get_mode_bits(),
            reg_idx: reg_idx,
            ext_word: None,
            brief_ext_word: None,
        }
    }

    pub(in crate::hardware) fn fetch_ext_word(&mut self, cpu: &mut Mc68k, size: Size) {
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
            },
            AddrModeType::Immediate => {
                let location = Location::memory(cpu.pc as usize);
                self.ext_word = Some(cpu.read(location, size));
                match size {
                    Size::Byte => self.ext_word = Some((cpu.instruction.operation_word() & 0xFF) as u32),
                    Size::Word => cpu.increment_pc(),
                    Size::Long => (0..2).for_each(|_| cpu.increment_pc()),
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
            AddrModeType::Immediate => format!("#{:08}", self.ext_word.unwrap()),
        };
        write!(f, "{}", disassembly)
    }
}
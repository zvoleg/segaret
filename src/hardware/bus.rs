use crate::{Mc68k, Vdp};
use crate::hardware::Size;
use crate::hardware::cartridge::cartridge::Cartridge;

use super::mc68k::Mc68kBus;
use super::z80::Z80Bus;
use super::z80::z80_emu::Z80Emu;

pub struct Bus {
    cartridge: Cartridge,
    ram: Vec<u8>,
    z80_ram: Vec<u8>,

    vdp: *mut Vdp,
    mc68k_cpu: *mut Mc68k,
    z80_cpu: *mut Z80Emu,
}

impl Bus {
    pub fn init(cartridge: Cartridge, vdp: *mut Vdp) -> Self {
        Self {
            cartridge: cartridge,
            ram: vec![0; 0x10000], // $FF0000	$FFFFFF
            z80_ram: vec![0; 0x10000], // $A00000	$A0FFFF

            
            vdp: vdp,
            mc68k_cpu: std::ptr::null_mut(),
            z80_cpu: std::ptr::null_mut(),
        }
    }

    pub(crate) fn set_mc68k_cpu(&mut self, cpu: *mut Mc68k) {
        self.mc68k_cpu = cpu;
    }

    pub(crate) fn set_z80_cpu(&mut self, cpu: *mut Z80Emu) {
        self.z80_cpu = cpu;
    }

    pub(crate) fn get_rom_ptr(&self) -> *const u8 {
        self.cartridge.rom.as_ptr()
    }

    pub fn send_interrupt(&self, interrupt_level: usize) {
        unsafe {
            (*self.mc68k_cpu).interrupt(interrupt_level);
        }
    }

    pub fn z80_dump(&self) -> &[u8] {
        &self.z80_ram   
    }
}

impl Mc68kBus for Bus {
    fn read(&self, address: usize, size: Size) -> u32 {
        if address <= 0x3FFFFF {
            self.cartridge.read(address, size)
        } else if address == 0xA10001 {
            0x8F
        } else if address == 0xC00000 || address == 0xC00002 {
            unsafe {
                (*self.vdp).read_data_port() as u32
            }
        } else if address == 0xC00004 || address == 0xC00006 {
            unsafe {
                (*self.vdp).read_control_port() as u32
            }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            match size {
                Size::Byte => self.ram[address] as u32,
                Size::Word => unsafe {
                    let mut ram_ptr = self.ram.as_ptr();
                    ram_ptr = ram_ptr.offset(address as isize);
                    let data = ram_ptr as *const _ as *const u16;
                    let data = (*data).to_be();
                    data as u32
                },
                Size::Long => unsafe {
                    let mut ram_ptr = self.ram.as_ptr();
                    ram_ptr = ram_ptr.offset(address as isize);
                    let data = ram_ptr as *const _ as *const u32;
                    let data = (*data).to_be();
                    data
                },
            }
        } else {
            0
        }
    }

    fn write(&mut self, address: usize, data: u32, size: Size) {
        if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            self.z80_ram[address] = data as u8;
        } else if address == 0xA11200 {
            if data == 0x100 {
                unsafe {
                    (*self.z80_cpu).reset();
                }
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            unsafe {
                (*self.vdp).write_data_port(data as u16);
            }
        } else if address == 0xC00004 || address == 0xC00006 {
            unsafe {
                (*self.vdp).write_control_port(data as u16);
            }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            match size {
                Size::Byte => self.ram[address] = data as u8,
                Size::Word => unsafe {
                    let mut ram_ptr = self.ram.as_mut_ptr();
                    ram_ptr = ram_ptr.offset(address as isize);
                    let ram_ptr_casted = ram_ptr as *mut _ as *mut u16;
                    *ram_ptr_casted = (data as u16).to_be();
                },
                Size::Long => unsafe {
                    let mut ram_ptr = self.ram.as_mut_ptr();
                    ram_ptr = ram_ptr.offset(address as isize);
                    let ram_ptr_casted = ram_ptr as *mut _ as *mut u32;
                    *ram_ptr_casted = data.to_be();
                },
            }
        }
    }
}

impl Z80Bus for Bus {
    fn read(&self, address: u16, size: Size) -> u16 {
        let address = (address & 0xFFFF) as usize;
        match size {
            Size::Byte => self.z80_ram[address] as u16,
            Size::Word => unsafe {
                let mut ram_ptr = self.z80_ram.as_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let data = ram_ptr as *const _ as *const u16;
                let data = *data;
                data as u16
            },
            Size::Long => panic!("Z80Bus::Bus::read: unsuported size"),
        }
    }

    fn write(&mut self, address: u16, data: u16, size: Size) {
        let address = (address & 0xFFFF) as usize;
        match size {
            Size::Byte => self.z80_ram[address] = data as u8,
            Size::Word => unsafe {
                let mut ram_ptr = self.z80_ram.as_mut_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let ram_ptr_casted = ram_ptr as *mut _ as *mut u16;
                *ram_ptr_casted = data;
            },
            Size::Long => panic!("Z80Bus::Bus::write: unsuported size"),
        }
    }
}
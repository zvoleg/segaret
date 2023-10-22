use crate::hardware::Size;
use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub(crate) rom: Vec<u8>,
}

impl Cartridge {
    pub fn init(rom_file: &str) -> Self {
        let mut file = File::open(rom_file).unwrap();
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);

        Self {
            rom: buffer,
        }
    }

    pub fn read(&self, address: usize, size: Size) -> u32 {
        match size {
            Size::Byte => self.rom[address] as u32,
            Size::Word => unsafe {
                let mut ram_ptr = self.rom.as_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let data = ram_ptr as *const _ as *const u16;
                let data = (*data).to_be();
                data as u32
            },
            Size::Long => unsafe {
                let mut ram_ptr = self.rom.as_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let data = ram_ptr as *const _ as *const u32;
                let data = (*data).to_be();
                data
            },
        }
    }

    pub fn write(&mut self, address: usize, data: u32, size: Size) {
        match size {
            Size::Byte => {
                self.rom[address] = data as u8;
            }
            Size::Word => unsafe {
                let mut ram_ptr = self.rom.as_mut_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let ram_ptr = ram_ptr as *mut _ as *mut u16;
                *ram_ptr = (data as u16).to_le();
            },
            Size::Long => unsafe {
                let mut ram_ptr = self.rom.as_mut_ptr();
                ram_ptr = ram_ptr.offset(address as isize);
                let ram_ptr = ram_ptr as *mut _ as *mut u32;
                *ram_ptr = (data as u32).to_le();
            },
        }
    }
}
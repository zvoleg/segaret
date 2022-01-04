use crate::hardware::Size;
use crate::hardware::cartridge::cartridge::Cartridge;

pub struct Bus {
    cartridge: Cartridge,
    ram: Vec<u8>,
}

impl Bus {
    pub fn init(cartridge: Cartridge) -> Self {
        Self {
            cartridge: cartridge,
            ram: vec![0; 0x1000000],
        }
    }

    pub(crate) fn get_rom_ptr(&self) -> *const u8 {
        self.cartridge.rom.as_ptr()
    }

    pub fn read(&self, address: usize, size: Size) -> u32 {
        if address <= 0x3FFFFF {
            self.cartridge.read(address, size)
        } else {
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
        }
    }

    pub fn write(&mut self, address: usize, data: u32, size: Size) {
        
    }
}


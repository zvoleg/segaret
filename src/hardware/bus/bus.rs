use crate::Vdp;
use crate::hardware::Size;
use crate::hardware::cartridge::cartridge::Cartridge;

pub struct Bus {
    cartridge: Cartridge,
    vdp: *mut Vdp,
    ram: Vec<u8>,
}

impl Bus {
    pub fn init(cartridge: Cartridge, vdp: *mut Vdp) -> Self {
        Self {
            cartridge: cartridge,
            vdp: vdp,
            // z80_ram: vec[0; 0x10000], $A00000	$A0FFFF
            ram: vec![0; 0x10000], // $FF0000	$FFFFFF
        }
    }

    pub(crate) fn get_rom_ptr(&self) -> *const u8 {
        self.cartridge.rom.as_ptr()
    }

    pub fn read(&self, address: usize, size: Size) -> u32 {
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

    pub fn write(&mut self, address: usize, data: u32, size: Size) {
        if address == 0xC00000 || address == 0xC00002 {
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


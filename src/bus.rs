use m68k_emu::bus::BusM68k;

pub struct Bus {
    rom: Vec<u8>,
    ram: Vec<u8>,
    z80_ram: Vec<u8>,
    controllers: [u8; 4],
}

impl Bus {
    pub fn init(rom: Vec<u8>) -> Self {
        Self {
            rom: rom,
            ram: vec![0; 0x10000],     // $FF0000	$FFFFFF
            z80_ram: vec![0; 0x10000], // $A00000	$A0FFFF
            controllers: [0; 4],
        }
    }

    pub(crate) fn get_rom_ptr(&self) -> *const u8 {
        self.rom.as_ptr()
    }

    pub fn z80_dump(&self) -> &[u8] {
        &self.z80_ram
    }
}

impl BusM68k for Bus {
    fn set_address(&self, address: u32) -> *mut u8 {
        if address <= 0x3FFFFF {
            unsafe {
                let rom_ptr = self.rom.as_ptr().offset(address as isize);
                rom_ptr as *const _ as *mut u8
            }
        } else if address == 0xA10001 {
            std::ptr::null_mut()
        // } else if address == 0xC00000 || address == 0xC00002 {
        //     // unsafe {
        //     //     (*self.vdp).read_data_port() as u32
        //     // }
        // } else if address == 0xC00004 || address == 0xC00006 {
        //     // unsafe {
        //     //     (*self.vdp).read_control_port() as u32
        //     // }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            unsafe {
                let ram_ptr = self.ram.as_ptr().offset(address as isize);
                ram_ptr as *const _ as *mut u8
            }
        } else {
            &self.controllers[0] as *const u8 as *const _ as *mut u8
        }
    }
}

// impl Z80Bus for Bus {
//     fn read(&self, address: u16, size: Size) -> u16 {
//         let address = (address & 0xFFFF) as usize;
//         match size {
//             Size::Byte => self.z80_ram[address] as u16,
//             Size::Word => unsafe {
//                 let mut ram_ptr = self.z80_ram.as_ptr();
//                 ram_ptr = ram_ptr.offset(address as isize);
//                 let data = ram_ptr as *const _ as *const u16;
//                 let data = *data;
//                 data as u16
//             },
//             Size::Long => panic!("Z80Bus::Bus::read: unsuported size"),
//         }
//     }

//     fn write(&mut self, address: u16, data: u16, size: Size) {
//         let address = (address & 0xFFFF) as usize;
//         match size {
//             Size::Byte => self.z80_ram[address] = data as u8,
//             Size::Word => unsafe {
//                 let mut ram_ptr = self.z80_ram.as_mut_ptr();
//                 ram_ptr = ram_ptr.offset(address as isize);
//                 let ram_ptr_casted = ram_ptr as *mut _ as *mut u16;
//                 *ram_ptr_casted = data;
//             },
//             Size::Long => panic!("Z80Bus::Bus::write: unsuported size"),
//         }
//     }
// }

use m68k_emu::M68kBus;
use z80_emu::Z80Bus;
use sg_vdp_emu::Vdp;

pub struct Bus {
    rom: Vec<u8>,
    ram: Vec<u8>,
    z80_ram: Vec<u8>,
}

impl Bus {
    pub fn init(rom: Vec<u8>) -> Self {
        Self {
            rom: rom,
            ram: vec![0; 0x10000], // $FF0000	$FFFFFF
            z80_ram: vec![0; 0x10000], // $A00000	$A0FFFF
        }
    }

    pub(crate) fn get_rom_ptr(&self) -> *const u8 {
        self.rom.as_ptr()
    }

    pub fn z80_dump(&self) -> &[u8] {
        &self.z80_ram   
    }
}

impl M68kBus for Bus {

    fn read(&self, address: u32, size: m68k_emu::Size) -> u32 {
        if address <= 0x3FFFFF {
            unsafe {
                let rom_ptr = self.rom.as_ptr().offset(address as isize);
                match size {
                    m68k_emu::Size::Byte => (*rom_ptr) as u32,
                    m68k_emu::Size::Word => (*(rom_ptr as *const u16)).to_le() as u32,
                    m68k_emu::Size::Long => (*(rom_ptr as *const u32)).to_le(),
                }
            }
        } else if address == 0xA10001 {
            0x8F
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
                match size {
                    m68k_emu::Size::Byte => (*ram_ptr) as u32,
                    m68k_emu::Size::Word => (*(ram_ptr as *const u16)).to_le() as u32,
                    m68k_emu::Size::Long => (*(ram_ptr as *const u32)).to_le(),
                }
            }
        } else {
            0
        }
    }

    fn write(&mut self, address: u32, data: u32, size: m68k_emu::Size) {
        if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            unsafe { 
                let z80_ram_ptr = self.z80_ram.as_mut_ptr().offset(address as isize);
                match size {
                    m68k_emu::Size::Byte => *z80_ram_ptr = data as u8,
                    m68k_emu::Size::Word => *(z80_ram_ptr as *mut u16) = data as u16,
                    m68k_emu::Size::Long => *(z80_ram_ptr as *mut u32) = data,
                }
            }
        } else if address == 0xA11200 {
            // if data == 0x100 {
            //     // unsafe {
            //     //     (*self.z80_cpu).reset();
            //     // }
            // }
            // zero.as_mut()
        } else if address == 0xC00000 || address == 0xC00002 {
            // unsafe {
            //     (*self.vdp).write_data_port(data as u16);
            // }
            // zero.as_mut()
        } else if address == 0xC00004 || address == 0xC00006 {
            // unsafe {
            //     (*self.vdp).write_control_port(data as u16);
            // }
            // zero.as_mut()
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            // unsafe { self.ram.as_mut_ptr().offset(address as isize) }
        } else {
            // zero.as_mut()
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

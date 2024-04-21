use m68k_emu::bus::BusM68k;

const VERSION_REGISTER: u32 = 0xA10000;
const CONTROLLER_A_DATA: u32 = 0xA10002;
const CONTROLLER_B_DATA: u32 = 0xA10004;
const CONTROLLER_A_CONTROL: u32 = 0xA10008;
const CONTROLLER_B_CONTROL: u32 = 0xA1000A;
const EXPANSION_PORT_CONTROL: u32 = 0xA1000C;
const Z80_REQUEST_BUS: u32 = 0xA11100;
const Z80_RESET: u32 = 0xA11200;
pub struct Bus {
    rom: Vec<u8>,
    m68k_ram: Vec<u8>,
    z80_ram: Vec<u8>,
    io_area: [u8; 0x20],
    null: u32,
}

impl Bus {
    pub fn init(rom: Vec<u8>) -> Self {
        let mut io_area = [0; 0x20];
        io_area[1] = 0x0090; // setup version register
        Self {
            rom: rom,
            z80_ram: vec![0; 0x10000],  // $A00000	$A0FFFF
            m68k_ram: vec![0; 0x10000], // $FF0000	$FFFFFF
            io_area: [0; 0x20],
            null: 0,
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
        } else if address >= 0xA10000 && address < 0xA20000 {
            let address = (address & 0x3f) as usize;
            &self.io_area[address] as *const _ as *mut u8
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
                let ram_ptr = self.m68k_ram.as_ptr().offset(address as isize);
                ram_ptr as *const _ as *mut u8
            }
        } else {
            &self.null as *const _ as *mut u8
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

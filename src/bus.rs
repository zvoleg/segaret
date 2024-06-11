use std::cell::RefCell;

use m68k_emu::bus::BusM68k;
use sg_vdp_emu::BusVdp;

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

    // TODO rewrite after implementation of the peripheral devices
    z80_bus_request_reg: RefCell<u32>,
    io_area_read: [u8; 0x20],
    io_area_m68k: [u8; 0x20],
}

impl Bus {
    pub fn init(rom: Vec<u8>) -> Self {
        let mut io_area_read = [0; 0x20];
        io_area_read[1] = 0x0090; // `setup version regi`ster
        Self {
            rom: rom,
            z80_ram: vec![0; 0x10000],  // $A00000	$A0FFFF
            m68k_ram: vec![0; 0x10000], // $FF0000	$FFFFFF
            z80_bus_request_reg: RefCell::new(0),
            io_area_read: io_area_read,
            io_area_m68k: [0; 0x20],
        }
    }

    pub fn z80_dump(&self) -> &[u8] {
        &self.z80_ram
    }

    fn read_ptr(&self, amount: u32, ptr: *const u8) -> u32 {
        unsafe {
            match amount {
                1 => *ptr as u32,
                2 => (*(ptr as *const u16)).to_be() as u32,
                4 => (*(ptr as *const u32)).to_be() as u32,
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    fn write_ptr(&self, data: u32, amount: u32, ptr: *mut u8) {
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = (data as u16).to_be(),
                4 => *(ptr as *mut _ as *mut u32) = data.to_be(),
                _ => panic!("Bus: write: wrong size"),
            }
        }
    }
}

impl BusM68k for Bus {
    fn read(&self, address: u32, amount: u32) -> u32 {
        let address = address & 0x00FFFFFF;
        if address <= 0x3FFFFF {
            self.read_ptr(amount, &self.rom[address as usize])
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            self.read_ptr(amount, &self.z80_ram[address as usize])
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == Z80_REQUEST_BUS {
                if *self.z80_bus_request_reg.borrow() == 0x0100 {
                    return 0;
                } else {
                    return 1;
                }
            }
            let address = (address & 0x3f) as usize;
            self.read_ptr(amount, &self.io_area_read[address])
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
            self.read_ptr(amount, &self.m68k_ram[address as usize])
        } else {
            let address = address & 0x1f;
            self.read_ptr(amount, &self.io_area_read[address as usize])
        }
    }

    fn write(&self, data: u32, address: u32, amount: u32) {
        let address = address & 0x00FFFFFF;
        if address <= 0x3FFFFF {
            let ptr = &self.rom[address as usize] as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        // } else if address >= 0xA00000 && address <= 0xA0FFFF {
        //     let address = address & 0xFFFF;
        //     &self.z80_ram[address as usize..(address + amount) as usize]
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == Z80_REQUEST_BUS {
                *self.z80_bus_request_reg.borrow_mut() = data;
            } else {
                let address = (address & 0x3f) as usize;
                self.write_ptr(
                    data,
                    amount,
                    &self.io_area_m68k[address] as *const _ as *mut u8,
                )
            }
        // // } else if address == 0xC00000 || address == 0xC00002 {
        // //     // unsafe {
        // //     //     (*self.vdp).read_data_port() as u32
        // //     // }
        // // } else if address == 0xC00004 || address == 0xC00006 {
        // //     // unsafe {
        // //     //     (*self.vdp).read_control_port() as u32
        // //     // }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            let ptr = &self.m68k_ram[address as usize] as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else {
            let address = address & 0x1f;
            let ptr = &self.io_area_m68k[address as usize] as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        };
    }
}

impl BusVdp for Bus {
    fn read_data_port(&self) -> u16 {
        todo!()
    }

    fn write_data_port(&self, data: u16) {
        todo!()
    }

    fn read_control_port(&self) -> u16 {
        todo!()
    }

    fn write_control_port(&self, data: u16) {
        todo!()
    }

    fn send_interrupt(&self, level: i32) {
        todo!()
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

use std::{cell::RefCell, rc::Rc};

use super::vdp_emu::vdp_port::VdpPorts;
use m68k_emu::bus::BusM68k;

use crate::memory_space::MemorySpace;

const VERSION_REGISTER: u32 = 0xA10001;
// const CONTROLLER_A_DATA: u32 = 0xA10002;
// const CONTROLLER_B_DATA: u32 = 0xA10004;
// const CONTROLLER_A_CONTROL: u32 = 0xA10008;
// const CONTROLLER_B_CONTROL: u32 = 0xA1000A;
// const EXPANSION_PORT_CONTROL: u32 = 0xA1000C;
const Z80_REQUEST_BUS: u32 = 0xA11100;
// const Z80_RESET: u32 = 0xA11200;

pub struct CpuBus<T: VdpPorts> {
    memory_space: Rc<RefCell<MemorySpace>>,
    // TODO rewrite after implementation of the peripheral devices
    vdp_ports: Option<Rc<RefCell<T>>>,
    z80_bus_request_reg: RefCell<u32>,
}

impl<T> CpuBus<T>
where
    T: VdpPorts,
{
    pub fn init(memory_space: Rc<RefCell<MemorySpace>>) -> Self {
        Self {
            memory_space: memory_space,
            vdp_ports: None,
            z80_bus_request_reg: RefCell::new(0),
        }
    }

    pub fn set_vdp_ports(&mut self, vdp_port: Rc<RefCell<T>>) {
        self.vdp_ports = Some(vdp_port);
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

impl<T> BusM68k for CpuBus<T>
where
    T: VdpPorts,
{
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()> {
        let address = address & 0x00FFFFFF;
        if address <= 0x3FFFFF {
            Ok(self.read_ptr(amount, &self.memory_space.borrow().rom[address as usize]))
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            let data = self.read_ptr(
                amount,
                &self.memory_space.borrow().z80_ram[address as usize],
            );
            Ok(data)
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == VERSION_REGISTER {
                let program_region = self.memory_space.borrow().rom[0x1F0];
                match program_region {
                    0x55 => Ok(0x80),
                    0x45 => Ok(0xC0),
                    0x4A => Ok(0x00),
                    _ => panic!("unexpected program region code {:02X}", program_region),
                }
            } else if address == Z80_REQUEST_BUS {
                if *self.z80_bus_request_reg.borrow() == 0x0100 {
                    return Ok(0);
                } else {
                    return Ok(1);
                }
            } else {
                let address = (address & 0x3f) as usize;
                Ok(self.read_ptr(amount, &self.memory_space.borrow().io_area_read[address]))
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            self.vdp_ports.as_ref().unwrap().borrow_mut().read_data_port()
        } else if address == 0xC00004 || address == 0xC00006 {
            self.vdp_ports
                .as_ref()
                .unwrap()
                .borrow()
                .read_control_port()
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            Ok(self.read_ptr(
                amount,
                &self.memory_space.borrow().m68k_ram[address as usize],
            ))
        } else {
            let address = address & 0x1f;
            Ok(self.read_ptr(
                amount,
                &self.memory_space.borrow().io_area_read[address as usize],
            ))
        }
    }

    fn write(&self, data: u32, address: u32, amount: u32) -> Result<(), ()> {
        let address = address & 0x00FFFFFF;
        if address <= 0x3FFFFF {
            let ptr = &self.memory_space.as_ref().borrow_mut().rom[address as usize] as *const _
                as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            let ptr = &self.memory_space.as_ref().borrow_mut().z80_ram[address as usize] as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == Z80_REQUEST_BUS {
                *self.z80_bus_request_reg.borrow_mut() = data;
            } else {
                let address = (address & 0x3f) as usize;
                self.write_ptr(
                    data,
                    amount,
                    &self.memory_space.as_ref().borrow_mut().io_area_m68k[address] as *const _
                        as *mut u8,
                )
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            let mut vdp_port_ref = self.vdp_ports.as_ref().unwrap().borrow_mut();
            if amount == 4 {
                vdp_port_ref.write_data_port((data >> 16) as u16)?;
                vdp_port_ref.write_data_port(data as u16)?;
            } else {
                vdp_port_ref.write_data_port(data as u16)?;
            }
        } else if address == 0xC00004 || address == 0xC00006 {
            let mut vdp_port_ref = self.vdp_ports.as_ref().unwrap().borrow_mut();
            if amount == 4 {
                vdp_port_ref.write_control_port((data >> 16) as u16)?;
                vdp_port_ref.write_control_port(data as u16)?;
            } else {
                vdp_port_ref.write_control_port(data as u16)?;
            }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            let ptr = &self.memory_space.as_ref().borrow_mut().m68k_ram[address as usize]
                as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else {
            let address = address & 0x1f;
            let ptr = &self.memory_space.as_ref().borrow_mut().io_area_m68k[address as usize]
                as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        };
        Ok(())
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

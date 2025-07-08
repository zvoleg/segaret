use std::{cell::RefCell, rc::Rc};

use log::{debug, info};

use super::vdp_emu::vdp_port::VdpPorts;
use m68k_emu::bus::BusM68k;

use crate::{controller::Controller, memory_space::MemorySpace, signal_bus::{Signal, SignalBus}};

const VERSION_REGISTER: u32 = 0xA10001;
const CONTROLLER_A_DATA: u32 = 0xA10002;
const CONTROLLER_B_DATA: u32 = 0xA10004;
const CONTROLLER_A_CONTROL: u32 = 0xA10008;
const CONTROLLER_B_CONTROL: u32 = 0xA1000A;
// const EXPANSION_PORT_CONTROL: u32 = 0xA1000C;
const Z80_REQUEST_BUS: u32 = 0xA11100;
const Z80_RESET: u32 = 0xA11200;

pub struct CpuBus<T: VdpPorts> {
    memory_space: Rc<RefCell<MemorySpace>>,

    vdp_ports: Option<Rc<RefCell<T>>>,
    z80_bus_reg: RefCell<u32>,

    controller_1: Rc<RefCell<Controller>>,
    controller_2: Rc<RefCell<Controller>>,

    signal_bus: Rc<RefCell<SignalBus>>,
}

impl<T> CpuBus<T>
where
    T: VdpPorts,
{
    pub fn init(
        memory_space: Rc<RefCell<MemorySpace>>,
        controller_1: Rc<RefCell<Controller>>,
        controller_2: Rc<RefCell<Controller>>,
        signal_bus: Rc<RefCell<SignalBus>>,
    ) -> Self {
        Self {
            memory_space: memory_space,
            vdp_ports: None,

            z80_bus_reg: RefCell::new(0),

            controller_1: controller_1,
            controller_2: controller_2,

            signal_bus: signal_bus,
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
        debug!("CPU reads address {:08X}\tsize: {}", address, amount);
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
                let bus_state = *self.z80_bus_reg.borrow();
                debug!("Z80 bus state flag is {:04X}", bus_state);
                if bus_state == 0x100 { Ok(0) } else { Ok(1) }
            } else if address == CONTROLLER_A_DATA || address == CONTROLLER_A_DATA + 1 {
                Ok(self.controller_1.borrow().read() as u32)
            } else if address == CONTROLLER_B_DATA || address == CONTROLLER_B_DATA + 1 {
                Ok(self.controller_2.borrow().read() as u32)
            } else {
                let address = (address & 0x3f) as usize;
                Ok(self.read_ptr(amount, &self.memory_space.borrow().io_area_read[address]))
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            self.vdp_ports
                .as_ref()
                .unwrap()
                .borrow_mut()
                .read_data_port()
        } else if address == 0xC00004 || address == 0xC00006 {
            self.vdp_ports
                .as_ref()
                .unwrap()
                .borrow_mut()
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
        debug!(
            "CPU writes address {:08X}\tdata {:08X}\tsize: {}",
            address, data, amount
        );
        if address <= 0x3FFFFF {
            let ptr = &self.memory_space.as_ref().borrow_mut().rom[address as usize] as *const _
                as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = address & 0xFFFF;
            let ptr = &self.memory_space.as_ref().borrow_mut().z80_ram[address as usize] as *const _
                as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == Z80_REQUEST_BUS {
                *self.z80_bus_reg.borrow_mut() = data;
                if data == 0x100 {
                    debug!("Z80_bus request");
                    self.signal_bus.borrow_mut().push_siganal(Signal::Z80BusRequest);
                } else {
                    debug!("Z80_bus release");
                    self.signal_bus.borrow_mut().push_siganal(Signal::Z80BusFree);
                }
            } else if address == Z80_RESET {
                debug!("Z80 send reset signal with data {:04X}", data);
                self.signal_bus.borrow_mut().push_siganal(Signal::Z80Reset);
            } else if address == CONTROLLER_A_DATA || address == CONTROLLER_A_DATA + 1 {
                self.controller_1.borrow_mut().write(data as u8);
            } else if address == CONTROLLER_B_DATA || address == CONTROLLER_B_DATA + 1 {
                self.controller_2.borrow_mut().write(data as u8);
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

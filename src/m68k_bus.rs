use log::{debug, info};
use m68k_emu::bus::BusM68k;
use z80_emu::bus::BusZ80;

use crate::{memory_space::MemorySpace, signal_bus::Signal, vdp_emu::vdp_port::VdpPorts};

const VERSION_REGISTER: u32 = 0xA10001;
const CONTROLLER_A_DATA: u32 = 0xA10002;
const CONTROLLER_B_DATA: u32 = 0xA10004;
const CONTROLLER_A_CONTROL: u32 = 0xA10008;
const CONTROLLER_B_CONTROL: u32 = 0xA1000A;
// const EXPANSION_PORT_CONTROL: u32 = 0xA1000C;
const Z80_REQUEST_BUS: u32 = 0xA11100;
const Z80_RESET: u32 = 0xA11200;

impl<T> BusM68k for MemorySpace<T> where T: VdpPorts {
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()> {
        let address = address & 0x00FFFFFF;
        debug!("CPU reads address {:08X}\tsize: {}", address, amount);
        if address <= 0x3FFFFF {
            Ok(self.read_ptr(amount, &self.rom[address as usize]))
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = (address & 0xFFFF) as u16;
            // TODO may be there should be a z80 bus register check
            let data = <MemorySpace<T> as BusZ80>::read(&self, address, amount)? as u32;
            Ok(data)
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == VERSION_REGISTER {
                let program_region = self.rom[0x1F0];
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
                Ok(self.read_ptr(amount, &self.io_area_read[address]))
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            self.vdp_ports
                .as_ref()
                .borrow_mut()
                .read_data_port()
        } else if address == 0xC00004 || address == 0xC00006 {
            self.vdp_ports
                .as_ref()
                .borrow_mut()
                .read_control_port()
        } else if address == 0xC00008 {
            info!("Reading of VDP HVCounter");
            self.vdp_ports
                .as_ref()
                .borrow_mut()
                .read_hv_counters_port()
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            Ok(self.read_ptr(
                amount,
                &self.m68k_ram[address as usize],
            ))
        } else {
            let address = address & 0x1f;
            Ok(self.read_ptr(
                amount,
                &self.io_area_read[address as usize],
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
            let ptr = &self.rom[address as usize] as *const _
                as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = (address & 0xFFFF) as u16;
            // TODO may be there should be a z80 bus register check
            <MemorySpace<T> as BusZ80>::write(self, data as u16, address, amount)?;
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
                    &self.io_area_m68k[address] as *const _
                        as *mut u8,
                )
            }
        } else if address == 0xC00000 || address == 0xC00002 {
            let mut vdp_port_ref = self.vdp_ports.as_ref().borrow_mut();
            if amount == 4 {
                vdp_port_ref.write_data_port((data >> 16) as u16)?;
                vdp_port_ref.write_data_port(data as u16)?;
            } else {
                vdp_port_ref.write_data_port(data as u16)?;
            }
        } else if address == 0xC00004 || address == 0xC00006 {
            let mut vdp_port_ref = self.vdp_ports.as_ref().borrow_mut();
            if amount == 4 {
                vdp_port_ref.write_control_port((data >> 16) as u16)?;
                vdp_port_ref.write_control_port(data as u16)?;
            } else {
                vdp_port_ref.write_control_port(data as u16)?;
            }
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            let ptr = &self.m68k_ram[address as usize]
                as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        } else {
            let address = address & 0x1f;
            let ptr = &self.io_area_m68k[address as usize]
                as *const _ as *mut u8;
            self.write_ptr(data, amount, ptr);
        };
        Ok(())
    }
}
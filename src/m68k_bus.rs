use log::{debug, info};
use m68k_emu::bus::BusM68k;
use z80_emu::bus::BusZ80;

use crate::{
    memory_space::MemorySpace, signal_bus::Signal, vdp_emu::vdp_port::VdpPorts, ym2612::Ym2612Ports,
};

const VERSION_REGISTER: u32 = 0xA10001;
const CONTROLLER_A_DATA: u32 = 0xA10002;
const CONTROLLER_B_DATA: u32 = 0xA10004;
// const CONTROLLER_A_CONTROL: u32 = 0xA10008;
// const CONTROLLER_B_CONTROL: u32 = 0xA1000A;
// const EXPANSION_PORT_CONTROL: u32 = 0xA1000C;
const Z80_REQUEST_BUS: u32 = 0xA11100;
const Z80_RESET: u32 = 0xA11200;

impl<T, Y> BusM68k for MemorySpace<T, Y>
where
    T: VdpPorts,
    Y: Ym2612Ports,
{
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()> {
        let address = address & 0x00FFFFFF;
        let mut buff = [0u8; 4];
        let buff_chunk = &mut buff[4 - amount as usize..];
        debug!("CPU reads address {:08X}\tsize: {}", address, amount);
        if address <= 0x3FFFFF {
            let memory_chunk = self.rom[address as usize..].split_at(amount as usize).0;
            buff_chunk.copy_from_slice(memory_chunk);
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = (address & 0xFFFF) as u16;
            // TODO may be there should be a z80 bus register check
            let data = <MemorySpace<T, Y> as BusZ80>::read(&self, address, amount)? as u32;
            debug!("BusM68k::read: M68000 read value from Z80 memory space (address: {:04x}, data: {:04X}), size: {}", address, data, amount);
            return Ok(data);
        } else if address >= 0xA10000 && address < 0xA20000 {
            return if address == VERSION_REGISTER {
                let program_region = self.rom[0x1F0];
                match program_region {
                    0x55 => Ok(0x80),
                    0x45 => Ok(0xC0),
                    0x4A => Ok(0x00),
                    _ => panic!("unexpected program region code {:02X}", program_region),
                }
            } else if address == Z80_REQUEST_BUS {
                let bus_state = *self.z80_bus_req.borrow();
                debug!("Z80 bus state flag is {}", bus_state);
                let bus_status_bit = if bus_state { Ok(0) } else { Ok(1) };
                debug!(
                    "Z80 bus status bit: {:04x}",
                    bus_status_bit.as_ref().unwrap()
                );
                bus_status_bit
            } else if address == CONTROLLER_A_DATA || address == CONTROLLER_A_DATA + 1 {
                Ok(self.controller_1.borrow().read() as u32)
            } else if address == CONTROLLER_B_DATA || address == CONTROLLER_B_DATA + 1 {
                Ok(self.controller_2.borrow().read() as u32)
            } else {
                let address = (address & 0x3f) as usize;
                let memory_chunk = self.io_area_read[address..].split_at(amount as usize).0;
                buff_chunk.copy_from_slice(memory_chunk);
                Ok(u32::from_be_bytes(buff))
            };
        } else if address == 0xC00000 || address == 0xC00002 {
            return self.vdp_ports.as_ref().borrow_mut().read_data_port();
        } else if address == 0xC00004 || address == 0xC00006 {
            return self.vdp_ports.as_ref().borrow_mut().read_control_port();
        } else if address == 0xC00008 {
            info!("Reading of VDP HVCounter");
            return self.vdp_ports.as_ref().borrow_mut().read_hv_counters_port();
        } else if address >= 0xFF0000 && address <= 0xFFFFFF {
            let address = address & 0xFFFF;
            let memory_chunk = self.m68k_ram[address as usize..]
                .split_at(amount as usize)
                .0;
            buff_chunk.copy_from_slice(memory_chunk);
        } else {
            let address = (address & 0x1f) as usize;
            let memory_chunk = self.io_area_read[address..].split_at(amount as usize).0;
            buff_chunk.copy_from_slice(memory_chunk);
        }
        Ok(u32::from_be_bytes(buff))
    }

    fn write(&mut self, data: u32, address: u32, amount: u32) -> Result<(), ()> {
        let address = address & 0x00FFFFFF;
        let bytes = data.to_be_bytes();
        let chunk = &bytes[4 - amount as usize..]; // 4 it is u32 size
        debug!(
            "CPU writes address {:08X}\tdata {:08X}\tsize: {}",
            address, data, amount
        );
        if address <= 0x3FFFFF {
            let address = address as usize;
            self.rom[address..address + amount as usize].copy_from_slice(chunk);
        } else if address >= 0xA00000 && address <= 0xA0FFFF {
            let address = (address & 0xFFFF) as u16;
            // TODO may be there should be a z80 bus register check
            <MemorySpace<T, Y> as BusZ80>::write(self, data as u16, address, amount)?;
        } else if address >= 0xA10000 && address < 0xA20000 {
            if address == Z80_REQUEST_BUS {
                *self.z80_bus_req.borrow_mut() = data != 0;
                if data != 0 {
                    debug!("Z80_bus requested");
                    self.signal_bus
                        .borrow_mut()
                        .push_signal(Signal::Z80BusRequest);
                } else {
                    debug!("Z80_bus released");
                    self.signal_bus.borrow_mut().push_signal(Signal::Z80BusFree);
                }
            } else if address == Z80_RESET {
                debug!("Z80 send reset signal with data {:04X}", data);
                *self.z80_res_req.borrow_mut() = data == 0;
                self.signal_bus.borrow_mut().push_signal(Signal::Z80Reset);
            } else if address == CONTROLLER_A_DATA || address == CONTROLLER_A_DATA + 1 {
                self.controller_1.borrow_mut().write(data as u8);
            } else if address == CONTROLLER_B_DATA || address == CONTROLLER_B_DATA + 1 {
                self.controller_2.borrow_mut().write(data as u8);
            } else {
                let address = (address & 0x3f) as usize;
                self.io_area_m68k[address..address + amount as usize].copy_from_slice(chunk);
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
            let address = (address & 0xFFFF) as usize;
            self.m68k_ram[address..address + amount as usize].copy_from_slice(chunk);
        } else {
            let address = (address & 0x1f) as usize;
            self.io_area_m68k[address..address + amount as usize].copy_from_slice(chunk);
        };
        Ok(())
    }
}

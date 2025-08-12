use std::{cell::RefCell, rc::Rc};

use log::{debug, info};
use z80_emu::bus::BusZ80;

use crate::{memory_space::MemorySpace, vdp_emu::vdp_port::VdpPorts};

impl<T> BusZ80 for MemorySpace<T> where T: VdpPorts {
    fn read(&self, address: u16, amount: u32) -> Result<u16, ()> {
        let data = self.read_ptr(
            amount,
            &self.z80_ram[address as usize],
        ) as u16;
        debug!("Z80 bus: reading address: {:04X}\tsize: {}\tdata: {:04X}", address, amount, data);
        Ok(data)
    }

    fn write(&self, data: u16, address: u16, amount: u32) -> Result<(), ()> {
        self.write_ptr(
            data as u32,
            amount,
            &self.z80_ram[address as usize] as *const _ as *mut u8
        );
        if address == 0x6000 {
            info!("Z80 bus: setup m68k bank: {:04X}", data);
        }
        debug!("Z80 bus: writing address: {:04X}\tsize: {}\tdata: {:04X}", address, amount, data);
        Ok(())
    }
}

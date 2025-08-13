use log::{debug, info};
use z80_emu::bus::BusZ80;

use crate::{memory_space::MemorySpace, vdp_emu::vdp_port::VdpPorts, ym2612::{RegisterPart, Ym2612Ports}};

impl<T, Y> BusZ80 for MemorySpace<T, Y>
where
    T: VdpPorts,
    Y: Ym2612Ports,
{
    fn read(&self, address: u16, amount: u32) -> Result<u16, ()> {
        let data = if 0x4000 <= address && address < 0x4004 {
            self.ym2612_ports.borrow().read_status() as u32
        } else {
            self.read_ptr(amount, &self.z80_ram[address as usize])
        } as u16;
        debug!(
            "Z80 bus: reading address: {:04X}\tsize: {}\tdata: {:04X}",
            address, amount, data
        );
        let data = self.read_ptr(amount, &self.z80_ram[address as usize]) as u16;
        Ok(data)
    }

    fn write(&self, data: u16, address: u16, amount: u32) -> Result<(), ()> {
        if address == 0x4000 {
            self.ym2612_ports.borrow_mut().register_set(RegisterPart::Fm1, data as u8);
        } else if address == 0x4001 {
            self.ym2612_ports.borrow_mut().register_data(RegisterPart::Fm1, data as u8);
        } else if address == 0x4002 {
            self.ym2612_ports.borrow_mut().register_set(RegisterPart::Fm2, data as u8);
        } else if address == 0x4003 {
            self.ym2612_ports.borrow_mut().register_data(RegisterPart::Fm2, data as u8);
        } else if address == 0x6000 {
            info!("Z80 bus: setup m68k bank: {:04X}", data);
        } else {
            self.write_ptr(
                data as u32,
                amount,
                &self.z80_ram[address as usize] as *const _ as *mut u8,
            );
            debug!(
                "Z80 bus: writing address: {:04X}\tsize: {}\tdata: {:04X}",
                address, amount, data
            );
        }
        Ok(())
    }
}

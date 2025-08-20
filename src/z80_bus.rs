use log::{debug, info};
use m68k_emu::bus::BusM68k;
use z80_emu::bus::BusZ80;

use crate::{memory_space::MemorySpace, vdp_emu::vdp_port::VdpPorts, ym2612::{RegisterPart, Ym2612Ports}};

impl<T, Y> BusZ80 for MemorySpace<T, Y>
where
    T: VdpPorts,
    Y: Ym2612Ports,
{
    fn read(&self, address: u16, amount: usize) -> Result<u16, ()> {
        let mut buff = [0u8; 2];
        // for Size::Byte, actual information contains in last buffer bytes
        let buff_chunk = &mut buff[..amount];
        let data = if 0x4000 <= address && address < 0x4004 {
            self.ym2612_ports.borrow().read_status() as u16
        } else if 0x8000 <= address && address <= 0xFFFF {
            let msb_address = (*self.bank_register.borrow() as u32) << 15;
            let lsb_address = (address & 0x7FFF) as u32;
            let m68k_address = msb_address | lsb_address;
            <MemorySpace<T, Y> as BusM68k>::read(self, m68k_address, amount)? as u16
        } else {
            let memory_chunk = &self.z80_ram[address as usize..address as usize + amount];
            buff_chunk.copy_from_slice(&memory_chunk);
            <u16>::from_le_bytes(buff)
        };
        debug!(
            "Z80 bus: reading address: {:04X}\tsize: {}\tdata: {:04X}",
            address, amount, data
        );
        Ok(data)
    }

    fn write(&mut self, data: u16, address: u16, amount: usize) -> Result<(), ()> {
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
            self.push_bank_register_bit(data);
        } else if 0x8000 <= address && address <= 0xFFFF {
            let msb_address = (*self.bank_register.borrow() as u32) << 15;
            let lsb_address = (address & 0x7FFF) as u32;
            let m68k_address = msb_address | lsb_address;
            // <MemorySpace<T, Y> as BusM68k>::write(self, data as u32, m68k_address, amount)? // TODO z80 can override m68k programm?
        } else {
            let bytes = data.to_le_bytes();
            // for Size::Byte, actual information contains in last data bytes
            let chunk = &bytes[..amount];
            self.z80_ram[address as usize..address as usize + amount].copy_from_slice(chunk);
            debug!(
                "Z80 bus: writing address: {:04X}\tsize: {}\tdata: {:04X}",
                address, amount, data
            );
        }
        Ok(())
    }
}

use log::debug;

use super::vdp_emu::bus::BusVdp;

use crate::{memory_space::MemorySpace, vdp_emu::vdp_port::VdpPorts, ym2612::Ym2612Ports};

impl<T, Y> BusVdp for MemorySpace<T, Y> where T: VdpPorts, Y: Ym2612Ports {
    fn read(&self, address: u32) -> u16 {
        debug!("VDP reads address {:08X}", address);
        let data = unsafe {
            if address < 0x400000 {
                *(self.rom.as_ptr().offset(address as isize) as *const _ as *const u16)
            } else if address >= 0xFF0000 {
                let address = address & 0xFFFF;
                *(self.m68k_ram.as_ptr().offset(address as isize) as *const _ as *const u16)
            } else {
                panic!("VdpBus: read: wrong address value: {:08X}", address)
            }
        };
        data
    }
}

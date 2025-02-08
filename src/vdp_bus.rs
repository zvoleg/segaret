use std::{cell::RefCell, rc::Rc};

use log::debug;

use super::vdp_emu::bus::BusVdp;

use crate::memory_space::MemorySpace;

pub struct VdpBus {
    memory_space: Rc<RefCell<MemorySpace>>,
}

impl VdpBus {
    pub fn new(memory_space: Rc<RefCell<MemorySpace>>) -> Self {
        Self { memory_space }
    }
}

impl BusVdp for VdpBus {
    fn read(&self, address: u32) -> u16 {
        debug!("VDP reads address {:08X}", address);
        let data = unsafe {
            if address < 0x400000 {
                *(self.memory_space.borrow().rom.as_ptr().offset(address as isize) as *const _ as *const u16)
            } else if address >= 0xFF0000 {
                let address = address & 0xFFFF;
                *(self.memory_space.borrow().m68k_ram.as_ptr().offset(address as isize) as *const _ as *const u16)
            } else {
                panic!("VdpBus: read: wrong address value: {:08X}", address)
            }
        };
        data
    }
}

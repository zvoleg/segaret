use std::{cell::RefCell, rc::Rc};

use vdp_emu::bus::BusVdp;

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
        unsafe {
            if address < 0x400000 {
                *(&self.memory_space.borrow().rom[address as usize] as *const _ as *const u16)
            } else if address >= 0xFF0000 {
                let address = address & 0xFFFF;
                *(&self.memory_space.borrow().m68k_ram[address as usize] as *const _ as *const u16)
            } else {
                panic!("VdpBus: read: wrong address value: {:08X}", address)
            }
        }
    }
}

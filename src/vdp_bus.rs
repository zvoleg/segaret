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
        todo!()
    }
}

use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::bus::BusM68k;

use super::{Pointer, Size};

pub(crate) struct MemoryPtr {
    address: u32,
    bus: Rc<RefCell<dyn BusM68k>>,
}

impl Display for MemoryPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.address)
    }
}

impl MemoryPtr {
    pub(crate) fn new(address: u32, bus: Rc<RefCell<dyn BusM68k>>) -> Self {
        Self { address, bus }
    }

    pub(crate) fn new_boxed(address: u32, bus: Rc<RefCell<dyn BusM68k>>) -> Box<Self> {
        Box::new(Self::new(address, bus))
    }
}

impl Pointer for MemoryPtr {
    fn read(&self, size: Size) -> Result<u32, ()> {
        self.bus.borrow().read(self.address, size as u32)
    }

    fn write(&self, data: u32, size: Size) -> Result<(), ()> {
        self.bus.borrow_mut().write(data, self.address, size as u32)
    }

    fn read_offset(&self, size: Size, offset: isize) -> Result<u32, ()> {
        self.bus
            .borrow()
            .read(self.address.wrapping_add(offset as u32), size as u32)
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) -> Result<(), ()> {
        self.bus
            .borrow_mut()
            .write(data, self.address.wrapping_add(offset as u32), size as u32)
    }
}

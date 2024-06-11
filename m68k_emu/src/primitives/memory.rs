use std::rc::Rc;

use crate::bus::BusM68k;

use super::{Pointer, Size};

pub(crate) struct MemoryPtr {
    address: u32,
    bus: Rc<dyn BusM68k>,
    // read_ptr: *const u8,
    // write_ptr: *mut u8,
}

impl MemoryPtr {
    pub(crate) fn new(address: u32, bus: Rc<dyn BusM68k>) -> Self {
        Self { address, bus }
    }

    pub(crate) fn new_boxed(address: u32, bus: Rc<dyn BusM68k>) -> Box<Self> {
        Box::new(Self::new(address, bus))
    }
}

impl Pointer for MemoryPtr {
    fn read(&self, size: Size) -> u32 {
        self.bus.read(self.address, size as u32)
    }

    fn write(&self, data: u32, size: Size) {
        self.bus.write(data, self.address, size as u32);
    }

    fn read_offset(&self, size: Size, offset: isize) -> u32 {
        self.bus
            .read(self.address.wrapping_add(offset as u32), size as u32)
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) {
        self.bus
            .write(data, self.address.wrapping_add(offset as u32), size as u32);
    }
}

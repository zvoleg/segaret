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

// #[cfg(test)]
// mod tests {
//     use crate::primitives::{memory::MemoryPtr, Pointer, Size};

//     #[test]
//     fn memory_register_byte() {
//         let mut data = 0u32;
//         let ptr = MemoryPtr {
//             read_ptr: &data as *const _ as *const u8,
//             write_ptr: &mut data as *mut _ as *mut u8,
//         };
//         ptr.write(0xFF, Size::Byte);
//         let res = ptr.read(Size::Word);
//         assert_eq!(res, 0xFF00);
//     }

//     #[test]
//     fn memory_register_word() {
//         let mut data = 0u32;
//         let ptr = MemoryPtr {
//             read_ptr: &data as *const _ as *const u8,
//             write_ptr: &mut data as *mut _ as *mut u8,
//         };
//         ptr.write(0x9911, Size::Word);
//         assert_eq!(ptr.read(Size::Byte), 0x99);
//     }

//     #[test]
//     fn memory_register_overlapping_writes() {
//         let mut data = 0u32;
//         let ptr = MemoryPtr {
//             read_ptr: &data as *const _ as *const u8,
//             write_ptr: &mut data as *mut _ as *mut u8,
//         };
//         ptr.write(0x99000099, Size::Long);
//         ptr.write(0x11, Size::Byte);
//         assert_eq!(ptr.read(Size::Long), 0x11000099);
//     }

//     #[test]
//     fn memory_write_with_offset() {
//         let mut data: [u8; 16] = [0; 16];
//         let ptr = MemoryPtr {
//             read_ptr: &data as *const _ as *const u8,
//             write_ptr: &mut data as *mut _ as *mut u8,
//         };
//         ptr.write_offset(0x99553311, Size::Long, 1);
//         assert_eq!(data[1], 0x99);
//         assert_eq!(data[2], 0x55);
//         assert_eq!(data[3], 0x33);
//         assert_eq!(data[4], 0x11);
//     }
// }

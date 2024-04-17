use std::fmt::Display;

use crate::SignExtending;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Byte => write!(f, "b"),
            Size::Word => write!(f, "w"),
            Size::Long => write!(f, "l"),
        }
    }
}

pub(crate) trait Pointer {
    fn read(&self, size: Size) -> u32;
    fn write(&self, data: u32, size: Size);
    fn read_offset(&self, size: Size, offset: isize) -> u32;
    fn write_offset(&self, data: u32, size: Size, offset: isize);
}

pub(crate) struct DataRegisterPtr(*mut u8);

impl DataRegisterPtr {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub(crate) fn new_boxed(ptr: *mut u8) -> Box<Self> {
        Box::new(Self::new(ptr))
    }

    fn read_ptr(&self, ptr: *mut u8, size: Size) -> u32 {
        unsafe {
            match size {
                Size::Byte => *ptr as u32,
                Size::Word => *(ptr as *mut u16) as u32,
                Size::Long => *(ptr as *mut u32),
            }
        }
    }

    fn write_ptr(&self, ptr: *mut u8, data: u32, size: Size) {
        unsafe {
            match size {
                Size::Byte => *ptr = data as u8,
                Size::Word => *(ptr as *mut u16) = data as u16,
                Size::Long => *(ptr as *mut u32) = data,
            }
        }
    }
}

pub(crate) struct AddressRegisterPtr(*mut u8);

impl AddressRegisterPtr {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub(crate) fn new_boxed(ptr: *mut u8) -> Box<Self> {
        Box::new(Self::new(ptr))
    }

    fn read_ptr(&self, ptr: *mut u8, size: Size) -> u32 {
        unsafe {
            match size {
                Size::Byte => panic!(
                    "AddressRegisterPtr: read: address register can't be to addressed by Byte size"
                ),
                Size::Word => *(ptr as *mut u16) as u32,
                Size::Long => *(ptr as *mut u32),
            }
        }
    }

    fn write_ptr(&self, ptr: *mut u8, data: u32, size: Size) {
        unsafe {
            match size {
                Size::Byte => panic!(
                    "AddressRegisterPtr: read: address register can't be to addressed by Byte size"
                ),
                Size::Word => *(ptr as *mut u32) = data.sign_extend(size),
                Size::Long => *(ptr as *mut u32) = data,
            }
        }
    }
}

pub(crate) struct MemoryPtr(*mut u8);

impl MemoryPtr {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    pub(crate) fn new_boxed(ptr: *mut u8) -> Box<Self> {
        Box::new(Self::new(ptr))
    }

    fn read_ptr(&self, ptr: *mut u8, size: Size) -> u32 {
        unsafe {
            match size {
                Size::Byte => *ptr as u32,
                Size::Word => {
                    let mut data = *(ptr as *mut u16);
                    data = data.to_be();
                    data as u32
                }
                Size::Long => {
                    let data = *(ptr as *mut u32);
                    data.to_be()
                }
            }
        }
    }

    fn write_ptr(&self, ptr: *mut u8, data: u32, size: Size) {
        unsafe {
            match size {
                Size::Byte => *ptr = data as u8,
                Size::Word => *(ptr as *mut u16) = (data as u16).to_be(),
                Size::Long => *(ptr as *mut u32) = data.to_be(),
            }
        }
    }
}

impl Pointer for DataRegisterPtr {
    fn read(&self, size: Size) -> u32 {
        self.read_ptr(self.0, size)
    }

    fn write(&self, data: u32, size: Size) {
        self.write_ptr(self.0, data, size);
    }

    fn read_offset(&self, size: Size, offset: isize) -> u32 {
        unsafe {
            let offset_ptr = self.0.offset(offset * 4);
            self.read_ptr(offset_ptr, size)
        }
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) {
        unsafe {
            let offset_ptr = self.0.offset(offset * 4);
            self.write_ptr(offset_ptr, data, size);
        }
    }
}

impl Pointer for AddressRegisterPtr {
    fn read(&self, size: Size) -> u32 {
        self.read_ptr(self.0, size)
    }

    fn write(&self, data: u32, size: Size) {
        self.write_ptr(self.0, data, size);
    }

    fn read_offset(&self, size: Size, offset: isize) -> u32 {
        unsafe {
            let offset_ptr = self.0.offset(offset * 4);
            self.read_ptr(offset_ptr, size)
        }
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) {
        unsafe {
            let offset_ptr = self.0.offset(offset * 4);
            self.write_ptr(offset_ptr, data, size);
        }
    }
}

impl Pointer for MemoryPtr {
    fn read(&self, size: Size) -> u32 {
        self.read_ptr(self.0, size)
    }

    fn write(&self, data: u32, size: Size) {
        self.write_ptr(self.0, data, size);
    }

    fn read_offset(&self, size: Size, offset: isize) -> u32 {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            self.read_ptr(offset_ptr, size)
        }
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            self.write_ptr(offset_ptr, data, size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AddressRegisterPtr, DataRegisterPtr, MemoryPtr, Pointer, Size};

    #[test]
    fn data_register_byte() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0xFF, Size::Byte);
        assert_eq!(ptr.read(Size::Word), 0xFF);
    }

    #[test]
    fn data_register_word() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x9911, Size::Word);
        assert_eq!(ptr.read(Size::Byte), 0x11);
    }

    #[test]
    fn data_register_overlapping_writes() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x99000000, Size::Long);
        ptr.write(0x11, Size::Byte);
        assert_eq!(ptr.read(Size::Long), 0x99000011);
    }

    #[test]
    fn address_register_word_sign_extended() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x8000, Size::Word);
        assert_eq!(ptr.read(Size::Long), 0xFFFF8000);
    }

    #[test]
    fn address_register_word() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x7000, Size::Word);
        assert_eq!(ptr.read(Size::Long), 0x00007000);
    }

    #[test]
    fn address_register_word_override() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x55559999, Size::Long);
        ptr.write(0x7000, Size::Word);
        assert_eq!(ptr.read(Size::Long), 0x00007000);
    }

    #[test]
    fn address_register_long() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x55559999, Size::Long);
        assert_eq!(ptr.read(Size::Word), 0x9999);
        assert_eq!(ptr.read(Size::Long), 0x55559999);
    }

    #[test]
    #[should_panic]
    fn address_register_cant_read_byte() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.read(Size::Byte);
    }

    #[test]
    #[should_panic]
    fn address_register_cant_write_byte() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x33, Size::Byte);
    }

    #[test]
    fn memory_register_byte() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0xFF, Size::Byte);
        let res = ptr.read(Size::Word);
        assert_eq!(res, 0xFF);
    }

    #[test]
    fn memory_register_word() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x9911, Size::Word);
        assert_eq!(ptr.read(Size::Byte), 0x11);
    }

    #[test]
    fn memory_register_overlapping_writes() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x99000000, Size::Long);
        ptr.write(0x11, Size::Byte);
        assert_eq!(ptr.read(Size::Long), 0x99000011);
    }

    #[test]
    fn data_register_write_byte_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = DataRegisterPtr(&mut data[0] as *mut _ as *mut u8);
        ptr.write_offset(0x55, Size::Byte, 2);
        assert_eq!(data[2], 0x55);
    }

    #[test]
    fn data_register_overlapping_write_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = DataRegisterPtr(&mut data[0] as *mut _ as *mut u8);
        ptr.write_offset(0x55000000, Size::Long, 15);
        ptr.write_offset(0x8000, Size::Word, 15);
        assert_eq!(data[15], 0x55008000);
    }

    #[test]
    fn address_register_write_word_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = AddressRegisterPtr(&mut data[0] as *mut _ as *mut u8);
        ptr.write_offset(0x8000, Size::Word, 5);
        assert_eq!(data[5], 0xFFFF8000);
    }

    #[test]
    fn memory_write_with_offset() {
        let mut data: [u8; 16] = [0; 16];
        let ptr = AddressRegisterPtr(&mut data[0] as *mut _ as *mut u8);
        ptr.write_offset(0x99553311, Size::Long, 1);
        assert_eq!(data[4], 0x11);
        assert_eq!(data[5], 0x33);
        assert_eq!(data[6], 0x55);
        assert_eq!(data[7], 0x99);
    }
}

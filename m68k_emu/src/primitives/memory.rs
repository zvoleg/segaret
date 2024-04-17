use super::{Pointer, Size};

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
    use crate::primitives::{memory::MemoryPtr, Pointer, Size};

    #[test]
    fn memory_register_byte() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0xFF, Size::Byte);
        let res = ptr.read(Size::Word);
        assert_eq!(res, 0xFF00);
    }

    #[test]
    fn memory_register_word() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x9911, Size::Word);
        assert_eq!(ptr.read(Size::Byte), 0x99);
    }

    #[test]
    fn memory_register_overlapping_writes() {
        let mut data = 0u32;
        let ptr = MemoryPtr(&mut data as *mut _ as *mut u8);
        ptr.write(0x99000099, Size::Long);
        ptr.write(0x11, Size::Byte);
        assert_eq!(ptr.read(Size::Long), 0x11000099);
    }

    #[test]
    fn memory_write_with_offset() {
        let mut data: [u8; 16] = [0; 16];
        let ptr: MemoryPtr = MemoryPtr(&mut data[0]);
        ptr.write_offset(0x99553311, Size::Long, 1);
        assert_eq!(data[1], 0x99);
        assert_eq!(data[2], 0x55);
        assert_eq!(data[3], 0x33);
        assert_eq!(data[4], 0x11);
    }
}

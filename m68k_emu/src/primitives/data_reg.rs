use super::{Pointer, Size};

pub(crate) struct DataRegisterPtr(*mut u32);

impl DataRegisterPtr {
    pub(crate) fn new(ptr: *mut u32) -> Self {
        Self(ptr)
    }

    pub(crate) fn new_boxed(ptr: *mut u32) -> Box<Self> {
        Box::new(Self::new(ptr))
    }

    fn read_ptr(ptr: *mut u32, size: Size) -> u32 {
        unsafe {
            match size {
                Size::Byte => *ptr as u8 as u32,
                Size::Word => *ptr as u16 as u32,
                Size::Long => *ptr,
            }
        }
    }

    fn write_ptr(ptr: *mut u32, data: u32, size: Size) {
        unsafe {
            match size {
                Size::Byte => *(ptr as *mut u8) = data as u8,
                Size::Word => *(ptr as *mut u16) = data as u16,
                Size::Long => *ptr = data,
            }
        }
    }
}

impl Pointer for DataRegisterPtr {
    fn read(&self, size: Size) -> u32 {
        DataRegisterPtr::read_ptr(self.0, size)
    }

    fn write(&self, data: u32, size: Size) {
        DataRegisterPtr::write_ptr(self.0, data, size);
    }

    fn read_offset(&self, size: Size, offset: isize) -> u32 {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            DataRegisterPtr::read_ptr(offset_ptr, size)
        }
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            DataRegisterPtr::write_ptr(offset_ptr, data, size);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::primitives::{data_reg::DataRegisterPtr, Pointer, Size};

    #[test]
    fn data_register_byte() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data);
        ptr.write(0xFF, Size::Byte);
        assert_eq!(ptr.read(Size::Word), 0xFF);
    }

    #[test]
    fn data_register_word() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data);
        ptr.write(0x9911, Size::Word);
        assert_eq!(ptr.read(Size::Byte), 0x11);
    }

    #[test]
    fn data_register_overlapping_writes() {
        let mut data = 0u32;
        let ptr = DataRegisterPtr(&mut data);
        ptr.write(0x99000000, Size::Long);
        ptr.write(0x11, Size::Byte);
        assert_eq!(ptr.read(Size::Long), 0x99000011);
    }

    #[test]
    fn data_register_write_byte_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = DataRegisterPtr(&mut data[0]);
        ptr.write_offset(0x55, Size::Byte, 2);
        assert_eq!(data[2], 0x55);
    }

    #[test]
    fn data_register_overlapping_write_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = DataRegisterPtr(&mut data[0]);
        ptr.write_offset(0x55000000, Size::Long, 15);
        ptr.write_offset(0x8000, Size::Word, 15);
        assert_eq!(data[15], 0x55008000);
    }
}

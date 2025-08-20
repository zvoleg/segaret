use std::slice;

use crate::SignExtending;

use super::{Pointer, Size};

pub(crate) struct AddressRegisterPtr(*mut u32);

impl AddressRegisterPtr {
    pub(crate) fn new(ptr: *mut u32) -> Self {
        Self(ptr)
    }

    pub(crate) fn new_boxed(ptr: *mut u32) -> Box<Self> {
        Box::new(Self::new(ptr))
    }

    fn read_ptr(ptr: *mut u32, size: Size) -> u32 {
        if size == Size::Byte {
            panic!("AddressRegisterPtr: read: address register can't be to addressed by Byte size");
        }
        let mut buff = [0u8; 4];
        let buff_chunk = &mut buff[..size.into()];
        let register = unsafe { slice::from_raw_parts::<u8>(ptr as *const u8, size_of::<u32>()) };
        buff_chunk.copy_from_slice(&register[..size.into()]);
        <u32>::from_le_bytes(buff)
    }

    fn write_ptr(ptr: *mut u32, data: u32, size: Size) {
        if size == Size::Byte {
            panic!(
                "AddressRegisterPtr: write: address register can't be to addressed by Byte size"
            );
        }
        let data = data.sign_extend(size);
        let register = unsafe { slice::from_raw_parts_mut::<u8>(ptr as *mut u8, size_of::<u32>()) };
        register.copy_from_slice(&data.to_le_bytes());
    }
}

impl Pointer for AddressRegisterPtr {
    fn read(&self, size: Size) -> Result<u32, ()> {
        Ok(AddressRegisterPtr::read_ptr(self.0, size))
    }

    fn write(&self, data: u32, size: Size) -> Result<(), ()> {
        AddressRegisterPtr::write_ptr(self.0, data, size);
        Ok(())
    }

    fn read_offset(&self, size: Size, offset: isize) -> Result<u32, ()> {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            Ok(AddressRegisterPtr::read_ptr(offset_ptr, size))
        }
    }

    fn write_offset(&self, data: u32, size: Size, offset: isize) -> Result<(), ()> {
        unsafe {
            let offset_ptr = self.0.offset(offset);
            AddressRegisterPtr::write_ptr(offset_ptr, data, size);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{address_reg::AddressRegisterPtr, Pointer, Size};

    #[test]
    fn address_register_word_sign_extended() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.write(0x8000, Size::Word).unwrap();
        assert_eq!(ptr.read(Size::Long), Ok(0xFFFF8000));
    }

    #[test]
    fn address_register_word() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.write(0x7000, Size::Word).unwrap();
        assert_eq!(ptr.read(Size::Long), Ok(0x00007000));
    }

    #[test]
    fn address_register_word_override() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.write(0x55559999, Size::Long).unwrap();
        ptr.write(0x7000, Size::Word).unwrap();
        assert_eq!(ptr.read(Size::Long), Ok(0x00007000));
    }

    #[test]
    fn address_register_long() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.write(0x55559999, Size::Long).unwrap();
        assert_eq!(ptr.read(Size::Word), Ok(0x9999));
        assert_eq!(ptr.read(Size::Long), Ok(0x55559999));
    }

    #[test]
    #[should_panic]
    fn address_register_cant_read_byte() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.read(Size::Byte).unwrap();
    }

    #[test]
    #[should_panic]
    fn address_register_cant_write_byte() {
        let mut data = 0u32;
        let ptr = AddressRegisterPtr(&mut data);
        ptr.write(0x33, Size::Byte).unwrap();
    }

    #[test]
    fn address_register_write_word_with_offset() {
        let mut data: [u32; 16] = [0; 16];
        let ptr = AddressRegisterPtr(&mut data[0]);
        ptr.write_offset(0x8000, Size::Word, 5).unwrap();
        assert_eq!(data[5], 0xFFFF8000);
    }
}

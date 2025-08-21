use std::{cell::RefCell, rc::Rc, slice};

use crate::{bus::BusZ80, Size};

pub(crate) trait Pointer {
    fn read(&self, size: Size) -> Result<u16, ()>;
    fn write(&self, data: u16, size: Size) -> Result<(), ()>;
}

pub(crate) struct MemPtr<T> {
    address: u16,
    bus: Rc<RefCell<T>>,
}

impl<T> MemPtr<T>
where
    T: BusZ80,
{
    pub(crate) fn new(address: u16, bus: Rc<RefCell<T>>) -> Self {
        Self { address, bus }
    }
}

impl<T> Pointer for MemPtr<T>
where
    T: BusZ80,
{
    fn read(&self, size: Size) -> Result<u16, ()> {
        self.bus.borrow().read(self.address, size.into())
    }

    fn write(&self, data: u16, size: Size) -> Result<(), ()> {
        self.bus.borrow_mut().write(data, self.address, size.into())
    }
}

pub(crate) struct RegisterPtr {
    register_ptr: *mut u8,
}

impl RegisterPtr {
    pub(crate) fn new(register_ptr: *mut u8) -> Self {
        Self { register_ptr }
    }
}

impl Pointer for RegisterPtr {
    fn read(&self, size: Size) -> Result<u16, ()> {
        let mut buff = [0u8; 2];
        let buff_chunk = &mut buff[..size.into()];
        let register = unsafe { slice::from_raw_parts(self.register_ptr, size.into()) };
        buff_chunk.copy_from_slice(register);
        Ok(<u16>::from_le_bytes(buff))
    }

    fn write(&self, data: u16, size: Size) -> Result<(), ()> {
        let register = unsafe { slice::from_raw_parts_mut(self.register_ptr, size.into()) };
        let data_chunk = &data.to_le_bytes()[..size.into()];
        register.copy_from_slice(data_chunk);
        Ok(())
    }
}

pub(crate) struct Operand {
    ptr: Box<dyn Pointer>,

    pub(crate) size: Size,
    pub(crate) address: Option<u16>,
}

impl Operand {
    pub(crate) fn new(ptr: Box<dyn Pointer>, size: Size, address: Option<u16>) -> Self {
        Self { ptr, size, address }
    }

    pub(crate) fn read(&self) -> Result<u16, ()> {
        self.ptr.read(self.size)
    }

    pub(crate) fn write(&self, data: u16) -> Result<(), ()> {
        self.ptr.write(data, self.size)
    }
}

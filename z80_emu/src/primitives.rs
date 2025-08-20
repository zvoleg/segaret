use std::{cell::RefCell, rc::Rc};

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
        self.bus.borrow().read(self.address, size as u32)
    }

    fn write(&self, data: u16, size: Size) -> Result<(), ()> {
        self.bus.borrow().write(data, self.address, size as u32)
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
        unsafe {
            let data = match size {
                Size::Byte => (*self.register_ptr) as u16,
                Size::Word => *(self.register_ptr as *const _ as *const u16),
            };
            Ok(data)
        }
    }

    fn write(&self, data: u16, size: Size) -> Result<(), ()> {
        unsafe {
            match size {
                Size::Byte => *self.register_ptr = data as u8,
                Size::Word => *(self.register_ptr as *mut _ as *mut u16) = data,
            }
            Ok(())
        }
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

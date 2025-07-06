use std::{cell::RefCell, rc::Rc};

use log::debug;
use z80_emu::bus::BusZ80;

use crate::memory_space::MemorySpace;

pub struct Z80Bus {
    memory_space: Rc<RefCell<MemorySpace>>,
}

impl Z80Bus {
    pub fn new(memory_space: Rc<RefCell<MemorySpace>>) -> Self {
        Self { memory_space }
    }

    fn read_ptr(&self, amount: u32, ptr: *const u8) -> u16 {
        unsafe {
            match amount {
                1 => *ptr as u16,
                2 => *(ptr as *const u16) as u16,
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    fn write_ptr(&self, data: u16, amount: u32, ptr: *mut u8) {
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = data as u16,
                _ => panic!("Bus: write: wrong size"),
            }
        }
    }
}

impl BusZ80 for Z80Bus {
    fn read(&self, address: u16, amount: u32) -> Result<u16, ()> {
        let data = self.read_ptr(
            amount,
            &self.memory_space.borrow().z80_ram[address as usize],
        );
        debug!("Z80 bus: reading address: {:04X}\tsize: {}\tdata: {:04X}", address, amount, data);
        Ok(data)
    }

    fn write(&self, data: u16, address: u16, amount: u32) -> Result<(), ()> {
        self.write_ptr(
            data,
            amount,
            &mut self.memory_space.borrow_mut().z80_ram[address as usize]
        );
        debug!("Z80 bus: writing address: {:04X}\tsize: {}\tdata: {:04X}", address, amount, data);
        Ok(())
    }
}

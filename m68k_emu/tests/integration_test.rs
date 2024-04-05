use std::cell::RefCell;

use m68k_emu::{bus::BusM68k, cpu::M68k};

struct Bus {
    ram: RefCell<[u8; 0xFF]>,
}

impl BusM68k for Bus {
    fn set_address(&self, address: u32) -> *mut u8 {
        &mut self.ram.borrow_mut()[address as usize] as *mut u8
    }
}

#[test]
fn cpu_running() {
    let mut cpu = M68k::new(Bus {
        ram: RefCell::new([0; 0xFF]),
    });
    cpu.clock();
}

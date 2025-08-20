use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use log::debug;
use z80_emu::{bus::BusZ80, cpu::Z80};

struct Bus {
    ram: RefCell<Vec<u8>>,
}

impl Bus {
    fn new(rom_file_path: &str) -> Self {
        let mut f = File::open(rom_file_path).unwrap();
        let mut ram = vec![0; 0x10000];
        f.read(&mut ram[0x100..]).unwrap();
        Self {
            ram: RefCell::new(ram),
        }
    }
}

impl BusZ80 for Bus {
    fn read(&self, address: u16, amount: u32) -> Result<u16, ()> {
        let address = if address == 0x2d16 { 0x2e2a } else { address }; // skip ABCD
        let address = if address == 0x2E2C { 0x2f40 } else { address }; // skip SBCD
        let address = if address == 0x2F42 { 0x2fDE } else { address }; // skip NBCD

        let ptr = &self.ram.borrow()[address as usize] as *const u8;
        unsafe {
            match amount {
                1 => Ok(*ptr as u16),
                2 => Ok(*(ptr as *const u16)),
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    fn write(&self, data: u16, address: u16, amount: u32) -> Result<(), ()> {
        let ptr = &mut self.ram.borrow_mut()[address as usize] as *mut u8;
        debug!(
            "CPU writes address {:08X}\tdata {:08X}\tsize: {}",
            address, data, amount
        );
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = data as u16,
                _ => panic!("Bus: write: wrong size"),
            }
        }
        Ok(())
    }
}

#[test]
fn cpu_running() {
    println!("Start Z80 test");
    env_logger::init();
    let bus = Rc::new(RefCell::new(Bus::new("zexdoc.com")));
    let mut cpu = Z80::new();
    cpu.set_bus(bus);
    cpu.restart();
    cpu.program_counter = 0x100;
    loop {
        match cpu.program_counter {
            0x0005 => { cpu.cpm_bdos(); },
            0x0000 => { break; },
            _ => { },
        }
        cpu.clock();
    }
}

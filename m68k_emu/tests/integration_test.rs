use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use log::debug;
use m68k_emu::{bus::BusM68k, cpu::M68k};

struct Bus {
    ram: RefCell<Vec<u8>>,
}

impl Bus {
    fn new(rom_file_path: &str) -> Self {
        let mut f = File::open(rom_file_path).unwrap();
        let mut ram = vec![0; 0x100000000];
        f.read(&mut ram).unwrap();
        Self {
            ram: RefCell::new(ram),
        }
    }
}

impl BusM68k for Bus {
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()> {
        let address = if address == 0x2d16 { 0x2e2a } else { address }; // skip ABCD
        let address = if address == 0x2E2C { 0x2f40 } else { address }; // skip SBCD
        let address = if address == 0x2F42 { 0x2fDE } else { address }; // skip NBCD

        let ptr = &self.ram.borrow()[address as usize] as *const u8;
        unsafe {
            match amount {
                1 => Ok(*ptr as u32),
                2 => Ok((*(ptr as *const u16)).to_be() as u32),
                4 => Ok((*(ptr as *const u32)).to_be() as u32),
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    fn write(&self, data: u32, address: u32, amount: u32) -> Result<(), ()> {
        let ptr = &mut self.ram.borrow_mut()[address as usize] as *mut u8;
        debug!(
            "CPU writes address {:08X}\tdata {:08X}\tsize: {}",
            address, data, amount
        );
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = (data as u16).to_be(),
                4 => *(ptr as *mut _ as *mut u32) = data.to_be(),
                _ => panic!("Bus: write: wrong size"),
            }
        }
        Ok(())
    }
}

#[test]
fn cpu_running() {
    env_logger::init();
    let bus = Rc::new(Bus::new("test.bin"));
    let mut cpu: M68k<Bus> = M68k::new();
    cpu.set_bus(bus);
    cpu.reset();
    let mut break_points = vec![0xF000];
    cpu.set_breakpoints(&mut break_points);
    while !cpu.breakpoint_hit {
        cpu.clock();
    }
    // source code of test.bin: https://github.com/MicroCoreLabs/Projects/blob/master/MCL68/MC68000_Test_Code/MC68000_test_all_opcodes.L68
}

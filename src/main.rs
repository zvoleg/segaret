extern crate spriter;
extern crate rand;

use std::{thread, fs::File, io::Read, sync::{Arc, Mutex}};

use bus::Bus;
use disassembler::Disassembler;
use m68k_emu::cpu::M68k;
use spriter::Color;

pub mod bus;
pub mod cartridge;

fn main() {
    let (runner, mut window) = spriter::init("segaret", 640, 448);
    let mut canvas = window.create_canvas(0, 0, 640, 448, 320, 224);
    canvas.set_clear_color(Color::from_u32(0xAAAAAA));

    let mut file = File::open("pop.md").unwrap();
    let mut rom = Vec::new();
    let _ = file.read_to_end(&mut rom);
    
    let bus = Bus::init(rom);

    // let mut disassembler = Disassembler::new("pop_disassm");
    let mut m68k = M68k::new(bus);
    
    thread::spawn(move || {
        loop {
            m68k.clock();
        }
    });
    
    
    runner.run(window, move |_| {
        true
    });
}

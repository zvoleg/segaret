extern crate spriter;

use std::{fs::File, io::Read};

use m68k_emu::cpu::M68k;

use bus::Bus;
use spriter::{if_pressed, Color};

pub mod bus;
// pub mod cartridge;

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

    let mut auto = false;
    runner.run(window, move |_| {
        if_pressed!(spriter::Key::A, { auto = !auto });
        if_pressed!(spriter::Key::C, {
            m68k.clock();
            auto = false;
        });
        if auto {
            m68k.clock();
        }
        true
    });
}

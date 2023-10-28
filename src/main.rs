extern crate spriter;
extern crate rand;

use std::fs::File;
use std::io;
use std::io::Write;

use hardware::z80::z80_emu::Z80Emu;
use spriter::Key;
use spriter::if_pressed;
use spriter::Color;

mod hardware;
pub mod disassembler;

use hardware::mc68k::mc68k_emu::Mc68k;
use hardware::bus::Bus;
use hardware::cartridge::cartridge::Cartridge;
use hardware::vdp::Vdp;

fn main() {
    let (runner, mut window) = spriter::init("segaret", 640, 448);
    let mut canvas = window.create_canvas(0, 0, 640, 448, 320, 224);
    canvas.set_clear_color(Color::from_u32(0xAAAAAA));

    let cartridge = Cartridge::init("pop.md");
    let mut vdp = Vdp::init(canvas);

    let mut bus = Bus::init(cartridge, &mut vdp);
    vdp.set_bus(&mut bus);

    let disassembler = disassembler::Disassembler::new("pop_test_01");
    let rom_ptr = bus.get_rom_ptr();
    let mut mc68k_cpu = Mc68k::init(&mut bus, rom_ptr, disassembler);
    let mut z80_cpu = Z80Emu::new(&mut bus);
    bus.set_mc68k_cpu(&mut mc68k_cpu);
    bus.set_z80_cpu(&mut z80_cpu);

    let mut auto_state = false;

    runner.run(window, move |_| {
        if_pressed!(Key::A, {
            auto_state = !auto_state;
        });
        if_pressed!(Key::C, {
            auto_state = false;
            mc68k_cpu.clock();
            vdp.clock();
            z80_cpu.clock();
        });
        if_pressed!(Key::S, {
            mc68k_cpu.save();
        });
        if_pressed!(Key::P, {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim_end();
            let new_pc = u32::from_str_radix(input, 16).expect(&format!("unexpected str format '{}'", input));
            mc68k_cpu.set_pc(new_pc);
        });
        if_pressed!(Key::Z, {
            let z80_dump = bus.z80_dump();
            let mut f = File::create("z80_dump").unwrap();
            f.write_all(z80_dump).unwrap();
        });
        if_pressed!(Key::Escape, {
            spriter::program_stop();
        });
        if auto_state {
            mc68k_cpu.clock();
            vdp.clock();
            z80_cpu.clock();
        };
        true
    });
}
extern crate spriter;

use spriter::Key;
use spriter::if_pressed;

mod hardware;

use hardware::cpu::mc68k_emu::Mc68k;
use hardware::bus::bus::Bus;
use hardware::cartridge::cartridge::Cartridge;

fn main() {
    let (runner, mut window) = spriter::init("segaret", 512, 512);

    let cartridge = Cartridge::init("pop.md");
    let bus = Bus::init(cartridge);
    let mut cpu = Mc68k::init(bus);

    let mut auto_state = false;

    runner.run(window, move |_| {
        if_pressed!(Key::A, {
            auto_state = !auto_state;
        });
        if_pressed!(Key::C, {
            auto_state = false;
            cpu.clock();
        });
        if auto_state {
            cpu.clock();
        };
        false
    });
}
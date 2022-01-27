mod hardware;

use hardware::cpu::mc68k_emu::Mc68k;
use hardware::bus::bus::Bus;
use hardware::cartridge::cartridge::Cartridge;

fn main() {
    let cartridge = Cartridge::init("pop.md");
    let bus = Bus::init(cartridge);
    let mut cpu = Mc68k::init(bus);

    for _ in 0..150 {
        cpu.clock();
    }
}
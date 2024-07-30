extern crate spriter;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use m68k_emu::cpu::M68k;

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use spriter::{if_pressed, Color};
use vdp_bus::VdpBus;
use vdp_emu::vdp_emu::Vdp;

mod cpu_bus;
mod memory_space;
mod vdp_bus;
// pub mod cartridge;

fn main() {
    let (runner, mut window) = spriter::init("segaret", 640, 448);
    let mut canvas = window.create_canvas(0, 0, 640, 448, 320, 224);
    canvas.set_clear_color(Color::from_u32(0xAAAAAA));

    let mut file = File::open("pop.md").unwrap();
    let mut rom = Vec::new();
    let _ = file.read_to_end(&mut rom);
    let memory_space = Rc::new(RefCell::new(MemorySpace::new(rom)));

    let mut m68k = M68k::new();
    let vdp = Rc::new(RefCell::new(Vdp::<VdpBus>::new(canvas)));

    let mut cpu_bus = CpuBus::init(memory_space.clone());
    cpu_bus.set_vdp_ports(vdp.clone());

    m68k.set_bus(cpu_bus);
    m68k.reset();

    let vdp_bus = VdpBus::new(memory_space.clone());
    vdp.borrow_mut().set_bus(vdp_bus);

    let interrupt_line = m68k.get_interrupt_lint();
    vdp.borrow_mut().set_interrupt_line(interrupt_line.clone());

    let mut auto = false;
    runner.run(window, move |_| {
        if_pressed!(spriter::Key::A, { auto = !auto });
        if_pressed!(spriter::Key::C, {
            m68k.clock();
            vdp.borrow_mut().clock();
            auto = false;
        });
        if_pressed!(spriter::Key::Escape, { spriter::program_stop() });
        if auto {
            m68k.clock();
            vdp.borrow_mut().clock();
        }
        true
    });
}

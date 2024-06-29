extern crate spriter;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use m68k_emu::{bus::BusM68k, cpu::M68k};

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use spriter::{if_pressed, Color};
use vdp_emu::Vdp;

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

    let cpu_bus = CpuBus::init(memory_space.clone());
    let mut m68k = M68k::<CpuBus>::new();
    m68k.set_bus(cpu_bus);
    m68k.reset();

    let interrupt_line = m68k.get_interrupt_lint();
    let vdp = Rc::new(RefCell::new(Vdp::new(canvas, interrupt_line)));
    // bus.borrow_mut().set_vdp(Some(vdp.clone()));

    let mut auto = false;
    runner.run(window, move |_| {
        if_pressed!(spriter::Key::A, { auto = !auto });
        if_pressed!(spriter::Key::C, {
            m68k.clock();
            auto = false;
        });
        if_pressed!(spriter::Key::Escape, { spriter::program_stop() });
        if auto {
            m68k.clock();
        }
        true
    });
}

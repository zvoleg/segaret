extern crate spriter;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use m68k_emu::cpu::M68k;

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use signal_bus::SignalBus;
use spriter::{if_pressed, Color};
use vdp_bus::VdpBus;
use vdp_emu::vdp_emu::Vdp;

mod cpu_bus;
mod memory_space;
mod signal_bus;
mod vdp_bus;
mod vdp_emu;
// pub mod cartridge;

const VDP_CLOCK_PER_CPU: f32 = 1.75;

fn main() {
    let (runner, mut window) = spriter::init("segaret", 916 + 256, 1024);

    let mut file = File::open("pop.md").unwrap();
    let mut rom = Vec::new();
    let _ = file.read_to_end(&mut rom);
    let memory_space = Rc::new(RefCell::new(MemorySpace::new(rom)));

    let mut m68k = M68k::new();
    let signal_bus = Rc::new(RefCell::new(SignalBus::new()));
    let vdp = Rc::new(RefCell::new(Vdp::<VdpBus>::new(&mut window, signal_bus.clone())));

    let mut cpu_bus = CpuBus::init(memory_space.clone());
    cpu_bus.set_vdp_ports(vdp.clone());

    m68k.set_bus(cpu_bus);
    m68k.reset();

    let vdp_bus = VdpBus::new(memory_space.clone());
    vdp.borrow_mut().set_bus(vdp_bus);

    let mut auto = false;
    let mut run = false;
    let mut vdp_clocks_remainder = 0.0f32;
    runner.run(window, move |_| {
        if_pressed!(spriter::Key::A, { auto = !auto });
        if_pressed!(spriter::Key::C, {
            auto = false;
            run = true;
        });
        if_pressed!(spriter::Key::Escape, { spriter::program_stop() });
        if auto {
            run = true;
        }
        if run {
            let mut vdp_clocks = 1;
            if signal_bus.borrow_mut().handle_signal(signal_bus::Signal::V_INTERRUPT) {
                m68k.interrupt(6);
            }
            if !signal_bus
                .borrow_mut()
                .handle_signal(signal_bus::Signal::CPU_HALT)
            {
                let vdp_clocks_rational = m68k.clock() as f32 * VDP_CLOCK_PER_CPU + vdp_clocks_remainder;
                vdp_clocks = vdp_clocks_rational.trunc() as i32;
                vdp_clocks_remainder = vdp_clocks_rational.fract();
            }
            for _ in 0..vdp_clocks {
                vdp.borrow_mut().clock();
            }
            run = false;
        }
        true
    });
}

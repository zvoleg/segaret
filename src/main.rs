extern crate spriter;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use controller::Controller;
use log::info;
use m68k_emu::cpu::M68k;

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use signal_bus::{Signal, SignalBus};
use spriter::if_pressed;
use vdp_bus::VdpBus;
use vdp_emu::vdp_emu::Vdp;

mod controller;
mod cpu_bus;
mod memory_space;
mod signal_bus;
mod vdp_bus;
mod vdp_emu;
// pub mod cartridge;

const VDP_CLOCK_PER_CPU: f32 = 1.75;

fn main() {
    env_logger::init();
    let (runner, mut window) = spriter::init("segaret", 916 + 256, 1024);

    let mut file = File::open("pop.md").unwrap();
    let mut rom = Vec::new();
    let _ = file.read_to_end(&mut rom);
    let memory_space = Rc::new(RefCell::new(MemorySpace::new(rom)));

    let mut m68k = M68k::new();
    // m68k.set_breakpoints(vec![]);

    let signal_bus = Rc::new(RefCell::new(SignalBus::new()));
    let vdp = Rc::new(RefCell::new(Vdp::<VdpBus>::new(
        &mut window,
        signal_bus.clone(),
    )));

    let controller = Rc::new(RefCell::new(Controller::new()));

    let mut cpu_bus = CpuBus::init(memory_space.clone(), controller.clone());
    cpu_bus.set_vdp_ports(vdp.clone());

    m68k.set_bus(cpu_bus);
    m68k.reset();

    let vdp_bus = VdpBus::new(memory_space.clone());
    vdp.borrow_mut().set_bus(vdp_bus);

    let mut auto = false;
    let mut vdp_clocks_remainder = 0.0f32;
    runner.run(window, move |_| {
        let mut manual_clock = false;
        if_pressed!(spriter::Key::A, {
            auto = !auto;
            info!("Auto Clock mode = {}", auto);
        });
        if_pressed!(spriter::Key::C, {
            auto = false;
            manual_clock = true;
            info!("Manual clock");
        });
        if_pressed!(spriter::Key::Escape, {
            spriter::program_stop();
            info!("Exit from segaret");
        });
        if auto {
            let mut update_screen = false;
            let mut clock_counter = 0;
            while !update_screen && clock_counter < 71680 {
                let mut vdp_clocks = 1;
                if signal_bus
                    .borrow_mut()
                    .handle_signal(Signal::VInterrupt)
                {
                    m68k.interrupt(6);
                }
                if !signal_bus
                    .borrow_mut()
                    .handle_signal(Signal::CpuHalt)
                {
                    let vdp_clocks_rational =
                        m68k.clock() as f32 * VDP_CLOCK_PER_CPU + vdp_clocks_remainder;
                    vdp_clocks = vdp_clocks_rational.trunc() as i32;
                    vdp_clocks_remainder = vdp_clocks_rational.fract();
                }
                for _ in 0..vdp_clocks {
                    let update = vdp.borrow_mut().clock();
                    if !update_screen {
                        update_screen = update;
                    }
                }
                clock_counter += vdp_clocks;
                if m68k.breakpoint_hit {
                    info!("CPU hits breakpoint");
                    auto = false;
                    break;
                }
            }
            controller.borrow_mut().clock();
            true
        } else if manual_clock {
            let mut vdp_clocks = 1;
            if signal_bus
                .borrow_mut()
                .handle_signal(Signal::VInterrupt)
            {
                m68k.interrupt(6);
            }
            if !signal_bus
                .borrow_mut()
                .handle_signal(Signal::CpuHalt)
            {
                let vdp_clocks_rational =
                    m68k.clock() as f32 * VDP_CLOCK_PER_CPU + vdp_clocks_remainder;
                vdp_clocks = vdp_clocks_rational.trunc() as i32;
                vdp_clocks_remainder = vdp_clocks_rational.fract();
            }
            for _ in 0..vdp_clocks {
                vdp.borrow_mut().clock();
            }
            true
        } else {
            false
        }
    });
}

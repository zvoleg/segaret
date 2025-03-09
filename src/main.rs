extern crate spriter;

use std::{cell::RefCell, fs::File, io::{stdin, Read}, rc::Rc};

use controller::Controller;
use log::info;
use m68k_emu::cpu::M68k;

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use signal_bus::{Signal, SignalBus};
use spriter::{if_pressed, Key};
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
    let mut break_points: Vec<u32> = vec![];
    // let mut break_points = vec![0x4b8, 0x4f0, 0x1F9A8];
    m68k.set_breakpoints(&break_points);

    let signal_bus = Rc::new(RefCell::new(SignalBus::new()));
    let vdp = Rc::new(RefCell::new(Vdp::<VdpBus>::new(
        &mut window,
        signal_bus.clone(),
    )));

    let controller_a = Rc::new(RefCell::new(Controller::new()));
    let controller_b = Rc::new(RefCell::new(Controller::new()));

    let mut cpu_bus = CpuBus::init(memory_space.clone(), controller_a.clone(), controller_b.clone());
    cpu_bus.set_vdp_ports(vdp.clone());

    m68k.set_bus(cpu_bus);
    m68k.reset();

    let vdp_bus = VdpBus::new(memory_space.clone());
    vdp.borrow_mut().set_bus(vdp_bus);

    let mut auto = false;
    let mut by_frame = false;
    let mut vdp_clocks_remainder = 0.0f32;
    runner.run(window, move |_| {
        let mut manual_clock = false;
        if_pressed!(Key::A, {
            auto = !auto;
            info!("Auto Clock mode = {}", auto);
        });
        if_pressed!(Key::F, {
            auto = !auto;
            by_frame = true;
        });
        if_pressed!(Key::U, {
            vdp.borrow_mut().update_vram_table_on_screen();
        });
        if_pressed!(Key::V, {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            let parts = buf.split(" ").collect::<Vec<&str>>();
            if parts.len() == 2 {
                let break_point = u32::from_str_radix(parts[0], 16).unwrap();
                let oparation = parts[1].trim();
                match oparation {
                    "a" | "A" => {
                        break_points.push(break_point);
                        info!("break point set: {:08X}", break_point)
                    },
                    "d" | "D" | "r" | "R" => {
                        if let Some(position) = break_points.iter().position(|b| *b == break_point) {
                            break_points.swap_remove(position);
                            info!("break point remove: {:08X}", break_point);
                        }
                    },
                    _ => (),
                }
                m68k.set_breakpoints(&break_points);
                info!("break points list: {:08X?}", break_points);
            }
        });
        if_pressed!(Key::C, {
            auto = false;
            manual_clock = true;
            info!("Manual clock");
        });
        if_pressed!(Key::Escape, {
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
            if by_frame {
                auto = !auto;
                by_frame = false;
                info!("Frame done")
            }
            controller_a.borrow_mut().clock();
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

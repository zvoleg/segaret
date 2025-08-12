extern crate spriter;

use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::File,
    io::{stdin, Read, Write},
    rc::Rc,
    str,
};

use controller::Controller;
use log::info;
use m68k_emu::cpu::M68k;

use cpu_bus::CpuBus;
use memory_space::MemorySpace;
use signal_bus::{Signal, SignalBus};
use spriter::{if_pressed, Key};
use vdp_bus::VdpBus;
use vdp_emu::vdp_emu::Vdp;
use z80_emu::cpu::Z80;

use crate::{vdp_emu::DisplayMod, z80_bus::Z80Bus};

mod controller;
mod cpu_bus;
mod memory_space;
mod signal_bus;
mod vdp_bus;
mod vdp_emu;
mod z80_bus;
mod ym2612;

const VDP_CLOCK_PER_CPU: f32 = 1.75;

fn main() {
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        info!("no file to run");
        return;
    }

    let (runner, mut window) = spriter::init("segaret", 916 + 256, 1024);

    let mut file = File::open(&args[1]).unwrap();
    let mut rom = Vec::new();
    let _ = file.read_to_end(&mut rom);
    let region_code = rom[0x1F0];
    let display_mod = match region_code {
        45 => DisplayMod::PAL, // "E" == 45
        _ => DisplayMod::NTSC, // "JU"
    };

    let memory_space = Rc::new(RefCell::new(MemorySpace::new(rom)));

    let mut m68k = M68k::new();
    let mut break_points: Vec<u32> = vec![];
    // let mut break_points = vec![0xB74, 0x1006, 0x1854];
    m68k.set_breakpoints(&break_points);
    let signal_bus = Rc::new(RefCell::new(SignalBus::new()));
    let vdp = Rc::new(RefCell::new(Vdp::<VdpBus>::new(
        &mut window,
        signal_bus.clone(),
        display_mod,
    )));

    let controller_a = Rc::new(RefCell::new(Controller::new()));
    let controller_b = Rc::new(RefCell::new(Controller::new()));

    let z80_bus = Rc::new(Z80Bus::new(memory_space.clone()));
    let mut cpu_bus = CpuBus::init(
        memory_space.clone(),
        z80_bus.clone(),
        controller_a.clone(),
        controller_b.clone(),
        signal_bus.clone(),
    );
    cpu_bus.set_vdp_ports(vdp.clone());

    m68k.set_bus(cpu_bus);
    m68k.reset();

    let mut z80 = Z80::new();
    z80.set_bus(z80_bus.clone());

    let vdp_bus = VdpBus::new(memory_space.clone());
    vdp.borrow_mut().set_bus(vdp_bus);

    let mut auto = false;
    let mut by_frame = false;
    let mut vdp_clocks_remainder = 0.0f32;
    let mut manual_clock_counter = 0;

    let mut values_map: HashMap<u8, Vec<u32>> = HashMap::new();
    let mut downgraded_values: Vec<u8> = vec![];

    let mut z80_bus_request = false;
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
            info!("Break point manage command ('<address> a' - add break point, '<address> d - delete break point'");
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
                    }
                    "d" | "D" => {
                        if let Some(position) = break_points.iter().position(|b| *b == break_point)
                        {
                            break_points.swap_remove(position);
                            info!("break point remove: {:08X}", break_point);
                        }
                    }
                    _ => (),
                }
                m68k.set_breakpoints(&break_points);
                info!("break points list: {:08X?}", break_points);
            }
        });
        if_pressed!(Key::S, {
            info!("Search value address ('<value>' - init searching addresses, '<first value> <new value>' - search changes)");
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            let parts = buf.split(" ").map(|p| p.trim()).collect::<Vec<&str>>();
            match parts.len() {
                1 => {
                    let byte = u8::from_str_radix(parts[0], 16).unwrap();
                    let addresses = memory_space
                        .borrow()
                        .m68k_ram
                        .iter()
                        .enumerate()
                        .filter(|b| *b.1 == byte)
                        .map(|a| a.0 as u32)
                        .collect::<Vec<u32>>();
                    values_map.insert(byte, addresses);
                }
                2 => {
                    let last_value = u8::from_str_radix(parts[0], 16).unwrap();
                    let new_value = u8::from_str_radix(parts[1], 16).unwrap();
                    let addresses = values_map
                        .get(&last_value)
                        .unwrap()
                        .iter()
                        .filter(|a| memory_space.borrow().m68k_ram[(**a) as usize] == new_value)
                        .map(|a| *a)
                        .collect::<Vec<u32>>();
                    values_map.insert(last_value, addresses);
                }
                _ => (),
            }
            for (key, val) in values_map.iter() {
                info!("value {:02X}: {:08X?}", key, val);
            }
        });
        if_pressed!(Key::D, {
            info!("Searching values downgraded by one");
            if downgraded_values.len() == 0 {
                downgraded_values = memory_space.borrow().m68k_ram.clone();
            } else {
                let addresses = downgraded_values
                    .iter()
                    .enumerate()
                    .filter(|v| (*v.1 - 1) == memory_space.borrow().m68k_ram[v.0])
                    .map(|v| v.0 as u32)
                    .collect::<Vec<u32>>();
                info!("downgraded addresses {:08X?}", addresses);
                downgraded_values = memory_space.borrow().m68k_ram.clone();
            }
        });
        if_pressed!(Key::C, {
            auto = false;
            manual_clock = true;
            info!("Manual clock");
        });
        if_pressed!(Key::Z, {
            let mut dump_file = File::create("z80_dump").unwrap();
            dump_file.write_all(&memory_space.borrow().z80_ram).unwrap();
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
                if signal_bus.borrow_mut().handle_signal(Signal::VInterrupt) {
                    m68k.interrupt(6);
                }
                if signal_bus.borrow_mut().handle_signal(Signal::HInterrupt) {
                    m68k.interrupt(4);
                }
                if signal_bus.borrow_mut().handle_signal(Signal::Z80BusRequest) {
                    z80_bus_request = true;
                }
                if signal_bus.borrow_mut().handle_signal(Signal::Z80BusFree) {
                    z80_bus_request = false;
                }
                if signal_bus.borrow_mut().handle_signal(Signal::Z80Reset) {
                    z80.restart();
                }
                if !signal_bus.borrow_mut().handle_signal(Signal::CpuHalt) {
                    let vdp_clocks_rational =
                        m68k.clock() as f32 * VDP_CLOCK_PER_CPU + vdp_clocks_remainder;
                    vdp_clocks = vdp_clocks_rational.trunc() as i32;
                    vdp_clocks_remainder = vdp_clocks_rational.fract();

                    if clock_counter % 15 == 0 {
                        if !z80_bus_request {
                            z80.clock();
                        }
                    }
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
            if signal_bus.borrow_mut().handle_signal(Signal::VInterrupt) {
                    m68k.interrupt(6);
            }
            if signal_bus.borrow_mut().handle_signal(Signal::HInterrupt) {
                m68k.interrupt(4);
            }
            if signal_bus.borrow_mut().handle_signal(Signal::Z80BusRequest) {
                z80_bus_request = true;
            }
            if signal_bus.borrow_mut().handle_signal(Signal::Z80BusFree) {
                z80_bus_request = false;
            }
            if signal_bus.borrow_mut().handle_signal(Signal::Z80Reset) {
                z80.restart();
            }
            if !signal_bus.borrow_mut().handle_signal(Signal::CpuHalt) {
                let vdp_clocks_rational =
                    m68k.clock() as f32 * VDP_CLOCK_PER_CPU + vdp_clocks_remainder;
                vdp_clocks = vdp_clocks_rational.trunc() as i32;
                vdp_clocks_remainder = vdp_clocks_rational.fract();

                if manual_clock_counter % 15 == 0 {
                    if !z80_bus_request {
                        z80.clock();
                    }
                }
            }
            for _ in 0..vdp_clocks {
                vdp.borrow_mut().clock();
            }
            manual_clock_counter += 1;
            true
        } else {
            false
        }
    });
}

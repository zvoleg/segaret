use std::{cell::RefCell, rc::Rc};

use spriter::{window::Window, Canvas, Color};

use log::{info, debug};

use crate::signal_bus::{Signal, SignalBus};

use super::{
    bus::BusVdp,
    registers::{
        RegisterSet, AUTO_INCREMENT, DMA_LENGTH_H, DMA_LENGTH_L, DMA_SOURCE_H, DMA_SOURCE_L, DMA_SOURCE_M, MODE_REGISTER_II
    },
    DmaMode, RamAccessMode, Status,
};

pub struct Vdp<T: BusVdp> {
    screen: Canvas,
    pub(crate) vram_table: Canvas,

    pub(crate) raw_registers: [u8; 24],
    pub(crate) register_set: RegisterSet,

    pub(super) vram: [u8; 0x10000],
    pub(super) cram: [u8; 0x80],
    pub(super) vsram: [u8; 0x50],

    pub(super) status_register: u16,

    pub(super) v_counter: u16,
    pub(super) h_counter: u16,
    pub(super) v_counter_jumped: bool,
    pub(super) h_counter_jumped: bool,

    pub(super) line_intrpt_counter: u8,

    pub(super) control_port_write_latch: bool,

    pub(super) dma_mode: Option<DmaMode>,
    pub(super) dma_run: bool,
    pub(super) dma_data_wait: bool,

    pub(super) ram_access_mode: RamAccessMode,
    pub(super) ram_address: u32,

    pub(super) data_port_reg: u16,

    pub(super) address_setting_raw_word: u32,
    pub(super) address_setting_latch: bool,

    pub(super) bus: Option<T>,
    pub(super) signal_bus: Rc<RefCell<SignalBus>>,

    clock_counter: u64,
}

impl<T> Vdp<T>
where
    T: BusVdp,
{
    pub fn new(window: &mut Window, signal_bus: Rc<RefCell<SignalBus>>) -> Self {
        let mut screen = window.create_canvas(0, 0, 640, 448, 320, 224);
        screen.set_clear_color(Color::from_u32(0xAAAAAA));
        screen.clear();
        let mut vram_table = window.create_canvas(660, 0, 512, 1024, 256, 512);
        vram_table.set_clear_color(Color::from_u32(0xAAAACC));
        vram_table.clear();
        let raw_registers = [0; 24];
        let register_set = RegisterSet::new(&raw_registers);
        Self {
            screen,
            vram_table,

            raw_registers: [0; 24],
            register_set: register_set,

            vram: [0; 0x10000],
            cram: [0; 0x80],
            vsram: [0; 0x50],

            status_register: 0x3400,

            v_counter: 0,
            h_counter: 0,
            v_counter_jumped: false,
            h_counter_jumped: false,

            line_intrpt_counter: 0,

            control_port_write_latch: false,

            dma_mode: None,
            dma_run: false,
            dma_data_wait: false,

            ram_access_mode: RamAccessMode::VramR,
            ram_address: 0,

            data_port_reg: 0,

            address_setting_raw_word: 0,
            address_setting_latch: false,

            bus: None,
            signal_bus: signal_bus,

            clock_counter: 0,
        }
    }

    pub fn set_bus(&mut self, bus: T) {
        self.bus = Some(bus);
    }

    pub fn clock(&mut self) -> bool {
        let mut update_screen = false;
        if let Some(_) = self.dma_mode.as_ref() {
            self.dma_clock();
        }
        if self.clock_counter % 286720 == 0 { // each frame?
            self.update_vram_table();
            if self.raw_registers[MODE_REGISTER_II] &0x20 == 0x20 {
                self.signal_bus.borrow_mut().push_siganal(Signal::V_INTERRUPT);
            }
            update_screen = true;
        }
        self.clock_counter = self.clock_counter.wrapping_add(1);
        update_screen
        // if self.h_interrupt_enable && self.line_intrpt_counter == 0 {
        //     self.interrupt_line.borrow_mut().send(4);
        //     self.line_intrpt_counter = self.line_intrpt_counter_value;
        // }
        // if self.v_interrupt_enable && self.v_counter == 0xE0 && self.h_counter == 0x08 {
        //     unsafe {
        //         (*self.bus).send_interrupt(6);
        //     }
        //     self.set_status(Status::V_INTRPT_PENDING, true);
        // }
        // self.update_counters();
        // for x in 0..320 {
        //     for y in 0..224 {
        //         let pixel_color = rand::random::<u32>() % 2;
        //         self.screen.set_pixel(x, y, Color::from_u32(0xFFFFFF * pixel_color)).unwrap();
        //     }
        // }
    }

    fn dma_clock(&mut self) {
        let dma_enabled = self.raw_registers[MODE_REGISTER_II] & 0x10 != 0;
        if dma_enabled && self.dma_run {
            let dma_length = self.get_dma_length();
            debug!("VDP: clock: dma enabled, dma cycles remined {}", dma_length);
            match self.dma_mode.as_ref().unwrap() {
                DmaMode::BusToRamCopy => self.dma_bus_to_ram_copy(),
                DmaMode::RamToRamCopy => (),
                DmaMode::RamFill => self.dma_ram_fill(),
            }
            if self.get_dma_length() == 0 {
                self.raw_registers[MODE_REGISTER_II] = self.raw_registers[MODE_REGISTER_II] & !0x10;
                self.dma_mode = None;
                self.dma_run = false;
                return;
            }
        }
    }

    fn dma_bus_to_ram_copy(&mut self) {
        let src_address = self.get_dma_src_address();
        let dst_address = self.ram_address;
        let data = self.bus.as_ref().unwrap().read(src_address);
        debug!("VDP: dma_bus_to_ram_copy: transfer word: {:04X}", data);
        unsafe {
            let ptr = match self.ram_access_mode {
                RamAccessMode::VramW => (&self.vram as *const u8).offset(dst_address as isize) as *mut u16,
                RamAccessMode::CramW => (&self.cram as *const u8).offset(dst_address as isize) as *mut u16,
                RamAccessMode::VSramW => (&self.vsram as *const u8).offset(dst_address as isize) as *mut u16,
                _ => panic!(
                    "VDP: dma_bus_to_ram_copy: unexpected RamAccessMode during of the DMA cycles: {}",
                    self.ram_access_mode
                ),
            };
            *ptr = data;
        }
        self.set_dma_src_address(src_address + 2);
        self.ram_address += self.get_address_increment();
        let dma_length = self.get_dma_length();
        self.set_dma_length(dma_length - 1);
        self.signal_bus.borrow_mut().push_siganal(Signal::CPU_HALT);
    }

    fn dma_ram_fill(&mut self) {
        let dst_address = self.ram_address as usize;
        let data = self.data_port_reg;
        let msb = (data >> 8) as u8;
        let lsb = data as u8;
        debug!(
            "VDP: dma_ram_fill: fill address {:08X} with data {:04X}",
            dst_address, data
        );
        match self.ram_access_mode {
            RamAccessMode::VramW => {
                if dst_address & 0x1 == 0 {
                    // even address
                    self.vram[dst_address] = lsb;
                    self.vram[dst_address + 1] = msb;
                } else {
                    // odd address
                    self.vram[dst_address] = lsb;
                    self.vram[dst_address - 1] = msb;
                }
            }
            _ => panic!(
                "VDP: dma_ram_fill: unexpected RamAccessMode during of the DMA cycles: {}",
                self.ram_access_mode
            ),
        }
        self.ram_address += self.get_address_increment();
        let dma_length = self.get_dma_length();
        self.set_dma_length(dma_length - 1);
    }

    fn update_vram_table(&mut self) {
        let color_table = [
            Color::from_u32(0xCCCCFF),
            Color::from_u32(0xAAAAAA),
            Color::from_u32(0x9999CC),
            Color::from_u32(0xCC9999),
            Color::from_u32(0x99CC99),
            Color::from_u32(0x883333),
            Color::from_u32(0x333388),
            Color::from_u32(0x338833),
            Color::from_u32(0xAAAA33),
            Color::from_u32(0x999999),
            Color::from_u32(0xCCCCCC),
            Color::from_u32(0x888888),
            Color::from_u32(0x333377),
            Color::from_u32(0x228888),
            Color::from_u32(0x555555),
            Color::from_u32(0x000000),
        ];
        for tile_idx in 0..2048 {
            for byte_idx in 0..32 {
                let idx = tile_idx * 32 + byte_idx;
                let data_byte = self.vram[idx].rotate_left(4);
                for pixel_num in 0..2 {
                    let x = (tile_idx % 32) * 8 + (byte_idx % 4) * 2 + pixel_num;
                    let y = (tile_idx / 32) * 8 + byte_idx / 4;
                    let dot = ((data_byte >> (4 * pixel_num)) & 0xF) as usize;
                    self.vram_table.set_pixel(x as i32, y as i32, color_table[dot]).unwrap();
                }
            }
        }
    }

    fn get_dma_src_address(&self) -> u32 {
        ((self.raw_registers[DMA_SOURCE_H] as u32) << 17)
            | ((self.raw_registers[DMA_SOURCE_M] as u32) << 9)
            | (self.raw_registers[DMA_SOURCE_L] as u32) << 1
    }

    fn set_dma_src_address(&mut self, address: u32) {
        self.raw_registers[DMA_SOURCE_L] = (address >> 1) as u8;
        self.raw_registers[DMA_SOURCE_M] = (address >> 9) as u8;
        self.raw_registers[DMA_SOURCE_H] = (address >> 17) as u8;
    }

    fn get_dma_length(&self) -> u32 {
        ((self.raw_registers[DMA_LENGTH_H] as u32) << 8) | self.raw_registers[DMA_LENGTH_L] as u32
    }

    fn set_dma_length(&mut self, value: u32) {
        self.raw_registers[DMA_LENGTH_L] = value as u8;
        self.raw_registers[DMA_LENGTH_H] = (value >> 8) as u8;
    }

    pub(crate) fn get_address_increment(&self) -> u32 {
        self.raw_registers[AUTO_INCREMENT] as u32
    }

    fn update_counters(&mut self) {
        self.h_counter += 1;

        if !self.h_counter_jumped && self.h_counter == 0xEA {
            self.h_counter = 0x93;
            self.h_counter_jumped = true;
        }
        if self.h_counter == 0x100 {
            self.h_counter = 0;
            self.h_counter_jumped = false;

            self.v_counter += 1;
        }

        if self.h_counter == 0xE4 {
            self.set_status(Status::H_BLANKING, true);
        }
        if self.h_counter == 0x08 {
            self.set_status(Status::H_BLANKING, false);
        }

        if !self.v_counter_jumped && self.v_counter == 0xEB {
            self.v_counter = 0xE5;
            self.v_counter_jumped = true;
        }
        if self.v_counter == 0x100 {
            self.v_counter = 0;
            self.v_counter_jumped = false;

            // self.line_intrpt_counter = self.line_intrpt_counter_value;
            self.set_status(Status::V_INTRPT_PENDING, false);
        }

        if self.v_counter == 0xE0 && self.h_counter == 0xAA {
            self.set_status(Status::V_BLANKING, true);
        }
        if self.v_counter == 0xFF && self.h_counter == 0xAA {
            self.set_status(Status::V_BLANKING, false);
        }

        // TODO add update line itrpt countr on lines between 225 and 261
    }

    fn set_status(&mut self, status: Status, set: bool) {
        let mask = 1 << status as u16;
        if set {
            self.status_register = self.status_register | mask;
        } else {
            self.status_register = self.status_register & !mask;
        }
    }
}

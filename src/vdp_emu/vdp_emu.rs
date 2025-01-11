use std::{cell::RefCell, rc::Rc};

use spriter::Canvas;

use crate::signal_bus::SignalBus;

use super::{
    bus::BusVdp,
    registers::{
        AUTO_INCREMENT, DMA_LENGTH_I, DMA_LENGTH_II, DMA_SOURC_I, DMA_SOURC_II, DMA_SOURC_III,
        MODE_REGISTER_I, MODE_REGISTER_II,
    },
    DmaMode, RamAccessMode, Status,
};

pub struct Vdp<T: BusVdp> {
    pub(super) screen: Canvas,

    pub(super) registers: [u8; 24],

    pub(super) vram: [u8; 0x10000],
    pub(super) cram: [u16; 0x40],
    pub(super) vsram: [u16; 0x28],

    pub(super) status_register: u16,

    pub(super) v_counter: u16,
    pub(super) h_counter: u16,
    pub(super) v_counter_jumped: bool,
    pub(super) h_counter_jumped: bool,

    pub(super) line_intrpt_counter: u8,

    pub(super) control_port_write_latch: bool,

    pub(super) dma_mode: Option<DmaMode>,

    pub(super) ram_access_mode: RamAccessMode,
    pub(super) ram_address: u32,

    pub(super) data_port_reg: u16,
    
    pub(super) address_setting_raw_word: u32,
    pub(super) address_setting_latch: bool,
    
    pub(super) bus: Option<T>,
    pub(super) signal_bus: Rc<RefCell<SignalBus>>,
}

impl<T> Vdp<T>
where
    T: BusVdp,
{
    pub fn new(canvas: Canvas, signal_bus: Rc<RefCell<SignalBus>>) -> Self {
        Self {
            screen: canvas,

            registers: [0; 24],

            vram: [0; 0x10000],
            cram: [0; 0x40],
            vsram: [0; 0x28],

            status_register: 0x3400,

            v_counter: 0,
            h_counter: 0,
            v_counter_jumped: false,
            h_counter_jumped: false,

            line_intrpt_counter: 0,

            control_port_write_latch: false,

            dma_mode: None,

            ram_access_mode: RamAccessMode::VramR,
            ram_address: 0,

            data_port_reg: 0,

            address_setting_raw_word: 0,
            address_setting_latch: false,

            bus: None,
            signal_bus: signal_bus,
        }
    }

    pub fn set_bus(&mut self, bus: T) {
        self.bus = Some(bus);
    }

    pub fn clock(&mut self) {
        if let Some(dma_mode) = self.dma_mode.as_ref() {
            let dma_enabled = self.registers[MODE_REGISTER_II] & 0x10 != 0;
            if dma_enabled {
                let dma_length = self.get_dma_length();
                println!("VDP: clock: dma enabled, dma cycles remined {}", dma_length);
                if dma_length == 0 {
                    self.registers[MODE_REGISTER_II] = self.registers[MODE_REGISTER_II] & !0x10;
                    self.dma_mode = None;
                    return;
                }
                match dma_mode {
                    DmaMode::BusToRamCopy => self.dma_bus_to_ram_copy(),
                    DmaMode::RamToRamCopy => (),
                    DmaMode::RamFill => self.dma_ram_fill(),
                }
            }
        } else {
            // TODO
        }
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

    fn dma_bus_to_ram_copy(&mut self) {
        let src_address = self.get_dma_src_address();
        let dst_address = self.ram_address;
        let data = self.bus.as_ref().unwrap().read(src_address);
        println!("VDP: dma_bus_to_ram_copy: transfer word: {:04X}", data);
        match self.ram_access_mode {
            RamAccessMode::VramW => self.vram[dst_address as usize] = data as u8,
            RamAccessMode::CramW => self.cram[dst_address as usize] = data,
            RamAccessMode::VSramW => self.vsram[dst_address as usize] = data,
            _ => panic!(
                "VDP: dma_bus_to_ram_copy: unexpected RamAccessMode during of the DMA cycles: {}",
                self.ram_access_mode
            ),
        }
        self.set_dma_src_address(src_address + 2);
        self.ram_address += self.get_address_increment();
        let dma_length = self.get_dma_length();
        self.set_dma_length(dma_length - 1);
    }

    fn dma_ram_fill(&mut self) {
        let dst_address = self.ram_address as usize;
        let data = self.data_port_reg;
        let msb = (data >> 8) as u8;
        let lsb = data as u8;
        println!(
            "VDP: dma_ram_fill: fill address {:08X} with data {:04X}",
            dst_address, data
        );
        match self.ram_access_mode {
            RamAccessMode::VramW => {
                // even address
                if dst_address & 0x1 == 0 {
                    self.vram[dst_address] = lsb;
                    self.vram[dst_address + 1] = msb;
                } else {
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

    fn get_dma_src_address(&self) -> u32 {
        ((self.registers[DMA_SOURC_III] as u32) << 17)
            | ((self.registers[DMA_SOURC_II] as u32) << 9)
            | (self.registers[DMA_SOURC_I] as u32) << 1
    }

    fn set_dma_src_address(&mut self, address: u32) {
        self.registers[DMA_SOURC_I] = (address >> 1) as u8;
        self.registers[DMA_SOURC_II] = (address >> 9) as u8;
        self.registers[DMA_SOURC_III] = (address >> 17) as u8;
    }

    fn get_dma_length(&self) -> u32 {
        ((self.registers[DMA_LENGTH_II] as u32) << 8) | self.registers[DMA_LENGTH_I] as u32
    }

    fn set_dma_length(&mut self, value: u32) {
        self.registers[DMA_LENGTH_I] = value as u8;
        self.registers[DMA_LENGTH_II] = (value >> 8) as u8;
    }

    fn get_address_increment(&self) -> u32 {
        self.registers[AUTO_INCREMENT] as u32
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

use std::{cell::RefCell, rc::Rc};

use m68k_emu::interrupt_line::InterruptLine;
use spriter::Canvas;

use crate::{bus::BusVdp, DmaMode, RamAccessMode, Status};

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

    pub(super) ram_access_mode: Option<RamAccessMode>,
    pub(super) ram_address: u16,

    pub(super) interrupt_line: Option<Rc<RefCell<InterruptLine>>>,

    pub(super) address_setting_raw_word: u32,
    pub(super) address_setting_latch: bool,

    pub(super) bus: Option<T>,
}

impl<T> Vdp<T>
where
    T: BusVdp,
{
    pub fn new(canvas: Canvas) -> Self {
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

            ram_access_mode: None,
            ram_address: 0,

            interrupt_line: None,
            address_setting_raw_word: 0,
            address_setting_latch: false,

            bus: None,
        }
    }

    pub fn set_interrupt_line(&mut self, interrupt_line: Rc<RefCell<InterruptLine>>) {
        self.interrupt_line = Some(interrupt_line);
    }

    pub fn set_bus(&mut self, bus: T) {
        self.bus = Some(bus);
    }

    pub fn clock(&mut self) {
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

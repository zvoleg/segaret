use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use m68k_emu::interrupt_line::{self, InterruptLine};
use spriter::Canvas;

pub mod vdp_port;

mod registers;

#[derive(PartialEq)]
enum DisplayMod {
    PAL,
    NTSC,
}

impl DisplayMod {
    fn line_amount(&self) -> u32 {
        match self {
            DisplayMod::PAL => 240,
            DisplayMod::NTSC => 224,
        }
    }
}

enum RamAccessMode {
    VramR,
    VramW,
    CramR,
    CramW,
    VSramR,
    VSramW,
}

impl Display for RamAccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode_str = match self {
            RamAccessMode::VramR => "VramR",
            RamAccessMode::VramW => "VramW",
            RamAccessMode::CramR => "CramR",
            RamAccessMode::CramW => "CramW",
            RamAccessMode::VSramR => "VSramR",
            RamAccessMode::VSramW => "VSramW",
        };
        write!(f, "{}", mode_str)
    }
}

impl RamAccessMode {
    fn new(mask: u16) -> RamAccessMode {
        match mask {
            0b0000 => RamAccessMode::VramR,
            0b0001 => RamAccessMode::VramW,
            0b0011 => RamAccessMode::CramW,
            0b0100 => RamAccessMode::VSramR,
            0b0101 => RamAccessMode::VSramW,
            0b1000 => RamAccessMode::CramR,
            _ => panic!("RamAccessMode: new: unexpected mode mask {:05b}", mask),
        }
    }
}

enum Status {
    PAL = 0,
    DMA_PROGRESS = 1,
    H_BLANKING = 2,
    V_BLANKING = 3,
    ODD_FRAME = 4,
    SPITE_COLLISION = 5,
    SPRITE_OVERFLOW = 6,
    V_INTRPT_PENDING = 7,
    FIFO_FULL = 8,
    FIFO_EMPTY = 9,
}

enum DmaMode {
    BusToRamCopy,
    RamToRamCopy,
    RamFill,
}

impl Display for DmaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode_str = match self {
            DmaMode::BusToRamCopy => "memory to vdp ram",
            DmaMode::RamToRamCopy => "vdp ram to vdp ram copy",
            DmaMode::RamFill => "vdp ram filling",
        };
        write!(f, "{}", mode_str)
    }
}

pub trait BusVdp {
    fn read_data_port(&self) -> u16;
    fn write_data_port(&self, data: u16);
    fn read_control_port(&self) -> u16;
    fn write_control_port(&self, data: u16);
    fn send_interrupt(&self, level: i32);
}

pub struct Vdp {
    screen: Canvas,

    registers: [u8; 24],

    vram: [u8; 0x10000],
    cram: [u16; 0x40],
    vsram: [u16; 0x28],

    status_register: u16,

    v_counter: u16,
    h_counter: u16,
    v_counter_jumped: bool,
    h_counter_jumped: bool,

    line_intrpt_counter: u8,

    control_port_write_latch: bool,

    dma_mode: Option<DmaMode>,

    ram_access_mode: Option<RamAccessMode>,
    ram_address: u16,

    interrupt_line: Option<Rc<RefCell<InterruptLine>>>,

    address_setting_raw_word: u32,
    address_setting_latch: bool,
}

impl Vdp {
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
        }
    }

    pub fn set_interrupt_line(&mut self, interrupt_line: Rc<RefCell<InterruptLine>>) {
        self.interrupt_line = Some(interrupt_line);
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

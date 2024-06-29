use std::cell::RefCell;
use std::rc::Rc;

use m68k_emu::interrupt_line::InterruptLine;
use spriter::Canvas;

pub mod vdp_port;

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
    VRAM_R,
    VRAM_W,
    CRAM_R,
    CRAM_W,
    VSRAM_R,
    VSRAM_W,
    TMP,
}

impl RamAccessMode {
    fn get_access_mode(mask: u16) -> RamAccessMode {
        match mask {
            0b0000 => RamAccessMode::VRAM_R,
            0b0001 => RamAccessMode::VRAM_W,
            0b0011 => RamAccessMode::CRAM_W,
            0b0100 => RamAccessMode::VSRAM_R,
            0b0101 => RamAccessMode::VSRAM_W,
            0b1000 => RamAccessMode::CRAM_R,
            _ => RamAccessMode::TMP,
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

pub trait BusVdp {
    fn read_data_port(&self) -> u16;
    fn write_data_port(&self, data: u16);
    fn read_control_port(&self) -> u16;
    fn write_control_port(&self, data: u16);
    fn send_interrupt(&self, level: i32);
}

pub struct Vdp {
    screen: Canvas,

    vram: [u8; 0x100],
    cram: [u16; 0x40],
    vsram: [u16; 0x28],

    status_register: u16,

    v_counter: u16,
    h_counter: u16,
    v_counter_jumped: bool,
    h_counter_jumped: bool,

    line_intrpt_counter: u8,

    control_port_write_latch: bool,
    first_command_word: u16,
    second_command_word: u16,

    ram_access_bits: u16,
    ram_address: u16,

    registers: [u8; 24],

    interrupt_line: Rc<RefCell<InterruptLine>>,
}

impl Vdp {
    pub fn new(canvas: Canvas, interrupt_line: Rc<RefCell<InterruptLine>>) -> Self {
        Self {
            screen: canvas,

            vram: [0; 0x100],
            cram: [0; 0x40],
            vsram: [0; 0x28],

            status_register: 0x3400,

            v_counter: 0,
            h_counter: 0,
            v_counter_jumped: false,
            h_counter_jumped: false,

            line_intrpt_counter: 0,

            control_port_write_latch: false,
            first_command_word: 0,
            second_command_word: 0,
            ram_access_bits: 0,
            ram_address: 0,

            registers: [0; 24],

            interrupt_line: interrupt_line,
        }
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

    pub fn write_data_port(&mut self, data: u16) {
        // self.control_port_write_latch = false;
        // let access_mode = RamAccessMode::get_access_mode(self.ram_access_bits);
        // match access_mode {
        //     RamAccessMode::VRAM_W => unsafe {
        //         let vram_ptr = self.vram.as_mut_ptr().offset(self.ram_address as isize);
        //         let vram_ptr = vram_ptr as *mut _ as *mut u16;
        //         (*vram_ptr) = data; // TODO shoud be some magic with swapping bytes
        //     },
        //     RamAccessMode::CRAM_W => self.cram[self.ram_address as usize] = data,
        //     RamAccessMode::VSRAM_W => self.vsram[self.ram_address as usize] = data,
        //     _ => (),
        // }
        // self.ram_address = self.ram_address.wrapping_add(self.address_increment as u16);
    }

    pub fn read_data_port(&mut self) -> u16 {
        // self.control_port_write_latch = false;
        // let access_mode = RamAccessMode::get_access_mode(self.ram_access_bits);
        // let data = match access_mode {
        //     RamAccessMode::VRAM_R => unsafe {
        //         let vram_ptr = self.vram.as_ptr().offset(self.ram_address as isize);
        //         let vram_ptr = vram_ptr as *const _ as *const u16;
        //         *vram_ptr
        //     },
        //     RamAccessMode::CRAM_R => self.cram[self.ram_address as usize],
        //     RamAccessMode::VSRAM_R => self.vsram[self.ram_address as usize],
        //     _ => 0,
        // };
        // self.ram_address = self.ram_address.wrapping_add(self.address_increment as u16);
        // data
        0
    }

    pub fn read_control_port(&mut self) -> u16 {
        self.control_port_write_latch = false;
        self.status_register
    }

    pub fn write_control_port(&mut self, data: u16) {
        let reg_setup_mode = data & 0x8000 != 0;

        if reg_setup_mode && !self.control_port_write_latch {
            let reg_idx = (data >> 8) & 0x1F;
            let reg_data = data & 0xFF;

            self.registers[reg_idx as usize] = reg_data as u8;
            println!("VDP: setup register {} = {:02X}", reg_idx, reg_data as u8);
        } else {
            if !self.control_port_write_latch {
                self.first_command_word = data;

                let access_bits = self.first_command_word >> 14;
                let address_bits = self.first_command_word & 0x3FFF;

                self.ram_access_bits = (self.ram_access_bits & !0x0003) | access_bits;
                self.ram_address = (self.ram_address & 0xC000) | address_bits;
            } else {
                self.second_command_word = data;

                let access_bits = (self.second_command_word & 0x00F0) >> 4;
                let address_bits = self.second_command_word & 0x0003;

                self.ram_access_bits = (self.ram_access_bits & !0xFFFC) | (access_bits << 2);
                self.ram_address = (self.ram_address & !0xC000) | (address_bits << 14);
            }
            self.control_port_write_latch = !self.control_port_write_latch;
        }
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

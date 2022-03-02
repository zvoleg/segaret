use crate::Bus;
use spriter::Color;
use rand;

use crate::hardware::Size;
use spriter::Canvas;

#[derive(PartialEq)]
enum DisplayMod {
    PAL,
    NTSC,
}

impl DisplayMod {
    fn line_amount(&self) -> u32 {
        match self {
            PAL => 240,
            NTSC => 224,
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

    //reg_0
    h_interrupt_enable: bool,
    hv_counter_disable: bool,
    pallete_select: bool,
    display_off: bool, // off image generation

    //reg_1
    display_enable: bool, // fill line with backdrop color
    v_interrupt_enable: bool,
    dma_enable: bool,
    display_mode: DisplayMod,

    //reg_2
    scroll_a_name_tbl_addr: u16,

    //reg_3
    window_name_tbl_addr: u16,

    //reg_4
    scroll_b_name_tbl_addr: u16,

    //reg_5
    spite_attr_base_addr: u16,

    //reg_7
    backdrop_color: u16,

    //reg_0A
    line_intrpt_counter_value: u8,

    //reg_0F
    address_increment: u8,

    dma_counter_reg_19: u8,
    dma_counter_reg_20: u8,

    dma_address_reg_21: u8,
    dma_address_reg_22: u8,
    dma_address_reg_23: u8,

    bus: *mut Bus,
}

impl Vdp {
    pub fn init(canvas: Canvas) -> Self {
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

            //reg_0
            h_interrupt_enable: false,
            hv_counter_disable: false,
            pallete_select: false,
            display_off: false, // off image generation

            //reg_1
            display_enable: false, // fill line with backdrop color
            v_interrupt_enable: false,
            dma_enable: false,
            display_mode: DisplayMod::NTSC,

            //reg_2
            scroll_a_name_tbl_addr: 0,

            //reg_3
            window_name_tbl_addr: 0,

            //reg_4
            scroll_b_name_tbl_addr: 0,

            //reg_5
            spite_attr_base_addr: 0,

            //reg_7
            backdrop_color: 0,

            //reg_0A
            line_intrpt_counter_value: 0,

            //reg_0F
            address_increment: 0,

            dma_counter_reg_19: 0,
            dma_counter_reg_20: 0,

            dma_address_reg_21: 0,
            dma_address_reg_22: 0,
            dma_address_reg_23: 0,

            bus: std::ptr::null_mut()
        }
    }

    pub fn set_bus(&mut self, bus: *mut Bus) {
        self.bus = bus;
    }

    pub fn clock(&mut self) {
        if self.h_interrupt_enable && self.line_intrpt_counter == 0 {
            unsafe {
                (*self.bus).send_interrupt(4);
            }
            self.line_intrpt_counter = self.line_intrpt_counter_value;
        }
        if self.v_interrupt_enable && self.v_counter == 0xE0 && self.h_counter == 0x08 {
            unsafe {
                (*self.bus).send_interrupt(6);
            }
            self.set_status(Status::V_INTRPT_PENDING, true);
        }
        self.update_counters();
        // for x in 0..320 {
        //     for y in 0..224 {
        //         let pixel_color = rand::random::<u32>() % 2;
        //         self.screen.set_pixel(x, y, Color::from_u32(0xFFFFFF * pixel_color)).unwrap();
        //     }
        // }
    }

    pub fn write_data_port(&mut self, data: u16) {
        self.control_port_write_latch = false;
        let access_mode = RamAccessMode::get_access_mode(self.ram_access_bits);
        match access_mode {
            RamAccessMode::VRAM_W => unsafe {
                let vram_ptr = self.vram.as_mut_ptr().offset(self.ram_address as isize);
                let vram_ptr = vram_ptr as *mut _ as *mut u16;
                (*vram_ptr) = data; // TODO shoud be some magic with swapping bytes
            },
            RamAccessMode::CRAM_W => self.cram[self.ram_address as usize] = data,
            RamAccessMode::VSRAM_W => self.vsram[self.ram_address as usize] = data,
            _ => (),
        }
        self.ram_address = self.ram_address.wrapping_add(self.address_increment as u16);
    }
    
    pub fn read_data_port(&mut self) -> u16 {
        self.control_port_write_latch = false;
        let access_mode = RamAccessMode::get_access_mode(self.ram_access_bits);
        let data = match access_mode {
            RamAccessMode::VRAM_R => unsafe {
                let vram_ptr = self.vram.as_ptr().offset(self.ram_address as isize);
                let vram_ptr = vram_ptr as *const _ as *const u16;
                *vram_ptr
            },
            RamAccessMode::CRAM_R => self.cram[self.ram_address as usize],
            RamAccessMode::VSRAM_R => self.vsram[self.ram_address as usize],
            _ => 0,
        };
        self.ram_address = self.ram_address.wrapping_add(self.address_increment as u16);
        data
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
            
            self.setup_reg(reg_idx, reg_data);
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
    
    fn setup_reg(&mut self, reg_idx: u16, reg_data: u16) {
        match reg_idx {
            0 => {
                let h_interrupt_enable_bit = (reg_data >> 4) & 1 != 0;
                self.h_interrupt_enable = h_interrupt_enable_bit;
                let hv_counter_stop_bit = (reg_data >> 1) & 1 != 0;
                self.hv_counter_disable = hv_counter_stop_bit;
            },
            1 => {
                let v_interrupt_enable_bit = (reg_data >> 5) & 1 != 0;
                self.v_interrupt_enable = v_interrupt_enable_bit;
            },
            2 => {
                let addr_msb = reg_data >> 3;
                self.scroll_a_name_tbl_addr = addr_msb << 13;
            },
            3 => {
                let mut addr_msb = reg_data >> 1;
                if self.display_mode == DisplayMod::NTSC {
                    addr_msb &= !0x1;
                }
                self.window_name_tbl_addr = addr_msb << 11;
            },
            4 => {
                self.scroll_b_name_tbl_addr = reg_data << 13
            },
            5 => {
                let mut addr_bits = reg_data;
                if self.display_mode == DisplayMod::NTSC {
                    addr_bits &= !0x1;
                }
                self.spite_attr_base_addr = addr_bits << 9;  
            },
            6 => {
                
            },
            7 => {
                
            },
            8 => {
                
            },
            9 => {
                
            },
            10 => {
                self.line_intrpt_counter_value = reg_data as u8;
            },
            11 => {
                
            },
            12 => {
                
            },
            13 => {
                
            },
            14 => {
                
            },
            0x0F => {
                self.address_increment = reg_data as u8;
            },
            16 => {
                
            },
            17 => {
                
            },
            18 => {
                
            },
            19 => {
                self.dma_counter_reg_19 = reg_data as u8;
            },
            20 => {
                self.dma_counter_reg_20 = reg_data as u8;
            },
            21 => {
                self.dma_address_reg_21 = reg_data as u8;
            },
            22 => {
                self.dma_address_reg_22 = reg_data as u8;
            },
            23 => {
                self.dma_address_reg_23 = reg_data as u8;
            },
            _ => (),
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

            self.line_intrpt_counter = self.line_intrpt_counter_value;
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

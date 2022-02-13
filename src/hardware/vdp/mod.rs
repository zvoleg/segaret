use spriter::Color;
use rand;

use crate::hardware::Size;
use spriter::Canvas;

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

pub struct Vdp {
    screen: Canvas,

    vram: [u8; 0x100],
    cram: [u16; 0x40],
    vsram: [u16; 0x28],

    control_port_write_latch: bool,
    first_word_to_access_ram: u16,
    second_word_to_access_ram: u16,
    ram_access_mode: RamAccessMode,
    ram_address: u16,

    column_counter: u32,
    line_counter: u32,

    //reg_0
    h_interrupt_enable: bool,
    hv_counter_enable: bool,
    pallete_select: bool,
    display_off: bool, // off image generation

    //reg_1
    display_enable: bool, // fill line with backdrop color
    v_interrupt_enable: bool,
    dma_enable: bool,
    display_mode: DisplayMod,

    //reg_7
    backdrop_color: u16,

    //reg_10
    h_interrupts_counter: u16,

    dma_counter_reg_19: u8,
    dma_counter_reg_20: u8,

    dma_address_reg_21: u8,
    dma_address_reg_22: u8,
    dma_address_reg_23: u8,
}

impl Vdp {
    pub fn init(canvas: Canvas) -> Self {
        Self {
            screen: canvas,

            vram: [0; 0x100],
            cram: [0; 0x40],
            vsram: [0; 0x28],

            control_port_write_latch: false,
            first_word_to_access_ram: 0,
            second_word_to_access_ram: 0,
            ram_access_mode: RamAccessMode::VRAM_R,
            ram_address: 0,

            column_counter: 0,
            line_counter: 0,

            //reg_0
            h_interrupt_enable: false,
            hv_counter_enable: false,
            pallete_select: false,
            display_off: false, // off image generation

            //reg_1
            display_enable: false, // fill line with backdrop color
            v_interrupt_enable: false,
            dma_enable: false,
            display_mode: DisplayMod::NTSC,

            //reg_7
            backdrop_color: 0,

            //reg_10
            h_interrupts_counter: 0,

            dma_counter_reg_19: 0,
            dma_counter_reg_20: 0,

            dma_address_reg_21: 0,
            dma_address_reg_22: 0,
            dma_address_reg_23: 0,
        }
    }

    pub fn clock(&mut self) {
        // for x in 0..320 {
        //     for y in 0..224 {
        //         let pixel_color = rand::random::<u32>() % 2;
        //         self.screen.set_pixel(x, y, Color::from_u32(0xFFFFFF * pixel_color)).unwrap();
        //     }
        // }
    }

    pub fn calculate_data_access(&mut self) {
        let lower_mode_bits = self.first_word_to_access_ram >> 14;
        let higher_mode_bits = (self.second_word_to_access_ram >> 4) & 0xF;
        let mode_bits = (higher_mode_bits << 2) | lower_mode_bits;
        let ram_access_mode = RamAccessMode::get_access_mode(mode_bits);

        let lower_addr_bits = self.first_word_to_access_ram & 0x3FFF;
        let higher_addr_bits = self.second_word_to_access_ram & 0x3;
        let ram_addr = (higher_addr_bits << 14) | lower_addr_bits;

        self.ram_access_mode = ram_access_mode;
        self.ram_address = ram_addr;
    }

    pub fn write_data_port(&mut self, data: u16) {
        self.control_port_write_latch = false;
        self.calculate_data_access();
        match self.ram_access_mode {
            RamAccessMode::VRAM_W => unsafe {
                let vram_ptr = self.vram.as_mut_ptr().offset(self.ram_address as isize);
                let vram_ptr = vram_ptr as *mut _ as *mut u16;
                (*vram_ptr) = data; // TODO shoud be some magic with swapping bytes
            },
            RamAccessMode::CRAM_W => self.cram[self.ram_address as usize] = data,
            RamAccessMode::VSRAM_W => self.vsram[self.ram_address as usize] = data,
            _ => (), //panic!("wrond RamAccessMode during write data port")
        }
    }
    
    pub fn read_data_port(&mut self) -> u16 {
        self.control_port_write_latch = false;
        self.calculate_data_access();
        match self.ram_access_mode {
            RamAccessMode::VRAM_R => unsafe {
                let vram_ptr = self.vram.as_ptr().offset(self.ram_address as isize);
                let vram_ptr = vram_ptr as *const _ as *const u16;
                *vram_ptr
            },
            RamAccessMode::CRAM_R => self.cram[self.ram_address as usize],
            RamAccessMode::VSRAM_R => self.vsram[self.ram_address as usize],
            _ => panic!("wrond RamAccessMode during read data port")
        }
    }
    
    pub fn read_control_port(&mut self) -> u16 {
        self.control_port_write_latch = false;
        0
    }

    pub fn write_control_port(&mut self, data: u16) {
        let reg_setup_mode = data & 0x8000 != 0;

        if reg_setup_mode {
            let reg_idx = (data >> 8) & 0x1F;
            let reg_data = data & 0xFF;

            self.setup_reg(reg_idx, reg_data);
        } else {
            if !self.control_port_write_latch {
                self.first_word_to_access_ram = data;
            } else {
                self.second_word_to_access_ram = data;
            }
            self.control_port_write_latch = !self.control_port_write_latch;
        }
    }

    fn setup_reg(&mut self, reg_idx: u16, reg_data: u16) {
        match reg_idx {
            0 => {
                
            },
            1 => {
                
            },
            2 => {
                
            },
            3 => {
                
            },
            4 => {
                
            },
            5 => {
                
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
                
            },
            11 => {
                
            },
            12 => {
                
            },
            13 => {
                
            },
            14 => {
                
            },
            15 => {
                
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
}

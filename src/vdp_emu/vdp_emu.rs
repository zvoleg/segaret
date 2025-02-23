use std::{cell::RefCell, rc::Rc};

use spriter::{window::Window, Canvas, Color};

use log::debug;

use crate::signal_bus::{Signal, SignalBus};

use super::{
    bus::BusVdp,
    dot::{Dot, Priority},
    registers::{RegisterSet, StatusFlag},
    DmaMode, RamAccessMode,
};

pub struct Vdp<T: BusVdp> {
    screen: Canvas,
    pub(crate) vram_table: Canvas,

    pub(crate) register_set: RegisterSet,

    pub(crate) vram: [u8; 0x10000],
    pub(crate) cram: [u8; 0x80],
    pub(crate) vsram: [u8; 0x50],

    pub(crate) v_counter: u16,
    pub(crate) h_counter: u16,

    pub(crate) dma_mode: Option<DmaMode>,
    pub(crate) dma_run: bool,
    pub(crate) dma_data_wait: bool,

    pub(crate) ram_access_mode: RamAccessMode,
    pub(crate) vdp_ram_address: u32,

    pub(crate) data_port_reg: u16, // The register that holds the last write data into data port

    pub(crate) address_setting_raw_word: u32,
    pub(crate) address_setting_latch: bool,

    pub(crate) bus: Option<T>,
    pub(crate) signal_bus: Rc<RefCell<SignalBus>>,

    pub(crate) dma_src_address: u32,
    pub(crate) dma_length: u16,
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
        let register_set = RegisterSet::new();
        Self {
            screen,
            vram_table,

            register_set,

            vram: [0; 0x10000],
            cram: [0; 0x80],
            vsram: [0; 0x50],

            v_counter: 0,
            h_counter: 0,

            dma_mode: None,
            dma_run: false,
            dma_data_wait: false,

            ram_access_mode: RamAccessMode::VramR,
            vdp_ram_address: 0,

            data_port_reg: 0,

            address_setting_raw_word: 0,
            address_setting_latch: false,

            bus: None,
            signal_bus: signal_bus,

            dma_src_address: 0,
            dma_length: 0,
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

        if self.v_counter < 0xE0 {
            let bg_palette_id = self.register_set.background_color.palette_id();
            let bg_color_id = self.register_set.background_color.color_id();
            let back_dot_color = self.get_color(bg_palette_id, bg_color_id);

            let plane_a_base_address = self.register_set.plane_a_table_location.address();
            let plane_a_dot = self.get_plane_dot(plane_a_base_address);

            let plane_b_base_address = self.register_set.plane_b_table_location.address();
            let plane_b_dot = self.get_plane_dot(plane_b_base_address);

            let dot = if self.register_set.mode_register.display_enabled() {
                let mut color = plane_a_dot
                    .color
                    .or_else(|| plane_b_dot.color)
                    .unwrap_or(back_dot_color);
                // let mut color = plane_a_dot.color.or_else(|| plane_b_dot.color).unwrap_or(back_dot_color);
                if let Some(plane_color) = plane_b_dot.color {
                    if plane_b_dot.priority == Priority::High {
                        color = plane_color;
                    }
                }
                if let Some(plane_color) = plane_a_dot.color {
                    if plane_a_dot.priority == Priority::High {
                        color = plane_color;
                    }
                }
                color
            } else {
                back_dot_color
            };
            self.screen
                .set_pixel(self.h_counter as i32, self.v_counter as i32, dot)
                .unwrap();
            self.h_counter += 1;
            if self.h_counter >= 320 {
                self.h_counter = 0;
                self.v_counter += 1;
            }
            if self.v_counter == 0xE0 {
                // self.v_counter = 0;
                update_screen = true;
                self.update_vram_table_on_screen();

                if self.register_set.mode_register.vinterrupt_enabled() {
                    self.signal_bus
                        .borrow_mut()
                        .push_siganal(Signal::VInterrupt);
                    debug!("VDP: send vinterrupt signtal");
                }
                self.register_set
                    .status
                    .set_flag(StatusFlag::Blanking, true);
            }
        } else {
            self.h_counter += 1;
            if self.h_counter >= 320 {
                self.h_counter = 0;
                self.v_counter += 1;
            }
            if self.v_counter == 0x1FF {
                self.v_counter = 0;
                self.register_set
                    .status
                    .set_flag(StatusFlag::Blanking, false);
            }
        }

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
        if self.register_set.mode_register.dma_enabled() && self.dma_run {
            debug!(
                "VDP: clock: dma enabled, dma cycles remined {}",
                self.dma_length
            );
            self.register_set
                .status
                .set_flag(StatusFlag::DmaProgress, true);
            match self.dma_mode.as_ref().unwrap() {
                DmaMode::BusToRam => self.dma_bus_to_ram_copy(),
                DmaMode::CopyRam => (),
                DmaMode::FillRam => self.dma_ram_fill(),
            }
            if self.dma_length == 0 {
                self.register_set
                    .status
                    .set_flag(StatusFlag::DmaProgress, false);
                self.register_set.mode_register.clear_dma_enabled();
                self.dma_mode = None;
                self.dma_run = false;
                return;
            }
        }
    }

    fn dma_bus_to_ram_copy(&mut self) {
        let data = self.bus.as_ref().unwrap().read(self.dma_src_address);
        debug!("VDP: dma_bus_to_ram_copy: transfer word: {:04X}", data);
        unsafe {
            let ptr = match self.ram_access_mode {
                RamAccessMode::VramW => (&self.vram as *const u8).offset(self.vdp_ram_address as isize) as *mut u16,
                RamAccessMode::CramW => (&self.cram as *const u8).offset(self.vdp_ram_address as isize) as *mut u16,
                RamAccessMode::VSramW => (&self.vsram as *const u8).offset(self.vdp_ram_address as isize) as *mut u16,
                _ => panic!(
                    "VDP: dma_bus_to_ram_copy: unexpected RamAccessMode during of the DMA cycles: {}",
                    self.ram_access_mode
                ),
            };
            *ptr = data;
        }
        self.dma_src_address += 2;
        self.vdp_ram_address += self.register_set.autoincrement.autoincrement();
        self.dma_length -= 1;
        self.signal_bus.borrow_mut().push_siganal(Signal::CpuHalt);
    }

    fn dma_ram_fill(&mut self) {
        let dst_address = self.vdp_ram_address as usize;
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
            // _ => panic!(
            //     "VDP: dma_ram_fill: unexpected RamAccessMode during of the DMA cycles: {}",
            //     self.ram_access_mode
            // ),
            _ => (),
        }
        self.vdp_ram_address += self.register_set.autoincrement.autoincrement();
        self.dma_length -= 1;
    }

    fn get_plane_dot(&self, plane_attribute_address: usize) -> Dot {
        let hplane_size = self.register_set.plane_size.hplane_size();
        let plane_address_offset =
            ((self.h_counter / 8 + (self.v_counter / 8) * hplane_size as u16) as usize) * 2;
        let attribute_data = unsafe {
            *(self
                .vram
                .as_ptr()
                .offset((plane_attribute_address + plane_address_offset) as isize)
                as *const _ as *const u16)
        }
        .to_be();

        let palette_id = (attribute_data >> 13) & 0x3;
        let sprite_id = (attribute_data & 0x7FF) * 32;

        // each sprite byte contains 2 dots
        let h_flip = attribute_data & 0x0800 != 0;
        let h_dot_offset = {
            let offset = (self.h_counter % 8) / 2;
            if h_flip {
                3 - offset
            } else {
                offset
            }
        };
        // and each sprite row contains 4 bytes
        let v_dot_offset = {
            let v_flip = attribute_data & 0x1000 != 0;
            let offset: u16 = (self.v_counter % 8) * 4;
            if v_flip {
                28 - offset
            } else {
                offset
            }
        };

        let sprite_point_address = sprite_id + h_dot_offset + v_dot_offset;
        let sprite_byte = self.vram[sprite_point_address as usize];
        // let color_id = if self.h_counter % 2 == 0 { sprite_byte.rotate_left(4) & 0xF } else { sprite_byte & 0xF };
        let color_id = if !h_flip {
            if self.h_counter % 2 == 0 {
                sprite_byte.rotate_left(4) & 0xF
            } else {
                sprite_byte & 0xF
            }
        } else {
            if self.h_counter % 2 == 0 {
                sprite_byte & 0xF
            } else {
                sprite_byte.rotate_left(4) & 0xF
            }
        };
        let color = if color_id != 0 {
            Some(self.get_color(palette_id as usize, color_id as usize))
        } else {
            None
        };

        let priority = if attribute_data & 0x8000 != 0 {
            Priority::High
        } else {
            Priority::Low
        };
        Dot::new(color, priority)
    }

    fn get_color(&self, palette_id: usize, color_id: usize) -> Color {
        let converter = |b: u16| -> u32 {
            match b {
                0x0 => 0x00u32,
                0x2 => 0x34,
                0x4 => 0x57,
                0x6 => 0x74,
                0x8 => 0x90,
                0xA => 0xAC,
                0xC => 0xCE,
                0xE => 0xFF,
                _ => 0x00,
            }
        };
        let hb = self.cram[palette_id * 32 + color_id * 2];
        let lb = self.cram[palette_id * 32 + color_id * 2 + 1];
        let raw_color = (hb as u16) << 8 | lb as u16;
        let r = converter(raw_color & 0xF);
        let g = converter(raw_color >> 4 & 0xF);
        let b = converter(raw_color >> 8 & 0xF);
        let color_code = r << 16 | g << 8 | b;
        Color::from_u32(color_code)
    }

    fn update_vram_table_on_screen(&mut self) {
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
                    self.vram_table
                        .set_pixel(x as i32, y as i32, color_table[dot])
                        .unwrap();
                }
            }
        }
    }
}

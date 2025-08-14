use std::{cell::RefCell, rc::Rc};

use spriter::{window::Window, Canvas, Color};

use log::debug;

use crate::{signal_bus::{Signal, SignalBus}, vdp_emu::{registers::{HCellMode, VCellMode}, DisplayMod}};

use super::{
    bus::BusVdp,
    dot::{Dot, Priority},
    registers::{
        HScrollMode, RegisterSet, StatusFlag, VScrollMode, WindowHPostion, WindowVPosition,
    },
    sprite::Sprite,
    tile::{Tile, TileDot},
    DmaMode, RamAccessMode,
};

pub struct Vdp {
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

    pub(crate) bus: Option<Rc<dyn BusVdp>>,
    pub(crate) signal_bus: Rc<RefCell<SignalBus>>,

    pub(crate) dma_src_address: u32,
    pub(crate) dma_length: u16,

    screen: Canvas,
    sprites: Vec<Sprite>,
    v_mode: VCellMode,
    h_mode: HCellMode,
}

impl Vdp{
    pub fn new(window: &mut Window, signal_bus: Rc<RefCell<SignalBus>>, display_mod: DisplayMod) -> Self {
        let height = display_mod.line_amount();
        let mut screen = window.create_canvas(0, 0, 640, height * 2, 320, height);
        screen.set_clear_color(Color::from_u32(0xAAAAAA));
        screen.clear();
        let mut vram_table = window.create_canvas(660, 0, 512, 1024, 256, 512);
        vram_table.set_clear_color(Color::from_u32(0xAAAACC));
        vram_table.clear();
        let register_set = RegisterSet::new(display_mod);
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

            sprites: vec![],
            v_mode: VCellMode::V30Cell,
            h_mode: HCellMode::H40Cell,
        }
    }

    pub fn set_bus(&mut self, bus: Rc<dyn BusVdp>) {
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

            let sprite_dot = self.get_sprite_dot();

            let plane_a_base_address = self.register_set.plane_a_table_location.address();
            let plane_a_dot = self.get_plane_dot(plane_a_base_address, 0);

            let plane_b_base_address = self.register_set.plane_b_table_location.address();
            let plane_b_dot = self.get_plane_dot(plane_b_base_address, 2);

            let window_dot = self.get_window_dot();

            let dot = if self.register_set.mode_register.display_enabled() {
                let mut color = sprite_dot
                    .color
                    .or_else(|| window_dot.color)
                    .or_else(|| plane_a_dot.color)
                    .or_else(|| plane_b_dot.color)
                    .unwrap_or(back_dot_color);
                if let Some(plane_color) = plane_b_dot.color {
                    if plane_b_dot.priority == Priority::High {
                        color = plane_color;
                    }
                }
                if let Some(plane_color) = plane_a_dot.color {
                    if plane_a_dot.priority == Priority::High {
                        color = window_dot.color.unwrap_or(plane_color);
                    }
                }
                if let Some(sprite_color) = sprite_dot.color {
                    if sprite_dot.priority == Priority::High {
                        color = sprite_color;
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
            if self.h_counter >= self.h_mode as u16 * 8 {
                self.h_counter = 0;
                self.v_counter += 1;

                self.collect_sprites();
            }
            if self.v_counter == 0xE0 {
                let v_mode = self.register_set.mode_register.vcell_mode();
                let h_mode = self.register_set.mode_register.hcell_mode();
                if self.v_mode != v_mode || self.h_mode != h_mode {
                    self.screen.resize_texture(h_mode as u32 * 8, v_mode as u32 * 8);
                    self.v_mode = v_mode;
                    self.h_mode = h_mode;
                }
                update_screen = true;
                self.update_vram_table_on_screen();

                if self.register_set.mode_register.vinterrupt_enabled() {
                    self.signal_bus
                        .borrow_mut()
                        .push_signal(Signal::VInterrupt);
                    self.signal_bus
                        .borrow_mut()
                        .push_signal(Signal::Z80NMI);
                    self.register_set
                        .status
                        .set_flag(StatusFlag::VInterruptPending, true);
                    debug!("VDP: send vinterrupt signtal");
                }
                self.register_set
                    .status
                    .set_flag(StatusFlag::Blanking, true);
            }
        } else {
            self.h_counter += 1;
            if self.h_counter >= self.h_mode as u16 * 8 {
                let hinterrupt_counter = self.register_set.hinterrupt_counter.hinterrupt_counter() as u16;
                if self.register_set.mode_register.hinterrupt_enabled() && self.v_counter % hinterrupt_counter == 0 {
                    self.signal_bus
                        .borrow_mut()
                        .push_signal(Signal::HInterrupt);
                }
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
        self.signal_bus.borrow_mut().push_signal(Signal::CpuHalt);
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

    fn collect_sprites(&mut self) {
        let sprite_table_location = self.register_set.sprite_table_location.address() as usize;
        let sprite = Sprite::new(&self.vram[sprite_table_location..sprite_table_location + 8]);
        let mut sprite_link = sprite.sprite_link();
        self.sprites.clear();
        self.sprites.push(sprite);
        while sprite_link != 0 && self.sprites.len() <= 80 {
            let sprite_location = sprite_table_location + (sprite_link * 8) as usize;
            let sprite = Sprite::new(&self.vram[sprite_location..sprite_location + 8]);
            sprite_link = sprite.sprite_link();
            if sprite.in_current_line(self.v_counter) {
                if sprite.h_position() == 0 {
                    break;
                }
                self.sprites.push(sprite);
            }
        }
    }

    fn get_plane_dot(&self, plane_attribute_address: usize, plane_num: isize) -> Dot {
        let hplane_size = self.register_set.plane_size.hplane_size() as u16;
        let vplane_size = self.register_set.plane_size.vplane_size() as u16;
        // let mut plane_address_offset =
        //     ((self.h_counter / 8 + (self.v_counter / 8) * hplane_size) as usize) * 2; // each tile attribute has 2 byte

        let h_scroll_mode = self.register_set.mode_register.hscroll_mode();
        let h_scroll_table_address = self.register_set.hscroll_data_location.address() as isize;
        let h_scroll_offset = match h_scroll_mode {
            HScrollMode::Full => unsafe {
                let ptr = self
                    .vram
                    .as_ptr()
                    .offset(h_scroll_table_address + plane_num)
                    as *const _ as *const u16;
                (*ptr).to_be() & 0x3FF
            },
            HScrollMode::Each1Cell => unsafe {
                let v_cell = (self.v_counter / 8) as isize;
                let ptr = self
                    .vram
                    .as_ptr()
                    .offset(h_scroll_table_address + plane_num + v_cell * 32)
                    as *const _ as *const u16;
                (*ptr).to_be() & 0x3FF
            },
            HScrollMode::Each1Line => unsafe {
                let ptr = self
                    .vram
                    .as_ptr()
                    .offset(h_scroll_table_address + plane_num + (self.v_counter as isize) * 4)
                    as *const _ as *const u16;
                (*ptr).to_be() & 0x3FF
            },
            HScrollMode::Prohibited => 0,
        };

        let v_scroll_mode = self.register_set.mode_register.vscroll_mode();
        let v_scroll_offset = match v_scroll_mode {
            VScrollMode::Full => unsafe {
                let ptr = self.vsram.as_ptr().offset(plane_num) as *const _ as *const u16;
                (*ptr).to_be() & 0x3FF
            },
            VScrollMode::Each2Cell => unsafe {
                let h_2cell = (self.h_counter / 16) as isize;
                let ptr =
                    self.vsram.as_ptr().offset(plane_num + h_2cell * 4) as *const _ as *const u16;
                (*ptr).to_be() & 0xFF
            },
        };
        let h_tile_offset = (self.h_counter.wrapping_sub(h_scroll_offset) % (hplane_size * 8)) / 8;
        let v_tile_offset = (self.v_counter.wrapping_add(v_scroll_offset) % (vplane_size * 8)) / 8;

        let plane_address_offset =
            (h_tile_offset.wrapping_add(v_tile_offset * hplane_size) as usize) * 2; // each tile attribute has 2 byte

        let attribute_data = unsafe {
            *(self
                .vram
                .as_ptr()
                .offset((plane_attribute_address + plane_address_offset) as isize)
                as *const _ as *const u16)
        }
        .to_be();

        let palette_id = (attribute_data >> 13) & 0x3;
        let tile_id = attribute_data & 0x7FF;
        let h_flip = attribute_data & 0x0800 != 0;
        let v_flip = attribute_data & 0x1000 != 0;

        let tile = Tile::new(tile_id.into(), h_flip, v_flip);
        let tile_dot = TileDot::new(
            tile,
            (self.h_counter.wrapping_sub(h_scroll_offset) % 8).into(),
            (self.v_counter.wrapping_add(v_scroll_offset) % 8).into(),
            self.h_counter.wrapping_sub(h_scroll_offset) % 2 == 0,
        );

        let color_id = self.get_tile_dot_byte(tile_dot);
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

    fn get_window_dot(&self) -> Dot {
        let v_hit = {
            let v_position = self.register_set.window_plane_vpostion.window_vpostion();
            let v_offset = self.register_set.window_plane_vpostion.window_voffset() * 8;

            match v_position {
                WindowVPosition::TopToVal => self.v_counter < v_offset,
                WindowVPosition::ValToDown => self.v_counter >= v_offset,
            }
        };

        let h_hit = {
            let h_position = self.register_set.window_plane_hpostion.window_hpostion();
            let h_offset = self.register_set.window_plane_hpostion.window_hoffset() * 8 * 2; // H coords corresponds to 2 dots

            match h_position {
                WindowHPostion::LeftToVal => self.h_counter < h_offset,
                WindowHPostion::ValToRight => self.h_counter >= h_offset,
            }
        };

        if h_hit || v_hit {
            let window_attribute_address = self.register_set.window_table_location.address();
            let table_width = match self.register_set.mode_register.hcell_mode() {
                HCellMode::H32Cell => 32,
                HCellMode::H40Cell => 64,
            };
            let plane_address_offset =
                ((self.h_counter / 8 + (self.v_counter / 8) * table_width) as usize) * 2;
            let attribute_data = unsafe {
                *(self
                    .vram
                    .as_ptr()
                    .offset((window_attribute_address + plane_address_offset) as isize)
                    as *const _ as *const u16)
            }
            .to_be();

            let palette_id = (attribute_data >> 13) & 0x3;
            let tile_id = attribute_data & 0x7FF;
            let h_flip = attribute_data & 0x0800 != 0;
            let v_flip = attribute_data & 0x1000 != 0;

            let tile = Tile::new(tile_id.into(), h_flip, v_flip);
            let tile_dot = TileDot::new(
                tile,
                (self.h_counter % 8).into(),
                (self.v_counter % 8).into(),
                self.h_counter % 2 == 0,
            );

            let color_id = self.get_tile_dot_byte(tile_dot);
            let color = if color_id != 0 {
                Some(self.get_color(palette_id.into(), color_id.into()))
            } else {
                None
            };

            let priority = if attribute_data & 0x8000 != 0 {
                Priority::High
            } else {
                Priority::Low
            };
            Dot::new(color, priority)
        } else {
            Dot::default()
        }
    }

    // sprite attribute table store 80 sprites
    // each sprite has 8 byte size
    fn get_sprite_dot(&mut self) -> Dot {
        let hited_sprites = self
            .sprites
            .iter()
            .filter(|s| s.sprite_hit(self.h_counter, self.v_counter))
            .collect::<Vec<&Sprite>>();

        let tile_dots = hited_sprites
            .iter()
            .map(|s| (*s, s.get_tile_dot(self.h_counter, self.v_counter)))
            .filter(|s_td_opt| s_td_opt.1.is_some())
            .map(|s_td_opt| (s_td_opt.0, self.get_tile_dot_byte(s_td_opt.1.unwrap())))
            .filter(|s_tb| s_tb.1 != 0)
            .collect::<Vec<(&Sprite, u8)>>();

        if tile_dots.len() > 1 {
            self.register_set
                .status
                .set_flag(StatusFlag::SpriteCollision, true);
        }

        for tile_dot in tile_dots {
            return Dot::new(
                Some(self.get_color(tile_dot.0.palette_id().into(), tile_dot.1.into())),
                tile_dot.0.priority(),
            );
        }
        Dot::default()
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

    pub fn update_vram_table_on_screen(&mut self) {
        let palette_id = 0;
        for tile_idx in 0..2048 {
            for byte_idx in 0..32 {
                let idx = tile_idx * 32 + byte_idx;
                let data_byte = self.vram[idx].rotate_left(4);
                for pixel_num in 0..2 {
                    let x = (tile_idx % 32) * 8 + (byte_idx % 4) * 2 + pixel_num;
                    let y = (tile_idx / 32) * 8 + byte_idx / 4;
                    let color_byte = (data_byte >> (4 * pixel_num)) & 0xF;
                    let color = self.get_color(palette_id, color_byte.into());
                    self.vram_table
                        .set_pixel(x as i32, y as i32, color)
                        .unwrap();
                }
            }
        }
    }

    fn get_tile_dot_byte(&self, tile_dot: TileDot) -> u8 {
        let tile_offset = tile_dot.tile.tile_id * 0x20;
        // each tile byte contains 2 dots
        let h_dot_offset = {
            let offset = tile_dot.x_position / 2;
            if tile_dot.tile.h_flip {
                3 - offset
            } else {
                offset
            }
        };
        // and each tile row contains 4 bytes
        let v_dot_offset = {
            let offset = tile_dot.y_position * 4;
            if tile_dot.tile.v_flip {
                28 - offset
            } else {
                offset
            }
        };
        let tile_point_offset = tile_offset + h_dot_offset + v_dot_offset;
        let tile_byte = self.vram[tile_point_offset];
        let mut rotate_position = 8;
        if tile_dot.tile.h_flip {
            if !tile_dot.even {
                rotate_position -= 4;
            }
        } else {
            if tile_dot.even {
                rotate_position -= 4;
            }
        };
        tile_byte.rotate_left(rotate_position) & 0xF
    }
}

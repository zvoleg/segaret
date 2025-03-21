use std::num::NonZeroU8;

use super::{
    dot::Priority,
    tile::{Tile, TileDot},
};

pub(crate) struct Sprite {
    h_position: i16,
    v_position: i16,
    size_x: u16,
    size_y: u16,
    priority: Priority,
    palette_id: u16,
    h_flip: bool,
    v_flip: bool,
    tile_id: u16,
    sprite_link: u16,
}

impl Sprite {
    pub(crate) fn new(data: &[u8]) -> Self {
        let v_position = unsafe {
            let ptr = data as *const _ as *const u16;
            let data = (*ptr).to_be();
            (data & 0x03FF) as i16
        };
        let h_position = unsafe {
            let ptr = data.as_ptr().offset(6) as *const _ as *const u16;
            let data = (*ptr).to_be();
            (data & 0x01FF) as i16
        };
        let hs_vs_data = data[2];
        let size_x = (((hs_vs_data >> 2) & 0x3) + 1) as u16;
        let size_y = ((hs_vs_data & 0x3) + 1) as u16;
        let sprite_link = (data[3] & 0x7F) as u16;
        let attributes_data =
            unsafe { *(data.as_ptr().offset(4) as *const _ as *const u16) }.to_be();
        let tile_id = attributes_data & 0x07FF;
        let h_flip = attributes_data & 0x0800 != 0;
        let v_flip = attributes_data & 0x1000 != 0;
        let palette_id = (attributes_data >> 13) & 0x3;
        let priority = if attributes_data & 0x8000 != 0 {
            Priority::High
        } else {
            Priority::Low
        };
        Self {
            v_position,
            h_position,
            size_x,
            size_y,
            priority,
            palette_id,
            h_flip,
            v_flip,
            tile_id,
            sprite_link,
        }
    }

    pub(crate) fn get_tile_dot(&self, v_position: u16, h_position: u16) -> Option<TileDot> {
        if self.sprite_hit(v_position, h_position) {
            let mut x_tile = (h_position - (self.h_position - 128) as u16) / 8;
            if self.h_flip {
                x_tile = self.size_x - x_tile - 1;
            }
            let mut y_tile = (v_position - (self.v_position - 128) as u16) / 8;
            if self.v_flip {
                y_tile = self.size_y - y_tile - 1;
            }
            let tile_offset: u16 = y_tile + (x_tile * self.size_y);
            let tile_id = (self.tile_id + tile_offset) as usize;
            let tile = Tile::new(tile_id, self.h_flip, self.v_flip);
            let tile_dot = TileDot::new(
                tile,
                ((h_position - (self.h_position - 128) as u16) % 8) as usize,
                ((v_position - (self.v_position - 128) as u16) % 8) as usize,
            );
            Some(tile_dot)
        } else {
            None
        }
    }

    pub(crate) fn sprite_hit(&self, v_position: u16, h_position: u16) -> bool {
        if self.h_position < 128 || self.v_position < 128 {
            return false;
        }
        if h_position < (self.h_position - 128) as u16
            || v_position < (self.v_position - 128) as u16
        {
            return false;
        }
        let h_right_point = (self.h_position - 128) as u16 + self.size_x * 8;
        let v_down_point = (self.v_position - 128) as u16 + self.size_y * 8;
        if h_position >= h_right_point || v_position >= v_down_point {
            return false;
        }
        true
    }

    pub(crate) fn palette_id(&self) -> u16 {
        self.palette_id
    }

    pub(crate) fn priority(&self) -> Priority {
        self.priority
    }
}

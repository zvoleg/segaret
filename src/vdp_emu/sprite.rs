use super::{
    dot::Priority,
    tile::{Tile, TileDot},
};

pub(crate) struct Sprite {
    h_position: u16,
    v_position: u16,
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
            data & 0x03FF
        };
        let h_position = unsafe {
            let ptr = data.as_ptr().offset(6) as *const _ as *const u16;
            let data = (*ptr).to_be();
            data & 0x01FF
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

    pub(crate) fn get_tile_dot(&self, h_position: u16, v_position: u16) -> Option<TileDot> {
        let v_pos_to_sprite_plane = v_position + 128;
        let h_pos_to_sprite_plane = h_position + 128;
        if self.in_sprite(h_pos_to_sprite_plane, v_pos_to_sprite_plane) {
            let mut x_tile = (h_pos_to_sprite_plane - self.h_position) / 8;
            if self.h_flip {
                x_tile = self.size_x - x_tile - 1;
            }
            let mut y_tile = (v_pos_to_sprite_plane - self.v_position) / 8;
            if self.v_flip {
                y_tile = self.size_y - y_tile - 1;
            }
            let tile_offset: u16 = y_tile + (x_tile * self.size_y);
            let tile_id = (self.tile_id + tile_offset) as usize;
            let tile = Tile::new(tile_id, self.h_flip, self.v_flip);
            let x_point = (h_pos_to_sprite_plane - self.h_position) % 8;
            let y_point = (v_pos_to_sprite_plane - self.v_position) % 8;
            let tile_dot = TileDot::new(
                tile,
                x_point.into(),
                y_point.into(),
                x_point % 2 == 0,
            );
            Some(tile_dot)
        } else {
            None
        }
    }

    fn in_sprite(&self, h_position: u16, v_position: u16) -> bool {
        let h_hit = self.h_position <= h_position && h_position < (self.h_position + self.size_x * 8);
        let v_hit = self.v_position <= v_position && v_position < (self.v_position + self.size_y * 8);
        h_hit && v_hit
    }

    pub(crate) fn sprite_hit(&self, h_position: u16, v_position: u16) -> bool {
        let h_pos_to_sprite_plane = h_position + 128;
        let v_pos_to_sprite_plane = v_position + 128;
        self.in_sprite(h_pos_to_sprite_plane, v_pos_to_sprite_plane)
    }

    pub(crate) fn in_current_line(&self, v_position: u16) -> bool {
        let upper_bound = self.v_position;
        let lower_bound = self.v_position + self.size_y * 8;
        let v_pos_to_sprite_plane = v_position + 128;
        upper_bound <= v_pos_to_sprite_plane && v_pos_to_sprite_plane < lower_bound
    }

    pub(crate) fn palette_id(&self) -> u16 {
        self.palette_id
    }

    pub(crate) fn priority(&self) -> Priority {
        self.priority
    }

    pub(crate) fn h_position(&self) -> u16 {
        self.h_position
    }

    pub(crate) fn sprite_link(&self) -> u16 {
        self.sprite_link
    }
}

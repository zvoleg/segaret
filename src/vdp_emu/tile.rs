pub(crate) struct Tile {
    pub(crate) tile_id: usize,
    pub(crate) h_flip: bool,
    pub(crate) v_flip: bool,
}

impl Tile {
    pub(crate) fn new(tile_id: usize, h_flip: bool, v_flip: bool) -> Self {
        Self {
            tile_id,
            h_flip,
            v_flip,
        }
    }
}

pub(crate) struct TileDot {
    pub(crate) tile: Tile,
    pub(crate) x_position: usize,
    pub(crate) y_position: usize,
}

impl TileDot {
    pub(crate) fn new(tile: Tile, x_position: usize, y_position: usize) -> Self {
        Self {
            tile,
            x_position,
            y_position,
        }
    }
}

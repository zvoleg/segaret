pub(crate) const MODE_REGISTER_I: usize = 0;
pub(crate) const MODE_REGISTER_II: usize = 1;
pub(crate) const PLANE_A_NAME_TABLE_LOCATION: usize = 2;
pub(crate) const WINDOW_NAME_TABLE_LOCATION: usize = 3;
pub(crate) const PLANE_B_NAME_TABLE_LOCATION: usize = 4;
pub(crate) const SPRITE_TABLE_LOCATION: usize = 5;
pub(crate) const BACKGROUND_COLOR: usize = 7;
pub(crate) const H_INTERRUPT_COUNTER: usize = 10;
pub(crate) const MODE_REGISTER_III: usize = 11;
pub(crate) const MODE_REGISTER_IV: usize = 12;
pub(crate) const H_SCROLL_DATA_LOCATION: usize = 13;
pub(crate) const AUTO_INCREMENT: usize = 15;
pub(crate) const PLANE_SIZE: usize = 16;
pub(crate) const WINDOW_PLANE_H_POSITION: usize = 17;
pub(crate) const WINDOW_PLANE_V_POSITION: usize = 18;
pub(crate) const DMA_LENGTH_L: usize = 19;
pub(crate) const DMA_LENGTH_H: usize = 20;
pub(crate) const DMA_SOURCE_L: usize = 21;
pub(crate) const DMA_SOURCE_M: usize = 22;
pub(crate) const DMA_SOURCE_H: usize = 23;

pub(crate) enum VScrollMode {
    Full,
    Each2Cell,
}

pub(crate) enum HScrollMode {
    Full,
    Each1Cell,
    Each1Line,
    Prohibited,
}

pub(crate) enum HCellMode {
    H32Cell,
    H40Cell,
    Prohibited,
}

pub(crate) enum VPlaneSize {
    V32Cell,
    V64Cell,
    V128Cell,
    Prohibited,
}

pub(crate) enum HPlaneSize {
    H32Cell,
    H64Cell,
    H128Cell,
    Prohibited,
}

pub(crate) enum InterlaceMode {
    No,
    Interlace,
    DoubleResolution,
    Prohibited,
}

pub(crate) enum WindowVPosition {
    Up,
    Down,
}

pub(crate) enum WindowHPostion {
    Left,
    Right,
}

pub(crate) enum RegDmaMode {
    BusToRam,
    FillRam,
    CopyRam,
}

pub(crate) struct ModeRegister {
    data_i: *const u8,
    data_ii: *const u8,
    data_iii: *const u8,
    data_iv: *const u8,
}

impl ModeRegister {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data_i: &data[MODE_REGISTER_I],
            data_ii: &data[MODE_REGISTER_II],
            data_iii: &data[MODE_REGISTER_III],
            data_iv: &data[MODE_REGISTER_IV],
        }
    }

    pub(crate) fn hinterrupt_enabled(&self) -> bool {
        unsafe { *self.data_i & 0x10 != 0 }
    }

    pub(crate) fn hv_counters_stoped(&self) -> bool {
        unsafe { *self.data_i & 0x02 != 0 }
    }

    pub(crate) fn display_enabled(&self) -> bool {
        unsafe { *self.data_ii & 0x40 != 0 }
    }

    pub(crate) fn v_interrupt_enabled(&self) -> bool {
        unsafe { *self.data_ii & 0x20 != 0 }
    }

    pub(crate) fn dma_enabled(&self) -> bool {
        unsafe { *self.data_ii & 0x10 != 0 }
    }

    pub(crate) fn pal_mode(&self) -> bool {
        unsafe { *self.data_ii & 0x08 != 0 }
    }

    pub(crate) fn ext_interrupt_enabled(&self) -> bool {
        unsafe { *self.data_iii & 0x80 != 0 }
    }

    pub(crate) fn vscroll_mode(&self) -> VScrollMode {
        unsafe {
            if *self.data_iii & 0x04 != 0 {
                VScrollMode::Each2Cell
            } else {
                VScrollMode::Full
            }
        }
    }

    pub(crate) fn hscroll_mode(&self) -> HScrollMode {
        unsafe {
            let mask = *self.data_iii & 0x03;
            match mask {
                0b00 => HScrollMode::Full,
                0b10 => HScrollMode::Each1Cell,
                0b11 => HScrollMode::Each1Line,
                _ => HScrollMode::Prohibited,
            }
        }
    }

    pub(crate) fn hcell_mode(&self) -> HCellMode {
        unsafe {
            let mask = *self.data_iv & 0x81;
            match mask {
                0x81 => HCellMode::H40Cell,
                0x00 => HCellMode::H32Cell,
                _ => HCellMode::Prohibited,
            }
        }
    }

    pub(crate) fn shadows_enabled(&self) -> bool {
        unsafe { *self.data_iv & 0x08 != 0 }
    }

    pub(crate) fn interlace_mode(&self) -> InterlaceMode {
        unsafe {
            match *self.data_iv & 0x06 {
                0b000 => InterlaceMode::No,
                0b010 => InterlaceMode::Interlace,
                0b110 => InterlaceMode::DoubleResolution,
                _ => InterlaceMode::Prohibited,
            }
        }
    }
}

pub(crate) struct PlaneANameTableLocation {
    data: *const u8,
}

impl PlaneANameTableLocation {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[PLANE_A_NAME_TABLE_LOCATION],
        }
    }

    pub(crate) fn plane_a_address(&self) -> u32 {
        unsafe {
            let mask = (*self.data >> 3) as u32 & 0x7;
            mask << 13
        }
    }
}

pub(crate) struct WindowNameTableLocation {
    data: *const u8,
}

impl WindowNameTableLocation {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[WINDOW_NAME_TABLE_LOCATION],
        }
    }

    pub(crate) fn window_address(&self) -> u32 {
        unsafe {
            let mask = (*self.data >> 1) as u32 & 0x1F;
            mask << 11
        }
    }
}

pub(crate) struct PlaneBNameTableLocation {
    data: *const u8,
}

impl PlaneBNameTableLocation {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[PLANE_B_NAME_TABLE_LOCATION],
        }
    }

    pub(crate) fn plane_b_address(&self) -> u32 {
        unsafe {
            let mask = *self.data as u32 & 0x03;
            mask << 13
        }
    }
}

pub(crate) struct SpriteTableLocation {
    data: *const u8,
}

impl SpriteTableLocation {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[SPRITE_TABLE_LOCATION],
        }
    }

    pub(crate) fn sprite_attribute_address(&self) -> u32 {
        unsafe {
            let mask = *self.data as u32 & 0x7F;
            mask << 9
        }
    }
}

pub(crate) struct BackgroundColor {
    data: *const u8,
}

impl BackgroundColor {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[BACKGROUND_COLOR],
        }
    }

    pub(crate) fn pallet_id(&self) -> usize {
        unsafe { (*self.data & 0x30) as usize >> 4 }
    }

    pub(crate) fn color_id(&self) -> usize {
        unsafe { (*self.data & 0x0F) as usize }
    }
}

pub(crate) struct HInterruptCounter {
    data: *const u8,
}

impl HInterruptCounter {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[H_INTERRUPT_COUNTER],
        }
    }

    pub(crate) fn hinterrupt_counter(&self) -> u8 {
        unsafe { *self.data } // TODO maybe heare should be decrement
    }
}

pub(crate) struct HScrollDataLocation {
    data: *const u8,
}

impl HScrollDataLocation {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[H_SCROLL_DATA_LOCATION],
        }
    }

    pub(crate) fn hscroll_address(&self) -> u32 {
        unsafe {
            let mask = *self.data as u32 & 0x3F;
            mask << 10
        }
    }
}

pub(crate) struct AutoIncrement {
    data: *const u8,
}

impl AutoIncrement {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[AUTO_INCREMENT],
        }
    }

    pub(crate) fn autoincrement(&self) -> u32 {
        unsafe { *self.data as u32 }
    }
}

pub(crate) struct PlaneSize {
    data: *const u8,
}

impl PlaneSize {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[PLANE_SIZE],
        }
    }

    pub(crate) fn vplane_size(&self) -> VPlaneSize {
        unsafe {
            match *self.data & 0x30 {
                0x00 => VPlaneSize::V32Cell,
                0x10 => VPlaneSize::V64Cell,
                0x30 => VPlaneSize::V128Cell,
                _ => VPlaneSize::Prohibited,
            }
        }
    }

    pub(crate) fn hplane_size(&self) -> HPlaneSize {
        unsafe {
            match *self.data & 0x03 {
                0x00 => HPlaneSize::H32Cell,
                0x01 => HPlaneSize::H64Cell,
                0x03 => HPlaneSize::H128Cell,
                _ => HPlaneSize::Prohibited,
            }
        }
    }
}

pub(crate) struct WindowPlaneHPostion {
    data: *const u8,
}

impl WindowPlaneHPostion {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[WINDOW_PLANE_H_POSITION],
        }
    }

    pub(crate) fn window_hpostion(&self) -> WindowHPostion {
        unsafe {
            if *self.data & 0x80 != 0 {
                WindowHPostion::Right
            } else {
                WindowHPostion::Left
            }
        }
    }

    pub(crate) fn window_hoffset(&self) -> u8 {
        unsafe { *self.data & 0x1F }
    }
}

pub(crate) struct WindowPlaneVPostion {
    data: *const u8,
}

impl WindowPlaneVPostion {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data: &data[WINDOW_PLANE_V_POSITION],
        }
    }

    pub(crate) fn window_vpostion(&self) -> WindowVPosition {
        unsafe {
            if *self.data & 0x80 != 0 {
                WindowVPosition::Down
            } else {
                WindowVPosition::Up
            }
        }
    }

    pub(crate) fn window_voffset(&self) -> u8 {
        unsafe { *self.data & 0x1F }
    }
}

pub(crate) struct DmaLnegth {
    data_l: *const u8,
    data_h: *const u8,
}

impl DmaLnegth {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data_l: &data[DMA_LENGTH_L],
            data_h: &data[DMA_LENGTH_H],
        }
    }

    pub(crate) fn length(&self) -> u32 {
        unsafe {
            ((*self.data_h as u32) << 8) | *self.data_l as u32
        }
    }
}

pub(crate) struct DmaSource {
    data_l: *const u8,
    data_m: *const u8,
    data_h: *const u8,
}

impl DmaSource {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            data_l: &data[DMA_SOURCE_L],
            data_m: &data[DMA_SOURCE_M],
            data_h: &data[DMA_SOURCE_H],
        }
    }

    pub(crate) fn dma_mode(&self) -> RegDmaMode {
        unsafe {
            match *self.data_h & 0xC0 {
                0x00 | 0x40 => RegDmaMode::BusToRam,
                0x80 => RegDmaMode::FillRam,
                0xC0 => RegDmaMode::CopyRam,
                _ => panic!("Vdp: DmaSource register: unexpected dma mode bit mask")
            }
        }
    }

    pub(crate) fn src_address(&self) -> u32 {
        unsafe {
            let mask_h = (*self.data_h & 0x7F) as u32;
            let mask_m = *self.data_m as u32;
            let mask_l = *self.data_l as u32;
            mask_h << 17 | mask_m << 9 | mask_l << 1
        }
    }
}

pub(crate) struct Status {
    data: u8,
}

impl Status {
    pub(crate) fn new() -> Self {
        Self { data: 0 }
    }
}

pub(crate) struct RegisterSet {
    pub(crate) mode_register: ModeRegister,
    pub(crate) plane_a_name_table_location: PlaneANameTableLocation,
    pub(crate) window_name_table_location: WindowNameTableLocation,
    pub(crate) plane_b_name_table_location: PlaneBNameTableLocation,
    pub(crate) sprite_table_location: SpriteTableLocation,
    pub(crate) background_color: BackgroundColor,
    pub(crate) hinterrupt_counter: HInterruptCounter,
    pub(crate) hscroll_data_location: HScrollDataLocation,
    pub(crate) autoincrement: AutoIncrement,
    pub(crate) plane_size: PlaneSize,
    pub(crate) window_plane_hpostion: WindowPlaneHPostion,
    pub(crate) window_plane_vpostion: WindowPlaneVPostion,
    pub(crate) dma_lnegth: DmaLnegth,
    pub(crate) dma_source: DmaSource,
    pub(crate) status: Status,
}

impl RegisterSet {
    pub(crate) fn new(data: &[u8]) -> Self {
        Self {
            mode_register: ModeRegister::new(data),
            plane_a_name_table_location: PlaneANameTableLocation::new(data),
            window_name_table_location: WindowNameTableLocation::new(data),
            plane_b_name_table_location: PlaneBNameTableLocation::new(data),
            sprite_table_location: SpriteTableLocation::new(data),
            background_color: BackgroundColor::new(data),
            hinterrupt_counter: HInterruptCounter::new(data),
            hscroll_data_location: HScrollDataLocation::new(data),
            autoincrement: AutoIncrement::new(data),
            plane_size: PlaneSize::new(data),
            window_plane_hpostion: WindowPlaneHPostion::new(data),
            window_plane_vpostion: WindowPlaneVPostion::new(data),
            dma_lnegth: DmaLnegth::new(data),
            dma_source: DmaSource::new(data),
            status: Status::new(),
        }
    }
}

use std::fmt::Display;

pub mod bus;
pub mod vdp_emu;
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

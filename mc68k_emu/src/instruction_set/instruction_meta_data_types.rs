// instruction types
use hardware::Size;

use crate::addressing_mode::AddrMode;

use crate::Condition;
use crate::Register;

// addressing_mode
#[derive(Clone)]
pub(in crate)struct AddrModeMetadata {
    pub(in crate)addr_mode: AddrMode,
}

impl AddrModeMetadata {
    pub(in crate) fn new(addr_mode: AddrMode) -> Self {
        Self { addr_mode }
    }
}

// addressing_mode immediate_data
#[derive(Clone)]
pub(in crate)struct AddrModeImmediateMetadata {
    pub(in crate) addr_mode: AddrMode,
    pub(in crate) immediate_data: u32,
}

impl AddrModeImmediateMetadata {
    pub(in crate) fn new(addr_mode: AddrMode) -> Self {
        Self {
            addr_mode: addr_mode,
            immediate_data: 0,
        }
    }
}

#[derive(Clone)]
pub(in crate)struct AddrModeDataMetadata {
    pub(in crate) addr_mode: AddrMode,
    pub(in crate) data: u32,
}

impl AddrModeDataMetadata {
    pub (in crate) fn new(addr_mode: AddrMode, data: u32) -> Self {
        Self {
            addr_mode,
            data,
        }
    }
}

// addr_mode extension word
#[derive(Clone)]
pub(in crate)struct AddrModeExtWordMetadata {
    pub(in crate)addr_mode: AddrMode,
    pub(in crate)ext_word: u16,
}

impl AddrModeExtWordMetadata {
    pub(in crate) fn new(addr_mode: AddrMode) -> Self {
        Self {
            addr_mode,
            ext_word: 0,
        }
    }
}

// rx addressing_mode
#[derive(Clone)]
pub(in crate)struct RxAddrModeMetadata {
    pub(in crate)reg_x: Register,
    pub(in crate)addr_mode: AddrMode,
}

impl RxAddrModeMetadata {
    pub(in crate) fn new(reg_x: Register, addr_mode: AddrMode) -> Self {
        Self {
            reg_x,
            addr_mode,
        }
    }
}

// move_instruction
#[derive(Clone)]
pub(in crate)struct MoveInstructionMetadata {
    pub(in crate)src_addr_mode: AddrMode,
    pub(in crate)dst_addr_mode: AddrMode,
}

impl MoveInstructionMetadata {
    pub(in crate) fn new(src_addr_mode: AddrMode, dst_addr_mode: AddrMode) -> Self {
        Self {
            src_addr_mode,
            dst_addr_mode,
        }
    }
}

// ry
#[derive(Clone)]
pub(in crate)struct RyMetadata {
    pub(in crate)reg_y: Register,
}

impl RyMetadata {
    pub(in crate) fn new(reg_y: Register) -> Self {
        Self {
            reg_y
        }
    }
}

// ry extension word
#[derive(Clone)]
pub(in crate)struct RyExtWordMetadata {
    pub(in crate)reg_y: Register,
    pub(in crate)ext_word: u32,
}

impl RyExtWordMetadata {
    pub(in crate) fn new(reg_y: Register) -> Self {
        Self {
            reg_y,
            ext_word: 0,
        }
    }
}

// vector
#[derive(Clone)]
pub(in crate)struct VectorMetadata {
    pub(in crate)vector: usize,
}

impl VectorMetadata {
    pub(in crate) fn new(vector: usize) -> Self {
        Self { vector }
    }
}

// data addressing_mode
#[derive(Clone)]
pub(in crate)struct DataAddrModeMetadata {
    pub(in crate) data: u32,
    pub(in crate) addr_mode: AddrMode,
}

impl DataAddrModeMetadata {
    pub(in crate) fn new(data: u32, addr_mode: AddrMode) -> Self {
        Self {
            data,
            addr_mode,
        }
    }
}

// condition addressing_mode
#[derive(Clone)]
pub(in crate)struct ConditionAddrModeMetadata {
    pub(in crate) condition: Condition,
    pub(in crate) addr_mode: AddrMode,
}

impl ConditionAddrModeMetadata {
    pub(in crate) fn new(condition: Condition, addr_mode: AddrMode) -> Self {
        Self {
            condition,
            addr_mode,
        }
    }
}

// condition ry
#[derive(Clone)]
pub(in crate)struct ConditionRyExtWordMetadata {
    pub(in crate) condition: Condition,
    pub(in crate) reg_y: Register,
    pub(in crate) ext_word: u32
}

impl ConditionRyExtWordMetadata {
    pub(in crate) fn new(condition: Condition, reg_y: Register) -> Self {
        Self {
            condition,
            reg_y,
            ext_word: 0,
        }
    }
}

// displacement
#[derive(Clone)]
pub(in crate)struct DisplacementMetadata {
    pub(in crate) displacement: u32,
    pub(in crate) displacement_size: Size,
}

impl DisplacementMetadata {
    pub(in crate) fn new(displacement: u32, displacement_size: Size) -> Self {
        Self { displacement, displacement_size }
    }
}

// condition displacement
#[derive(Clone)]
pub(in crate)struct ConditionDisplacementMetadata {
    pub(in crate) condition: Condition,
    pub(in crate) displacement: u32,
    pub(in crate) displacement_size: Size,
}

impl ConditionDisplacementMetadata {
    pub(in crate) fn new(condition: Condition, displacement: u32, displacement_size: Size) -> Self {
        Self {
            condition,
            displacement,
            displacement_size,
        }
    }
}

// rx data
#[derive(Clone)]
pub(in crate)struct RxDataMetadata {
    pub(in crate) reg_x: Register,
    pub(in crate) data: u32,
}

impl RxDataMetadata {
    pub(in crate) fn new(reg_x: Register, data: u32) -> Self {
        Self {
            reg_x,
            data,
        }
    }
}

// rx ry
#[derive(Clone)]
pub(in crate)struct RxRyMetadata {
    pub(in crate) reg_x: Register,
    pub(in crate) reg_y: Register,
}

impl RxRyMetadata {
    pub(in crate) fn new(reg_x: Register, reg_y: Register) -> Self {
        Self {
            reg_x, 
            reg_y,
        }
    }
}

#[derive(Clone)]
pub(in crate) struct RxRySpecAddrModeMetadata {
    pub(in crate) addr_mode_x: AddrMode,
    pub(in crate) addr_mode_y: AddrMode,
}

impl RxRySpecAddrModeMetadata {
    pub(in crate) fn new(addr_mode_x: AddrMode, addr_mode_y: AddrMode) -> Self {
        Self {
            addr_mode_x,
            addr_mode_y,
        }
    }
}

// rotation ry
#[derive(Clone)]
pub(in crate)struct RotationRyMetadata {
    pub(in crate) counter: u32,
    pub(in crate) reg_y: Register,
}

impl RotationRyMetadata {
    pub(in crate) fn new(counter: u32, reg_y: Register) -> Self {
        Self {
            counter,
            reg_y,
        }
    }
}

// explicit
#[derive(Clone)]
pub(in crate)struct ExplicitMetadata;

#[derive(Clone)]
pub(in crate)struct ExplicitImmediateMetadata {
    pub(in crate)immediate_data: u32,
}

impl ExplicitImmediateMetadata {
    pub(in crate) fn new() -> Self {
        Self {
            immediate_data: 0,
        }
    }
}

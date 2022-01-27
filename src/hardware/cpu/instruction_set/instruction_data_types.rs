// instruction types
use crate::hardware::Size;
use crate::hardware::cpu::addressing_mode::AddrMode;
use crate::hardware::cpu::{Condition, Register};

// addressing_mode
#[derive(Clone)]
pub(in crate::hardware)struct AddrModeMetadata {
    pub(in crate::hardware)addr_mode: AddrMode,
}

impl AddrModeMetadata {
    pub(in crate::hardware)fn new(addr_mode: AddrMode) -> Self {
        Self { addr_mode }
    }
}

// addressing_mode immediate_data
#[derive(Clone)]
pub(in crate::hardware)struct AddrModeImmediateMetadata {
    pub(in crate::hardware) addr_mode: AddrMode,
    pub(in crate::hardware) immediate_data: u32,
}

impl AddrModeImmediateMetadata {
    pub(in crate::hardware)fn new(addr_mode: AddrMode) -> Self {
        Self {
            addr_mode: addr_mode,
            immediate_data: 0,
        }
    }
}

#[derive(Clone)]
pub(in crate::hardware)struct AddrModeDataMetadata {
    pub(in crate::hardware) addr_mode: AddrMode,
    pub(in crate::hardware) data: u32,
}

impl AddrModeDataMetadata {
    pub (in crate::hardware)fn new(addr_mode: AddrMode, data: u32) -> Self {
        Self {
            addr_mode,
            data,
        }
    }
}

// addr_mode extension word
#[derive(Clone)]
pub(in crate::hardware)struct AddrModeExtWordMetadata {
    pub(in crate::hardware)addr_mode: AddrMode,
    pub(in crate::hardware)ext_word: u16,
}

impl AddrModeExtWordMetadata {
    pub(in crate::hardware)fn new(addr_mode: AddrMode) -> Self {
        Self {
            addr_mode,
            ext_word: 0,
        }
    }
}

// rx addressing_mode
#[derive(Clone)]
pub(in crate::hardware)struct RxAddrModeMetadata {
    pub(in crate::hardware)reg_x: Register,
    pub(in crate::hardware)addr_mode: AddrMode,
}

impl RxAddrModeMetadata {
    pub(in crate::hardware)fn new(reg_x: Register, addr_mode: AddrMode) -> Self {
        Self {
            reg_x,
            addr_mode,
        }
    }
}

// move_instruction
#[derive(Clone)]
pub(in crate::hardware)struct MoveInstructionMetadata {
    pub(in crate::hardware)src_addr_mode: AddrMode,
    pub(in crate::hardware)dst_addr_mode: AddrMode,
}

impl MoveInstructionMetadata {
    pub(in crate::hardware)fn new(src_addr_mode: AddrMode, dst_addr_mode: AddrMode) -> Self {
        Self {
            src_addr_mode,
            dst_addr_mode,
        }
    }
}

// ry
#[derive(Clone)]
pub(in crate::hardware)struct RyMetadata {
    pub(in crate::hardware)reg_y: Register,
}

impl RyMetadata {
    pub(in crate::hardware)fn new(reg_y: Register) -> Self {
        Self {
            reg_y
        }
    }
}

// ry extension word
#[derive(Clone)]
pub(in crate::hardware)struct RyExtWordMetadata {
    pub(in crate::hardware)reg_y: Register,
    pub(in crate::hardware)ext_word: u32,
}

impl RyExtWordMetadata {
    pub(in crate::hardware)fn new(reg_y: Register) -> Self {
        Self {
            reg_y,
            ext_word: 0,
        }
    }
}

// vector
#[derive(Clone)]
pub(in crate::hardware)struct VectorMetadata {
    pub(in crate::hardware)vector: usize,
}

impl VectorMetadata {
    pub(in crate::hardware)fn new(vector: usize) -> Self {
        Self { vector }
    }
}

// data addressing_mode
#[derive(Clone)]
pub(in crate::hardware)struct DataAddrModeMetadata {
    pub(in crate::hardware)data: u32,
    pub(in crate::hardware)addr_mode: AddrMode,
}

impl DataAddrModeMetadata {
    pub(in crate::hardware)fn new(data: u32, addr_mode: AddrMode) -> Self {
        Self {
            data,
            addr_mode,
        }
    }
}

// condition addressing_mode
#[derive(Clone)]
pub(in crate::hardware)struct ConditionAddrModeMetadata {
    pub(in crate::hardware)condition: Condition,
    pub(in crate::hardware)addr_mode: AddrMode,
}

impl ConditionAddrModeMetadata {
    pub(in crate::hardware)fn new(condition: Condition, addr_mode: AddrMode) -> Self {
        Self {
            condition,
            addr_mode,
        }
    }
}

// condition ry
#[derive(Clone)]
pub(in crate::hardware)struct ConditionRyExtWordMetadata {
    pub(in crate::hardware)condition: Condition,
    pub(in crate::hardware)reg_y: Register,
    pub(in crate::hardware)ext_word: u32
}

impl ConditionRyExtWordMetadata {
    pub(in crate::hardware)fn new(condition: Condition, reg_y: Register) -> Self {
        Self {
            condition,
            reg_y,
            ext_word: 0,
        }
    }
}

// displacement
#[derive(Clone)]
pub(in crate::hardware)struct DisplacementMetadata {
    pub(in crate::hardware) displacement: u32,
    pub(in crate::hardware) displacement_size: Size,
}

impl DisplacementMetadata {
    pub(in crate::hardware)fn new(displacement: u32) -> Self {
        Self { displacement, displacement_size: Size::Byte }
    }
}

// condition displacement
#[derive(Clone)]
pub(in crate::hardware)struct ConditionDisplacementMetadata {
    pub(in crate::hardware) condition: Condition,
    pub(in crate::hardware) displacement: u32,
    pub(in crate::hardware) displacement_size: Size,
}

impl ConditionDisplacementMetadata {
    pub(in crate::hardware)fn new(condition: Condition, displacement: u32, displacement_size: Size) -> Self {
        Self {
            condition,
            displacement,
            displacement_size,
        }
    }
}

// rx data
#[derive(Clone)]
pub(in crate::hardware)struct RxDataMetadata {
    pub(in crate::hardware)reg_x: Register,
    pub(in crate::hardware)data: u32,
}

impl RxDataMetadata {
    pub(in crate::hardware)fn new(reg_x: Register, data: u32) -> Self {
        Self {
            reg_x,
            data,
        }
    }
}

// rx ry
#[derive(Clone)]
pub(in crate::hardware)struct RxRyMetadata {
    pub(in crate::hardware)reg_x: Register,
    pub(in crate::hardware)reg_y: Register,
}

impl RxRyMetadata {
    pub(in crate::hardware)fn new(reg_x: Register, reg_y: Register) -> Self {
        Self {
            reg_x, 
            reg_y,
        }
    }
}

#[derive(Clone)]
pub(in crate::hardware) struct RxRySpecAddrModeMetadata {
    pub(in crate::hardware) addr_mode_x: AddrMode,
    pub(in crate::hardware) addr_mode_y: AddrMode,
}

impl RxRySpecAddrModeMetadata {
    pub(in crate::hardware)fn new(addr_mode_x: AddrMode, addr_mode_y: AddrMode) -> Self {
        Self {
            addr_mode_x,
            addr_mode_y,
        }
    }
}

// rotation ry
#[derive(Clone)]
pub(in crate::hardware)struct RotationRyMetadata {
    pub(in crate::hardware)counter: u32,
    pub(in crate::hardware)reg_y: Register,
}

impl RotationRyMetadata {
    pub(in crate::hardware)fn new(counter: u32, reg_y: Register) -> Self {
        Self {
            counter,
            reg_y,
        }
    }
}

// explicit
#[derive(Clone)]
pub(in crate::hardware)struct ExplicitMetadata;

#[derive(Clone)]
pub(in crate::hardware)struct ExplicitImmediateMetadata {
    pub(in crate::hardware)immediate_data: u32,
}

impl ExplicitImmediateMetadata {
    pub(in crate::hardware) fn new() -> Self {
        Self {
            immediate_data: 0,
        }
    }
}

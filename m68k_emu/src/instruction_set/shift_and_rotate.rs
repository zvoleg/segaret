use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size,
};

use super::ShiftDirection;

pub(crate) struct ASd_data_reg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ASd_data_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct ASd_implied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for ASd_implied {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {}
}

pub(crate) struct ASd_memory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ASd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct LSd_data_reg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for LSd_data_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct LSd_implied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for LSd_implied {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {}
}

pub(crate) struct LSd_memory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for LSd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ROd_data_reg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) extended: bool,
}

impl Instruction for ROd_data_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct ROd_implied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
    pub(crate) extended: bool,
}

impl Instruction for ROd_implied {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {}
}

pub(crate) struct ROd_memory {
    pub(crate) direction: ShiftDirection,
    pub(crate) extended: bool,
}

impl Instruction for ROd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct SWAP();

impl Instruction for SWAP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

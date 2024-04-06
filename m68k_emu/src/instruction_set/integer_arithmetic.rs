use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet, primitives::Size
};

use super::{RegisterFieldMode, WriteDirection};

pub(crate) struct ADD {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl Instruction for ADD {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ADDA {
    pub(crate) size: Size,
}

impl Instruction for ADDA {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ADDI {
    pub(crate) size: Size,
}

impl Instruction for ADDI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ADDQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
}

impl Instruction for ADDQ {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ADDX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl Instruction for ADDX {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct SUB {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl Instruction for SUB {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct SUBA {
    pub(crate) size: Size,
}

impl Instruction for SUBA {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct SUBI {
    pub(crate) size: Size,
}

impl Instruction for SUBI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct SUBQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
}

impl Instruction for SUBQ {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct SUBX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl Instruction for SUBX {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct CLR {
    pub(crate) size: Size,
}

impl Instruction for CLR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct CMP {
    pub(crate) size: Size,
}

impl Instruction for CMP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct CMPA {
    pub(crate) size: Size,
}

impl Instruction for CMPA {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct CMPI {
    pub(crate) size: Size,
}

impl Instruction for CMPI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct CMPM {
    pub(crate) size: Size,
}

impl Instruction for CMPM {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct EXT {
    pub(crate) size: Size,
}

impl Instruction for EXT {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct NEG {
    pub(crate) size: Size,
}

impl Instruction for NEG {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct NEGX {
    pub(crate) size: Size,
}

impl Instruction for NEGX {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct MULS();

impl Instruction for MULS {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct MULU();

impl Instruction for MULU {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct DIVS();

impl Instruction for DIVS {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct DIVU();

impl Instruction for DIVU {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

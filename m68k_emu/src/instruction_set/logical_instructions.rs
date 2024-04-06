use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet, primitives::Size
};

use super::WriteDirection;

pub(crate) struct AND {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl Instruction for AND {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ANDI {
    pub(crate) size: Size,
}

impl Instruction for ANDI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct EOR {
    pub(crate) size: Size,
}

impl Instruction for EOR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct EORI {
    pub(crate) size: Size,
}

impl Instruction for EORI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct OR {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl Instruction for OR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ORI {
    pub(crate) size: Size,
}

impl Instruction for ORI {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct NOT {
    pub(crate) size: Size,
}

impl Instruction for NOT {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

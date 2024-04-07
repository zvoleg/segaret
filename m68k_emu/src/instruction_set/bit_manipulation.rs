use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size,
};

pub(crate) struct BCHG {
    pub(crate) size: Size,
}

impl Instruction for BCHG {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct BCLR {
    pub(crate) size: Size,
}

impl Instruction for BCLR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct BSET {
    pub(crate) size: Size,
}

impl Instruction for BSET {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct BTST {
    pub(crate) size: Size,
}

impl Instruction for BTST {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

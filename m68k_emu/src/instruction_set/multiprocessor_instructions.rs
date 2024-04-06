use crate::{cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct TAS();

impl Instruction for TAS {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

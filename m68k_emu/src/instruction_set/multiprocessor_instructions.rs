use crate::{bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct TAS();

impl<T: BusM68k> Instruction<T> for TAS {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

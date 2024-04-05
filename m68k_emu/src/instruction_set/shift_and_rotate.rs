use crate::{bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct ASd_data();

impl<T: BusM68k> Instruction<T> for ASd_data {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ASd_memory();

impl<T: BusM68k> Instruction<T> for ASd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct LSd_data();

impl<T: BusM68k> Instruction<T> for LSd_data {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct LSd_memory();

impl<T: BusM68k> Instruction<T> for LSd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ROd_data();

impl<T: BusM68k> Instruction<T> for ROd_data {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ROd_memory();

impl<T: BusM68k> Instruction<T> for ROd_memory {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SWAP();

impl<T: BusM68k> Instruction<T> for SWAP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

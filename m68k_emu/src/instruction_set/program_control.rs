use crate::{bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct TST();

impl<T: BusM68k> Instruction<T> for TST {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct Bcc();

impl<T: BusM68k> Instruction<T> for Bcc {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct DBcc();

impl<T: BusM68k> Instruction<T> for DBcc {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct Scc();

impl<T: BusM68k> Instruction<T> for Scc {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BRA();

impl<T: BusM68k> Instruction<T> for BRA {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BSR();

impl<T: BusM68k> Instruction<T> for BSR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct JMP();

impl<T: BusM68k> Instruction<T> for JMP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct JSR();

impl<T: BusM68k> Instruction<T> for JSR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct NOP();

impl<T: BusM68k> Instruction<T> for NOP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {}
}
pub(crate) struct RTR();

impl<T: BusM68k> Instruction<T> for RTR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct RTS();

impl<T: BusM68k> Instruction<T> for RTS {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

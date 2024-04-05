use crate::{bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct BCHG_reg();

impl<T: BusM68k> Instruction<T> for BCHG_reg {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BCHG_ext_word();

impl<T: BusM68k> Instruction<T> for BCHG_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BCLR_reg();

impl<T: BusM68k> Instruction<T> for BCLR_reg {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BCLR_ext_word();

impl<T: BusM68k> Instruction<T> for BCLR_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BSET_reg();

impl<T: BusM68k> Instruction<T> for BSET_reg {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BSET_ext_word();

impl<T: BusM68k> Instruction<T> for BSET_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BTST_reg();

impl<T: BusM68k> Instruction<T> for BTST_reg {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct BTST_ext_word();

impl<T: BusM68k> Instruction<T> for BTST_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

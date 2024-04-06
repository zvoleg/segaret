use crate::{cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet};

pub(crate) struct BCHG_reg();

impl Instruction for BCHG_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BCHG_ext_word();

impl Instruction for BCHG_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BCLR_reg();

impl Instruction for BCLR_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BCLR_ext_word();

impl Instruction for BCLR_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BSET_reg();

impl Instruction for BSET_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BSET_ext_word();

impl Instruction for BSET_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BTST_reg();

impl Instruction for BTST_reg {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BTST_ext_word();

impl Instruction for BTST_ext_word {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

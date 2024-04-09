use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size,
};

use super::Condition;

pub(crate) struct TST {
    pub(crate) size: Size,
}

impl Instruction for TST {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct Bcc {
    pub(crate) condition: Condition,
    pub(crate) displacement: u32,
}

impl Instruction for Bcc {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct DBcc {
    pub(crate) condition: Condition,
}

impl Instruction for DBcc {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct Scc {
    pub(crate) condition: Condition,
}

impl Instruction for Scc {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BRA {
    pub(crate) displacement: u32,
}

impl Instruction for BRA {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct BSR {
    pub(crate) displacement: u32,
}

impl Instruction for BSR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct JMP();

impl Instruction for JMP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct JSR();

impl Instruction for JSR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct NOP();

impl Instruction for NOP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {}
}
pub(crate) struct RTR();

impl Instruction for RTR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct RTS();

impl Instruction for RTS {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

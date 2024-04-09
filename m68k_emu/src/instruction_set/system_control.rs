use crate::{cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet};

use super::MoveDirection;

pub(crate) struct MOVE_to_SR();

impl Instruction for MOVE_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct MOVE_from_SR();

impl Instruction for MOVE_from_SR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct MOVE_USP {
    pub(crate) direction: MoveDirection,
}

impl Instruction for MOVE_USP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct MOVE_to_CCR();

impl Instruction for MOVE_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct RTE();

impl Instruction for RTE {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct ANDI_to_CCR();

impl Instruction for ANDI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ANDI_to_SR();

impl Instruction for ANDI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct EORI_to_CCR();

impl Instruction for EORI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct EORI_to_SR();

impl Instruction for EORI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct ORI_to_CCR();

impl Instruction for ORI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}
pub(crate) struct ORI_to_SR();

impl Instruction for ORI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct CHK();

impl Instruction for CHK {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct ILLEAGL();

impl Instruction for ILLEAGL {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct TRAP {
    pub(crate) vector: u32,
}

impl Instruction for TRAP {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct TRAPV();

impl Instruction for TRAPV {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct RESET();

impl Instruction for RESET {
    fn execute(&self, operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        todo!()
    }
}

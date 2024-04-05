use crate::{bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet};

use super::MoveDirection;

pub(crate) struct MOVE_to_SR();

impl<T: BusM68k> Instruction<T> for MOVE_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct MOVE_from_SR();

impl<T: BusM68k> Instruction<T> for MOVE_from_SR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct MOVE_USP {
    pub(crate) direction: MoveDirection,
}

impl<T: BusM68k> Instruction<T> for MOVE_USP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct MOVE_to_CCR();

impl<T: BusM68k> Instruction<T> for MOVE_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct RTE();

impl<T: BusM68k> Instruction<T> for RTE {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct ANDI_to_CCR();

impl<T: BusM68k> Instruction<T> for ANDI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ANDI_to_SR();

impl<T: BusM68k> Instruction<T> for ANDI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct EORI_to_CCR();

impl<T: BusM68k> Instruction<T> for EORI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct EORI_to_SR();

impl<T: BusM68k> Instruction<T> for EORI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct ORI_to_CCR();

impl<T: BusM68k> Instruction<T> for ORI_to_CCR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ORI_to_SR();

impl<T: BusM68k> Instruction<T> for ORI_to_SR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct CHK();

impl<T: BusM68k> Instruction<T> for CHK {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct ILLEAGL();

impl<T: BusM68k> Instruction<T> for ILLEAGL {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct TRAP();

impl<T: BusM68k> Instruction<T> for TRAP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct TRAPV();

impl<T: BusM68k> Instruction<T> for TRAPV {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct RESET();

impl<T: BusM68k> Instruction<T> for RESET {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

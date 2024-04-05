use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet, primitives::Size,
};

use super::WriteDirection;

pub(crate) struct AND {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl<T: BusM68k> Instruction<T> for AND {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ANDI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for ANDI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct EOR {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for EOR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct EORI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for EORI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct OR {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl<T: BusM68k> Instruction<T> for OR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ORI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for ORI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

pub(crate) struct NOT {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for NOT {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

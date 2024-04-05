use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet, primitives::Size,
};

use super::{RegisterFieldMode, WriteDirection};

pub(crate) struct ADD {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl<T: BusM68k> Instruction<T> for ADD {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ADDA {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for ADDA {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ADDI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for ADDI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ADDQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
}

impl<T: BusM68k> Instruction<T> for ADDQ {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct ADDX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl<T: BusM68k> Instruction<T> for ADDX {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SUB {
    pub(crate) size: Size,
    pub(crate) direction: WriteDirection,
}

impl<T: BusM68k> Instruction<T> for SUB {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SUBA {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for SUBA {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SUBI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for SUBI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SUBQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
}

impl<T: BusM68k> Instruction<T> for SUBQ {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct SUBX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl<T: BusM68k> Instruction<T> for SUBX {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct CLR {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for CLR {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct CMP {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for CMP {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct CMPA {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for CMPA {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct CMPI {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for CMPI {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct CMPM {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for CMPM {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct EXT {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for EXT {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct NEG {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for NEG {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct NEGX {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for NEGX {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct MULS();

impl<T: BusM68k> Instruction<T> for MULS {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct MULU();

impl<T: BusM68k> Instruction<T> for MULU {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct DIVS();

impl<T: BusM68k> Instruction<T> for DIVS {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}
pub(crate) struct DIVU();

impl<T: BusM68k> Instruction<T> for DIVU {
    fn execute(&self, operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        todo!()
    }
}

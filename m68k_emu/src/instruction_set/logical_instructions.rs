use std::fmt::Display;

use crate::{
    bus::BusM68k, cpu::M68k, cpu_internals::CpuInternals, instruction_set::Instruction,
    operand::OperandSet, primitives::Size, status_flag::StatusFlag, IsNegate, IsZero,
};

pub(crate) struct AND {
    pub(crate) size: Size,
}

impl Display for AND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AND.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for AND {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read();
        let dst_data = dst_operand.read();

        let result = src_data & dst_data;
        dst_operand.write(result);

        let sr = &mut cpu.internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct ANDI {
    pub(crate) size: Size,
}

impl Display for ANDI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANDI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ANDI {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) {
        AND { size: self.size }.execute(operand_set, cpu);
    }
}

pub(crate) struct EOR {
    pub(crate) size: Size,
}

impl Display for EOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EOR.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for EOR {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read();
        let dst_data = dst_operand.read();

        let result = src_data ^ dst_data;
        dst_operand.write(result);

        let sr = &mut cpu.internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct EORI {
    pub(crate) size: Size,
}

impl Display for EORI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EORI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for EORI {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) {
        EOR { size: self.size }.execute(operand_set, cpu);
    }
}

pub(crate) struct OR {
    pub(crate) size: Size,
}

impl Display for OR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OR.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for OR {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read();
        let dst_data = dst_operand.read();

        let result = src_data | dst_data;
        dst_operand.write(result);

        let sr = &mut cpu.internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct ORI {
    pub(crate) size: Size,
}

impl Display for ORI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ORI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ORI {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) {
        OR { size: self.size }.execute(operand_set, cpu);
    }
}

pub(crate) struct NOT {
    pub(crate) size: Size,
}

impl Display for NOT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOT.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for NOT {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let operand = operand_set.next();
        let data = operand.read();

        let result = !data;
        operand.write(result);

        let sr = &mut cpu.internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

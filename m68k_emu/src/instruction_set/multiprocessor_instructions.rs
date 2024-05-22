use std::fmt::Display;

use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet, primitives::Size,
    status_flag::StatusFlag, IsNegate, IsZero,
};

pub(crate) struct TAS();

impl Display for TAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAS.{}", Size::Byte)
    }
}

impl<T: BusM68k> Instruction<T> for TAS {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let operand = operand_set.next();
        let data = operand.read();

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, data.is_negate(Size::Byte));
        sr.set_flag(StatusFlag::Z, data.is_zero(Size::Byte));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);

        let result = data | 0x80;
        operand.write(result);
    }
}

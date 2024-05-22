use std::fmt::Display;

use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet, primitives::Size, status_flag::StatusFlag
};

pub(crate) struct BCHG {
    pub(crate) size: Size,
}

impl Display for BCHG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCHG.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for BCHG {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let mut bit_number = operand_set.next().read();
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BCHG: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read();
        let bit = (data >> bit_number) & 1;
        let result = data ^ (1 << bit_number);
        operand.write(result);

        cpu.internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BCLR {
    pub(crate) size: Size,
}

impl Display for BCLR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCLR.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for BCLR {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let mut bit_number = operand_set.next().read();
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BCLR: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read();
        let bit = (data >> bit_number) & 1;
        let result = data & !(1 << bit_number);
        operand.write(result);

        cpu.internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BSET {
    pub(crate) size: Size,
}

impl Display for BSET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BSET.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for BSET {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let mut bit_number = operand_set.next().read();
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BSET: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read();
        let bit = (data >> bit_number) & 1;
        let result = data | (1 << bit_number);
        operand.write(result);

        cpu.internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BTST {
    pub(crate) size: Size,
}

impl Display for BTST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BTST.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for BTST {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let mut bit_number = operand_set.next().read();
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BTST: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read();
        let bit = (data >> bit_number) & 1;

        cpu.internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size, status_flag::StatusFlag,
};

pub(crate) struct BCHG {
    pub(crate) bit_number_src_size: Size,
    pub(crate) size: Size,
}

impl Instruction for BCHG {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let mut bit_number = operand_set.next().read(self.bit_number_src_size);
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BCHG: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read(self.size);
        let bit = (data >> bit_number) & 1;
        let result = data ^ (1 << bit_number);
        operand.write(result, self.size);

        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BCLR {
    pub(crate) bit_number_src_size: Size,
    pub(crate) size: Size,
}

impl Instruction for BCLR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let mut bit_number = operand_set.next().read(self.bit_number_src_size);
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BCLR: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read(self.size);
        let bit = (data >> bit_number) & 1;
        let result = data & !(1 << bit_number);
        operand.write(result, self.size);

        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BSET {
    pub(crate) bit_number_src_size: Size,
    pub(crate) size: Size,
}

impl Instruction for BSET {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let mut bit_number = operand_set.next().read(self.bit_number_src_size);
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BSET: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read(self.size);
        let bit = (data >> bit_number) & 1;
        let result = data | (1 << bit_number);
        operand.write(result, self.size);

        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

pub(crate) struct BTST {
    pub(crate) bit_number_src_size: Size,
    pub(crate) size: Size,
}

impl Instruction for BTST {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let mut bit_number = operand_set.next().read(self.bit_number_src_size);
        match self.size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("BTST: execute: wrong instruction size"),
        }
        let operand = operand_set.next();

        let data = operand.read(self.size);
        let bit = (data >> bit_number) & 1;

        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, bit == 0);
    }
}

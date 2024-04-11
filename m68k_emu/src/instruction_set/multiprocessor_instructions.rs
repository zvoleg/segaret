use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size, status_flag::StatusFlag, IsNegate, IsZero,
};

pub(crate) struct TAS();

impl Instruction for TAS {
    fn execute(&self, mut operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Byte);

        let sr = &mut cpu_interanls.register_set.sr;
        sr.set_flag(StatusFlag::N, data.is_negate(Size::Byte));
        sr.set_flag(StatusFlag::Z, data.is_zero(Size::Byte));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);

        let result = data | 0x80;
        operand.write(result, Size::Byte);
    }
}

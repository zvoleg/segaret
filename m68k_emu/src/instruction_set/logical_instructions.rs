use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size, status_flag::StatusFlag, IsNegate, IsZero,
};

pub(crate) struct AND {
    pub(crate) size: Size,
}

impl Instruction for AND {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);

        let result = src_data & dst_data;
        dst_operand.write(result, self.size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}
pub(crate) struct ANDI {
    pub(crate) size: Size,
}

impl Instruction for ANDI {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        AND { size: self.size }.execute(operand_set, cpu_internals);
    }
}

pub(crate) struct EOR {
    pub(crate) size: Size,
}

impl Instruction for EOR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);

        let result = src_data ^ dst_data;
        dst_operand.write(result, self.size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}
pub(crate) struct EORI {
    pub(crate) size: Size,
}

impl Instruction for EORI {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        EOR { size: self.size }.execute(operand_set, cpu_internals);
    }
}

pub(crate) struct OR {
    pub(crate) size: Size,
}

impl Instruction for OR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);

        let result = src_data | dst_data;
        dst_operand.write(result, self.size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}
pub(crate) struct ORI {
    pub(crate) size: Size,
}

impl Instruction for ORI {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        OR { size: self.size }.execute(operand_set, cpu_internals);
    }
}

pub(crate) struct NOT {
    pub(crate) size: Size,
}

impl Instruction for NOT {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(self.size);

        let result = !data;
        operand.write(result, self.size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

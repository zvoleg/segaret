use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size, status_flag::StatusFlag, vectors::DIVISION_BY_ZERO, IsNegate, IsZero,
    MsbIsSet, SignExtending,
};

use super::RegisterFieldMode;

pub(crate) struct ADD {
    pub(crate) size: Size,
}

impl Instruction for ADD {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);
        let result = src_data.wrapping_add(dst_data);
        dst_operand.write(result, self.size);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct ADDA {
    pub(crate) size: Size,
}

impl Instruction for ADDA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size).sign_extend(self.size);
        let dst_data = dst_operand.read(Size::Long);
        let result = dst_data.wrapping_add(src_data);

        dst_operand.write(result, Size::Long);
    }
}

pub(crate) struct ADDI {
    pub(crate) size: Size,
}

impl Instruction for ADDI {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);
        let result = dst_data.wrapping_add(src_data);

        dst_operand.write(result, self.size);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct ADDQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
    pub(crate) to_address_reg: bool,
}

impl Instruction for ADDQ {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let dst_operand = operand_set.next();
        let dst_data = dst_operand.read(self.size);
        let result = self.data.wrapping_add(dst_data);
        dst_operand.write(result, self.size);

        if self.to_address_reg {
            return;
        }
        let src_msb = self.data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct ADDX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl Instruction for ADDX {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);

        let x_bit = cpu_internals.register_set.sr.get_bit(StatusFlag::X);

        let result = src_data.wrapping_add(dst_data).wrapping_add(x_bit);
        dst_operand.write(result, self.size);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let is_zero = result.is_zero(self.size);
        if !is_zero {
            sr.set_flag(StatusFlag::Z, is_zero);
        }
    }
}

pub(crate) struct SUB {
    pub(crate) size: Size,
}

impl Instruction for SUB {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);
        let result = dst_data.wrapping_sub(src_data);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct SUBA {
    pub(crate) size: Size,
}

impl Instruction for SUBA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size).sign_extend(self.size);
        let dst_data = dst_operand.read(Size::Long);
        let result = dst_data.wrapping_sub(src_data);

        dst_operand.write(result, Size::Long);
    }
}

pub(crate) struct SUBI {
    pub(crate) size: Size,
}

impl Instruction for SUBI {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);
        let result = dst_data.wrapping_sub(src_data);

        dst_operand.write(result, self.size);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct SUBQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
    pub(crate) to_address_reg: bool,
}

impl Instruction for SUBQ {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let dst_operand = operand_set.next();
        let dst_data = dst_operand.read(self.size);
        let result = dst_data.wrapping_sub(self.data);
        dst_operand.write(result, self.size);

        if self.to_address_reg {
            return;
        }
        let src_msb = self.data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct SUBX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode,
}

impl Instruction for SUBX {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);

        let x_bit = cpu_internals.register_set.sr.get_bit(StatusFlag::X);

        let result = dst_data.wrapping_sub(src_data).wrapping_sub(x_bit);
        dst_operand.write(result, self.size);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let is_zero = result.is_zero(self.size);
        if !is_zero {
            sr.set_flag(StatusFlag::Z, is_zero);
        }
    }
}

pub(crate) struct CLR {
    pub(crate) size: Size,
}

impl Instruction for CLR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        operand.write(0, self.size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, false);
        sr.set_flag(StatusFlag::Z, true);
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct CMP {
    pub(crate) size: Size,
}

impl Instruction for CMP {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(self.size);
        let dst_data = dst_operand.read(self.size);
        let result = dst_data.wrapping_sub(src_data);

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::N, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct CMPA {
    pub(crate) size: Size,
}

impl Instruction for CMPA {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        CMP { size: self.size }.execute(operand_set, cpu_internals);
    }
}
pub(crate) struct CMPI {
    pub(crate) size: Size,
}

impl Instruction for CMPI {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        CMP { size: self.size }.execute(operand_set, cpu_internals);
    }
}
pub(crate) struct CMPM {
    pub(crate) size: Size,
}

impl Instruction for CMPM {
    fn execute(&self, operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        CMP { size: self.size }.execute(operand_set, cpu_internals);
    }
}
pub(crate) struct EXT {
    pub(crate) src_size: Size,
    pub(crate) target_size: Size,
}

impl Instruction for EXT {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(self.src_size);
        let result = data.sign_extend(self.target_size);
        operand.write(result, self.target_size);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.target_size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.target_size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct NEG {
    pub(crate) size: Size,
}

impl Instruction for NEG {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(self.size);
        let result = 0u32.wrapping_sub(data);
        operand.write(result, self.size);

        let negate = result.is_negate(self.size);
        let zero = result.is_zero(self.size);
        let carry = !zero; // в описании инструкции указано !zero, в таблице вычисления флагов dm || rm

        let src_msb = data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);
        let overflow = src_msb & res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct NEGX {
    pub(crate) size: Size,
}

impl Instruction for NEGX {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(self.size);
        let x_bit = cpu_internals.register_set.sr.get_bit(StatusFlag::X);

        let result = 0u32.wrapping_sub(data).wrapping_sub(x_bit);
        operand.write(result, self.size);

        let dst_msb = data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);
        let overflow = dst_msb & res_msb;
        let carry = dst_msb | res_msb;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let is_zero = result.is_zero(self.size);
        if !is_zero {
            sr.set_flag(StatusFlag::Z, is_zero);
        }
    }
}

pub(crate) struct MULS();

impl Instruction for MULS {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read(Size::Word);
        let dst_data = dst_operand.read(Size::Word);

        let (result, overflow) = (src_data as i32).overflowing_mul(dst_data as i32); // TODO may be there is needs use casting to i16 for correct calculation of the overflow status
        let result = result as u32;
        dst_operand.write(result, Size::Long);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(Size::Long));
        sr.set_flag(StatusFlag::Z, result.is_zero(Size::Long));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct MULU();

impl Instruction for MULU {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(Size::Word);
        let dst_data = dst_operand.read(Size::Word);
        let (result, overflow) = src_data.overflowing_mul(dst_data); // TODO may be there is needs use casting to u16 for correct calculation of the overflow status
        dst_operand.write(result, Size::Long);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(Size::Long));
        sr.set_flag(StatusFlag::Z, result.is_zero(Size::Long));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct DIVS();

impl Instruction for DIVS {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(Size::Word) as i32;
        let dst_data = dst_operand.read(Size::Word) as i32;

        if src_data == 0 {
            cpu_internals.trap = Some(DIVISION_BY_ZERO);
            return;
        }
        let (quotient, overflow) = dst_data.overflowing_div(src_data);
        if overflow {
            cpu_internals
                .register_set
                .sr
                .set_flag(StatusFlag::V, overflow);
            return;
        }
        let remainder = dst_data % src_data;
        let result = remainder << 16 | (quotient & 0xFFFF);

        dst_operand.write(result as u32, Size::Long);

        let negate = (quotient as u32).is_negate(Size::Word);
        let zero = (quotient as u32).is_zero(Size::Word);
        let carry = false;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

pub(crate) struct DIVU();

impl Instruction for DIVU {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read(Size::Word);
        let dst_data = dst_operand.read(Size::Word);

        if src_data == 0 {
            cpu_internals.trap = Some(DIVISION_BY_ZERO);
            return;
        }
        let (quotient, overflow) = dst_data.overflowing_div(src_data);
        if overflow {
            cpu_internals
                .register_set
                .sr
                .set_flag(StatusFlag::V, overflow);
            return;
        }
        let remainder = dst_data % src_data;
        let result = remainder << 16 | (quotient & 0xFFFF);

        dst_operand.write(result, Size::Long);

        let negate = (quotient as u32).is_negate(Size::Word);
        let zero = (quotient as u32).is_zero(Size::Word);
        let carry = false;

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
    }
}

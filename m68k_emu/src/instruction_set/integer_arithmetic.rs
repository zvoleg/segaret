use std::fmt::Display;

use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::OperandSet, primitives::Size,
    status_flag::StatusFlag, vectors::DIVISION_BY_ZERO, IsNegate, IsZero, MsbIsSet, SignExtending,
};

use super::RegisterFieldMode;

pub(crate) struct ADD {
    pub(crate) size: Size,
}

impl Display for ADD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADD.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ADD {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;
        let result = src_data.wrapping_add(dst_data);
        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct ADDA {
    pub(crate) size: Size,
}

impl Display for ADDA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADDA.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ADDA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?.sign_extend(self.size);
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_add(src_data);

        dst_operand.write(result)?;
        Ok(())
    }
}

pub(crate) struct ADDI {
    pub(crate) size: Size,
}

impl Display for ADDI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADDI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ADDI {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_add(src_data);

        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct ADDQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
    pub(crate) to_address_reg: bool,
}

impl Display for ADDQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADDQ.{} #{:02X}", self.size, self.data)
    }
}

impl<T: BusM68k> Instruction<T> for ADDQ {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let dst_operand = operand_set.next();
        let dst_data = dst_operand.read()?;
        let result = self.data.wrapping_add(dst_data);
        dst_operand.write(result)?;

        if self.to_address_reg {
            return Ok(());
        }
        let src_msb = self.data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct ADDX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode, // TODO remove it (not used in the execute function)
}

impl Display for ADDX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADDX.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for ADDX {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;

        let x_bit = cpu.register_set.sr.get_bit(StatusFlag::X);

        let result = src_data.wrapping_add(dst_data).wrapping_add(x_bit);
        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = src_msb && dst_msb && !res_msb || !src_msb && !dst_msb && res_msb;
        let carry = src_msb && dst_msb || !res_msb && dst_msb || src_msb && !res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let zero_flag = sr.get_flag(StatusFlag::Z);
        let is_zero = result.is_zero(self.size);
        sr.set_flag(StatusFlag::Z, zero_flag & is_zero);
        Ok(())
    }
}

pub(crate) struct SUB {
    pub(crate) size: Size,
}

impl Display for SUB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUB.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for SUB {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_sub(src_data);
        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct SUBA {
    pub(crate) size: Size,
}

impl Display for SUBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUBA.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for SUBA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?.sign_extend(self.size);
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_sub(src_data);

        dst_operand.write(result)?;
        Ok(())
    }
}

pub(crate) struct SUBI {
    pub(crate) size: Size,
}

impl Display for SUBI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUBI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for SUBI {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_sub(src_data);

        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct SUBQ {
    pub(crate) size: Size,
    pub(crate) data: u32,
    pub(crate) to_address_reg: bool,
}

impl Display for SUBQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUBQ.{} #{:02X}", self.size, self.data)
    }
}

impl<T: BusM68k> Instruction<T> for SUBQ {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let dst_operand = operand_set.next();
        let dst_data = dst_operand.read()?;
        let result = dst_data.wrapping_sub(self.data);
        dst_operand.write(result)?;

        if self.to_address_reg {
            return Ok(());
        }
        let src_msb = self.data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct SUBX {
    pub(crate) size: Size,
    pub(crate) register_field_mode: RegisterFieldMode, // TODO remove it (not used in the execute function)
}

impl Display for SUBX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SUBX.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for SUBX {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;

        let x_bit = cpu.register_set.sr.get_bit(StatusFlag::X);

        let result = dst_data.wrapping_sub(src_data).wrapping_sub(x_bit);
        dst_operand.write(result)?;

        let src_msb = src_data.msb_is_set(self.size);
        let dst_msb = dst_data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);

        let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
        let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let zero_flag = sr.get_flag(StatusFlag::Z);
        let is_zero = result.is_zero(self.size);
        sr.set_flag(StatusFlag::Z, zero_flag & is_zero);
        Ok(())
    }
}

pub(crate) struct CLR {
    pub(crate) size: Size,
}

impl Display for CLR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLR.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for CLR {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = operand_set.next();
        operand.write(0)?;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, false);
        sr.set_flag(StatusFlag::Z, true);
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
        Ok(())
    }
}

fn cmp<T: BusM68k>(src_data: u32, dst_data: u32, size: Size, cpu: &mut M68k<T>) -> Result<(), ()> {
    let result = dst_data.wrapping_sub(src_data);

    let src_msb = src_data.msb_is_set(size);
    let dst_msb = dst_data.msb_is_set(size);
    let res_msb = result.msb_is_set(size);

    let overflow = !src_msb && dst_msb && !res_msb || src_msb && !dst_msb && res_msb;
    let carry = src_msb && !dst_msb || res_msb && !dst_msb || src_msb && res_msb;

    let sr = &mut cpu.register_set.sr;
    sr.set_flag(StatusFlag::Z, result.is_zero(size));
    sr.set_flag(StatusFlag::N, result.is_negate(size));
    sr.set_flag(StatusFlag::V, overflow);
    sr.set_flag(StatusFlag::C, carry);
    Ok(())
}

pub(crate) struct CMP {
    pub(crate) size: Size,
}

impl Display for CMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMP.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for CMP {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;

        cmp(src_data, dst_data, self.size, cpu)
    }
}

pub(crate) struct CMPA {
    pub(crate) size: Size,
}

impl Display for CMPA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMPA.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for CMPA {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let mut src_data = src_operand.read()?;
        if self.size == Size::Word {
            src_data = src_data.sign_extend(self.size)
        }
        let dst_operand = operand_set.next();
        let dst_data = dst_operand.read()?;

        cmp(src_data, dst_data, Size::Long, cpu)
    }
}
pub(crate) struct CMPI {
    pub(crate) size: Size,
}

impl Display for CMPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMPI.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for CMPI {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        CMP { size: self.size }.execute(operand_set, cpu)
    }
}
pub(crate) struct CMPM {
    pub(crate) size: Size,
}

impl Display for CMPM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMPM.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for CMPM {
    fn execute(&self, operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        CMP { size: self.size }.execute(operand_set, cpu)
    }
}
pub(crate) struct EXT {
    pub(crate) src_size: Size,
    pub(crate) target_size: Size,
}

impl Display for EXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXT.{}", self.target_size)
    }
}

impl<T: BusM68k> Instruction<T> for EXT {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = operand_set.next();
        let data = operand.read_sized(self.src_size)?;
        let result = data.sign_extend(self.src_size);
        operand.write(result)?;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(self.target_size));
        sr.set_flag(StatusFlag::Z, result.is_zero(self.target_size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
        Ok(())
    }
}

pub(crate) struct NEG {
    pub(crate) size: Size,
}

impl Display for NEG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NEG.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for NEG {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = operand_set.next();
        let data = operand.read()?;
        let result = 0u32.wrapping_sub(data);
        operand.write(result)?;

        let negate = result.is_negate(self.size);
        let zero = result.is_zero(self.size);

        let src_msb = data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);
        let overflow = src_msb && res_msb;
        let carry = src_msb || res_msb; // instruction description sais '!zero' but the flag calculation table sais dm || rm

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct NEGX {
    pub(crate) size: Size,
}

impl Display for NEGX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NEGX.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for NEGX {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = operand_set.next();
        let data = operand.read()?;
        let x_bit = cpu.register_set.sr.get_bit(StatusFlag::X);

        let result = 0u32.wrapping_sub(data).wrapping_sub(x_bit);
        operand.write(result)?;

        let dst_msb = data.msb_is_set(self.size);
        let res_msb = result.msb_is_set(self.size);
        let overflow = dst_msb && res_msb;
        let carry = dst_msb || res_msb;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::X, carry);
        sr.set_flag(StatusFlag::N, result.is_negate(self.size));
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);

        let is_zero = result.is_zero(self.size);
        if !is_zero {
            sr.set_flag(StatusFlag::Z, is_zero);
        }
        Ok(())
    }
}

pub(crate) struct MULS();

impl Display for MULS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MULS.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for MULS {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read()?.sign_extend(Size::Word) as i32;
        let dst_data = dst_operand.read()?.sign_extend(Size::Word) as i32;

        let (result, _) = (src_data).overflowing_mul(dst_data); // TODO may be there is needs use casting to i16 for correct calculation of the overflow status
        let result = result as u32;
        dst_operand.write_sized(result, Size::Long)?;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(Size::Long));
        sr.set_flag(StatusFlag::Z, result.is_zero(Size::Long));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
        Ok(())
    }
}

pub(crate) struct MULU();

impl Display for MULU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MULU.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for MULU {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;
        let (result, _) = src_data.overflowing_mul(dst_data); // TODO may be there is needs use casting to u16 for correct calculation of the overflow status
        dst_operand.write_sized(result, Size::Long)?;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, result.is_negate(Size::Long));
        sr.set_flag(StatusFlag::Z, result.is_zero(Size::Long));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
        Ok(())
    }
}

pub(crate) struct DIVS();

impl Display for DIVS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DIVS.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for DIVS {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?.sign_extend(Size::Word) as i32;
        let dst_data = dst_operand.read()? as i32;

        if src_data == 0 {
            cpu.trap = Some(DIVISION_BY_ZERO);
            return Ok(());
        }

        let quotient = dst_data.wrapping_div(src_data);

        let overflow = quotient <= i16::MIN as i32 || quotient > i16::MAX as i32;

        if overflow {
            cpu.register_set.sr.set_flag(StatusFlag::V, overflow);
            return Ok(());
        }

        let remainder = dst_data.wrapping_rem(src_data);
        let result = (remainder as u32) << 16 | ((quotient as u32) & 0xFFFF);

        dst_operand.write_sized(result as u32, Size::Long)?;

        let negate = (quotient as u32).is_negate(Size::Word);
        let zero = (quotient as u32).is_zero(Size::Word);
        let carry = false;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

pub(crate) struct DIVU();

impl Display for DIVU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DIVU.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for DIVU {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();

        let src_data = src_operand.read()?;
        let dst_data = dst_operand.read()?;

        if src_data == 0 {
            cpu.trap = Some(DIVISION_BY_ZERO);
            return Ok(());
        }
        let (quotient, _) = dst_data.overflowing_div(src_data);
        let dst_msw = dst_data >> 16;
        let overflow = dst_msw >= src_data;
        if overflow {
            cpu.register_set.sr.set_flag(StatusFlag::V, overflow);
            return Ok(());
        }
        let remainder = dst_data % src_data;
        let result = remainder << 16 | (quotient & 0xFFFF);

        dst_operand.write_sized(result, Size::Long)?;

        let negate = (quotient as u32).is_negate(Size::Word);
        let zero = (quotient as u32).is_zero(Size::Word);
        let carry = false;

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, negate);
        sr.set_flag(StatusFlag::Z, zero);
        sr.set_flag(StatusFlag::V, overflow);
        sr.set_flag(StatusFlag::C, carry);
        Ok(())
    }
}

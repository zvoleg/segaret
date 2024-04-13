use crate::{
    cpu_internals::CpuInternals,
    instruction_set::Instruction,
    operand::{Operand, OperandSet},
    primitives::Size,
    status_flag::StatusFlag,
    status_register::StatusRegister,
    IsNegate, IsZero, MsbIsSet,
};

use super::ShiftDirection;

pub(crate) struct ASdDataReg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ASdDataReg {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let operand = operand_set.next();
        let count = data_reg_operand.read(Size::Long) % 64;
        match self.direction {
            ShiftDirection::Right => asr(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => asl(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ASdImplied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for ASdImplied {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => asr(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => asl(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ASdMemory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ASdMemory {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => {
                asr(1, operand, Size::Word, &mut cpu_internals.register_set.sr)
            }
            ShiftDirection::Left => asl(1, operand, Size::Word, &mut cpu_internals.register_set.sr),
        }
    }
}

fn asl(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    let mut overflow = false;
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let poped_bit = data.msb_is_set(size);
        data <<= 1;
        let msb_after = data.msb_is_set(size);

        if !overflow && poped_bit != msb_after {
            overflow = true;
        }
        sr.set_flag(StatusFlag::X, poped_bit);
        sr.set_flag(StatusFlag::C, poped_bit);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, overflow);
}

fn asr(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    let msb = if data.msb_is_set(size) { 1 } else { 0 };
    let msb_mask = msb << ((8 * size as u32) - 1);
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let poped_bit = data & 1 == 1;

        data >>= 1;
        data |= msb_mask;

        sr.set_flag(StatusFlag::X, poped_bit);
        sr.set_flag(StatusFlag::C, poped_bit);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

pub(crate) struct LSdDataReg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for LSdDataReg {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let operand = operand_set.next();
        let count = data_reg_operand.read(Size::Long) % 64;
        match self.direction {
            ShiftDirection::Right => lsr(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => lsl(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct LSdImplied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for LSdImplied {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => lsr(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => lsl(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct LSdMemory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for LSdMemory {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => {
                lsr(1, operand, Size::Word, &mut cpu_internals.register_set.sr)
            }
            ShiftDirection::Left => lsl(1, operand, Size::Word, &mut cpu_internals.register_set.sr),
        }
    }
}

fn lsl(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let poped_bit = data.msb_is_set(size);
        data <<= 1;

        sr.set_flag(StatusFlag::X, poped_bit);
        sr.set_flag(StatusFlag::C, poped_bit);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

fn lsr(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let poped_bit = data & 1 == 1;
        data >>= 1;

        sr.set_flag(StatusFlag::X, poped_bit);
        sr.set_flag(StatusFlag::C, poped_bit);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

pub(crate) struct ROdDataReg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ROdDataReg {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let operand = operand_set.next();
        let count = data_reg_operand.read(Size::Long) % 64;
        match self.direction {
            ShiftDirection::Right => ror(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => rol(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ROdImplied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for ROdImplied {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => ror(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => rol(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ROdMemory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ROdMemory {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => {
                ror(1, operand, Size::Word, &mut cpu_internals.register_set.sr)
            }
            ShiftDirection::Left => rol(1, operand, Size::Word, &mut cpu_internals.register_set.sr),
        }
    }
}

fn rol(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let msb = (data >> ((8 * size as u32) - 1)) & 1;
        data <<= 1;
        data |= msb;

        sr.set_flag(StatusFlag::C, msb == 1);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

fn ror(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, false); // cleared if count == 0
    for _ in 0..count {
        let lsb = data & 1;
        let msb_mask = lsb << ((8 * size as u32) - 1);
        data >>= 1;
        data |= msb_mask;

        sr.set_flag(StatusFlag::C, lsb == 1);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

pub(crate) struct ROXdDataReg {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ROXdDataReg {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let operand = operand_set.next();
        let count = data_reg_operand.read(Size::Long) % 64;
        match self.direction {
            ShiftDirection::Right => roxr(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => roxl(
                count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ROXdImplied {
    pub(crate) size: Size,
    pub(crate) direction: ShiftDirection,
    pub(crate) count: u32,
}

impl Instruction for ROXdImplied {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => roxr(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
            ShiftDirection::Left => roxl(
                self.count,
                operand,
                self.size,
                &mut cpu_internals.register_set.sr,
            ),
        }
    }
}

pub(crate) struct ROXdMemory {
    pub(crate) direction: ShiftDirection,
}

impl Instruction for ROXdMemory {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        match self.direction {
            ShiftDirection::Right => {
                roxr(1, operand, Size::Word, &mut cpu_internals.register_set.sr)
            }
            ShiftDirection::Left => {
                roxl(1, operand, Size::Word, &mut cpu_internals.register_set.sr)
            }
        }
    }
}

fn roxl(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, sr.get_flag(StatusFlag::X)); // if count == 0 then C == X
    for _ in 0..count {
        let msb = (data >> ((8 * size as u32) - 1)) & 1;
        data <<= 1;
        data |= sr.get_bit(StatusFlag::X);

        sr.set_flag(StatusFlag::X, msb == 1);
        sr.set_flag(StatusFlag::C, msb == 1);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

fn roxr(count: u32, operand: Operand, size: Size, sr: &mut StatusRegister) {
    let mut data = operand.read(size);
    sr.set_flag(StatusFlag::C, sr.get_flag(StatusFlag::X)); // if count == 0 then C == X
    for _ in 0..count {
        let lsb = data & 1;
        let msb_mask = sr.get_bit(StatusFlag::X) << ((8 * size as u32) - 1);
        data >>= 1;
        data |= msb_mask;

        sr.set_flag(StatusFlag::X, lsb == 1);
        sr.set_flag(StatusFlag::C, lsb == 1);
    }
    operand.write(data, size);

    sr.set_flag(StatusFlag::N, data.is_negate(size));
    sr.set_flag(StatusFlag::Z, data.is_zero(size));
    sr.set_flag(StatusFlag::V, false);
}

pub(crate) struct SWAP();

impl Instruction for SWAP {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let mut data = operand.read(Size::Long);

        let msw = (data & 0xFFFF0000) >> 16;
        let lsw = (data & 0x0000FFFF) << 16;
        data = lsw | msw;

        operand.write(data, Size::Long);

        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, data.is_negate(Size::Long));
        sr.set_flag(StatusFlag::Z, data.is_zero(Size::Long));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

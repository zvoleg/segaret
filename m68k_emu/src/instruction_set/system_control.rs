use crate::{
    cpu_internals::{CpuInternals, RegisterType},
    instruction_set::Instruction,
    operand::OperandSet,
    primitives::Size,
    status_flag::StatusFlag,
    vectors::{CHK_INSTRUCTION, ILLEGAL_INSTRUCTION, RESET_SP},
    IsNegate, STACK_REGISTER,
};

use super::MoveDirection;

pub(crate) struct MOVEtoSR();

impl Instruction for MOVEtoSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Word);
        cpu_internals.register_set.sr.set_sr(data);
    }
}

pub(crate) struct MOVEfromSR();

impl Instruction for MOVEfromSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        operand.write(cpu_internals.register_set.sr.get_sr() as u32, Size::Word);
    }
}

pub(crate) struct MOVEUSP {
    pub(crate) direction: MoveDirection,
}

impl Instruction for MOVEUSP {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let usp_reg = cpu_internals
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        match self.direction {
            MoveDirection::RegisterToMemory => {
                let data = usp_reg.read(Size::Long);
                operand.write(data, Size::Long);
            }
            MoveDirection::MemoryToRegister => {
                let data = operand.read(Size::Long);
                usp_reg.write(data, Size::Long);
            }
        }
    }
}

pub(crate) struct MOVEtoCCR();

impl Instruction for MOVEtoCCR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Word) & 0xFF; // operand size is word but used only low order byte
        cpu_internals.register_set.sr.set_ccr(data);
    }
}

pub(crate) struct RTE();

impl Instruction for RTE {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let sr_operand = operand_set.next();
        let pc_operand = operand_set.next();
        let sr = sr_operand.read(Size::Word);
        cpu_internals.register_set.sr.set_sr(sr);
        let pc = pc_operand.read(Size::Long);
        cpu_internals.register_set.pc = pc;
    }
}

pub(crate) struct ANDItoCCR();

impl Instruction for ANDItoCCR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Byte);
        let mut ccr = cpu_internals.register_set.sr.get_ccr();
        ccr &= data as u16;
        cpu_internals.register_set.sr.set_ccr(ccr as u32);
    }
}

pub(crate) struct ANDItoSR();

impl Instruction for ANDItoSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Word);
        let mut sr = cpu_internals.register_set.sr.get_sr();
        sr &= data as u16;
        cpu_internals.register_set.sr.set_sr(sr as u32);
    }
}

pub(crate) struct EORItoCCR();

impl Instruction for EORItoCCR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Byte);
        let mut ccr = cpu_internals.register_set.sr.get_ccr();
        ccr ^= data as u16;
        cpu_internals.register_set.sr.set_ccr(ccr as u32);
    }
}

pub(crate) struct EORItoSR();

impl Instruction for EORItoSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Word);
        let mut sr = cpu_internals.register_set.sr.get_sr();
        sr ^= data as u16;
        cpu_internals.register_set.sr.set_sr(sr as u32);
    }
}

pub(crate) struct ORItoCCR();

impl Instruction for ORItoCCR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Byte);
        let mut ccr = cpu_internals.register_set.sr.get_ccr();
        ccr |= data as u16;
        cpu_internals.register_set.sr.set_ccr(ccr as u32);
    }
}

pub(crate) struct ORItoSR();

impl Instruction for ORItoSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let data = operand.read(Size::Word);
        let mut sr = cpu_internals.register_set.sr.get_sr();
        sr |= data as u16;
        cpu_internals.register_set.sr.set_sr(sr as u32);
    }
}

pub(crate) struct CHK();

impl Instruction for CHK {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let operand = operand_set.next();
        let chk_data = data_reg_operand.read(Size::Word);
        let upper_bound = operand.read(Size::Word);

        let less_zerro = chk_data.is_negate(Size::Word);
        let greater_upper_bound = (chk_data as i16) > (upper_bound as i16);

        if less_zerro || greater_upper_bound {
            cpu_internals
                .register_set
                .sr
                .set_flag(StatusFlag::N, less_zerro);
            cpu_internals.trap = Some(CHK_INSTRUCTION);
        }
    }
}

pub(crate) struct ILLEAGL();

impl Instruction for ILLEAGL {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let pc_stack_operand = operand_set.next();
        let sr_stack_operand = operand_set.next();

        pc_stack_operand.write(cpu_internals.register_set.pc, Size::Long);
        sr_stack_operand.write(cpu_internals.register_set.sr.get_sr() as u32, Size::Word);

        cpu_internals.trap = Some(ILLEGAL_INSTRUCTION);
    }
}

pub(crate) struct TRAP {
    pub(crate) vector: u32,
}

impl Instruction for TRAP {
    fn execute(&self, _: OperandSet, _: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct TRAPV();

impl Instruction for TRAPV {
    fn execute(&self, _: OperandSet, _: &mut CpuInternals) {
        todo!()
    }
}

pub(crate) struct RESET();

impl Instruction for RESET {
    fn execute(&self, _: OperandSet, cpu_internals: &mut CpuInternals) {
        cpu_internals.trap = Some(RESET_SP)
    }
}

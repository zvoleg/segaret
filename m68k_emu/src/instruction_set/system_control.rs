use std::fmt::Display;

use crate::{
    bus::BusM68k,
    cpu::M68k,
    instruction_set::Instruction,
    operand::Operand,
    primitives::Size,
    status_flag::StatusFlag,
    vectors::{CHK_INSTRUCTION, ILLEGAL_INSTRUCTION, RESET_SP, TRAPV_INSTRUCTION, TRAP_0_15},
    IsNegate,
};

use super::MoveDirection;

pub(crate) struct MOVEtoSR();

impl Display for MOVEtoSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVE_to_SR.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEtoSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        cpu.register_set.sr.set_sr(data);
        Ok(())
    }
}

pub(crate) struct MOVEfromSR();

impl Display for MOVEfromSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVE_from_SR.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEfromSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        operand.write(cpu.register_set.sr.get_sr() as u32)?;
        Ok(())
    }
}

pub(crate) struct MOVEUSP {
    pub(crate) direction: MoveDirection,
}

impl Display for MOVEUSP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVEUSP")
    }
}

impl<T: BusM68k> Instruction<T> for MOVEUSP {
    fn execute(&self, operand_set: Vec<Operand>, _: &mut M68k<T>) -> Result<(), ()> {
        let src_operand = &operand_set[0];
        let dst_operand = &operand_set[1];
        let src_data = src_operand.read()?;
        dst_operand.write(src_data)?;
        Ok(())
    }
}

pub(crate) struct MOVEtoCCR();

impl Display for MOVEtoCCR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVE_to_CCR.{}", Size::Byte)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEtoCCR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()? & 0xFF; // operand size is word but used only low order byte
        cpu.register_set.sr.set_ccr(data);
        Ok(())
    }
}

pub(crate) struct RTE();

impl Display for RTE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTE")
    }
}

impl<T: BusM68k> Instruction<T> for RTE {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let sr_data = cpu.stack_pop(Size::Word);
        cpu.register_set.sr.set_sr(sr_data);
        cpu.register_set.pc = cpu.stack_pop(Size::Long);
        Ok(())
    }
}

pub(crate) struct ANDItoCCR();

impl Display for ANDItoCCR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANDI_to_CCR.{}", Size::Byte)
    }
}

impl<T: BusM68k> Instruction<T> for ANDItoCCR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut ccr = cpu.register_set.sr.get_ccr();
        ccr &= data as u16;
        cpu.register_set.sr.set_ccr(ccr as u32);
        Ok(())
    }
}

pub(crate) struct ANDItoSR();

impl Display for ANDItoSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANDI_to_SR.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for ANDItoSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut sr = cpu.register_set.sr.get_sr();
        sr &= data as u16;
        cpu.register_set.sr.set_sr(sr as u32);
        Ok(())
    }
}

pub(crate) struct EORItoCCR();

impl Display for EORItoCCR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EORI_to_CCR.{}", Size::Byte)
    }
}

impl<T: BusM68k> Instruction<T> for EORItoCCR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut ccr = cpu.register_set.sr.get_ccr();
        ccr ^= data as u16;
        cpu.register_set.sr.set_ccr(ccr as u32);
        Ok(())
    }
}

pub(crate) struct EORItoSR();

impl Display for EORItoSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EORI_to_SR.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for EORItoSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut sr = cpu.register_set.sr.get_sr();
        sr ^= data as u16;
        cpu.register_set.sr.set_sr(sr as u32);
        Ok(())
    }
}

pub(crate) struct ORItoCCR();

impl Display for ORItoCCR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ORI_to_CCR.{}", Size::Byte)
    }
}

impl<T: BusM68k> Instruction<T> for ORItoCCR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut ccr = cpu.register_set.sr.get_ccr();
        ccr |= data as u16;
        cpu.register_set.sr.set_ccr(ccr as u32);
        Ok(())
    }
}

pub(crate) struct ORItoSR();

impl Display for ORItoSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ORI_to_SR.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for ORItoSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let data = operand.read()?;
        let mut sr = cpu.register_set.sr.get_sr();
        sr |= data as u16;
        cpu.register_set.sr.set_sr(sr as u32);
        Ok(())
    }
}

pub(crate) struct CHK();

impl Display for CHK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHK.{}", Size::Word)
    }
}

impl<T: BusM68k> Instruction<T> for CHK {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let data_reg_operand = &operand_set[0];
        let operand = &operand_set[0];
        let chk_data = data_reg_operand.read()?;
        let upper_bound = operand.read()?;

        let less_zerro = chk_data.is_negate(Size::Word);
        let greater_upper_bound = (chk_data as i16) > (upper_bound as i16);

        if less_zerro || greater_upper_bound {
            cpu.register_set.sr.set_flag(StatusFlag::N, less_zerro);
            cpu.trap = Some(CHK_INSTRUCTION);
        }
        Ok(())
    }
}

pub(crate) struct ILLEAGL();

impl Display for ILLEAGL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ILLEGAL")
    }
}

impl<T: BusM68k> Instruction<T> for ILLEAGL {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        cpu.stack_push(cpu.register_set.pc, Size::Long);
        cpu.stack_push(cpu.register_set.sr.get_sr() as u32, Size::Word);

        cpu.trap = Some(ILLEGAL_INSTRUCTION);
        Ok(())
    }
}

pub(crate) struct TRAP {
    pub(crate) vector: u32,
}

impl Display for TRAP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TRAP")
    }
}

impl<T: BusM68k> Instruction<T> for TRAP {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let vector_address_offset = self.vector * 4;
        cpu.trap = Some(TRAP_0_15 + vector_address_offset);
        Ok(())
    }
}

pub(crate) struct TRAPV();

impl Display for TRAPV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TRAPV")
    }
}

impl<T: BusM68k> Instruction<T> for TRAPV {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let overflow = cpu.register_set.sr.get_flag(StatusFlag::V);
        if overflow {
            cpu.trap = Some(TRAPV_INSTRUCTION);
        }
        Ok(())
    }
}

pub(crate) struct RESET();

impl Display for RESET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RESET")
    }
}

impl<T: BusM68k> Instruction<T> for RESET {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        cpu.trap = Some(RESET_SP);
        Ok(())
    }
}

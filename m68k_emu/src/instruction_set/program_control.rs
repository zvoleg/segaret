use std::fmt::Display;

use crate::{
    bus::BusM68k, cpu::M68k, instruction_set::Instruction, operand::Operand, primitives::Size,
    status_flag::StatusFlag, status_register::StatusRegister, IsNegate, IsZero, SignExtending,
};

use super::Condition;

pub(crate) struct TST {
    pub(crate) size: Size,
}

impl Display for TST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TST.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for TST {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let data = operand_set[0].read()?;
        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, data.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, data.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
        Ok(())
    }
}

pub(crate) struct Bcc {
    pub(crate) condition: Condition,
    pub(crate) displacement: u32,
}

impl Display for Bcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.displacement == 0 {
            write!(f, "B{}", self.condition)
        } else {
            write!(f, "B{} #{:02X}", self.condition, self.displacement)
        }
    }
}

impl<T: BusM68k> Instruction<T> for Bcc {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let displacement = if self.displacement == 0 {
            operand_set[0].read()?.sign_extend(Size::Word)
        } else {
            self.displacement.sign_extend(Size::Byte)
        };
        if check_condition(self.condition, &cpu.register_set.sr) {
            let pc = &mut cpu.register_set.pc;
            if self.displacement == 0 {
                *pc -= 2; // for the calculation of the branch address it uses address just after instruction word without extension word
            }
            *pc = pc.wrapping_add(displacement);
        } else {
            let clock_corection = if self.displacement == 0 { 2 } else { -2i32 };
            cpu.cycles_counter = cpu.cycles_counter.wrapping_add(clock_corection);
        }
        Ok(())
    }
}

pub(crate) struct DBcc {
    pub(crate) condition: Condition,
}

impl Display for DBcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DB{}", self.condition)
    }
}

impl<T: BusM68k> Instruction<T> for DBcc {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let data_reg_operand = &operand_set[0];
        let displacement_operand = &operand_set[1];
        let displacement = displacement_operand.read()?.sign_extend(Size::Word);

        if !check_condition(self.condition, &cpu.register_set.sr) {
            let mut counter = data_reg_operand.read()?;
            counter = counter.wrapping_sub(1);
            data_reg_operand.write(counter)?;
            // compare counter with -1
            if counter != 0xFFFFFFFF {
                let pc = &mut cpu.register_set.pc;
                *pc = pc.wrapping_sub(2); // the PC pointer should to point on the extension word after dbcc instruction opcode
                *pc = pc.wrapping_add(displacement);
            } else {
                cpu.cycles_counter += 4 // if loop counter expired
            }
        } else {
            cpu.cycles_counter += 2 // if condition true
        }
        Ok(())
    }
}

pub(crate) struct Scc {
    pub(crate) condition: Condition,
}

impl Display for Scc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}", self.condition)
    }
}

impl<T: BusM68k> Instruction<T> for Scc {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        let condition = check_condition(self.condition, &cpu.register_set.sr);
        let result = if condition { 0xFF } else { 0x00 };
        operand.write(result)?;
        Ok(())
    }
}

pub(crate) struct BRA {
    pub(crate) displacement: u32,
}

impl Display for BRA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.displacement == 0 {
            write!(f, "BRA")
        } else {
            write!(f, "BRA #{:02X}", self.displacement)
        }
    }
}

impl<T: BusM68k> Instruction<T> for BRA {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let target: u32;
        let displacement = if self.displacement == 0 {
            let operand = &operand_set[0];
            target = operand.operand_address;
            operand.read()?.sign_extend(Size::Word)
        } else {
            target = cpu.register_set.pc;
            self.displacement.sign_extend(Size::Byte)
        };

        cpu.register_set.pc = target.wrapping_add(displacement);
        Ok(())
    }
}

pub(crate) struct BSR {
    pub(crate) displacement: u32,
}

impl Display for BSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.displacement == 0 {
            write!(f, "BSR")
        } else {
            write!(f, "BSR #{:02X}", self.displacement)
        }
    }
}

impl<T: BusM68k> Instruction<T> for BSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let target: u32;
        let displacement = if self.displacement == 0 {
            let operand = &operand_set[0];
            target = operand.operand_address;
            operand.read()?.sign_extend(Size::Word)
        } else {
            target = cpu.register_set.pc;
            self.displacement.sign_extend(Size::Byte)
        };

        let pc = cpu.register_set.pc;
        cpu.stack_push(pc, Size::Long);
        cpu.register_set.pc = target.wrapping_add(displacement);
        Ok(())
    }
}

pub(crate) struct JMP();

impl Display for JMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JMP")
    }
}

impl<T: BusM68k> Instruction<T> for JMP {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];
        cpu.register_set.pc = operand.operand_address;
        Ok(())
    }
}

pub(crate) struct JSR();

impl Display for JSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSR")
    }
}

impl<T: BusM68k> Instruction<T> for JSR {
    fn execute(&self, operand_set: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let operand = &operand_set[0];

        cpu.stack_push(cpu.register_set.pc, Size::Long);
        cpu.register_set.pc = operand.operand_address;
        Ok(())
    }
}

pub(crate) struct NOP();

impl Display for NOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOP")
    }
}

impl<T: BusM68k> Instruction<T> for NOP {
    fn execute(&self, _: Vec<Operand>, _: &mut M68k<T>) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) struct RTR();

impl Display for RTR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTR")
    }
}

impl<T: BusM68k> Instruction<T> for RTR {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let ccr = cpu.stack_pop(Size::Word);
        cpu.register_set.sr.set_ccr(ccr);

        let pc = cpu.stack_pop(Size::Long);
        cpu.register_set.pc = pc;
        Ok(())
    }
}

pub(crate) struct RTS();

impl Display for RTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTS")
    }
}

impl<T: BusM68k> Instruction<T> for RTS {
    fn execute(&self, _: Vec<Operand>, cpu: &mut M68k<T>) -> Result<(), ()> {
        let pc = cpu.stack_pop(Size::Long);
        cpu.register_set.pc = pc;
        Ok(())
    }
}

fn check_condition(condition: Condition, sr: &StatusRegister) -> bool {
    match condition {
        Condition::TRUE => true,
        Condition::FALSE => false,
        Condition::HI => {
            let carry = sr.get_flag(StatusFlag::C);
            let zero = sr.get_flag(StatusFlag::Z);
            !carry & !zero
        }
        Condition::LS => {
            let carry = sr.get_flag(StatusFlag::C);
            let zero = sr.get_flag(StatusFlag::Z);
            carry | zero
        }
        Condition::CC => {
            let carry = sr.get_flag(StatusFlag::C);
            !carry
        }
        Condition::CS => {
            let carry = sr.get_flag(StatusFlag::C);
            carry
        }
        Condition::NE => {
            let zero = sr.get_flag(StatusFlag::Z);
            !zero
        }
        Condition::EQ => {
            let zero = sr.get_flag(StatusFlag::Z);
            zero
        }
        Condition::VC => {
            let overflow = sr.get_flag(StatusFlag::V);
            !overflow
        }
        Condition::VS => {
            let overflow = sr.get_flag(StatusFlag::V);
            overflow
        }
        Condition::PL => {
            let negate = sr.get_flag(StatusFlag::N);
            !negate
        }
        Condition::MI => {
            let negate = sr.get_flag(StatusFlag::N);
            negate
        }
        Condition::GE => {
            let negate = sr.get_flag(StatusFlag::N);
            let overflow = sr.get_flag(StatusFlag::V);
            negate & overflow | !negate & !overflow
        }
        Condition::LT => {
            let negate = sr.get_flag(StatusFlag::N);
            let overflow = sr.get_flag(StatusFlag::V);
            negate & !overflow | !negate & overflow
        }
        Condition::GT => {
            let negate = sr.get_flag(StatusFlag::N);
            let overflow = sr.get_flag(StatusFlag::V);
            let zero = sr.get_flag(StatusFlag::Z);
            negate & overflow & !zero | !negate & !overflow & !zero
        }
        Condition::LE => {
            let negate = sr.get_flag(StatusFlag::N);
            let overflow = sr.get_flag(StatusFlag::V);
            let zero = sr.get_flag(StatusFlag::Z);
            zero | negate & !overflow | !negate & overflow
        }
    }
}

use crate::{
    cpu_internals::CpuInternals, instruction_set::Instruction, operand::OperandSet,
    primitives::Size, status_flag::StatusFlag, status_register::StatusRegister, IsNegate, IsZero,
    SignExtending,
};

use super::Condition;

pub(crate) struct TST {
    pub(crate) size: Size,
}

impl Instruction for TST {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data = operand_set.next().read(self.size);
        let sr = &mut cpu_internals.register_set.sr;
        sr.set_flag(StatusFlag::N, data.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, data.is_zero(self.size));
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct Bcc {
    pub(crate) condition: Condition,
    pub(crate) displacement: u32,
}

impl Instruction for Bcc {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let displacement = if self.displacement == 0 {
            operand_set.next().read(Size::Word).sign_extend(Size::Word)
        } else {
            self.displacement.sign_extend(Size::Byte)
        };
        if check_condition(self.condition, &cpu_internals.register_set.sr) {
            let pc = &mut cpu_internals.register_set.pc;
            *pc = pc.wrapping_add(displacement);
        } else {
            let clock_corection = if self.displacement == 0 { 2 } else { -2i32 };
            cpu_internals.cycles = cpu_internals.cycles.wrapping_add(clock_corection);
        }
    }
}

pub(crate) struct DBcc {
    pub(crate) condition: Condition,
}

impl Instruction for DBcc {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let data_reg_operand = operand_set.next();
        let displacement_operand = operand_set.next();
        let displacement = displacement_operand
            .read(Size::Word)
            .sign_extend(Size::Word);

        if !check_condition(self.condition, &cpu_internals.register_set.sr) {
            let mut counter = data_reg_operand.read(Size::Word);
            counter = counter.wrapping_sub(1);
            data_reg_operand.write(counter, Size::Word);
            if counter != 0xFFFF {
                // -1
                let pc = &mut cpu_internals.register_set.pc;
                *pc = pc.wrapping_add(displacement);
            } else {
                cpu_internals.cycles += 4 // if loop counter expired
            }
        } else {
            cpu_internals.cycles += 2 // if condition true
        }
    }
}

pub(crate) struct Scc {
    pub(crate) condition: Condition,
}

impl Instruction for Scc {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        let condition = check_condition(self.condition, &cpu_internals.register_set.sr);
        let result = if condition { 0xFF } else { 0x00 };
        operand.write(result, Size::Byte);
    }
}

pub(crate) struct BRA {
    pub(crate) displacement: u32,
}

impl Instruction for BRA {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let displacement = if self.displacement == 0 {
            operand_set.next().read(Size::Word).sign_extend(Size::Word)
        } else {
            self.displacement.sign_extend(Size::Byte)
        };

        let pc = &mut cpu_internals.register_set.pc;
        *pc = pc.wrapping_add(displacement);
    }
}

pub(crate) struct BSR {
    pub(crate) displacement: u32,
}

impl Instruction for BSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let displacement = if self.displacement == 0 {
            operand_set.next().read(Size::Word).sign_extend(Size::Word)
        } else {
            self.displacement.sign_extend(Size::Byte)
        };
        let stack_operand = operand_set.next();

        let pc = &mut cpu_internals.register_set.pc;
        stack_operand.write(*pc, Size::Long);
        *pc = pc.wrapping_add(displacement);
    }
}

pub(crate) struct JMP();

impl Instruction for JMP {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let operand = operand_set.next();
        cpu_internals.register_set.pc = operand.operand_address;
    }
}

pub(crate) struct JSR();

impl Instruction for JSR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let stack_operand = operand_set.next();
        let operand = operand_set.next();

        let pc = &mut cpu_internals.register_set.pc;
        stack_operand.write(*pc, Size::Long);
        *pc = operand.operand_address;
    }
}

pub(crate) struct NOP();

impl Instruction for NOP {
    fn execute(&self, _: OperandSet, _: &mut CpuInternals) {}
}

pub(crate) struct RTR();

impl Instruction for RTR {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let stack_ccr_operand = operand_set.next();
        let stack_pc_operand = operand_set.next();

        let ccr = stack_ccr_operand.read(Size::Byte);
        cpu_internals.register_set.sr.set_ccr(ccr);

        let pc = stack_pc_operand.read(Size::Long);
        cpu_internals.register_set.pc = pc;
    }
}

pub(crate) struct RTS();

impl Instruction for RTS {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let stack_pc_operand = operand_set.next();

        let pc = stack_pc_operand.read(Size::Long);
        cpu_internals.register_set.pc = pc;
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

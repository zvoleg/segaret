use crate::{
    addressing_mode_set::AddressingModeType,
    bus::BusM68k,
    cpu::M68k,
    cpu_internals::RegisterType,
    instruction_set::Instruction,
    operand::{Operand, OperandSet},
    primitives::Size,
    status_flag::StatusFlag,
    IsNegate, SignExtending,
};

use super::{ExchangeMode, MoveDirection};

pub(crate) struct MOVE {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for MOVE {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_operand = operand_set.next();
        dst_operand.write(src_data, self.size);

        cpu.register_set
            .sr
            .set_flag(StatusFlag::N, src_data.is_negate(self.size));
        cpu.register_set.sr.set_flag(StatusFlag::Z, src_data == 0);
        cpu.register_set.sr.set_flag(StatusFlag::V, false);
        cpu.register_set.sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct MOVEA {
    pub(crate) size: Size,
}

impl<T: BusM68k> Instruction<T> for MOVEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let src_data = operand_set.next().read(self.size);
        operand_set.next().write(src_data, self.size);
    }
}

/// MOVEM instruction has the opcode and one or two extension word.
/// First extension word is bit mask described the affected registers.
/// So, first addressing mode should be the Immediate with the size equals to Word.
/// And second addressing mode setup a memory location for reading/writing
///
/// This instruction has only Word and Long sizes
/// With the Word size transfers to registers the value is sign-extends to the Long size and whole register revrites.
pub(crate) struct MOVEM {
    pub(crate) size: Size,
    pub(crate) direction: MoveDirection,
    pub(crate) addressing_mode_type: AddressingModeType,
    pub(crate) am_register_idx: isize, // needs to determine writes into the sorce register of adressing mode
}

impl<T: BusM68k> Instruction<T> for MOVEM {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let extension_word = operand_set.next().read(Size::Word);
        let affected_register_offsets =
            self.collect_affected_register_offsets(extension_word as u16);

        // TODO additional cycles

        match self.direction {
            MoveDirection::RegisterToMemory => {
                self.write_registers_to_memory(&affected_register_offsets, operand_set.next(), cpu)
            }
            MoveDirection::MemoryToRegister => {
                self.write_memory_to_registers(&affected_register_offsets, operand_set.next(), cpu)
            }
        }
    }
}

impl MOVEM {
    fn collect_affected_register_offsets(&self, extension_word: u16) -> Vec<isize> {
        let mut affected_register_offsets = Vec::new();
        for i in 0..16 {
            if (extension_word >> i) & 0x1 == 1 {
                affected_register_offsets.push(i);
            }
        }
        affected_register_offsets
    }

    fn write_registers_to_memory<T: BusM68k>(
        &self,
        affected_register_offsets: &[isize],
        dst_operand: Operand,
        cpu: &mut M68k<T>,
    ) {
        match self.addressing_mode_type {
            AddressingModeType::AddressRegisterPreDecrement => {
                let register_ptr = cpu.register_set.get_register_ptr(7, RegisterType::Address);
                let mut memory_offset = 0;
                for reg_offset in affected_register_offsets {
                    let data = register_ptr.read_offset(self.size, -1 * reg_offset);
                    dst_operand
                        .operand_ptr
                        .write_offset(data, self.size, memory_offset);
                    if *reg_offset == (15 - (self.am_register_idx + 8)) {
                        // convert address register into the offset value
                        dst_operand.operand_ptr.write_offset(
                            data + self.size as u32,
                            self.size,
                            memory_offset,
                        );
                    }
                    memory_offset += 1;
                }
                let src_am_address = dst_operand.operand_address;
                dst_operand.address_register_ptr.as_ref().unwrap().write(
                    src_am_address + (memory_offset - 1) as u32 * self.size as u32,
                    Size::Long,
                );
            }
            _ => {
                let register_ptr = cpu.register_set.get_register_ptr(0, RegisterType::Data);
                let mut memory_offset = 0;
                for reg_offset in affected_register_offsets {
                    let data = register_ptr.read_offset(self.size, *reg_offset);
                    dst_operand
                        .operand_ptr
                        .write_offset(data, self.size, memory_offset);
                    memory_offset += 1;
                }
            }
        }
    }

    fn write_memory_to_registers<T: BusM68k>(
        &self,
        affected_register_offsets: &[isize],
        dst_operand: Operand,
        cpu: &mut M68k<T>,
    ) {
        match self.addressing_mode_type {
            AddressingModeType::AddressRegisterPostIncrement => {
                let register_ptr = cpu.register_set.get_register_ptr(0, RegisterType::Data);
                let mut memory_offset = 0;
                let mut am_register_writed = false;
                for reg_offset in affected_register_offsets {
                    let data = dst_operand
                        .operand_ptr
                        .read_offset(self.size, memory_offset)
                        .sign_extend(self.size);
                    register_ptr.write_offset(data, Size::Long, *reg_offset);
                    am_register_writed = *reg_offset == (self.am_register_idx + 8); // convert address register into the offset value
                    memory_offset += 1;
                }
                if !am_register_writed {
                    let src_am_address = dst_operand.operand_address;
                    dst_operand.address_register_ptr.as_ref().unwrap().write(
                        src_am_address + memory_offset as u32 * self.size as u32,
                        Size::Long,
                    );
                }
            }
            _ => {
                let register_ptr = cpu.register_set.get_register_ptr(0, RegisterType::Data);
                let mut memory_offset = 0;
                for reg_offset in affected_register_offsets {
                    let data = dst_operand
                        .operand_ptr
                        .read_offset(self.size, memory_offset)
                        .sign_extend(self.size);
                    register_ptr.write_offset(data, Size::Long, *reg_offset);
                    memory_offset += 1;
                }
            }
        }
    }
}

pub(crate) struct MOVEP {
    pub(crate) size: Size,
    pub(crate) direction: MoveDirection,
}

impl<T: BusM68k> Instruction<T> for MOVEP {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let iterations = self.size as isize;
        let first_operand = operand_set.next();
        let second_operand = operand_set.next();
        match self.direction {
            MoveDirection::RegisterToMemory => {
                for i in 0..iterations {
                    let data = first_operand.operand_ptr.read(self.size);
                    let byte_ = data >> self.size as isize - i;
                    second_operand
                        .operand_ptr
                        .write_offset(byte_, Size::Byte, 2 * i);
                }
            }
            MoveDirection::MemoryToRegister => {
                let mut data = 0;
                for i in 0..iterations {
                    let byte_ = second_operand.operand_ptr.read_offset(Size::Byte, 2 * i);
                    data |= (byte_ as u32) << i;
                }
                first_operand.operand_ptr.write(data, self.size);
            }
        }
    }
}

pub(crate) struct MOVEQ {
    pub(crate) data: u32,
}

impl<T: BusM68k> Instruction<T> for MOVEQ {
    fn execute(&self, mut operand_set: OperandSet, cpu_state: &mut M68k<T>) {
        let data = self.data.sign_extend(Size::Byte);
        operand_set.next().operand_ptr.write(data, Size::Long);

        cpu_state
            .register_set
            .sr
            .set_flag(StatusFlag::N, data.is_negate(Size::Long));
        cpu_state.register_set.sr.set_flag(StatusFlag::Z, data == 0);
        cpu_state.register_set.sr.set_flag(StatusFlag::V, false);
        cpu_state.register_set.sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct EXG {
    pub(crate) mode: ExchangeMode,
}

impl<T: BusM68k> Instruction<T> for EXG {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let first_operand = operand_set.next();
        let second_operand = operand_set.next();
        let first_data = first_operand.read(Size::Long);
        let second_data = second_operand.read(Size::Long);
        first_operand.write(second_data, Size::Long);
        second_operand.write(first_data, Size::Long);
    }
}

pub(crate) struct LEA();

impl<T: BusM68k> Instruction<T> for LEA {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let address = operand_set.next().operand_address;
        let dst_reg = operand_set.next();
        dst_reg.write(address, Size::Long);
    }
}
pub(crate) struct PEA();

impl<T: BusM68k> Instruction<T> for PEA {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let address = operand_set.next().operand_address;
        let dst_operand = operand_set.next();
        dst_operand.write(address, Size::Long);
    }
}
pub(crate) struct LINK {
    pub(crate) register: usize,
}

impl<T: BusM68k> Instruction<T> for LINK {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let address_register_ptr = cpu
            .register_set
            .get_register_ptr(self.register, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long);
        cpu.push(address, Size::Long);
        let stack_address = cpu.get_stack_address();
        address_register_ptr.write(stack_address, Size::Long);
        let displacement = operand_set
            .next()
            .operand_ptr
            .read(Size::Word)
            .sign_extend(Size::Word);
        cpu.set_stack_address(stack_address.wrapping_add(displacement));
    }
}
pub(crate) struct UNLK {
    pub(crate) register: usize,
}

impl<T: BusM68k> Instruction<T> for UNLK {
    fn execute(&self, _: OperandSet, cpu: &mut M68k<T>) {
        let address_register_ptr = cpu
            .register_set
            .get_register_ptr(self.register, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long);
        cpu.set_stack_address(address);
        let data = cpu.pop(Size::Long);
        address_register_ptr.write(data, Size::Long);
    }
}

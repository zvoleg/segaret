use crate::{
    addressing_mode_set::AddressingModeType,
    cpu_internals::{CpuInternals, RegisterType},
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

impl Instruction for MOVE {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let src_operand = operand_set.next();
        let src_data = src_operand.read(self.size);
        let dst_operand = operand_set.next();
        dst_operand.write(src_data, self.size);

        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::N, src_data.is_negate(self.size));
        cpu_internals
            .register_set
            .sr
            .set_flag(StatusFlag::Z, src_data == 0);
        cpu_internals.register_set.sr.set_flag(StatusFlag::V, false);
        cpu_internals.register_set.sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct MOVEA {
    pub(crate) size: Size,
}

impl Instruction for MOVEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
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

impl Instruction for MOVEM {
    fn execute(&self, mut operand_set: OperandSet, cpu_internals: &mut CpuInternals) {
        let extension_word = operand_set.next().read(Size::Word);
        let affected_register_offsets =
            self.collect_affected_register_offsets(extension_word as u16);

        // TODO additional cycles

        match self.direction {
            MoveDirection::RegisterToMemory => self.write_registers_to_memory(
                &affected_register_offsets,
                operand_set.next(),
                cpu_internals,
            ),
            MoveDirection::MemoryToRegister => self.write_memory_to_registers(
                &affected_register_offsets,
                operand_set.next(),
                cpu_internals,
            ),
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

    fn write_registers_to_memory(
        &self,
        affected_register_offsets: &[isize],
        dst_operand: Operand,
        cpu_internals: &mut CpuInternals,
    ) {
        match self.addressing_mode_type {
            AddressingModeType::AddressRegisterPreDecrement => {
                let register_ptr = cpu_internals
                    .register_set
                    .get_register_ptr(7, RegisterType::Address);
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
                let register_ptr = cpu_internals
                    .register_set
                    .get_register_ptr(0, RegisterType::Data);
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

    fn write_memory_to_registers(
        &self,
        affected_register_offsets: &[isize],
        dst_operand: Operand,
        cpu_internals: &mut CpuInternals,
    ) {
        match self.addressing_mode_type {
            AddressingModeType::AddressRegisterPostIncrement => {
                let register_ptr = cpu_internals
                    .register_set
                    .get_register_ptr(0, RegisterType::Data);
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
                let register_ptr = cpu_internals
                    .register_set
                    .get_register_ptr(0, RegisterType::Data);
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

impl Instruction for MOVEP {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
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

impl Instruction for MOVEQ {
    fn execute(&self, mut operand_set: OperandSet, cpu_interanls: &mut CpuInternals) {
        let data = self.data.sign_extend(Size::Byte);
        operand_set.next().operand_ptr.write(data, Size::Long);

        cpu_interanls
            .register_set
            .sr
            .set_flag(StatusFlag::N, data.is_negate(Size::Long));
        cpu_interanls
            .register_set
            .sr
            .set_flag(StatusFlag::Z, data == 0);
        cpu_interanls.register_set.sr.set_flag(StatusFlag::V, false);
        cpu_interanls.register_set.sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct EXG {
    pub(crate) mode: ExchangeMode,
}

impl Instruction for EXG {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        let first_operand = operand_set.next();
        let second_operand = operand_set.next();
        let first_data = first_operand.read(Size::Long);
        let second_data = second_operand.read(Size::Long);
        first_operand.write(second_data, Size::Long);
        second_operand.write(first_data, Size::Long);
    }
}

pub(crate) struct LEA();

impl Instruction for LEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        let address = operand_set.next().operand_address;
        let dst_reg = operand_set.next();
        dst_reg.write(address, Size::Long);
    }
}
pub(crate) struct PEA();

impl Instruction for PEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        let address = operand_set.next().operand_address;
        let dst_operand = operand_set.next();
        dst_operand.write(address, Size::Long);
    }
}
pub(crate) struct LINK();

impl Instruction for LINK {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        // SP - 4 → SP; An → (SP); SP → An; SP + dn → SP
        let stack_ptr = operand_set.next();
        let address_register_ptr = operand_set.next();
        let displacement_ptr = operand_set.next();

        let address = address_register_ptr.read(Size::Long);
        stack_ptr.write(address, Size::Long);

        let stack_address = stack_ptr.operand_address;
        address_register_ptr.write(stack_address, Size::Long);

        let displacement = displacement_ptr.read(Size::Word).sign_extend(Size::Word);
        stack_ptr
            .address_register_ptr
            .unwrap()
            .write(stack_address.wrapping_add(displacement), Size::Long);
    }
}
pub(crate) struct UNLK();

impl Instruction for UNLK {
    fn execute(&self, mut operand_set: OperandSet, _: &mut CpuInternals) {
        // An → SP; (SP) → An; SP + 4 → SP
        let address_register_ptr = operand_set.next();
        let stack_ptr = operand_set.next();

        let new_stack_address = address_register_ptr.read(Size::Long);
        stack_ptr
            .address_register_ptr
            .unwrap()
            .write(new_stack_address + (Size::Long as u32), Size::Long); // update stack register value with posth incrementing

        // be cause an instruction doesn't have access to stack interface there is a bad decision about using writing into memory by offset
        let memory_offset = (new_stack_address.wrapping_sub(stack_ptr.operand_address) as i32)
            / (Size::Long as i32);
        let data = stack_ptr
            .operand_ptr
            .read_offset(Size::Long, memory_offset as isize);
        address_register_ptr.write(data, Size::Long);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cpu_internals::{CpuInternals, RegisterType},
        instruction_set::Instruction,
        operand::{Operand, OperandSet},
        primitives::{MemoryPtr, Size},
        STACK_REGISTER,
    };

    use super::{LINK, UNLK};

    const ADDRESS_REGISTER_IDX: usize = 0;
    const ADDRESS_REGISTER_VALUE: u32 = 0x00FF8855;
    const STACK_INIT_ADDDRESS: u32 = 0x50;
    const OFFSET_ADDRESS: usize = 0x00;
    const OFFSET_VALUE: u32 = 0x10;

    fn prepare_link_operands(cpu: &mut CpuInternals, memory: &mut [u8; 0x100]) -> OperandSet {
        // be cause test runs without opcode, we don't have to prepare properly placed or aranged in the memory the values
        let mut operand_set = OperandSet::new();

        // setup stack ptr.
        // There is no necessary of implementation for incrementing/decrementing of address,
        // so stack is just the propper register with address
        let stack_register_ptr = cpu
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let stack_address = STACK_INIT_ADDDRESS - (Size::Long as u32); // immitation of the predecrementing addressing mode
        stack_register_ptr.write(stack_address, Size::Long);
        let stack_ptr = Box::new(MemoryPtr::new(&mut memory[stack_address as usize]));
        operand_set.add(Operand::new(
            stack_ptr,
            Some(stack_register_ptr),
            stack_address,
        ));

        // setup address register ptr which holds a value that will be pushed on to the stack
        let address_reg_ptr = cpu
            .register_set
            .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address);
        address_reg_ptr.write(ADDRESS_REGISTER_VALUE, Size::Long);
        operand_set.add(Operand::new(
            address_reg_ptr,
            None,
            ADDRESS_REGISTER_IDX as u32,
        ));

        // and offset value
        // it just value in some place of memory
        unsafe {
            *(&mut memory[OFFSET_ADDRESS] as *mut _ as *mut u32) = OFFSET_VALUE;
        }
        let memory_ptr = Box::new(MemoryPtr::new(
            &mut memory[OFFSET_ADDRESS] as *mut _ as *mut u8,
        ));
        operand_set.add(Operand::new(memory_ptr, None, 0));

        operand_set
    }

    fn prepare_unlk_operands(cpu: &mut CpuInternals, memory: &mut [u8; 0x100]) -> OperandSet {
        let mut operand_set = OperandSet::new();

        // setup address register ptr which holds a value that will be pushed on to the stack
        let address_reg_ptr = cpu
            .register_set
            .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address);
        operand_set.add(Operand::new(
            address_reg_ptr,
            None,
            ADDRESS_REGISTER_IDX as u32,
        ));

        // setup stack ptr.
        // There is no necessary of implementation for incrementing/decrementing of address,
        // so stack is just the propper register with address
        let stack_register_ptr = cpu
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let stack_address = stack_register_ptr.read(Size::Long);
        let stack_ptr = Box::new(MemoryPtr::new(&mut memory[stack_address as usize]));
        operand_set.add(Operand::new(
            stack_ptr,
            Some(stack_register_ptr),
            stack_address,
        ));

        operand_set
    }

    #[test]
    fn test_link() {
        let mut cpu = CpuInternals::new();
        let mut memory = [0u8; 0x100];
        let link_operand_set = prepare_link_operands(&mut cpu, &mut memory);
        let link = LINK();
        link.execute(link_operand_set, &mut cpu);

        let decremented_stack_address = STACK_INIT_ADDDRESS - (Size::Long as u32);
        unsafe {
            assert_eq!(
                *(&memory[decremented_stack_address as usize] as *const _ as *const u32),
                ADDRESS_REGISTER_VALUE
            );
            assert_eq!(
                cpu.register_set
                    .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address)
                    .read(Size::Long),
                decremented_stack_address
            );
            assert_eq!(
                cpu.register_set
                    .get_register_ptr(STACK_REGISTER, RegisterType::Address)
                    .read(Size::Long),
                decremented_stack_address + OFFSET_VALUE
            )
        }
    }

    #[test]
    fn test_unlk() {
        let mut cpu = CpuInternals::new();
        let mut memory = [0u8; 0x100];
        let link_operand_set = prepare_link_operands(&mut cpu, &mut memory);
        let link = LINK();
        link.execute(link_operand_set, &mut cpu);

        let unlk_operand_set = prepare_unlk_operands(&mut cpu, &mut memory);
        let unlk = UNLK();
        unlk.execute(unlk_operand_set, &mut cpu);

        assert_eq!(
            cpu.register_set
                .get_register_ptr(STACK_REGISTER, RegisterType::Address)
                .read(Size::Long),
            STACK_INIT_ADDDRESS
        );
        assert_eq!(
            cpu.register_set
                .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address)
                .read(Size::Long),
            ADDRESS_REGISTER_VALUE
        );
    }
}

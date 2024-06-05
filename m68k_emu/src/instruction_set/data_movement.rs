use std::fmt::Display;

use crate::{
    addressing_mode_set::AddressingModeType,
    bus::BusM68k,
    cpu::M68k,
    instruction_set::Instruction,
    operand::OperandSet,
    primitives::{Pointer, Size},
    register_set::RegisterType,
    status_flag::StatusFlag,
    IsNegate, SignExtending,
};

use super::{ExchangeMode, MoveDirection};

pub(crate) struct MOVE {
    pub(crate) size: Size,
}

impl Display for MOVE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVE.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for MOVE {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let src_data = src_operand.read();
        let dst_operand = operand_set.next();
        dst_operand.write(src_data);

        let sr = &mut cpu.register_set.sr;
        sr.set_flag(StatusFlag::N, src_data.is_negate(self.size));
        sr.set_flag(StatusFlag::Z, src_data == 0);
        sr.set_flag(StatusFlag::V, false);
        sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct MOVEA {
    pub(crate) size: Size,
}

impl Display for MOVEA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVEA.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let src_data = operand_set.next().read();
        operand_set.next().write(src_data);
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
    pub(crate) am_register_idx: isize, // needs to determine the writes into the source register of addressing mode
}

impl Display for MOVEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVEM.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEM {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let extension_word = operand_set.next().read();
        let register_offsets = self.collect_affected_register_offsets(extension_word as u16);

        // TODO additional cycles

        let operand = operand_set.next();
        let register_ptr = cpu.register_set.get_register_ptr(0, RegisterType::Data);

        let mut memory_offsets = Vec::new();
        let mut memory_offset = 0;
        for _ in 0..register_offsets.len() {
            match self.addressing_mode_type {
                AddressingModeType::AddressRegisterPreDecrement => {
                    memory_offsets.push(memory_offset);
                    memory_offset -= self.size as isize;
                }
                _ => {
                    memory_offsets.push(memory_offset);
                    memory_offset += self.size as isize;
                }
            }
        }

        let src_ptr: Box<dyn Pointer>;
        let src_offsets: Vec<isize>;
        let dst_ptr: Box<dyn Pointer>;
        let dst_offsets: Vec<isize>;
        match self.direction {
            MoveDirection::RegisterToMemory => {
                src_ptr = register_ptr;
                src_offsets = register_offsets;
                dst_ptr = operand.operand_ptr;
                dst_offsets = memory_offsets;
            }
            MoveDirection::MemoryToRegister => {
                src_ptr = operand.operand_ptr;
                src_offsets = memory_offsets;
                dst_ptr = register_ptr;
                dst_offsets = register_offsets;
            }
        }

        for i in 0..src_offsets.len() {
            let data = src_ptr.read_offset(self.size, src_offsets[i]);
            match self.direction {
                MoveDirection::RegisterToMemory => {
                    dst_ptr.write_offset(data, self.size, dst_offsets[i]);
                }
                MoveDirection::MemoryToRegister => {
                    let data = data.sign_extend(self.size);
                    dst_ptr.write_offset(data, Size::Long, dst_offsets[i]);
                }
            }
        }

        let address_reg_ptr = operand.address_register_ptr.unwrap();
        let base_address = operand.operand_address;
        match self.addressing_mode_type {
            AddressingModeType::AddressRegisterPostIncrement => {
                address_reg_ptr.write(
                    base_address + src_offsets.len() as u32 * self.size as u32,
                    Size::Long,
                );
            }
            AddressingModeType::AddressRegisterPreDecrement => {
                address_reg_ptr.write(
                    base_address - (src_offsets.len() - 1) as u32 * self.size as u32,
                    Size::Long,
                );
            }
            _ => (),
        }
    }
}

impl MOVEM {
    fn collect_affected_register_offsets(&self, bit_mask: u16) -> Vec<isize> {
        let mut affected_register_offsets = Vec::new();
        let mut reg_index_list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        if self.addressing_mode_type == AddressingModeType::AddressRegisterPreDecrement {
            reg_index_list.reverse();
        }
        for i in 0..16 {
            if (bit_mask >> i) & 0x1 == 1 {
                let reg_index = reg_index_list[i];
                affected_register_offsets.push(reg_index);
            }
        }
        affected_register_offsets
    }
}

pub(crate) struct MOVEP {
    pub(crate) size: Size,
    pub(crate) direction: MoveDirection,
}

impl Display for MOVEP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVEP.{}", self.size)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEP {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let src_operand = operand_set.next();
        let dst_operand = operand_set.next();
        let src_data = src_operand.read();
        let iterations = self.size as isize;
        match self.direction {
            MoveDirection::RegisterToMemory => {
                for i in 0..iterations {
                    let byte_ = src_data >> self.size as isize - i;
                    dst_operand
                        .operand_ptr
                        .write_offset(byte_, Size::Byte, 2 * i);
                }
            }
            MoveDirection::MemoryToRegister => {
                let mut data = 0;
                for i in 0..iterations {
                    let byte_ = src_operand.operand_ptr.read_offset(Size::Byte, 2 * i);
                    data |= (byte_ as u32) << i;
                }
                dst_operand.operand_ptr.write(data, self.size);
            }
        }
    }
}

pub(crate) struct MOVEQ {
    pub(crate) data: u32,
}

impl Display for MOVEQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MOVEQ #{:02X}", self.data)
    }
}

impl<T: BusM68k> Instruction<T> for MOVEQ {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        let data = self.data.sign_extend(Size::Byte);
        operand_set.next().operand_ptr.write(data, Size::Long);

        cpu.register_set
            .sr
            .set_flag(StatusFlag::N, data.is_negate(Size::Long));
        cpu.register_set.sr.set_flag(StatusFlag::Z, data == 0);
        cpu.register_set.sr.set_flag(StatusFlag::V, false);
        cpu.register_set.sr.set_flag(StatusFlag::C, false);
    }
}

pub(crate) struct EXG {
    pub(crate) mode: ExchangeMode,
}

impl Display for EXG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXG.{}", Size::Long)
    }
}

impl<T: BusM68k> Instruction<T> for EXG {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let first_operand = operand_set.next();
        let second_operand = operand_set.next();
        let first_data = first_operand.read();
        let second_data = second_operand.read();
        first_operand.write(second_data);
        second_operand.write(first_data);
    }
}

pub(crate) struct LEA();

impl Display for LEA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LEA")
    }
}

impl<T: BusM68k> Instruction<T> for LEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let address = operand_set.next().operand_address;
        let dst_reg = operand_set.next();
        dst_reg.write(address);
    }
}
pub(crate) struct PEA();

impl Display for PEA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PEA")
    }
}

impl<T: BusM68k> Instruction<T> for PEA {
    fn execute(&self, mut operand_set: OperandSet, _: &mut M68k<T>) {
        let address = operand_set.next().operand_address;
        let dst_operand = operand_set.next();
        dst_operand.write(address);
    }
}
pub(crate) struct LINK();

impl Display for LINK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LINK")
    }
}

impl<T: BusM68k> Instruction<T> for LINK {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        // SP - 4 → SP; An → (SP); SP → An; SP + dn → SP
        // let stack_ptr = operand_set.next();
        let address_register_ptr = operand_set.next();
        let displacement_ptr = operand_set.next();

        let address = address_register_ptr.read();
        cpu.stack_push(address, Size::Long);

        let stack_address = cpu.get_stack_address();
        address_register_ptr.write(stack_address);

        let displacement = displacement_ptr.read().sign_extend(Size::Word);
        let new_stack_address = stack_address.wrapping_add(displacement);
        cpu.set_stack_address(new_stack_address);
    }
}
pub(crate) struct UNLK();

impl Display for UNLK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UNLK")
    }
}

impl<T: BusM68k> Instruction<T> for UNLK {
    fn execute(&self, mut operand_set: OperandSet, cpu: &mut M68k<T>) {
        // An → SP; (SP) → An; SP + 4 → SP
        let address_register_ptr = operand_set.next();

        let new_stack_address = address_register_ptr.read();
        cpu.set_stack_address(new_stack_address);
        let data = cpu.stack_pop(Size::Long);
        address_register_ptr.write(data);
    }
}

// #[cfg(test)]
// mod test {
//     use std::{cell::RefCell, rc::Rc};

//     use crate::{
//         addressing_mode_set::{
//             AddressRegisterIndirect, AddressRegisterPostIncrement, AddressRegisterPreDecrement,
//             AddressingMode, AddressingModeType,
//         },
//         bus::BusM68k,
//         cpu::M68k,
//         instruction_set::{Instruction, MoveDirection},
//         operand::{Operand, OperandSet},
//         primitives::{memory::MemoryPtr, Pointer, Size},
//         register_set::RegisterType,
//         STACK_REGISTER,
//     };

//     use super::{LINK, MOVEM, UNLK};

//     const ADDRESS_REGISTER_IDX: usize = 0;
//     const ADDRESS_REGISTER_VALUE: u32 = 0x00FF8855;
//     const STACK_INIT_ADDDRESS: u32 = 0x50;
//     const OFFSET_ADDRESS: usize = 0x00;
//     const OFFSET_VALUE: u32 = 0x10;

//     struct Bus {
//         ram: Rc<RefCell<[u8; 0xFF]>>,
//     }

//     impl BusM68k for Bus {
//         fn set_address_read(&self, address: u32) -> *const u8 {
//             &self.ram.as_ref().borrow()[address as usize]
//         }

//         fn set_address_write(&self, address: u32) -> *mut u8 {
//             &self.ram.as_ref().borrow()[address as usize] as *const _ as *mut u8
//         }
//     }

//     fn prepare_link_operands(cpu: &mut M68k<Bus>, ram: Rc<RefCell<[u8; 0xFF]>>) -> OperandSet {
//         // be cause test runs without opcode, we don't have to prepare properly placed or aranged in the memory the values
//         let mut operand_set = OperandSet::new();

//         // setup address register ptr which holds a value that will be pushed on to the stack
//         let address_reg_ptr = cpu
//             .register_set
//             .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address);
//         address_reg_ptr.write(ADDRESS_REGISTER_VALUE, Size::Long);
//         operand_set.add(Operand::new(
//             address_reg_ptr,
//             None,
//             ADDRESS_REGISTER_IDX as u32,
//             Size::Long,
//         ));

//         // and offset value
//         // it just value in some place of memory
//         let offset_ptr = MemoryPtr::new_boxed(
//             &ram.as_ref().borrow()[OFFSET_ADDRESS],
//             &ram.as_ref().borrow()[OFFSET_ADDRESS] as *const _ as *mut u8,
//         );
//         offset_ptr.write(OFFSET_VALUE, Size::Word);
//         operand_set.add(Operand::new(offset_ptr, None, 0, Size::Word));

//         operand_set
//     }

//     fn prepare_unlk_operands(cpu: &mut M68k<Bus>) -> OperandSet {
//         let mut operand_set = OperandSet::new();

//         // setup address register ptr which holds a value that will be pushed on to the stack
//         let address_reg_ptr = cpu
//             .register_set
//             .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address);
//         operand_set.add(Operand::new(
//             address_reg_ptr,
//             None,
//             ADDRESS_REGISTER_IDX as u32,
//             Size::Long,
//         ));

//         operand_set
//     }

//     #[test]
//     fn test_link() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);
//         cpu.set_stack_address(STACK_INIT_ADDDRESS);
//         let link_operand_set = prepare_link_operands(&mut cpu, ram.clone());
//         let link = LINK();
//         link.execute(link_operand_set, &mut cpu);

//         let old_stack_address = STACK_INIT_ADDDRESS - (Size::Long as u32); // stack address should be decremented after pushing data to it
//         let mem_ptr = MemoryPtr::new_read_only(&ram.as_ref().borrow()[old_stack_address as usize]); // pointer to memory where data had been to push on the stack
//         assert_eq!(mem_ptr.read(Size::Long), ADDRESS_REGISTER_VALUE);
//         assert_eq!(
//             cpu.register_set
//                 .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address)
//                 .read(Size::Long),
//             old_stack_address
//         );
//         assert_eq!(
//             cpu.register_set
//                 .get_register_ptr(STACK_REGISTER, RegisterType::Address)
//                 .read(Size::Long),
//             old_stack_address + OFFSET_VALUE
//         )
//     }

//     #[test]
//     fn test_unlk() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);
//         cpu.set_stack_address(STACK_INIT_ADDDRESS);
//         let link_operand_set = prepare_link_operands(&mut cpu, ram.clone());
//         let link = LINK();
//         link.execute(link_operand_set, &mut cpu);

//         let unlk_operand_set = prepare_unlk_operands(&mut cpu);
//         let unlk = UNLK();
//         unlk.execute(unlk_operand_set, &mut cpu);

//         assert_eq!(
//             cpu.register_set
//                 .get_register_ptr(STACK_REGISTER, RegisterType::Address)
//                 .read(Size::Long),
//             STACK_INIT_ADDDRESS
//         );
//         assert_eq!(
//             cpu.register_set
//                 .get_register_ptr(ADDRESS_REGISTER_IDX, RegisterType::Address)
//                 .read(Size::Long),
//             ADDRESS_REGISTER_VALUE
//         );
//     }

//     #[test]
//     fn test_movem_predecremented() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let am_bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);

//         let d2 = cpu.register_set.get_register_ptr(2, RegisterType::Data);
//         d2.write(0xDDDD2222, Size::Long);
//         let a3 = cpu.register_set.get_register_ptr(3, RegisterType::Address);
//         a3.write(0xAAAA3333, Size::Long);
//         let a5_am = cpu.register_set.get_register_ptr(5, RegisterType::Address);
//         a5_am.write(0x0000000A, Size::Long);

//         let mut operand_set = OperandSet::new();
//         let mask = [0x20u8, 0x10];
//         let mem_ptr = MemoryPtr::new_boxed(
//             &mask[0] as *const _ as *const u8,
//             &mask[0] as *const _ as *mut u8,
//         );
//         let operand = Operand::new(mem_ptr, None, 0, Size::Word);
//         operand_set.add(operand);

//         let am = AddressRegisterPreDecrement {
//             reg: 5,
//             size: Size::Word,
//         };
//         let operand = am.get_operand(&mut cpu.register_set, &am_bus);
//         operand_set.add(operand);

//         let movem = MOVEM {
//             size: Size::Word,
//             direction: MoveDirection::RegisterToMemory,
//             addressing_mode_type: AddressingModeType::AddressRegisterPreDecrement,
//             am_register_idx: 5,
//         };
//         movem.execute(operand_set, &mut cpu);

//         assert_eq!(a5_am.read(Size::Long), 0x0000000A - 2 * Size::Word as u32);
//         unsafe {
//             assert_eq!(
//                 *(&ram.borrow()[0xA - 2] as *const _ as *const u16),
//                 0x3333u16
//             );
//             assert_eq!(
//                 *(&ram.borrow()[0xA - 4] as *const _ as *const u16),
//                 0x2222u16
//             );
//         }
//     }

//     #[test]
//     fn test_movem_postincremented_word() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let am_bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);

//         unsafe { *(&mut ram.borrow_mut()[0xA + 0] as *mut _ as *mut u16) = 0x2222u16 };
//         unsafe { *(&mut ram.borrow_mut()[0xA + 2] as *mut _ as *mut u16) = 0x3333u16 };
//         let d2 = cpu.register_set.get_register_ptr(2, RegisterType::Data);
//         let a3 = cpu.register_set.get_register_ptr(3, RegisterType::Address);
//         let a5_am = cpu.register_set.get_register_ptr(5, RegisterType::Address);
//         a5_am.write(0x0000000A, Size::Long);

//         let mut operand_set = OperandSet::new();
//         let mask = [0x08u8, 0x04];
//         let mem_ptr = MemoryPtr::new_boxed(
//             &mask[0] as *const _ as *const u8,
//             &mask[0] as *const _ as *mut u8,
//         );
//         let operand = Operand::new(mem_ptr, None, 0, Size::Word);
//         operand_set.add(operand);

//         let am = AddressRegisterPostIncrement {
//             reg: 5,
//             size: Size::Word,
//         };
//         let operand = am.get_operand(&mut cpu.register_set, &am_bus);
//         operand_set.add(operand);

//         let movem = MOVEM {
//             size: Size::Word,
//             direction: MoveDirection::MemoryToRegister,
//             addressing_mode_type: AddressingModeType::AddressRegisterPostIncrement,
//             am_register_idx: 5,
//         };
//         movem.execute(operand_set, &mut cpu);
//         assert_eq!(a5_am.read(Size::Long), 0x0000000A + 2 * Size::Word as u32);
//         assert_eq!(d2.read(Size::Long), 0x2222u32);
//         assert_eq!(a3.read(Size::Long), 0x3333u32);
//     }

//     #[test]
//     fn test_movem_postincremented_long() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let am_bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);

//         unsafe { *(&mut ram.borrow_mut()[0xA + 0] as *mut _ as *mut u32) = 0x11112222u32.to_be() };
//         unsafe { *(&mut ram.borrow_mut()[0xA + 4] as *mut _ as *mut u32) = 0x33334444u32.to_be() };
//         let d2 = cpu.register_set.get_register_ptr(2, RegisterType::Data);
//         let a3 = cpu.register_set.get_register_ptr(3, RegisterType::Address);
//         let a5_am = cpu.register_set.get_register_ptr(5, RegisterType::Address);
//         a5_am.write(0x0000000A, Size::Long);

//         let mut operand_set = OperandSet::new();
//         let mask = [0x08u8, 0x04];
//         let mem_ptr = MemoryPtr::new_boxed(
//             &mask[0] as *const _ as *const u8,
//             &mask[0] as *const _ as *mut u8,
//         );
//         let operand = Operand::new(mem_ptr, None, 0, Size::Word);
//         operand_set.add(operand);

//         let am = AddressRegisterPostIncrement {
//             reg: 5,
//             size: Size::Word,
//         };
//         let operand = am.get_operand(&mut cpu.register_set, &am_bus);
//         operand_set.add(operand);

//         let movem = MOVEM {
//             size: Size::Long,
//             direction: MoveDirection::MemoryToRegister,
//             addressing_mode_type: AddressingModeType::AddressRegisterPostIncrement,
//             am_register_idx: 5,
//         };
//         movem.execute(operand_set, &mut cpu);
//         assert_eq!(a5_am.read(Size::Long), 0x0000000A + 2 * Size::Long as u32);
//         assert_eq!(d2.read(Size::Long), 0x11112222u32);
//         assert_eq!(a3.read(Size::Long), 0x33334444u32);
//     }

//     #[test]
//     fn test_movem_memory_to_register() {
//         let ram = Rc::new(RefCell::new([0; 0xFF]));
//         let bus = Bus { ram: ram.clone() };
//         let am_bus = Bus { ram: ram.clone() };
//         let mut cpu = M68k::new(bus);

//         unsafe { *(&mut ram.borrow_mut()[0xA + 0] as *mut _ as *mut u16) = 0x00007055u16.to_be() };
//         unsafe { *(&mut ram.borrow_mut()[0xA + 2] as *mut _ as *mut u16) = 0x00008099u16.to_be() };
//         let d2 = cpu.register_set.get_register_ptr(2, RegisterType::Data);
//         let a3 = cpu.register_set.get_register_ptr(3, RegisterType::Address);
//         let a5_am = cpu.register_set.get_register_ptr(5, RegisterType::Address);
//         a5_am.write(0x0000000A, Size::Long);

//         let mut operand_set = OperandSet::new();
//         let mask = [0x08u8, 0x04];
//         let mem_ptr = MemoryPtr::new_boxed(
//             &mask[0] as *const _ as *const u8,
//             &mask[0] as *const _ as *mut u8,
//         );
//         let operand = Operand::new(mem_ptr, None, 0, Size::Word);
//         operand_set.add(operand);

//         let am = AddressRegisterIndirect {
//             reg: 5,
//             size: Size::Word,
//         };
//         let operand = am.get_operand(&mut cpu.register_set, &am_bus);
//         operand_set.add(operand);

//         let movem = MOVEM {
//             size: Size::Word,
//             direction: MoveDirection::MemoryToRegister,
//             addressing_mode_type: AddressingModeType::AddressRegisterIndirect,
//             am_register_idx: 5,
//         };
//         movem.execute(operand_set, &mut cpu);

//         assert_eq!(a5_am.read(Size::Long), 0xA);
//         assert_eq!(d2.read(Size::Long), 0x00007055);
//         assert_eq!(a3.read(Size::Long), 0xFFFF8099);
//     }
// }

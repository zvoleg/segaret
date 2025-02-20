use std::rc::Rc;

use crate::{
    bus::BusM68k,
    extension_word::BriefExtensionWord,
    operand::Operand,
    primitives::{memory::MemoryPtr, Pointer},
    register_set::{RegisterSet, RegisterType},
    SignExtending, Size, STACK_REGISTER,
};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum AddressingModeType {
    DataRegister,
    AddressRegister,
    AddressRegisterIndirect,
    AddressRegisterPostIncrement,
    AddressRegisterPreDecrement,
    AddressRegisterDisplacement,
    AddressRegisterIndexed,
    ProgramCounterDisplacement,
    ProgramCounterIndexed,
    AbsShort,
    AbsLong,
    Immediate,
}

pub(crate) trait AddressingMode {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()>;
    fn disassembly(&self, extension_word: u32) -> String;
    fn extension_word_length(&self) -> u32;
}

pub(crate) struct DataRegister {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for DataRegister {
    fn get_operand(&self, rs: &mut RegisterSet, _: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let operand_ptr = rs.get_register_ptr(self.reg, RegisterType::Data);
        Ok(Operand::new(operand_ptr, None, self.reg as u32, self.size))
    }

    fn disassembly(&self, _: u32) -> String {
        format!("D{}", self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        0
    }
}

pub(crate) struct AddressRegister {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegister {
    fn get_operand(&self, rs: &mut RegisterSet, _: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let operand_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        Ok(Operand::new(operand_ptr, None, self.reg as u32, self.size))
    }

    fn disassembly(&self, _: u32) -> String {
        format!("A{}", self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        0
    }
}

pub(crate) struct AddressRegisterIndirect {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterIndirect {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long)?;
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(
            operand_ptr,
            Some(address_register_ptr),
            address,
            self.size,
        ))
    }

    fn disassembly(&self, _: u32) -> String {
        format!("(A{})", self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        0
    }
}

pub(crate) struct AddressRegisterPostIncrement {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterPostIncrement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long)?;
        let size = if self.reg == STACK_REGISTER && self.size == Size::Byte {
            Size::Word
        } else {
            self.size
        };
        address_register_ptr.write(address.wrapping_add(size as u32), Size::Long).unwrap();
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(
            operand_ptr,
            Some(address_register_ptr),
            address,
            size,
        ))
    }

    fn disassembly(&self, _: u32) -> String {
        format!("(A{})+", self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        0
    }
}

pub(crate) struct AddressRegisterPreDecrement {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterPreDecrement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let mut address = address_register_ptr.read(Size::Long)?;
        let size = if self.reg == STACK_REGISTER && self.size == Size::Byte {
            Size::Word
        } else {
            self.size
        };
        address = address.wrapping_sub(size as u32);
        address_register_ptr.write(address, Size::Long).unwrap();
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(
            operand_ptr,
            Some(address_register_ptr),
            address,
            size,
        ))
    }

    fn disassembly(&self, _: u32) -> String {
        format!("-(A{})", self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        0
    }
}

pub(crate) struct AddressRegisterDisplacement {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterDisplacement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let displacement = extension_word_ptr.read(Size::Word)?.sign_extend(Size::Word);
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let base_address = address_register_ptr.read(Size::Long)?;
        let address = base_address.wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(
            operand_ptr,
            Some(address_register_ptr),
            address,
            self.size,
        ))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!("({:04X}, A{})", extension_word, self.reg)
    }

    fn extension_word_length(&self) -> u32 {
        1
    }
}

pub(crate) struct AddressRegisterIndexed {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterIndexed {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let extension_word = extension_word_ptr.read(Size::Word)?;

        let brief_extension_word = BriefExtensionWord::new(extension_word as u16, rs);
        let index_reg_data = brief_extension_word
            .index_register_ptr
            .read(brief_extension_word.size)?;
        let index_data = index_reg_data
            .sign_extend(brief_extension_word.size)
            .wrapping_mul(brief_extension_word.scale);
        let displacement = brief_extension_word.displacement;

        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let mut address = address_register_ptr.read(Size::Long)?;
        address = address.wrapping_add(index_data).wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(
            operand_ptr,
            Some(address_register_ptr),
            address,
            self.size,
        ))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!(
            "{}",
            BriefExtensionWord::disassembly(&format!("A{}", self.reg), extension_word)
        )
    }

    fn extension_word_length(&self) -> u32 {
        1
    }
}

pub(crate) struct ProgramCounterDisplacement {
    pub(crate) size: Size,
}

impl AddressingMode for ProgramCounterDisplacement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let base_address = rs.pc; // takes the address of the extension word
        let extention_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let displacement = extention_word_ptr.read(Size::Word)?.sign_extend(Size::Word);
        let address = base_address.wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(operand_ptr, None, address, self.size))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!("({:04X}, PC)", extension_word)
    }

    fn extension_word_length(&self) -> u32 {
        1
    }
}

pub(crate) struct ProgramCounterIndexed {
    pub(crate) size: Size,
}

impl AddressingMode for ProgramCounterIndexed {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let mut address = rs.pc; // takes the address of the extension word
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let extension_word = extension_word_ptr.read(Size::Word)?;

        let brief_extension_word = BriefExtensionWord::new(extension_word as u16, rs);
        let index_data = brief_extension_word
            .index_register_ptr
            .read(brief_extension_word.size)?
            .sign_extend(brief_extension_word.size)
            .wrapping_mul(brief_extension_word.scale);
        let displacement = brief_extension_word.displacement;

        address = address.wrapping_add(index_data).wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(operand_ptr, None, address, self.size))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!("{}", BriefExtensionWord::disassembly("PC", extension_word))
    }

    fn extension_word_length(&self) -> u32 {
        1
    }
}

pub(crate) struct AbsShort {
    pub(crate) size: Size,
}

impl AddressingMode for AbsShort {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let address = extension_word_ptr.read(Size::Word)?.sign_extend(Size::Word);
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(operand_ptr, None, address, self.size))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!("({:04X})", extension_word.sign_extend(Size::Word))
    }

    fn extension_word_length(&self) -> u32 {
        1
    }
}

pub(crate) struct AbsLong {
    pub(crate) size: Size,
}

impl AddressingMode for AbsLong {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let address_high = extension_word_ptr.read(Size::Word)?;
        let extension_word_ptr = MemoryPtr::new(rs.get_and_increment_pc(), bus.clone());
        let address_low = extension_word_ptr.read(Size::Word)?;
        let address = (address_high << 0x10) | address_low;
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        Ok(Operand::new(operand_ptr, None, address, self.size))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        format!("({:08X})", extension_word)
    }

    fn extension_word_length(&self) -> u32 {
        2
    }
}

pub(crate) struct Immediate {
    pub(crate) size: Size,
}

impl AddressingMode for Immediate {
    fn get_operand(&self, rs: &mut RegisterSet, bus: Rc<dyn BusM68k>) -> Result<Operand, ()> {
        let address = rs.get_and_increment_pc();
        let operand_ptr = MemoryPtr::new_boxed(address, bus.clone());
        match self.size {
            Size::Long => rs.get_and_increment_pc(),
            _ => 0,
        };
        // an extension word can't hold only one byte, it requires at least two bytes
        let size = match self.size {
            Size::Byte => Size::Word,
            _ => self.size,
        };
        Ok(Operand::new(operand_ptr, None, address, size))
    }

    fn disassembly(&self, extension_word: u32) -> String {
        match self.size {
            Size::Byte | Size::Word => format!("#{:04x}", extension_word),
            Size::Long => format!("#{:08x}", extension_word),
        }
    }

    fn extension_word_length(&self) -> u32 {
        match self.size {
            Size::Byte => 1,
            Size::Word => 1,
            Size::Long => 2,
        }
    }
}

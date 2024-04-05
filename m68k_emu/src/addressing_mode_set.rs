use crate::{
    bus::BusM68k,
    cpu_internals::{RegisterSet, RegisterType},
    extension_word::BriefExtensionWord,
    operand::Operand,
    primitives::{MemoryPtr, Pointer},
    SignExtending, Size,
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
    Implied,
}

pub(crate) trait AddressingMode {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand;
    fn type_info(&self) -> AddressingModeType;
}

pub(crate) struct DataRegister {
    pub(crate) reg: usize,
}

impl AddressingMode for DataRegister {
    fn get_operand(&self, rs: &mut RegisterSet, _: &dyn BusM68k) -> Operand {
        let operand_ptr = rs.get_register_ptr(self.reg, RegisterType::Data);
        Operand::new(operand_ptr, None, self.reg as u32)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::DataRegister
    }
}

pub(crate) struct AddressRegister {
    pub(crate) reg: usize,
}

impl AddressingMode for AddressRegister {
    fn get_operand(&self, rs: &mut RegisterSet, _: &dyn BusM68k) -> Operand {
        let operand_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        Operand::new(operand_ptr, None, self.reg as u32)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegister
    }
}

pub(crate) struct AddressRegisterIndirect {
    pub(crate) reg: usize,
}

impl AddressingMode for AddressRegisterIndirect {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, Some(address_register_ptr), address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegisterIndirect
    }
}

pub(crate) struct AddressRegisterPostIncrement {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl AddressingMode for AddressRegisterPostIncrement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let address = address_register_ptr.read(Size::Long);
        address_register_ptr.write(address.wrapping_add(self.size as u32), Size::Long);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, Some(address_register_ptr), address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegisterPostIncrement
    }
}

pub(crate) struct AddressRegisterPreDecrement {
    pub(crate) reg: usize,
    pub(crate) size: Size,
}

impl Default for AddressRegisterPreDecrement {
    fn default() -> Self {
        Self {
            reg: 0,
            size: Size::Byte,
        }
    }
}

impl AddressingMode for AddressRegisterPreDecrement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let mut address = address_register_ptr.read(Size::Long);
        address = address.wrapping_sub(self.size as u32);
        address_register_ptr.write(address, Size::Long);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, Some(address_register_ptr), address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegisterPreDecrement
    }
}

pub(crate) struct AddressRegisterDisplacement {
    pub(crate) reg: usize,
}

impl AddressingMode for AddressRegisterDisplacement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let displacement = extension_word_ptr.read(Size::Word).sign_extend(Size::Word);
        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let base_address = address_register_ptr.read(Size::Long);
        let address = base_address.wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, Some(address_register_ptr), address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegisterDisplacement
    }
}

pub(crate) struct AddressRegisterIndexed {
    pub(crate) reg: usize,
}

impl AddressingMode for AddressRegisterIndexed {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let extension_word = extension_word_ptr.read(Size::Long);

        let brief_extension_word = BriefExtensionWord::new(extension_word as u16, rs);
        let index_data = brief_extension_word
            .index_register_ptr
            .read(brief_extension_word.size);
        let displacement = brief_extension_word.displacement;

        let address_register_ptr = rs.get_register_ptr(self.reg, RegisterType::Address);
        let mut address = address_register_ptr.read(Size::Long);
        address = address.wrapping_add(index_data).wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, Some(address_register_ptr), address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AddressRegisterIndexed
    }
}

pub(crate) struct ProgramCounterDisplacement();

impl AddressingMode for ProgramCounterDisplacement {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extention_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let displacement = extention_word_ptr.read(Size::Word).sign_extend(Size::Word);
        let base_address = rs.pc;
        let address = base_address.wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, None, address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::ProgramCounterDisplacement
    }
}

pub(crate) struct ProgramCounterIndexed();

impl AddressingMode for ProgramCounterIndexed {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let extension_word = extension_word_ptr.read(Size::Word);

        let brief_extension_word = BriefExtensionWord::new(extension_word as u16, rs);
        let index_data = brief_extension_word
            .index_register_ptr
            .read(brief_extension_word.size);
        let displacement = brief_extension_word.displacement;

        let mut address = rs.pc;
        address = address.wrapping_add(index_data).wrapping_add(displacement);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, None, address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::ProgramCounterIndexed
    }
}

pub(crate) struct AbsShort();

impl AddressingMode for AbsShort {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let address = extension_word_ptr.read(Size::Word).sign_extend(Size::Word);
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, None, address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AbsShort
    }
}

pub(crate) struct AbsLong();

impl AddressingMode for AbsLong {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let address_high = extension_word_ptr.read(Size::Word);
        let extension_word_ptr = MemoryPtr::new_boxed(bus.set_address(rs.get_and_increment_pc()));
        let address_low = extension_word_ptr.read(Size::Word);
        let address = (address_high << 0x10) | address_low;
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        Operand::new(operand_ptr, None, address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::AbsLong
    }
}

pub(crate) struct Immediate {
    pub(crate) size: Size,
}

impl AddressingMode for Immediate {
    fn get_operand(&self, rs: &mut RegisterSet, bus: &dyn BusM68k) -> Operand {
        let address = rs.get_and_increment_pc();
        let operand_ptr = MemoryPtr::new_boxed(bus.set_address(address));
        match self.size {
            Size::Long => rs.get_and_increment_pc(),
            _ => 0,
        };
        Operand::new(operand_ptr, None, address)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::Immediate
    }
}

pub(crate) struct Implied();

impl AddressingMode for Implied {
    fn get_operand(&self, _: &mut RegisterSet, _: &dyn BusM68k) -> Operand {
        Operand::new(MemoryPtr::new_boxed(std::ptr::null_mut()), None, 0)
    }

    fn type_info(&self) -> AddressingModeType {
        AddressingModeType::Implied
    }
}

use crate::{
    bus::BusZ80,
    cpu::Z80,
    primitives::{MemPtr, Operand},
    register_set::{IndexRegister, Register},
    SignExtending, Size,
};

pub(crate) trait AddressingMode<T>
where
    T: BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand;
}

pub(crate) struct Immediate();

impl<T> AddressingMode<T> for Immediate
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let address = cpu.program_counter;
        cpu.increment_pc(Size::Byte);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            Size::Byte,
            Some(address),
        )
    }
}

pub(crate) struct ImmediateExt();

impl<T> AddressingMode<T> for ImmediateExt
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let address = cpu.program_counter;
        cpu.increment_pc(Size::Word);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            Size::Word,
            Some(address),
        )
    }
}

pub(crate) struct ModifiedPageZero {
    address: u16,
}

impl<T> AddressingMode<T> for ModifiedPageZero
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        Operand::new(
            Box::new(MemPtr::new(self.address, cpu.bus_share())),
            Size::Byte,
            Some(self.address),
        )
    }
}

pub(crate) struct Relative();

impl<T> AddressingMode<T> for Relative
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let address = cpu.program_counter;
        cpu.increment_pc(Size::Byte);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            Size::Byte,
            Some(address),
        )
    }
}

pub(crate) struct Extended {
    pub(crate) size: Size,
}

impl<T> AddressingMode<T> for Extended
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let address = cpu.read_pc(Size::Word);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            self.size,
            Some(address),
        )
    }
}

pub(crate) struct Indexed {
    index_reg: IndexRegister,
    size: Size,
}

impl<T> AddressingMode<T> for Indexed
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let displacement = cpu.read_pc(Size::Byte).sign_extend(Size::Byte);
        let index = cpu
            .register_set
            .read_register(Register::Index(self.index_reg), Size::Word);
        let address = index.wrapping_add(displacement);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            self.size,
            Some(address),
        )
    }
}

pub(crate) struct RegisterAddressing {
    pub(crate) register: Register,
    pub(crate) size: Size,
}

impl<T> AddressingMode<T> for RegisterAddressing
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        Operand::new(
            Box::new(cpu.register_set.get_register_ptr(self.register)),
            self.size,
            None,
        )
    }
}

pub(crate) struct Implied();

impl<T> AddressingMode<T> for Implied
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        Operand::new(Box::new(MemPtr::new(0, cpu.bus_share())), Size::Byte, None)
    }
}

pub(crate) struct RegisterIndirect {
    pub(crate) register: Register,
    pub(crate) size: Size,
}

impl<T> AddressingMode<T> for RegisterIndirect
where
    T: 'static + BusZ80,
{
    fn fetch(&self, cpu: &mut Z80<T>) -> Operand {
        let address = cpu.register_set.read_register(self.register, Size::Word);
        Operand::new(
            Box::new(MemPtr::new(address, cpu.bus_share())),
            self.size,
            Some(address),
        )
    }
}

use std::rc::Rc;

use crate::{bus::BusZ80, opcode_table_generator::tables::opcode_table, register_set::RegisterSet, Size};

pub struct Z80<T: BusZ80> {
    pub(crate) register_set: RegisterSet,
    stack_pointer: u16,
    pub(crate) program_counter: u16,

    refresh_register: u8,
    interrupt_vector: u16,

    bus: Option<Rc<T>>,
}

const NMI_VECTOR: u16 = 0x0066;

impl<T> Z80<T>
where
    T: 'static + BusZ80,
{
    pub fn new() -> Self {
        Self {
            register_set: RegisterSet::new(),
            stack_pointer: 0,
            program_counter: 0,
            refresh_register: 0,
            interrupt_vector: 0,
            bus: None,
        }
    }

    pub fn set_bus(&mut self, bus: Rc<T>) {
        self.bus = Some(bus)
    }

    pub fn restart(&mut self) {}

    pub fn clock(&mut self) {
        let opcode = self.read_pc(Size::Byte);
        let opcodes = opcode_table();
        let operation = &opcodes[opcode as usize];
        let mut operands = Vec::new();
        if let Some(am) = &operation.dst_am {
            let operand = am.fetch(self);
            operands.push(operand);
        }
        if let Some(am) = &operation.src_am {
            let operand = am.fetch(self);
            operands.push(operand);
        }
        operation.instruction.execute(self, operands);
    }

    fn write_interrupt_vector(&mut self, data: u8) {
        self.interrupt_vector |= (data as u16) << 8;
    }

    fn nmi(&mut self) {
        self.restart();
        self.program_counter = NMI_VECTOR;
    }

    pub(crate) fn push(&mut self, data: u16, size: Size) -> Result<(), ()> {
        self.bus
            .as_ref()
            .unwrap()
            .write(data, self.stack_pointer, size)?;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        Ok(())
    }

    pub(crate) fn pop(&mut self, size: Size) -> Result<u16, ()> {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.bus.as_ref().unwrap().read(self.stack_pointer, size)
    }

    pub(crate) fn increment_pc(&mut self, size: Size) {
        self.program_counter = self.program_counter.wrapping_add(size as u16);
    }

    pub(crate) fn read_pc(&mut self, size: Size) -> u16 {
        let data = self
            .bus
            .as_ref()
            .unwrap()
            .read(self.program_counter, size)
            .unwrap();
        self.increment_pc(size);
        data
    }

    pub(crate) fn bus_share(&self) -> Rc<T> {
        self.bus.as_ref().unwrap().clone()
    }
}

use std::fmt::Display;

use crate::{
    bus::BusM68k,
    header::Header,
    instruction_set::system_control::ILLEAGL,
    opcode_generators::generate_opcode_list,
    operand::OperandSet,
    operation::Operation,
    primitives::{memory::MemoryPtr, Pointer, Size},
    register_set::{RegisterSet, RegisterType},
    vectors::{RESET_PC, RESET_SP},
    STACK_REGISTER,
};

pub struct M68k<T: BusM68k> {
    pub(crate) register_set: RegisterSet,
    pub(crate) trap: Option<usize>,
    pub(crate) cycles_counter: i32,
    header: Header,

    operation_set: Vec<Operation<T>>,
    bus: T,
}

impl<T> M68k<T>
where
    T: BusM68k,
{
    pub fn new(bus: T) -> Self {
        let mut table: Vec<Operation<T>> = Vec::with_capacity(0x10000);
        table.resize_with(0x10000, || {
            Operation::new(Box::new(ILLEAGL()), Vec::new(), 5)
        });
        generate_opcode_list(&mut table);
        let header_ptr = MemoryPtr::new_read_only(bus.set_address_read(0));
        let header = Header::new(header_ptr);
        let mut register_set = RegisterSet::new();
        M68k::<T>::reset(&header, &mut register_set);
        Self {
            register_set: register_set,
            header: header,
            operation_set: table,
            bus: bus,
            trap: None,
            cycles_counter: 0,
        }
    }

    pub fn clock(&mut self) {
        let opcode_address = self.register_set.pc;
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        let operation_ptr = MemoryPtr::new_read_only(self.bus.set_address_read(opcode_address));
        println!("{}", operation.disassembly(opcode_address, operation_ptr));
        self.cycles_counter = operation.cycles;

        let mut operands = OperandSet::new();
        for am in &operation.addressing_mode_list {
            operands.add(am.get_operand(&mut self.register_set, &self.bus));
        }
        let instruction = &operation.instruction;
        instruction.execute(operands, self);
        println!("{}", self);
        if let Some(vector) = self.trap {
            if vector == RESET_SP {
                M68k::<T>::reset(&self.header, &mut self.register_set);
            } else {
                let vector_address = self.header.get_vector(vector);
                self.register_set.pc = vector_address;
            }
            self.trap = None;
        }
    }

    pub(crate) fn stack_push(&mut self, data: u32, size: Size) {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let mut address = stack_register_ptr.read(Size::Long);
        address = address.wrapping_sub(size as u32); // predecrementing
        stack_register_ptr.write(address, Size::Long);

        let write_ptr = MemoryPtr::new_write_only(self.bus.set_address_write(address));
        write_ptr.write(data, size);
    }

    pub(crate) fn stack_pop(&mut self, size: Size) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long);

        let read_ptr = MemoryPtr::new_read_only(self.bus.set_address_read(address));
        let data = read_ptr.read(size);

        stack_register_ptr.write(address.wrapping_add(size as u32), Size::Long); // postincrement
        data
    }

    pub(crate) fn get_stack_address(&mut self) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr.read(Size::Long)
    }

    pub(crate) fn set_stack_address(&mut self, new_stack_address: u32) {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr.write(new_stack_address, Size::Long);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode_ptr = MemoryPtr::new_read_only(
            self.bus
                .set_address_read(self.register_set.get_and_increment_pc()),
        );
        opcode_ptr.read(Size::Word) as u16
    }

    fn reset(header: &Header, register_set: &mut RegisterSet) {
        let stack_pointer = header.get_vector(RESET_SP);
        let stack_register = register_set.get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register.write(stack_pointer, Size::Long);

        let pc = header.get_vector(RESET_PC);
        register_set.pc = pc;
    }
}

impl<T: BusM68k> Display for M68k<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register_set.to_string())
    }
}

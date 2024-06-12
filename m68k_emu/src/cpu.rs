use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    bus::BusM68k,
    header::Header,
    instruction_set::system_control::ILLEAGL,
    interrupt_line::InterruptLine,
    opcode_generators::generate_opcode_list,
    operand::OperandSet,
    operation::Operation,
    primitives::{memory::MemoryPtr, Pointer, Size},
    register_set::{RegisterSet, RegisterType},
    vectors::{LEVEL_1, LEVEL_2, LEVEL_3, LEVEL_4, LEVEL_5, LEVEL_6, LEVEL_7, RESET_PC, RESET_SP},
    STACK_REGISTER,
};

pub struct M68k<T: 'static + BusM68k> {
    pub(crate) register_set: RegisterSet,
    pub(crate) trap: Option<usize>,
    pub(crate) cycles_counter: i32,
    header: Header,

    operation_set: Vec<Operation<T>>,
    bus: Rc<T>,
    interrupt_line: Rc<RefCell<InterruptLine>>,
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
        let bus = Rc::new(bus);
        let header_ptr = MemoryPtr::new(0, bus.clone());
        let header = Header::new(header_ptr);
        let mut register_set = RegisterSet::new();
        M68k::<T>::reset(&header, &mut register_set);
        Self {
            register_set: register_set,
            trap: None,
            cycles_counter: 0,
            header: header,

            operation_set: table,
            bus: bus,
            interrupt_line: Rc::new(RefCell::new(InterruptLine::new())),
        }
    }

    pub fn get_interrupt_lint(&self) -> Rc<RefCell<InterruptLine>> {
        self.interrupt_line.clone()
    }

    pub fn clock(&mut self) {
        let interrupt_level = self.interrupt_line.borrow_mut().receive();
        let ipl = self.register_set.sr.ipl() as usize;
        if interrupt_level >= ipl && interrupt_level != 0 {
            let vector = match interrupt_level {
                1 => LEVEL_1,
                2 => LEVEL_2,
                3 => LEVEL_3,
                4 => LEVEL_4,
                5 => LEVEL_5,
                6 => LEVEL_6,
                7 => LEVEL_7,
                _ => panic!("M68k: clock: wrong interrupt level: {}", interrupt_level),
            };
            self.stack_push(self.register_set.pc, Size::Long);
            self.stack_push(self.register_set.sr.get_sr() as u32, Size::Word);
            self.register_set.pc = self.header.get_vector(vector);
        }
        let opcode_address = self.register_set.pc;
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        let operation_ptr = MemoryPtr::new(opcode_address, self.bus.clone());
        println!("{}", operation.disassembly(opcode_address, operation_ptr));
        self.cycles_counter = operation.cycles;

        let mut operands = OperandSet::new();
        for am in &operation.addressing_mode_list {
            operands.add(am.get_operand(&mut self.register_set, self.bus.clone()));
        }
        let instruction = &operation.instruction;
        instruction.execute(operands, self);
        println!("{}", self);
        if let Some(vector) = self.trap {
            if vector == RESET_SP {
                M68k::<T>::reset(&self.header, &mut self.register_set);
            } else {
                self.stack_push(self.register_set.pc, Size::Long);
                self.stack_push(self.register_set.sr.get_sr() as u32, Size::Word);
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

        let write_ptr = MemoryPtr::new(address, self.bus.clone());
        write_ptr.write(data, size);
    }

    pub(crate) fn stack_pop(&mut self, size: Size) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long);

        let read_ptr = MemoryPtr::new(address, self.bus.clone());
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
        let opcode_ptr = MemoryPtr::new(self.register_set.get_and_increment_pc(), self.bus.clone());
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

use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    bus::BusM68k,
    instruction_set::system_control::ILLEAGL,
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
    pub(crate) trap: Option<u32>,
    pub(crate) cycles_counter: i32,

    operation_set: Vec<Operation<T>>,
    bus: Option<Rc<T>>,
}

impl<T> M68k<T>
where
    T: BusM68k,
{
    pub fn new() -> Self {
        let mut table: Vec<Operation<T>> = Vec::with_capacity(0x10000);
        table.resize_with(0x10000, || {
            Operation::new(Box::new(ILLEAGL()), Vec::new(), 5)
        });
        generate_opcode_list(&mut table);
        let register_set = RegisterSet::new();
        Self {
            register_set: register_set,
            trap: None,
            cycles_counter: 0,

            operation_set: table,
            bus: None,
        }
    }

    pub fn set_bus(&mut self, bus: T) {
        self.bus = Some(Rc::new(bus));
    }

    pub fn reset(&mut self) {
        let stack_pointer = self.read_header(RESET_SP);
        let stack_register = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register.write(stack_pointer, Size::Long);

        let pc = self.read_header(RESET_PC);
        self.register_set.pc = pc;
    }

    pub fn clock(&mut self) {
        let opcode_address = self.register_set.pc;
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        let operation_ptr = MemoryPtr::new(opcode_address, self.bus.as_ref().unwrap().clone());
        println!("{}", operation.disassembly(opcode_address, operation_ptr));
        self.cycles_counter = operation.cycles;

        let mut operands = OperandSet::new();
        for am in &operation.addressing_mode_list {
            operands
                .add(am.get_operand(&mut self.register_set, self.bus.as_ref().unwrap().clone()));
        }
        let instruction = &operation.instruction;
        instruction.execute(operands, self);
        println!("{}", self);
        if let Some(vector) = self.trap {
            if vector == RESET_SP {
                self.reset();
            } else {
                self.stack_push(self.register_set.pc, Size::Long);
                self.stack_push(self.register_set.sr.get_sr() as u32, Size::Word);
                let vector_address = self.read_header(vector);
                self.register_set.pc = vector_address;
            }
            self.trap = None;
        }
    }

    pub fn interrupt(&mut self, level: u32) {
        let ipl = self.register_set.sr.ipl();
        if level >= ipl && level != 0 {
            let vector = match level {
                1 => LEVEL_1,
                2 => LEVEL_2,
                3 => LEVEL_3,
                4 => LEVEL_4,
                5 => LEVEL_5,
                6 => LEVEL_6,
                7 => LEVEL_7,
                _ => panic!("M68k: clock: wrong interrupt level: {}", level),
            };
            self.stack_push(self.register_set.pc, Size::Long);
            self.stack_push(self.register_set.sr.get_sr() as u32, Size::Word);
            self.register_set.pc = self.read_header(vector);
        }
    }

    pub(crate) fn stack_push(&mut self, data: u32, size: Size) {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let mut address = stack_register_ptr.read(Size::Long);
        address = address.wrapping_sub(size as u32); // predecrementing
        stack_register_ptr.write(address, Size::Long);

        let write_ptr = MemoryPtr::new(address, self.bus.as_ref().unwrap().clone());
        write_ptr.write(data, size);
    }

    pub(crate) fn stack_pop(&mut self, size: Size) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long);

        let read_ptr = MemoryPtr::new(address, self.bus.as_ref().unwrap().clone());
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
        let opcode_ptr = MemoryPtr::new(
            self.register_set.get_and_increment_pc(),
            self.bus.as_ref().unwrap().clone(),
        );
        opcode_ptr.read(Size::Word) as u16
    }

    fn read_header(&self, vector: u32) -> u32 {
        self.bus.as_ref().unwrap().read(vector, Size::Long as u32)
    }
}

impl<T: BusM68k> Display for M68k<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register_set.to_string())
    }
}

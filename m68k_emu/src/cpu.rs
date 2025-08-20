use std::{cell::RefCell, fmt::Display, rc::Rc};

use log::debug;

use crate::{
    bus::BusM68k,
    instruction_set::system_control::ILLEAGL,
    opcode_generators::generate_opcode_list,
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
    bus: Option<Rc<RefCell<T>>>,

    breakpoints: Option<Vec<u32>>,
    pub breakpoint_hit: bool,
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

            breakpoints: None,
            breakpoint_hit: false,
        }
    }

    pub fn set_breakpoints(&mut self, breakpoints: &Vec<u32>) {
        self.breakpoints = Some(breakpoints.clone());
    }

    pub fn set_bus(&mut self, bus: Rc<RefCell<T>>) {
        self.bus = Some(bus);
    }

    pub fn reset(&mut self) {
        let stack_pointer = self.read_header(RESET_SP);
        let stack_register = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register.write(stack_pointer, Size::Long).unwrap();

        let pc = self.read_header(RESET_PC);
        self.register_set.pc = pc;
    }

    pub fn clock(&mut self) -> i32 {
        let register_set_backup = self.register_set;
        let opcode_address = self.register_set.pc;
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        self.cycles_counter = operation.cycles;

        let operation_ptr = MemoryPtr::new(opcode_address, self.bus.as_ref().unwrap().clone());
        debug!("{}", operation.disassembly(operation_ptr).unwrap());

        let mut operands = Vec::new();
        for am in &operation.addressing_mode_list {
            let operand =
                match am.get_operand(&mut self.register_set, self.bus.as_ref().unwrap().clone()) {
                    Ok(o) => o,
                    Err(_) => {
                        self.register_set = register_set_backup; // if cpu can't get operand from any memory location then tere is roll back its state for another try
                        return 1;
                    }
                };
            if let Some(breakpoints) = self.breakpoints.as_ref() {
                self.breakpoint_hit = breakpoints.iter().any(|b| *b == operand.operand_address);
            }
            operands.push(operand);
        }
        let instruction = &operation.instruction;
        match instruction.execute(operands, self) {
            Ok(_) => (),
            Err(_) => {
                self.register_set = register_set_backup; // if cpu can't execute an instruction then tere is roll back its state for another try
                return 1;
            }
        }
        debug!("\n{}", self);
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
        if let Some(breakpoints) = self.breakpoints.as_ref() {
            self.breakpoint_hit = breakpoints.iter().any(|b| *b == self.register_set.pc);
        }
        self.cycles_counter
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
        let mut address = stack_register_ptr.read(Size::Long).unwrap();
        address = address.wrapping_sub(size as u32); // predecrementing
        stack_register_ptr.write(address, Size::Long).unwrap();

        let write_ptr = MemoryPtr::new(address, self.bus.as_ref().unwrap().clone());
        write_ptr.write(data, size).unwrap();
    }

    pub(crate) fn stack_pop(&mut self, size: Size) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long).unwrap();

        let read_ptr = MemoryPtr::new(address, self.bus.as_ref().unwrap().clone());
        let data = read_ptr.read(size).unwrap();

        stack_register_ptr
            .write(address.wrapping_add(size as u32), Size::Long)
            .unwrap(); // postincrement
        data
    }

    pub(crate) fn get_stack_address(&mut self) -> u32 {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr.read(Size::Long).unwrap()
    }

    pub(crate) fn set_stack_address(&mut self, new_stack_address: u32) {
        let stack_register_ptr = self
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr
            .write(new_stack_address, Size::Long)
            .unwrap();
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode_ptr = MemoryPtr::new(
            self.register_set.get_and_increment_pc(),
            self.bus.as_ref().unwrap().clone(),
        );
        match opcode_ptr.read(Size::Word) {
            Ok(opcode_bytes) => opcode_bytes as u16,
            Err(_) => panic!(
                "M68k: fetch_opcode: can't fetching opcode by address: {}",
                opcode_ptr
            ),
        }
    }

    fn read_header(&self, vector: u32) -> u32 {
        match self.bus.as_ref().unwrap().borrow().read(vector, Size::Long as u32) {
            Ok(header) => header,
            Err(_) => panic!(
                "M68k: read_header: can't read header by vector: {:08X}",
                vector
            ),
        }
    }
}

impl<T: BusM68k> Display for M68k<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register_set.to_string())
    }
}

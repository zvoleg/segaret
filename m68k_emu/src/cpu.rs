use crate::{
    bus::BusM68k,
    cpu_internals::{CpuInternals, RegisterSet, RegisterType},
    header::Header,
    instruction_set::system_control::ILLEAGL,
    opcode_generators::generate_opcode_list,
    operand::OperandSet,
    operation::Operation,
    primitives::{memory::MemoryPtr, Pointer, Size},
    vectors::{RESET_PC, RESET_SP},
    STACK_REGISTER,
};

pub struct M68k<T: BusM68k> {
    pub(crate) internals: CpuInternals,
    header: Header,

    operation_set: Vec<Operation>,
    bus: T,
}

impl<T> M68k<T>
where
    T: BusM68k,
{
    pub fn new(bus: T) -> Self {
        let mut table: Vec<Operation> = Vec::with_capacity(0x10000);
        table.resize_with(0x10000, || {
            Operation::new(Box::new(ILLEAGL()), Vec::new(), 5)
        });
        generate_opcode_list(&mut table);
        let header_ptr = MemoryPtr::new(bus.set_address(0));
        let header = Header::new(header_ptr);
        let mut internals = CpuInternals::new();
        M68k::<T>::reset(&header, &mut internals.register_set);
        Self {
            internals: internals,
            header: header,
            operation_set: table,
            bus: bus,
        }
    }

    pub fn clock(&mut self) {
        let opcode_address = self.internals.register_set.pc;
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        let operation_ptr = MemoryPtr::new(self.bus.set_address(opcode_address));
        println!("{}", operation.disassembly(opcode_address, operation_ptr));
        self.internals.cycles = operation.cycles;

        let mut operands = OperandSet::new();
        for am in &operation.addressing_mode_list {
            operands.add(am.get_operand(&mut self.internals.register_set, &self.bus));
        }
        let instruction = &operation.instruction;
        instruction.execute(operands, &mut self.internals);
        if let Some(vector) = self.internals.trap {
            if vector == RESET_SP {
                M68k::<T>::reset(&self.header, &mut self.internals.register_set);
            } else {
                let vector_address = self.header.get_vector(vector);
                self.internals.register_set.pc = vector_address;
            }
            self.internals.trap = None;
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode_ptr = MemoryPtr::new(
            self.bus
                .set_address(self.internals.register_set.get_and_increment_pc()),
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

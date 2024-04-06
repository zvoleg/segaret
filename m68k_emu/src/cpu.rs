use crate::{
    bus::BusM68k,
    cpu_internals::{CpuInternals, RegisterType},
    instruction_set::program_control::NOP,
    opcode_generators::generate_opcode_list,
    operand::OperandSet,
    operation::Operation,
    primitives::{MemoryPtr, Pointer, Size},
    STACK_REGISTER,
};

pub struct M68k<T: BusM68k> {
    pub(crate) internals: CpuInternals,

    operation_set: Vec<Operation>,
    bus: T,
}

impl<T> M68k<T>
where
    T: BusM68k,
{
    pub fn new(bus: T) -> Self {
        let mut table: Vec<Operation> = Vec::with_capacity(0x10000);
        table.resize_with(0x10000, || Operation::new(Box::new(NOP()), Vec::new(), 5));
        generate_opcode_list(&mut table);
        Self {
            internals: CpuInternals::new(),
            operation_set: table,
            bus: bus,
        }
    }

    pub fn clock(&mut self) {
        let opcode = self.fetch_opcode();

        // hack for ignoring the immutable reference to own field
        // it is needed because when an instruction will execute, it needs the mutable reference to self
        let operation_set = self.operation_set.as_ptr();
        let operation = unsafe { &*operation_set.offset(opcode as isize) };

        let mut operands = OperandSet::new();
        for am in &operation.addressing_mode_list {
            operands.add(am.get_operand(&mut self.internals.register_set, &self.bus));
        }
        let instruction = &operation.instruction;
        instruction.execute(operands, &mut self.internals);
    }

    pub(crate) fn push(&mut self, data: u32, size: Size) {
        let stack_register_ptr = self
            .internals
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long) - size as u32; // predecrement
        stack_register_ptr.write(address, Size::Long);
        let memory_ptr = MemoryPtr::new(self.bus.set_address(address));
        memory_ptr.write(data, size);
    }

    pub(crate) fn pop(&mut self, size: Size) -> u32 {
        let stack_register_ptr = self
            .internals
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        let address = stack_register_ptr.read(Size::Long);
        stack_register_ptr.write(address + size as u32, Size::Long); // postincrement
        let memory_ptr = MemoryPtr::new(self.bus.set_address(address));
        memory_ptr.read(size)
    }

    pub(crate) fn get_stack_address(&mut self) -> u32 {
        let stack_register_ptr = self
            .internals
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr.read(Size::Long)
    }

    pub(crate) fn set_stack_address(&mut self, address: u32) {
        let stack_register_ptr = self
            .internals
            .register_set
            .get_register_ptr(STACK_REGISTER, RegisterType::Address);
        stack_register_ptr.write(address, Size::Long);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode_ptr = MemoryPtr::new(
            self.bus
                .set_address(self.internals.register_set.get_and_increment_pc()),
        );
        opcode_ptr.read(Size::Word) as u16
    }
}

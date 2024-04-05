use std::collections::VecDeque;

use crate::{primitives::Pointer, Size};

pub(crate) struct Operand {
    pub(crate) operand_ptr: Box<dyn Pointer>,
    pub(crate) address_register_ptr: Option<Box<dyn Pointer>>, // needs for the MOMEM instruction which changes value in the address register after execution
    pub(crate) operand_address: u32, // needs for the LEA instruction which loads the calculated by addressing mode address value into an address register
}

impl Operand {
    pub(crate) fn new(
        operand_ptr: Box<dyn Pointer>,
        address_register_ptr: Option<Box<dyn Pointer>>,
        operand_address: u32,
    ) -> Self {
        Self {
            operand_ptr,
            address_register_ptr,
            operand_address,
        }
    }

    pub(crate) fn read(&self, size: Size) -> u32 {
        self.operand_ptr.read(size)
    }

    pub(crate) fn write(&self, data: u32, size: Size) {
        self.operand_ptr.write(data, size)
    }
}

pub(crate) struct OperandSet {
    operands: VecDeque<Operand>,
}

impl OperandSet {
    pub(crate) fn new() -> Self {
        Self {
            operands: VecDeque::new(),
        }
    }

    pub(crate) fn add(&mut self, operand: Operand) {
        self.operands.push_front(operand);
    }

    pub(crate) fn next(&mut self) -> Operand {
        self.operands.pop_front().unwrap()
    }
}

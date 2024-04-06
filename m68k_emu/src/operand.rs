use std::collections::VecDeque;

use crate::{primitives::Pointer, Size};

/// The Operand is representation of data which handled by an instruction
///
/// `operand_ptr` is a memory location of data
///
/// `address_register_ptr` is the pointer to the address register from which an memory address was calculated. The instruction some times needs to access to this data (b.e. MOVEM, LINK)
///
/// `operand_address` is the calculated memory address
pub(crate) struct Operand {
    pub(crate) operand_ptr: Box<dyn Pointer>,
    pub(crate) address_register_ptr: Option<Box<dyn Pointer>>,
    pub(crate) operand_address: u32,
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
        self.operands.pop_back().unwrap()
    }
}

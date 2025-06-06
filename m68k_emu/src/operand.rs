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
    size: Size,
}

impl Operand {
    pub(crate) fn new(
        operand_ptr: Box<dyn Pointer>,
        address_register_ptr: Option<Box<dyn Pointer>>,
        operand_address: u32,
        size: Size,
    ) -> Self {
        Self {
            operand_ptr,
            address_register_ptr,
            operand_address,
            size,
        }
    }

    pub(crate) fn read(&self) -> Result<u32, ()> {
        self.operand_ptr.read(self.size)
    }

    pub(crate) fn read_sized(&self, size: Size) -> Result<u32, ()> {
        self.operand_ptr.read(size)
    }

    pub(crate) fn write(&self, data: u32) -> Result<(), ()> {
        self.operand_ptr.write(data, self.size)
    }

    pub(crate) fn write_sized(&self, data: u32, size: Size) -> Result<(), ()> {
        self.operand_ptr.write(data, size)
    }
}

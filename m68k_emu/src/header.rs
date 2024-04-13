use crate::primitives::{MemoryPtr, Pointer, Size};

pub(crate) struct Header {
    header_ptr: MemoryPtr,
}

impl Header {
    pub(crate) fn new(header_ptr: MemoryPtr) -> Self {
        Self { header_ptr }
    }

    pub(crate) fn get_vector(&self, vector: usize) -> u32 {
        self.header_ptr.read_offset(Size::Long, vector as isize)
    }
}

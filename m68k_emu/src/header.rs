pub(crate) struct Header {
    header_ptr: *mut u8,
}

impl Header {
    pub(crate) fn new(header_ptr: *mut u8) -> Self {
        Self { header_ptr }
    }

    pub(crate) fn get_vector(&self, vector: usize) -> u32 {
        unsafe {
            let vector_ptr = self.header_ptr.offset(vector as isize);
            *(vector_ptr as *const _ as *const u32)
        }
    }
}

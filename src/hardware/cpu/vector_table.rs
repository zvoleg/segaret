const STACK_PTR: isize = 0x000000;
const PROGRAM_COUNTER: isize = 0x000004;

pub(crate) struct VectorTable {
    header_ptr: *const u8,
}

impl VectorTable {
    pub(crate) fn init(header_ptr: *const u8) -> Self {
        Self {
            header_ptr,
        }
    }

    fn get_offseted_value(&self, offset: isize) -> u32 {
        unsafe {
            (*(self.header_ptr.offset(offset) as *const _ as *const u32)).to_be()
        }
    }

    pub(crate) fn reset_stack_pointer(&self) -> u32 {
        self.get_offseted_value(STACK_PTR)
    }

    pub(crate) fn reset_program_counter(&self) -> u32 {
        self.get_offseted_value(PROGRAM_COUNTER)
    }
}
const STACK_PTR: isize = 0x000000;
const PROGRAM_COUNTER: isize = 0x000004;

const ILLEGAL_INSTR: isize = 0x000010;
const ZERO_DIVISION: isize = 0x000014;

const INTERRUPT_LEVEL_1: isize = 0x000064;

pub(in crate) struct VectorTable {
    header_ptr: *const u8,
}

impl VectorTable {
    pub(in crate) fn init(header_ptr: *const u8) -> Self {
        Self {
            header_ptr,
        }
    }

    fn get_offseted_value(&self, offset: isize) -> u32 {
        unsafe {
            (*(self.header_ptr.offset(offset) as *const _ as *const u32)).to_be()
        }
    }

    pub(in crate) fn reset_stack_pointer(&self) -> u32 {
        self.get_offseted_value(STACK_PTR)
    }

    pub(in crate) fn reset_program_counter(&self) -> u32 {
        self.get_offseted_value(PROGRAM_COUNTER)
    }

    pub(in crate) fn illegal_instruction(&self) -> u32 {
        self.get_offseted_value(ILLEGAL_INSTR)
    }

    pub(in crate) fn zero_division_exception(&self) -> u32 {
        self.get_offseted_value(ZERO_DIVISION)
    }

    pub(in crate) fn interrupt_level(&self, interrupt_level: usize) -> u32 {
        let interrupt_offset = 4 * (interrupt_level - 1) as isize;
        self.get_offseted_value(INTERRUPT_LEVEL_1 + interrupt_offset)
    }
}
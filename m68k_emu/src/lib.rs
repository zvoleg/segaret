pub mod bus;
pub mod cpu;

mod addressing_mode_set;
mod cpu_internals;
mod extension_word;
mod instruction_set;
mod operand;
mod operation;
mod primitives;
mod status_flag;
mod status_register;
// mod decoder;
mod opcode_generators;

use primitives::Size;

const STACK_REGISTER: usize = 7;

trait IsNegate {
    fn is_negate(&self, size: Size) -> bool;
}

impl IsNegate for u32 {
    fn is_negate(&self, size: Size) -> bool {
        match size {
            Size::Byte => self & 0x00000080 != 0,
            Size::Word => self & 0x00008000 != 0,
            Size::Long => self & 0x80000000 != 0,
        }
    }
}

trait SignExtending {
    fn sign_extend(&self, size: Size) -> Self;
}

impl SignExtending for u32 {
    fn sign_extend(&self, size: Size) -> Self {
        if self.is_negate(size) {
            match size {
                Size::Byte => 0xFFFFFF00 | self,
                Size::Word => 0xFFFF0000 | self,
                Size::Long => *self,
            }
        } else {
            *self
        }
    }
}

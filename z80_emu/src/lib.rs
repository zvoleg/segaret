pub mod bus;
pub mod cpu;

mod addressing_mode;
mod instruction_set;
mod opcode_table_generator;
mod operation;
mod primitives;
mod register_set;

#[derive(Clone, Copy)]
pub(crate) enum Size {
    Byte = 1,
    Word = 2,
}

impl Into<usize> for Size {
    fn into(self) -> usize {
        self as usize
    }
}

enum InterruptMode {
    Mode0,
    Mode1,
    Mode2,
}

trait SignExtending {
    fn sign_extend(&self, size: Size) -> Self;
}

impl SignExtending for u16 {
    fn sign_extend(&self, size: Size) -> Self {
        match size {
            Size::Byte => *self as u8 as i8 as i16 as u16,
            Size::Word => *self,
        }
    }
}

trait IsNegate {
    fn is_negate(&self, size: Size) -> bool;
}

impl IsNegate for u16 {
    fn is_negate(&self, size: Size) -> bool {
        match size {
            Size::Byte => self & 0x80 != 0,
            Size::Word => self & 0x8000 != 0,
        }
    }
}

trait MostSignificantBit {
    fn get_msb(&self, size: Size) -> bool;
}

impl MostSignificantBit for u16 {
    fn get_msb(&self, size: Size) -> bool {
        match size {
            Size::Byte => self & 0x80 != 0,
            Size::Word => self & 0x8000 != 0,
        }
    }
}

trait GetBit {
    fn get_bit(&self, position: u16) -> bool;
}

impl GetBit for u16 {
    fn get_bit(&self, position: u16) -> bool {
        self & (1 << position) != 0
    }
}

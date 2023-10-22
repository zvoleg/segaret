use super::Size;

pub mod z80_emu;
pub(in crate::hardware::z80) mod macros;

pub trait Z80Bus {
    fn read(&self, address: u16, size: Size) -> u16;
    fn write(&mut self, address: u16, data: u16, size: Size);
}

#[derive(Clone, Copy)]
pub(in crate::hardware::z80) enum Register {
    A,
    A_,
    B,
    B_,
    C,
    C_,
    D,
    D_,
    E,
    E_,
    H,
    H_,
    L,
    L_,
    AF,
    AF_,
    BC,
    BC_,
    DE,
    DE_,
    HL,
    HL_,
    IX,
    IY,
    SP,
}

pub(in crate::hardware::z80) enum AmType {
    Imm,
    ImmExt,
    PageZero(u16),
    Relative,
    Extended,
    Indexed(Register),
    Register(Register),
    Implied,
    RegIndirect(Register),
    BitAddr(u16),
}

#[derive(Clone, Copy)]
pub(in crate::hardware::z80) enum Location {
    Memory(u16),
    Register(Register),
    Const,
}

pub(in crate::hardware::z80) struct Operand {
    pub(in crate::hardware::z80) location: Location,
    pub(in crate::hardware::z80) data: u16,
}

impl Operand {
    pub(in crate::hardware::z80) fn new(location: Location, data: u16) -> Self {
        Self { location, data }
    }

    pub(in crate::hardware::z80) fn memory_operand(address: u16, data: u16) -> Self {
        Self::new(
            Location::Memory(address),
            data,
        )
    }

    pub(in crate::hardware::z80) fn register_operand(register: Register, data: u16) -> Self {
        Self::new(
            Location::Register(register),
            data,
        )
    }

    pub(in crate::hardware::z80) fn constant_operand(data: u16) -> Self {
        Self::new(
            Location::Const,
            data,
        )
    }
}

pub(in crate::hardware::z80) struct Instruction {
    pub(in crate::hardware::z80) src_am: Option<AmType>,
    pub(in crate::hardware::z80) dst_am: Option<AmType>,
    pub(in crate::hardware::z80) size: Size,
    pub(in crate::hardware::z80) handler: fn(&mut z80_emu::Z80Emu),
}

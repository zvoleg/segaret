pub mod z80_emu;
#[macro_use]
pub(crate) mod macros;

pub trait Z80Bus {
    fn read(&self, address: u16, size: Size) -> u16;
    fn write(&mut self, address: u16, data: u16, size: Size);
}

#[derive(Clone, Copy)]
pub(crate) enum Register {
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
    I,
    R,
    IX,
    IY,
    SP,
}

pub(crate) enum AmType {
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
pub(crate) enum Location {
    Memory(u16),
    Register(Register),
    Const,
}

pub(crate) struct Operand {
    pub(crate) location: Location,
    pub(crate) data: u16,
}

impl Operand {
    pub(crate) fn new(location: Location, data: u16) -> Self {
        Self { location, data }
    }

    pub(crate) fn memory_operand(address: u16, data: u16) -> Self {
        Self::new(
            Location::Memory(address),
            data,
        )
    }

    pub(crate) fn register_operand(register: Register, data: u16) -> Self {
        Self::new(
            Location::Register(register),
            data,
        )
    }

    pub(crate) fn constant_operand(data: u16) -> Self {
        Self::new(
            Location::Const,
            data,
        )
    }
}

pub(crate) struct Instruction {
    pub(crate) src_am: Option<AmType>,
    pub(crate) dst_am: Option<AmType>,
    pub(crate) size: Size,
    pub(crate) handler: fn(&mut z80_emu::Z80Emu),
}

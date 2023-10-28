use hardware::Size;

pub mod z80_emu;
#[macro_use]
pub(in crate) mod macros;

pub trait Z80Bus {
    fn read(&self, address: u16, size: Size) -> u16;
    fn write(&mut self, address: u16, data: u16, size: Size);
}

#[derive(Clone, Copy)]
pub(in crate) enum Register {
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

pub(in crate) enum AmType {
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
pub(in crate) enum Location {
    Memory(u16),
    Register(Register),
    Const,
}

pub(in crate) struct Operand {
    pub(in crate) location: Location,
    pub(in crate) data: u16,
}

impl Operand {
    pub(in crate) fn new(location: Location, data: u16) -> Self {
        Self { location, data }
    }

    pub(in crate) fn memory_operand(address: u16, data: u16) -> Self {
        Self::new(
            Location::Memory(address),
            data,
        )
    }

    pub(in crate) fn register_operand(register: Register, data: u16) -> Self {
        Self::new(
            Location::Register(register),
            data,
        )
    }

    pub(in crate) fn constant_operand(data: u16) -> Self {
        Self::new(
            Location::Const,
            data,
        )
    }
}

pub(in crate) struct Instruction {
    pub(in crate) src_am: Option<AmType>,
    pub(in crate) dst_am: Option<AmType>,
    pub(in crate) size: Size,
    pub(in crate) handler: fn(&mut z80_emu::Z80Emu),
}

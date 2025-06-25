use crate::{primitives::RegisterPtr, Size};

#[derive(Clone, Copy)]
pub(crate) enum RegisterType {
    A,
    F,
    AF,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,

    A_,
    F_,
    AF_,
    B_,
    C_,
    BC_,
    D_,
    E_,
    DE_,
    H_,
    L_,
    HL_,
}

impl RegisterType {
    pub(crate) fn idx(&self) -> usize {
        match self {
            RegisterType::A | RegisterType::AF => 0,
            RegisterType::F => 1,
            RegisterType::B | RegisterType::BC => 2,
            RegisterType::C => 3,
            RegisterType::D | RegisterType::DE => 4,
            RegisterType::E => 5,
            RegisterType::H | RegisterType::HL => 6,
            RegisterType::L => 7,

            RegisterType::A_ | RegisterType::AF_ => 8,
            RegisterType::F_ => 9,
            RegisterType::B_ | RegisterType::BC_ => 10,
            RegisterType::C_ => 11,
            RegisterType::D_ | RegisterType::DE_ => 12,
            RegisterType::E_ => 13,
            RegisterType::H_ | RegisterType::HL_ => 14,
            RegisterType::L_ => 15,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum IndexRegister {
    X = 0,
    Y = 1,
}

#[derive(Clone, Copy)]
pub(crate) enum Register {
    General(RegisterType),
    Index(IndexRegister),
    StackPointer,
    InterruptVector,
    MemoryRefresh,
}

impl Register {
    pub(crate) fn size(&self) -> Size {
        match self {
            Register::General(register_type) => match register_type {
                RegisterType::A
                | RegisterType::A_
                | RegisterType::F
                | RegisterType::F_
                | RegisterType::B
                | RegisterType::B_
                | RegisterType::C
                | RegisterType::C_
                | RegisterType::D
                | RegisterType::D_
                | RegisterType::E
                | RegisterType::E_
                | RegisterType::H
                | RegisterType::H_
                | RegisterType::L
                | RegisterType::L_ => Size::Byte,
                RegisterType::AF
                | RegisterType::BC
                | RegisterType::DE
                | RegisterType::HL
                | RegisterType::AF_
                | RegisterType::BC_
                | RegisterType::DE_
                | RegisterType::HL_ => Size::Word,
            },
            Register::Index(_) => Size::Word,
            Register::StackPointer => Size::Word,
            Register::InterruptVector => Size::Word,
            Register::MemoryRefresh => Size::Byte,
        }
    }
}

pub(crate) enum Status {
    C = 0,
    N = 1,
    PV = 2,
    H = 4,
    Z = 6,
    S = 7,
}

pub(crate) struct RegisterSet {
    registers: [u8; 16],
    index_register: [u16; 2],
    stack_pointer: u16,
    program_counter: u16,
    interrupt_vector: u8,
    memory_refresh: u8,
}

impl RegisterSet {
    pub(crate) fn new() -> Self {
        Self {
            registers: [0; 16],
            index_register: [0; 2],
            stack_pointer: 0,
            program_counter: 0,
            interrupt_vector: 0,
            memory_refresh: 0,
        }
    }

    pub(crate) fn read_register(&self, reg: Register, size: Size) -> u16 {
        match reg {
            Register::General(register_type) => match size {
                Size::Byte => self.registers[register_type.idx()] as u16,
                Size::Word => unsafe {
                    let ptr = self.registers.as_ptr().offset(register_type.idx() as isize);
                    *(ptr as *const _ as *const u16)
                },
            },
            Register::Index(index_register) => self.index_register[index_register as usize],
            Register::StackPointer => self.stack_pointer,
            Register::InterruptVector => self.interrupt_vector as u16,
            Register::MemoryRefresh => self.memory_refresh as u16,
        }
    }

    pub(crate) fn write_register(&mut self, data: u16, reg: Register, size: Size) {
        match reg {
            Register::General(register_type) => match size {
                Size::Byte => self.registers[register_type.idx()] = data as u8,
                Size::Word => unsafe {
                    let ptr = self
                        .registers
                        .as_mut_ptr()
                        .offset(register_type.idx() as isize);
                    *(ptr as *mut _ as *mut u16) = data;
                },
            },
            Register::Index(index_register) => self.index_register[index_register as usize] = data,
            Register::StackPointer => self.stack_pointer = data,
            Register::InterruptVector => self.interrupt_vector = data as u8,
            Register::MemoryRefresh => self.memory_refresh = data as u8,
        }
    }

    pub(crate) fn set_flag(&mut self, status: Status, set: bool) {
        let mut flag_field: u16 =
            self.read_register(Register::General(RegisterType::F), Size::Byte);
        let bit_mask = 1u16 << status as u16;
        if set {
            flag_field |= bit_mask
        } else {
            flag_field &= !bit_mask
        }
        self.write_register(flag_field, Register::General(RegisterType::F), Size::Byte);
    }

    pub(crate) fn get_flag(&self, status: Status) -> bool {
        let flag_field = self.read_register(Register::General(RegisterType::F), Size::Byte);
        flag_field & (1 << status as u16) != 0
    }

    pub(crate) fn get_register_ptr(&mut self, reg: Register) -> RegisterPtr {
        let ptr = unsafe {
            match reg {
                Register::General(register_type) => self
                    .registers
                    .as_mut_ptr()
                    .offset(register_type.idx() as isize),
                Register::Index(index_register) => {
                    self.index_register
                        .as_mut_ptr()
                        .offset(index_register as isize) as *mut _ as *mut u8
                }
                Register::StackPointer => &mut self.stack_pointer as *mut _ as *mut u8,
                Register::InterruptVector => &mut self.interrupt_vector,
                Register::MemoryRefresh => &mut self.memory_refresh,
            }
        };
        RegisterPtr::new(ptr)
    }

    pub(crate) fn exchange_general_registers(&mut self) {
        for i in 2..8 {
            // skip 'A' and 'F' registers
            let a = self.registers[i];
            let b = self.registers[i + 8];

            self.registers[i] = b;
            self.registers[i + 8] = a;
        }
    }
}

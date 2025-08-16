use std::fmt::Display;

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

impl Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RegisterType::A => "a",
            RegisterType::F => "f",
            RegisterType::AF => "af",
            RegisterType::B => "b",
            RegisterType::C => "c",
            RegisterType::BC => "bc",
            RegisterType::D => "d",
            RegisterType::E => "e",
            RegisterType::DE => "de",
            RegisterType::H => "h",
            RegisterType::L => "l",
            RegisterType::HL => "hl",
            RegisterType::A_ => "a_",
            RegisterType::F_ => "f_",
            RegisterType::AF_ => "af_",
            RegisterType::B_ => "b_",
            RegisterType::C_ => "c_",
            RegisterType::BC_ => "bc_",
            RegisterType::D_ => "d_",
            RegisterType::E_ => "e_",
            RegisterType::DE_ => "de_",
            RegisterType::H_ => "h_",
            RegisterType::L_ => "l_",
            RegisterType::HL_ => "hl_",
        };
        write!(f, "{}", s)
    }
}

impl RegisterType {
    pub(crate) fn idx(&self) -> usize {
        match self {
            RegisterType::F | RegisterType::AF => 0,
            RegisterType::A => 1,
            RegisterType::C | RegisterType::BC => 2,
            RegisterType::B => 3,
            RegisterType::E | RegisterType::DE => 4,
            RegisterType::D => 5,
            RegisterType::L | RegisterType::HL => 6,
            RegisterType::H => 7,

            RegisterType::F_ | RegisterType::AF_ => 8,
            RegisterType::A_ => 9,
            RegisterType::C_ | RegisterType::BC_ => 10,
            RegisterType::B_ => 11,
            RegisterType::E_ | RegisterType::DE_ => 12,
            RegisterType::D_ => 13,
            RegisterType::L_ | RegisterType::HL_ => 14,
            RegisterType::H_ => 15,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum IndexRegister {
    X = 0,
    Y = 1,
}

impl Display for IndexRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IndexRegister::X => "x",
            IndexRegister::Y => "y",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Register {
    General(RegisterType),
    Index(IndexRegister),
    StackPointer,
    InterruptVector,
    MemoryRefresh,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Register::General(register_type) => &format!("{}", register_type),
            Register::Index(index_register) => &format!("{}", index_register),
            Register::StackPointer => "sp",
            Register::InterruptVector => "i",
            Register::MemoryRefresh => "r",
        };
        write!(f, "{}", s)
    }
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
    pub(crate) interrupt_vector: u8,
    pub(crate) memory_refresh: u8,
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            write!(f, 
                "\nAF: {:04X}\tAF_: {:04X}\nBC: {:04X}\tBC_: {:04X}\nDE: {:04X}\tDE_: {:04X}\nHL: {:04X}\tHL_: {:04X}\n",
                *(&self.registers[0] as *const _ as *const u16), *(&self.registers[8] as *const _ as *const u16),
                *(&self.registers[2] as *const _ as *const u16), *(&self.registers[10] as *const _ as *const u16),
                *(&self.registers[4] as *const _ as *const u16), *(&self.registers[12] as *const _ as *const u16),
                *(&self.registers[6] as *const _ as *const u16), *(&self.registers[14] as *const _ as *const u16),
            )
        }
    }
}

impl RegisterSet {
    pub(crate) fn new() -> Self {
        Self {
            registers: [0; 16],
            index_register: [0; 2],
            stack_pointer: 0,
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

    pub(crate) fn get_stack_ptr(&self) -> u16 {
        self.stack_pointer
    }

    pub(crate) fn set_stack_ptr(&mut self, address: u16) {
        self.stack_pointer = address;
    }
}

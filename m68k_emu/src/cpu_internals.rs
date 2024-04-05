use crate::{
    primitives::{AddressRegisterPtr, DataRegisterPtr, Pointer},
    status_register::StatusRegister,
};

pub(crate) enum RegisterType {
    Address,
    Data,
}

pub(crate) struct RegisterSet {
    registers: [u32; 16],
    pub(crate) pc: u32,
    pub(crate) sr: StatusRegister,
}

impl RegisterSet {
    pub(crate) fn new() -> Self {
        Self {
            registers: [0; 16],
            pc: 0,
            sr: StatusRegister::new(),
        }
    }

    pub(crate) fn get_and_increment_pc(&mut self) -> u32 {
        let v = self.pc;
        self.pc = self.pc.wrapping_add(2);
        v
    }

    pub(crate) fn get_register_ptr(
        &mut self,
        register_index: usize,
        register_type: RegisterType,
    ) -> Box<dyn Pointer> {
        match register_type {
            RegisterType::Address => AddressRegisterPtr::new_boxed(
                &mut self.registers[register_index + 8] as *mut _ as *mut u8,
            ),
            RegisterType::Data => {
                DataRegisterPtr::new_boxed(&mut self.registers[register_index] as *mut _ as *mut u8)
            }
        }
    }
}

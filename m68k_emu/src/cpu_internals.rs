use std::fmt::Display;

use crate::{
    primitives::{address_reg::AddressRegisterPtr, data_reg::DataRegisterPtr, Pointer},
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
            RegisterType::Address => {
                AddressRegisterPtr::new_boxed(&mut self.registers[register_index + 8])
            }
            RegisterType::Data => DataRegisterPtr::new_boxed(&mut self.registers[register_index]),
        }
    }
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = Vec::new();
        for i in 0..8 {
            let data = self.registers[i];
            let address = self.registers[i + 8];
            buffer.push(format!("D{}: {:08X}\tA{}: {:08X}\n", i, data, i, address))
        }
        buffer.push(format!("{:>34}\n", "10SM_210___XNZVC"));
        // PC: 00000202	SR:0000000000000000
        buffer.push(format!(
            "PC: {:08X}\tSR:{:016b}\n",
            self.pc,
            self.sr.get_sr()
        ));
        write!(f, "{}", buffer.join(""))
    }
}

pub(crate) struct CpuInternals {
    pub(crate) register_set: RegisterSet,
    pub(crate) cycles: i32,
    pub(crate) trap: Option<usize>,
}

impl CpuInternals {
    pub(crate) fn new() -> Self {
        Self {
            register_set: RegisterSet::new(),
            cycles: 0,
            trap: None,
        }
    }
}

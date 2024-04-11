use crate::status_flag::StatusFlag;

pub(crate) struct StatusRegister {
    reg: u16,
}

impl StatusRegister {
    pub(crate) fn new() -> Self {
        Self { reg: 0 }
    }

    pub(crate) fn set_flag(&mut self, status_flag: StatusFlag, set: bool) {
        let mask = 1 << status_flag as u16;
        if set {
            self.reg = self.reg | mask;
        } else {
            self.reg = self.reg & !mask;
        }
    }

    pub(crate) fn get_flag(&self, status_flag: StatusFlag) -> bool {
        let mask = 1 << status_flag as u16;
        self.reg & mask != 0
    }

    pub(crate) fn get_bit(&self, status_flag: StatusFlag) -> u32 {
        ((self.reg >> status_flag as u16) & 1) as u32
    }

    pub(crate) fn set_ccr(&mut self, ccr: u32) {
        self.reg &= 0xFF00;
        self.reg |= ccr as u16;
    }
}

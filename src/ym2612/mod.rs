pub mod ym2612;

pub(crate) mod channel;

pub enum RegisterPart {
    Fm1,
    Fm2,
}

pub trait Ym2612Ports {
    fn register_set(&mut self, part: RegisterPart, register: u8);
    fn register_data(&mut self, part: RegisterPart, data: u8);
    fn read_status(&self) -> u8;
}
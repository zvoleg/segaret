use crate::Size;

pub trait BusZ80 {
    fn read(&self, address: u16, size: Size) -> Result<u16, ()>;
    fn write(&self, data: u16, address: u16, size: Size) -> Result<(), ()>;
}

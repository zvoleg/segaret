pub trait BusZ80 {
    fn read(&self, address: u16, amount: u32) -> Result<u16, ()>;
    fn write(&self, data: u16, address: u16, amount: u32) -> Result<(), ()>;
}

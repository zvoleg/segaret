pub trait BusZ80 {
    fn read(&self, address: u16, amount: usize) -> Result<u16, ()>;
    fn write(&mut self, data: u16, address: u16, amount: usize) -> Result<(), ()>;
}

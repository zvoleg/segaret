pub trait BusM68k {
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()>;
    fn write(&mut self, data: u32, address: u32, amount: u32) -> Result<(), ()>;
}

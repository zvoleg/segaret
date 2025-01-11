pub trait BusM68k {
    fn read(&self, address: u32, amount: u32) -> Result<u32, ()>;
    fn write(&self, data: u32, address: u32, amount: u32) -> Result<(), ()>;
    // fn set_address_read(&self, address: u32) -> *const u8;
    // fn set_address_write(&self, address: u32) -> *mut u8;
}

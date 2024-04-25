pub trait BusM68k {
    fn set_address_read(&self, address: u32) -> *const u8;
    fn set_address_write(&self, address: u32) -> *mut u8;
}

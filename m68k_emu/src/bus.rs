pub trait BusM68k {
    fn set_address(&self, address: u32) -> *mut u8;
}

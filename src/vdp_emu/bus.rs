pub trait BusVdp {
    fn read(&self, address: u32) -> u16;
}

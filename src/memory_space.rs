pub struct MemorySpace {
    pub rom: Vec<u8>,
    pub m68k_ram: Vec<u8>,
    pub z80_ram: Vec<u8>,

    pub io_area_read: [u8; 0x20],
    pub io_area_m68k: [u8; 0x20],
}

impl MemorySpace {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut io_area_read = [0; 0x20];
        io_area_read[1] = 0x0090; // `setup version register
        Self {
            rom: rom,
            z80_ram: vec![0; 0x10000],  // $A00000	$A0FFFF
            m68k_ram: vec![0; 0x10000], // $FF0000	$FFFFFF

            io_area_read: [0; 0x20],
            io_area_m68k: [0; 0x20],
        }
    }
}

use super::{bus::BusVdp, registers::DMA_SOURC_III, vdp_emu::Vdp, DmaMode, RamAccessMode};

const VDP_CTRL_OPERATION_TYPE_MASK: u16 = 0x7 << 13;
const VDP_CTRL_REGISTER_SET_MODE_MASK: u16 = 0x1 << 15;
const VDP_CTRL_REGISTER_ID_MASK: u16 = 0x1F00;

pub trait VdpPorts {
    fn read_data_port(&mut self) -> Result<u32, ()>;
    fn write_data_port(&mut self, data: u16) -> Result<(), ()>;
    fn read_control_port(&self) -> Result<u32, ()>;
    fn write_control_port(&mut self, data: u16) -> Result<(), ()>;
}

impl<T> VdpPorts for Vdp<T>
where
    T: BusVdp,
{
    fn read_data_port(&mut self) -> Result<u32, ()> {
        match self.ram_access_mode {
            RamAccessMode::VramR => (),
            RamAccessMode::CramR => (),
            RamAccessMode::VSramR => (),
            _ => (), // wron access mode just ignoring (by docks)
        }
        self.ram_address += self.get_address_increment();
        Ok(0)
    }

    fn write_data_port(&mut self, data: u16) -> Result<(), ()> {
        if let Some(DmaMode::RamFill) = self.dma_mode {
            self.dma_run = true;
            self.dma_data_wait = false;
        } else {
            println!("write to data port, mode: '{}', address: {:04X}", self.ram_access_mode, self.ram_address);
            match self.ram_access_mode {
                RamAccessMode::VramW => unsafe {
                    let ptr = self.vram.as_ptr().offset(self.ram_address as isize) as *const _ as *mut u16;
                    *ptr = data;
                },
                RamAccessMode::CramW => unsafe {
                    let ptr = self.cram.as_ptr().offset(self.ram_address as isize) as *const _ as *mut u16;
                    *ptr = data;
                },
                RamAccessMode::VSramW => unsafe {
                    let ptr = self.vsram.as_ptr().offset(self.ram_address as isize) as *const _ as *mut u16;
                    *ptr = data;
                },
                _ => (), // wron access mode just ignoring (by docks)
            }
            self.ram_address += self.get_address_increment();
        }
        Ok(())
    }

    fn read_control_port(&self) -> Result<u32, ()> {
        Ok(self.status_register as u32)
    }

    fn write_control_port(&mut self, data: u16) -> Result<(), ()> {
        if data & VDP_CTRL_OPERATION_TYPE_MASK == VDP_CTRL_REGISTER_SET_MODE_MASK {
            self.set_register(data);
        } else {
            self.set_ram_access(data);
        }
        Ok(())
    }
}

impl<T> Vdp<T> where T: BusVdp {
    fn set_register(&mut self, data: u16) {
        let register_id = (data & VDP_CTRL_REGISTER_ID_MASK).swap_bytes() as u8;
        let register_data = data as u8;
        self.registers[register_id as usize] = register_data;
        println!(
            "VDP: set register {:02X} to value {:02X}",
            register_id, register_data
        )
    }

    fn set_ram_access(&mut self, data: u16) {
        if !self.address_setting_latch {
            // first word
            self.address_setting_raw_word &= 0xFFFF; // clear msb
            self.address_setting_raw_word |= (data as u32) << 16;
        } else {
            // second word
            self.address_setting_raw_word &= 0xFFFF0000; // clear lsb
            self.address_setting_raw_word |= data as u32;

            let ram_access_mode_mask = (((self.address_setting_raw_word >> 4) & 0xF) << 2)
                | (self.address_setting_raw_word >> 30) & 0b11;
            let address = ((self.address_setting_raw_word & 0b11) << 14)
                | (self.address_setting_raw_word >> 16) & 0x3FFF;
            if self.address_setting_raw_word & 0x00000080 != 0 {
                // it is dma transfer mode
                let dma_mode_mask = ram_access_mode_mask >> 4;
                let dma_src_reg = self.registers[DMA_SOURC_III];
                if (dma_mode_mask == 0b10) && (dma_src_reg & 0x80 == 0) {
                    self.dma_mode = Some(DmaMode::BusToRamCopy);
                    self.dma_run = true;
                } else if (dma_mode_mask == 0b10) && (dma_src_reg & 0x80 != 0) {
                    self.dma_mode = Some(DmaMode::RamFill);
                    self.dma_data_wait = true;
                } else if (dma_mode_mask == 0b11) && (dma_src_reg & 0xC0 != 0) {
                    self.dma_mode = Some(DmaMode::RamToRamCopy);
                    self.dma_run = true;
                } else {
                    panic!("VDP: write_control_port: unexpected dma mode bits sequence");
                }
                println!("VDP: set dma mode '{}'", self.dma_mode.as_ref().unwrap());
            }
            // it is address set mode
            self.ram_access_mode = RamAccessMode::new((ram_access_mode_mask & 0x7) as u16);
            self.ram_address = address;

            println!(
                "VDP: set ram access mode '{}' and address {:04X}",
                self.ram_access_mode, self.ram_address
            );
        }
        self.address_setting_latch = !self.address_setting_latch
    }
}
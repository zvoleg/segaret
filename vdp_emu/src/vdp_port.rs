use crate::{
    registers::{DMA_SOURC_III, STATUS_REGISTER},
    DmaMode, RamAccessMode, Vdp,
};

pub trait VdpPorts {
    fn read_data_port(&self) -> u32;
    fn write_data_port(&mut self, data: u16);
    fn read_control_port(&self) -> u32;
    fn write_control_port(&mut self, data: u16);
}

impl VdpPorts for Vdp {
    fn read_data_port(&self) -> u32 {
        0
    }

    fn write_data_port(&mut self, data: u16) {
        ()
    }

    fn read_control_port(&self) -> u32 {
        self.status_register as u32
    }

    fn write_control_port(&mut self, data: u16) {
        if data & (0b111 << 13) == (0b1 << 15) {
            // then it is register set mode
            let register_id = (data >> 8) & 0x1F;
            let register_data = (data & 0xFF) as u8;
            self.registers[register_id as usize] = register_data;
            println!(
                "VDP: set register {:02X} to value {:02X}",
                register_id, register_data
            )
        } else {
            // else it is address set mod
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
                    } else if (dma_mode_mask == 0b10) && (dma_src_reg & 0x80 != 0) {
                        self.dma_mode = Some(DmaMode::RamFill);
                    } else if (dma_mode_mask == 0b11) && (dma_src_reg & 0xC0 != 0) {
                        self.dma_mode = Some(DmaMode::RamToRamCopy);
                    } else {
                        panic!("VDP: write_control_port: unexpected dma mode bits sequence");
                    }
                    println!("VDP: set dma mode '{}'", self.dma_mode.as_ref().unwrap());
                }
                // it is address set mode
                self.ram_access_mode =
                    Some(RamAccessMode::new((ram_access_mode_mask & 0x7) as u16));
                self.ram_address = address as u16;

                self.dma_mode = None;
                println!(
                    "VDP: set ram access mode '{}' and address {:04X}",
                    self.ram_access_mode.as_ref().unwrap(),
                    self.ram_address
                );
            }
            self.address_setting_latch = !self.address_setting_latch
        }
    }
}

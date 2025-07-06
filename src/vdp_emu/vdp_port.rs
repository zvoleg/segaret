use log::debug;

use super::{bus::BusVdp, vdp_emu::Vdp, DmaMode, RamAccessMode};

const VDP_CTRL_OPERATION_TYPE_MASK: u16 = 0x7 << 13;
const VDP_CTRL_REGISTER_SET_MODE_MASK: u16 = 0x1 << 15;
const VDP_CTRL_REGISTER_ID_MASK: u16 = 0x1F00;

pub trait VdpPorts {
    fn read_data_port(&mut self) -> Result<u32, ()>;
    fn write_data_port(&mut self, data: u16) -> Result<(), ()>;
    fn read_control_port(&mut self) -> Result<u32, ()>;
    fn write_control_port(&mut self, data: u16) -> Result<(), ()>;
    fn read_hv_counters_port(&mut self) -> Result<u32, ()>;
}

impl<T> VdpPorts for Vdp<T>
where
    T: BusVdp,
{
    fn read_data_port(&mut self) -> Result<u32, ()> {
        self.address_setting_latch = false;
        let data = unsafe {
            match self.ram_access_mode {
                RamAccessMode::VramR => {
                    *(self.vram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *const u16)
                }
                RamAccessMode::CramR => {
                    *(self.cram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *const u16)
                }
                RamAccessMode::VSramR => {
                    *(self.vsram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *const u16)
                }
                _ => 0, // wrong access mode just ignoring (by docks)
            }
        }
        .to_be();
        self.vdp_ram_address += self.register_set.autoincrement.autoincrement();
        Ok(data as u32)
    }

    fn write_data_port(&mut self, data: u16) -> Result<(), ()> {
        self.address_setting_latch = false;
        if let Some(DmaMode::FillRam) = self.dma_mode {
            let dma_enabled = self.register_set.mode_register.dma_enabled();
            if dma_enabled {
                self.dma_run = true;
                self.dma_data_wait = false;
            }
        } else {
            debug!(
                "write to data port, mode: '{}', address: {:04X}",
                self.ram_access_mode, self.vdp_ram_address
            );
            match self.ram_access_mode {
                RamAccessMode::VramW => unsafe {
                    let ptr = self.vram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *mut u16;
                    *ptr = data.to_be();
                },
                RamAccessMode::CramW => unsafe {
                    let ptr = self.cram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *mut u16;
                    *ptr = data.to_be();
                },
                RamAccessMode::VSramW => unsafe {
                    let ptr = self.vsram.as_ptr().offset(self.vdp_ram_address as isize) as *const _
                        as *mut u16;
                    *ptr = data.to_be();
                },
                _ => (), // wron access mode just ignoring (by docks)
            }
            self.vdp_ram_address += self.register_set.autoincrement.autoincrement();
        }
        self.data_port_reg = data;
        Ok(())
    }

    fn read_control_port(&mut self) -> Result<u32, ()> {
        self.address_setting_latch = false;
        let status = self.register_set.status.read();
        self.register_set.status.reset();
        Ok(status as u32)
    }

    fn write_control_port(&mut self, data: u16) -> Result<(), ()> {
        if data & VDP_CTRL_OPERATION_TYPE_MASK == VDP_CTRL_REGISTER_SET_MODE_MASK {
            self.set_register(data);
        } else {
            self.set_ram_access(data);
        }
        Ok(())
    }

    fn read_hv_counters_port(&mut self) -> Result<u32, ()> {
        Ok((self.v_counter << 8 | self.h_counter & 0xFF) as u32)
    }
}

impl<T> Vdp<T>
where
    T: BusVdp,
{
    fn set_register(&mut self, data: u16) {
        let register_id = (data & VDP_CTRL_REGISTER_ID_MASK).swap_bytes() as usize;
        let register_data = data as u8;
        self.register_set
            .set_register_by_id(register_id, register_data);
        debug!(
            "VDP: set register {:02X} to value {:02X}",
            register_id, register_data
        )
    }

    fn set_ram_access(&mut self, data: u16) {
        debug!("VDP: set_ram_access: receiving data: {:04X}", data);
        if !self.address_setting_latch {
            // first word
            self.address_setting_raw_word = 0; // clear msb
            self.address_setting_raw_word |= (data as u32) << 16;
            self.address_setting_latch = !self.address_setting_latch
        } else {
            // second word
            self.address_setting_raw_word &= 0xFFFF0000; // clear lsb
            self.address_setting_raw_word |= data as u32;
            self.address_setting_latch = !self.address_setting_latch
        }
        let ram_access_mode_mask = (((self.address_setting_raw_word >> 4) & 0xF) << 2)
            | ((self.address_setting_raw_word >> 30) & 0b11);
        debug!("VDP: ram_access_mode_mask: {:06b}", ram_access_mode_mask);
        let address = ((self.address_setting_raw_word & 0b11) << 14)
            | (self.address_setting_raw_word >> 16) & 0x3FFF;
        if self.address_setting_raw_word & 0x00000080 != 0 {
            // it is dma transfer mode
            let dma_mode_mask = ram_access_mode_mask >> 4;
            let reg_dma_mode = self.register_set.dma_source.dma_mode();
            let dma_enabled = self.register_set.mode_register.dma_enabled();
            if (dma_mode_mask == 0b10) && reg_dma_mode == DmaMode::BusToRam {
                self.dma_mode = Some(DmaMode::BusToRam);
                self.dma_run = dma_enabled;
            } else if (dma_mode_mask == 0b10) && (reg_dma_mode == DmaMode::FillRam) {
                self.dma_mode = Some(DmaMode::FillRam);
                self.dma_data_wait = true;
            } else if (dma_mode_mask == 0b11) && reg_dma_mode == DmaMode::CopyRam {
                self.dma_mode = Some(DmaMode::CopyRam);
                self.dma_run = dma_enabled;
            } else {
                panic!("VDP: write_control_port: unexpected dma mode bits sequence. dma_mode_mask = '{:02b}'\treg_dma_mod = '{}'", dma_mode_mask, reg_dma_mode);
            }
            self.dma_src_address = self.register_set.dma_source.src_address();
            self.dma_length = self.register_set.dma_lnegth.length();
            debug!("VDP: set dma mode '{}'", self.dma_mode.as_ref().unwrap());
        }
        // it is address set mode
        self.ram_access_mode = RamAccessMode::new((ram_access_mode_mask & 0xF) as u16);
        self.vdp_ram_address = address;

        debug!(
            "VDP: set ram access mode '{}' and address {:04X}",
            self.ram_access_mode, self.vdp_ram_address
        );
    }
}

use crate::Vdp;

pub trait VdpPorts {
    fn read_data_port(&self) -> u32;
    fn write_data_port(&mut self, data: u16);
    fn read_control_port(&self) -> u32;
    fn write_control_port(&mut self, data: u16);
}

impl VdpPorts for Vdp {
    fn read_data_port(&self) -> u32 {
        todo!()
    }

    fn write_data_port(&mut self, data: u16) {
        todo!()
    }

    fn read_control_port(&self) -> u32 {
        todo!()
    }

    fn write_control_port(&mut self, data: u16) {
        todo!()
    }
}

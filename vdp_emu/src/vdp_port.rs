use crate::Vdp;

pub trait VdpPorts {
    fn read(port_id: u32) -> u32;
    fn write(data: u32, port_id: u32);
}

impl VdpPorts for Vdp {
    fn read(port_id: u32) -> u32 {
        0
    }

    fn write(data: u32, port_id: u32) {
        todo!()
    }
}

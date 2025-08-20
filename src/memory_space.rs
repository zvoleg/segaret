use std::{cell::RefCell, rc::Rc};

use crate::{controller::Controller, signal_bus::SignalBus, vdp_emu::vdp_port::VdpPorts, ym2612::Ym2612Ports};

pub struct MemorySpace<T, Y> where T: VdpPorts, Y: Ym2612Ports {
    pub(crate) rom: Vec<u8>,
    pub(crate) m68k_ram: Vec<u8>,
    pub(crate) z80_ram: Vec<u8>,

    pub(crate) z80_bus_req: RefCell<bool>,
    pub(crate) z80_res_req: RefCell<bool>,

    pub(crate) io_area_read: [u8; 0x20],
    pub(crate) io_area_m68k: [u8; 0x20],

    pub(crate) vdp_ports: Rc<RefCell<T>>,
    pub(crate) ym2612_ports: Rc<RefCell<Y>>,

    pub(crate) controller_1: Rc<RefCell<Controller>>,
    pub(crate) controller_2: Rc<RefCell<Controller>>,

    pub(crate) signal_bus: Rc<RefCell<SignalBus>>,

    pub(crate) bank_register: RefCell<u16>,
}

impl<T,Y> MemorySpace<T, Y>
where
    T: VdpPorts,
    Y: Ym2612Ports,
{
    pub fn new(
        rom: Vec<u8>,
        vdp_ports: Rc<RefCell<T>>,
        ym2612_ports: Rc<RefCell<Y>>,
        controller_1: Rc<RefCell<Controller>>,
        controller_2: Rc<RefCell<Controller>>,
        signal_bus: Rc<RefCell<SignalBus>>,
    ) -> Self {
        let mut io_area_read = [0; 0x20];
        io_area_read[1] = 0x0090; // `setup version register
        Self {
            rom: rom,
            z80_ram: vec![0; 0x10000],  // $A00000	$A0FFFF
            m68k_ram: vec![0; 0x10000], // $FF0000	$FFFFFF

            z80_bus_req: RefCell::new(false),
            z80_res_req: RefCell::new(false),

            io_area_read: [0; 0x20],
            io_area_m68k: [0; 0x20],

            vdp_ports: vdp_ports,
            ym2612_ports: ym2612_ports,

            controller_1: controller_1,
            controller_2: controller_2,

            signal_bus: signal_bus,

            bank_register: RefCell::new(0),
        }
    }

    pub(crate) fn push_bank_register_bit(&self, data: u16) {
        let mut bank_register = *self.bank_register.borrow();
        bank_register = (bank_register << 1) | data & 0x01; // push single bit to the register end
        bank_register &= 0x01FF; // remain only 9 bits
        *self.bank_register.borrow_mut() = bank_register;
    }
}

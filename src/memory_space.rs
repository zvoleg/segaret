use std::{cell::RefCell, rc::Rc};

use crate::{controller::Controller, signal_bus::SignalBus, vdp_emu::vdp_port::VdpPorts};

pub struct MemorySpace<T: VdpPorts> {
    pub(crate) rom: Vec<u8>,
    pub(crate) m68k_ram: Vec<u8>,
    pub(crate) z80_ram: Vec<u8>,

    pub(crate) z80_bus_reg: RefCell<u32>,

    pub(crate) io_area_read: [u8; 0x20],
    pub(crate) io_area_m68k: [u8; 0x20],

    pub(crate) vdp_ports: Rc<RefCell<T>>,

    pub(crate) controller_1: Rc<RefCell<Controller>>,
    pub(crate) controller_2: Rc<RefCell<Controller>>,

    pub(crate) signal_bus: Rc<RefCell<SignalBus>>,
}

impl<T> MemorySpace<T>
where
    T: VdpPorts,
{
    pub fn new(
        rom: Vec<u8>,
        vdp_ports: Rc<RefCell<T>>,
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

            z80_bus_reg: RefCell::new(0),

            io_area_read: [0; 0x20],
            io_area_m68k: [0; 0x20],

            vdp_ports: vdp_ports,

            controller_1: controller_1,
            controller_2: controller_2,

            signal_bus: signal_bus,
        }
    }

    pub(crate) fn read_ptr(&self, amount: u32, ptr: *const u8) -> u32 {
        unsafe {
            match amount {
                1 => *ptr as u32,
                2 => (*(ptr as *const u16)).to_be() as u32,
                4 => (*(ptr as *const u32)).to_be() as u32,
                _ => panic!("Bus: read: wrong size"),
            }
        }
    }

    pub(crate) fn write_ptr(&self, data: u32, amount: u32, ptr: *mut u8) {
        unsafe {
            match amount {
                1 => *ptr = data as u8,
                2 => *(ptr as *mut _ as *mut u16) = (data as u16).to_be(),
                4 => *(ptr as *mut _ as *mut u32) = data.to_be(),
                _ => panic!("Bus: write: wrong size"),
            }
        }
    }
}

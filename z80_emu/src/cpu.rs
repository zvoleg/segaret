use std::rc::Rc;

use log::{debug, info};

use crate::{bus::BusZ80, opcode_table_generator::tables::{cb_opcode_table, dd_opcode_table, ddcb_opcode_table, ed_opcode_table, fd_opcode_table, fdcb_opcode_table, opcode_table}, register_set::{Register, RegisterSet, RegisterType}, Size};

enum IntMode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
}

impl IntMode {
    fn from_u32(d: u32) -> Self {
        match d {
            0 => IntMode::Mode0,
            1 => IntMode::Mode1,
            2 => IntMode::Mode2,
            _ => panic!("Unexpected value for IntMode"),
        }
    }
}

pub struct Z80<T: BusZ80> {
    pub(crate) register_set: RegisterSet,
    pub  program_counter: u16,
    pub(crate) current_opcode: u32,
    interrupt_mode: IntMode,
    iff1: u8,
    iff2: u8,

    bus: Option<Rc<T>>,
}

const NMI_VECTOR: u16 = 0x0066;
const INT_MODE1_VECTOR: u16 = 0x0038;

impl<T> Z80<T>
where
    T: 'static + BusZ80,
{
    pub fn new() -> Self {
        Self {
            register_set: RegisterSet::new(),
            program_counter: 0,
            current_opcode: 0,
            interrupt_mode: IntMode::Mode0,
            iff1: 0,
            iff2: 0,
            bus: None,
        }
    }

    pub fn set_bus(&mut self, bus: Rc<T>) {
        self.bus = Some(bus)
    }

    pub fn restart(&mut self) {
         self.program_counter = 0;
         debug!("Z80: restart")
    }

    pub fn nmi(&mut self) {
        self.push(self.program_counter, Size::Word).unwrap();
        self.program_counter = NMI_VECTOR;
        self.iff2 = self.iff1;
        self.iff1 = 0;
    }

    pub fn int(&mut self, data: u16) {
        if self.iff1 == 1 {
            self.reset_iff();
            self.push(self.program_counter, Size::Word).unwrap();
            match self.interrupt_mode {
                IntMode::Mode0 => {}, // TODO caller should pass a single byte instruction RES or CALL
                IntMode::Mode1 => self.program_counter = INT_MODE1_VECTOR,
                IntMode::Mode2 => {}, // TODO hsb of vector's address is I register, lsb of vector's address is value from caller
            }
        }
    }

    pub fn clock(&mut self) {
        let pc = self.program_counter;
        self.current_opcode = 0;
        let mut opcode = self.read_pc(Size::Byte);
        self.current_opcode |= opcode as u32;
        let opcodes = match opcode {
            0xED => {
                opcode = self.read_pc(Size::Byte);
                self.current_opcode <<= 8;
                self.current_opcode |= opcode as u32;
                ed_opcode_table()
            },
            0xCB => {
                opcode = self.read_pc(Size::Byte);
                self.current_opcode <<= 8;
                self.current_opcode |= opcode as u32;
                cb_opcode_table()
            },
            0xDD => {
                opcode = self.read_pc(Size::Byte);
                self.current_opcode <<= 8;
                self.current_opcode |= opcode as u32;
                match opcode {
                    0xCB => {
                        opcode = self.read_pc(Size::Byte);
                        self.current_opcode <<= 8;
                        self.current_opcode |= opcode as u32;
                        opcode = self.read_pc(Size::Byte);
                        self.current_opcode <<= 8;
                        self.current_opcode |= opcode as u32;
                        ddcb_opcode_table()
                    }
                    _ => dd_opcode_table(),
                }
            },
            0xFD => {
                opcode = self.read_pc(Size::Byte);
                self.current_opcode <<= 8;
                self.current_opcode |= opcode as u32;
                match opcode {
                    0xCB => {
                        opcode = self.read_pc(Size::Byte);
                        self.current_opcode <<= 8;
                        self.current_opcode |= opcode as u32;
                        opcode = self.read_pc(Size::Byte);
                        self.current_opcode <<= 8;
                        self.current_opcode |= opcode as u32;
                        fdcb_opcode_table()
                    }
                    _ => fd_opcode_table(),
                }
            }
            _ => opcode_table()
        };
        let operation = &opcodes[opcode as usize];
        let mut operands = Vec::new();
        if let Some(am) = &operation.dst_am {
            let operand = am.fetch(self);
            operands.push(operand);
        }
        if let Some(am) = &operation.src_am {
            let operand = am.fetch(self);
            operands.push(operand);
        }
        operation.instruction.execute(self, operands);
        debug!("{:04X}: {}", pc, operation);
        debug!("{}", self.register_set);
    }

    fn write_interrupt_vector(&mut self, data: u8) {
        self.register_set.interrupt_vector = data;
    }

    pub(crate) fn push(&mut self, data: u16, size: Size) -> Result<(), ()> {
        let stack_pointer = self.register_set.get_stack_ptr().wrapping_sub(size as u16);
        self.register_set.set_stack_ptr(stack_pointer);
        self.bus
            .as_ref()
            .unwrap()
            .write(data, stack_pointer, size as u32)?;
        Ok(())
    }

    pub(crate) fn pop(&mut self, size: Size) -> Result<u16, ()> {
        let stack_pointer = self.register_set.get_stack_ptr();
        let data = self.bus.as_ref().unwrap().read(stack_pointer, size as u32)?;
        self.register_set.set_stack_ptr(stack_pointer.wrapping_add(size as u16));
        Ok(data)
    }

    pub(crate) fn increment_pc(&mut self, size: Size) {
        self.program_counter = self.program_counter.wrapping_add(size as u16);
    }

    pub(crate) fn read_pc(&mut self, size: Size) -> u16 {
        let data = self
            .bus
            .as_ref()
            .unwrap()
            .read(self.program_counter, size as u32)
            .unwrap();
        self.increment_pc(size);
        data
    }

    pub(crate) fn bus_share(&self) -> Rc<T> {
        self.bus.as_ref().unwrap().clone()
    }
    
    pub(crate) fn set_interrupt_mode(&mut self, interrupt_mode: u32) {
        self.interrupt_mode = IntMode::from_u32(interrupt_mode);
    }

    pub(crate) fn restore_iff(&mut self) {
        self.iff1 = self.iff2;
    }

    pub(crate) fn reset_iff(&mut self) {
        self.iff1 = 0;
        self.iff2 = 0;
    }

    pub(crate) fn set_iff(&mut self) {
        self.iff1 = 1;
        self.iff2 = 1;
    }
    
    pub fn cpm_bdos(&mut self) {
        let mut buff = vec![];
        match self.register_set.read_register(Register::General(RegisterType::C), Size::Byte) {
            2 => {
                // output character in register E
                let e_val = self.register_set.read_register(Register::General(RegisterType::E), Size::Byte);
                buff.push(e_val as u8 as char);
            },
            9 => {
                // output a string at register DE until '$'
                let mut addr = self.register_set.read_register(Register::General(RegisterType::DE), Size::Word);
                loop {
                    let c = self.bus_share().read(addr, Size::Byte as u32).unwrap() as u8 as char;
                    addr = (addr + 1) & 0xFFFF;
                    if c != '$' {
                        buff.push(c);
                    }
                    else {
                        break;
                    }
                }
            },
            _ => {
                panic!("Unknown CP/M call {}!", self.register_set.read_register(Register::General(RegisterType::C), Size::Byte));
            }
        }
        info!("{}", buff.iter().collect::<String>());
        self.program_counter = self.pop(Size::Word).unwrap();
    }
}

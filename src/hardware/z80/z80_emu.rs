use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::hardware::{sign_extend, Size, is_negate};

use super::{Z80Bus, Instruction, Operand, AmType, Location, Register};

lazy_static! {
    static ref EXTENDING_TO_TWO_BYTES: HashSet<u32> = HashSet::from([0xCB, 0xDD, 0xED, 0xFDu32]);
    static ref EXTENDING_TO_FOUR_BYTES: HashSet<u32> = HashSet::from([0xDDCB, 0xFDCBu32]);

    static ref OPCODE_TABLE: Vec<Instruction> = vec![];
}

enum Status {
    S = 7,
    Z = 6,
    H = 4,
    PV = 2,
    N = 1,
    C = 0,
}

pub struct Z80Emu {
    pc: u16,
    sp: u16,

    i: u8, // interrupt page address, stores high-order 8 bits of interrupt address
    r: u8, // memory refresh register (counter), 7 bits increments after each instruction fetch, 8 bit is programmed, resulting from an LD R, A instruction

    af: u16, // acc and flag
    af_: u16, // acc' and flag'
    
    bc: u16,
    de: u16,
    hl: u16,

    bc_: u16,
    de_: u16,
    hl_: u16,

    ix: u16, // index register X
    iy: u16, // index register Y

    iff1: bool,
    iff2: bool,

    //internal
    curr_opcode: u32,
    curr_opcode_size: i32,
    instruction_size: Size,
    src_operand: Option<Operand>,
    dst_operand: Option<Operand>,

    bus: *mut dyn Z80Bus,
}

impl Z80Emu {
    fn clock(&mut self) {
        self.fetch_current_opcode();
        let instruction = &OPCODE_TABLE[self.curr_opcode as u8 as usize];

        self.instruction_size = instruction.size;
        self.src_operand = match &instruction.src_am {
            Some(am_type) => Some(self.call_am(am_type)),
            None => None,
        };
        self.dst_operand = match &instruction.dst_am {
            Some(am_type) => Some(self.call_am(am_type)),
            None => None,
        };
        (instruction.handler)(self);
    }

    fn fetch_current_opcode(&mut self) {
        // excluded bytes CB, DD, ED, FD, DDCB, FDCB
        let mut opcode = self.read_pc_and_increment() as u32;
        let mut byte_counter = 1;
        if EXTENDING_TO_TWO_BYTES.contains(&opcode) {
            let additional_byte = self.read_pc_and_increment();
            opcode = (opcode << 8) | additional_byte as u32;
            byte_counter += 1;
            if EXTENDING_TO_FOUR_BYTES.contains(&opcode) {
                let additional_byte = self.read_pc_and_increment();
                opcode = (opcode << 8) | additional_byte as u32;
                byte_counter += 1;
                let additional_byte = self.read_pc_and_increment();
                opcode = (opcode << 8) | additional_byte as u32;
                byte_counter += 1;
            }
        }
        self.curr_opcode = opcode;
        self.curr_opcode_size = byte_counter;
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    fn read_pc_and_increment(&mut self) -> u8 {
        let data = self.read_memory(self.pc, Size::Byte) as u8;
        self.increment_pc();
        data
    }

    fn call_am(&mut self, am_type: &AmType) -> Operand {
        match am_type {
            AmType::Imm => self.immediate_am(),
            AmType::ImmExt => self.immediate_extended_am(),
            AmType::PageZero(addr) => self.modified_page_zero_am(*addr),
            AmType::Relative => self.relative_am(),
            AmType::Extended => self.extended_am(),
            AmType::Indexed(reg) => self.indexed_am(*reg),
            AmType::Register(reg) => self.register_am(*reg),
            AmType::Implied => self.implied_am(),
            AmType::RegIndirect(reg) => self.register_indirect_am(*reg),
            AmType::BitAddr(offset) => self.bit_am(*offset),
        }
    }

    fn write(&mut self, location: Location, data: u16, size: Size) {
        match location {
            Location::Register(reg) => self.write_register(reg, data, size),
            Location::Memory(addr) => self.write_memory(addr, data, size),
            Location::Const => panic!("Z80::write: can't write into Location::Const"),
        }
    }

    fn read_memory(&mut self, address: u16, size: Size) -> u16 {
        unsafe {
            (*self.bus).read(address, size)
        }
    }

    fn write_memory(&mut self, address: u16, data: u16, size: Size) {
        unsafe {
            (*self.bus).write(address, data, size);
        }
    }

    fn read_register(&self, register: Register) -> u16 {
        match register {
            Register::B => self.bc >> 8,
            Register::C => self.bc & 0xFF,
            Register::D => self.de >> 8,
            Register::E => self.de & 0xFF,
            Register::H => self.hl >> 8,
            Register::L => self.hl & 0xFF,
            Register::BC => self.bc,
            Register::DE => self.de,
            Register::HL => self.hl,
            Register::IX => self.ix,
            Register::IY => self.iy,
            Register::SP => self.sp,
        }
    }

    fn write_register(&mut self, register: Register, data: u16, size: Size) {
        let write_high = |reg, data| -> u16 {
            let reg_data = reg & 0x00FF;
            (data << 8) | reg_data
        };
        let write_low = |reg, data| -> u16 {
            let reg_data = reg & 0xFF00;
            data | reg_data
        };
        match register {
            Register::B => self.bc = write_high(self.bc, data),
            Register::C => self.bc = write_low(self.bc, data),
            Register::D => self.de = write_high(self.de, data),
            Register::E => self.de = write_low(self.de, data),
            Register::H => self.hl = write_high(self.hl, data),
            Register::L => self.hl = write_low(self.hl, data),
            Register::BC => self.bc = data,
            Register::DE => self.de = data,
            Register::HL => self.hl = data,
            Register::IX => self.ix = data,
            Register::IY => self.iy = data,
            Register::SP => self.sp = data,
        }
    }

    fn get_flag(&self, status: Status) -> bool {
        let mask = 1 << status as u16;
        self.af & mask != 0
    }

    fn set_flag(&mut self, status: Status, set: bool) {
        let mask = 1 << status as u16;
        if set {
            self.af = self.af | mask;
        } else {
            self.af = self.af & !mask;
        }
    }

    fn get_accumulator(&self) -> u16 {
        self.af >> 8
    }

    fn set_accumulator(&mut self, data: u16) {
        self.af &= 0x00FF;
        self.af |= data << 8;
    }
}

/* Addressing modes */
impl Z80Emu {
    fn immediate_am(&mut self) -> Operand {
        let operand = self.read_pc_and_increment();
        Operand::constant_operand(operand as u16)
    }

    fn immediate_extended_am(&mut self) -> Operand {
        let low_order_bits = self.read_pc_and_increment();
        let mut data = low_order_bits as u16;

        let high_order_bits = self.read_pc_and_increment();
        data |= (high_order_bits as u16) << 8;

        Operand::constant_operand(data)
    }

    fn modified_page_zero_am(&mut self, address: u16) -> Operand {
        Operand::memory_operand(address, 0)
    }

    fn relative_am(&mut self) -> Operand {
        // addressing mode for the jump instructions, do not need to read data by the calculated offset
        let byte = self.read_pc_and_increment();
        let offset = sign_extend(byte as u32, Size::Byte) as u16;
        let address = self.pc.wrapping_add(offset);
        
        Operand::memory_operand(address, 0)
    }

    // fetched data is address of operand or address for jump instruction
    fn extended_am(&mut self) -> Operand {
        let address = self.read_memory(self.pc, Size::Word);
        self.increment_pc();
        self.increment_pc();
        
        let data = self.read_memory(address, Size::Byte);
        Operand::memory_operand(address, data)
    }

    fn indexed_am(&mut self, register: Register) -> Operand {
        let byte= if self.curr_opcode_size == 4 {
            (self.curr_opcode >> 8) as u8
        } else {
            self.read_pc_and_increment()
        };

        let register_value = self.read_register(register);
        let displacement = sign_extend(byte as u32, Size::Byte) as u16;
        let address = register_value.wrapping_add(displacement);
        let data = self.read_memory(address, self.instruction_size);
        Operand::memory_operand(address, data)
    }

    fn register_am(&mut self, register: Register) -> Operand {
        let data = self.read_register(register);
        Operand::register_operand(register, data)
    }

    fn implied_am(&mut self) -> Operand {
        Operand::constant_operand(0)
    }

    fn register_indirect_am(&mut self, register: Register) -> Operand {
        let address = self.read_register(register);
        let data = self.read_memory(address, self.instruction_size);
        Operand::memory_operand(address, data)
    }

    fn bit_am(&mut self, bit_offseet: u16) -> Operand {
        Operand::constant_operand(bit_offseet)
    }
}

/* Instruction set */
#[allow(non_snake_case)]
impl Z80Emu {
    // load data from src to dst (load 8 or 16 bits)
    fn LD(&mut self) {
        let src_data = self.src_operand.as_ref().unwrap().data;
        let dst_location = self.dst_operand.as_ref().unwrap().location;

        self.write(dst_location, src_data, self.instruction_size)
    }

    // push data on the stack
    fn PUSH(&mut self) {
        self.sp -= 2;

        let address = self.sp;
        let data = self.src_operand.as_ref().unwrap().data;
        self.write_memory(address, data, self.instruction_size);
    }

    // pop data from the stack
    fn POP(&mut self) {
        let address = self.pc;
        let data = self.read_memory(address, self.instruction_size);

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data, self.instruction_size);

        self.pc += 2;
    }

    // exchange data between registers
    fn EX(&mut self) {
        let reg_a = self.src_operand.as_ref().unwrap().location;
        let data_a = self.src_operand.as_ref().unwrap().data;
        let reg_b = self.dst_operand.as_ref().unwrap().location;
        let data_b = self.dst_operand.as_ref().unwrap().data;

        self.write(reg_a, data_b, self.instruction_size);
        self.write(reg_b, data_a, self.instruction_size);
    }

    // exchange all 2-bytes registers between its pair
    fn EXX(&mut self) {
        std::mem::swap(&mut self.bc, &mut self.bc_);
        std::mem::swap(&mut self.de, &mut self.de_);
        std::mem::swap(&mut self.hl, &mut self.hl_);
    }

    // transfer data from memory to memory
    fn LDI(&mut self) {
        let data = self.read_memory(self.hl, Size::Byte);
        self.write_memory(self.de, data, Size::Byte);

        self.hl += 1;
        self.de += 1;
        self.bc -= 1;

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::PV, self.bc - 1 != 0);
    }

    fn LDIR(&mut self) {
        self.LDI();

        if self.bc - 1 != 0 {
            self.pc -= 2;
        }
    }

    fn LDD(&mut self) {
        let data = self.read_memory(self.hl, Size::Byte);
        self.write_memory(self.de, data, Size::Byte);

        self.hl -= 1;
        self.de -= 1;
        self.bc -= 1;

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::PV, self.bc - 1 != 0);
    }

    fn LDDR(&mut self) {
        self.LDD();

        if self.bc - 1 != 0 {
            self.pc -= 2;
        }
    }

    fn CPI(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc - data;

        self.hl += 1;
        self.bc -= 1;

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::N, true);

    }

    fn CPIR(&mut self) {

    }

    fn CPD(&mut self) {

    }

    fn CPDR(&mut self) {

    }

    fn ADD(&mut self) {

    }

    fn ADC(&mut self) {

    }

    fn SUB(&mut self) {

    }

    fn SBC(&mut self) {

    }

    fn AND(&mut self) {

    }

    fn OR(&mut self) {

    }

    fn XOR(&mut self) {

    }

    fn CP(&mut self) {

    }

    fn INC(&mut self) {

    }

    // BCD addition
    fn DEC(&mut self) {

    }

    // Inverts accumulator (one's complement)
    fn CPL(&mut self) {

    }

    // Inverts accumulator (two's complement)
    fn NEG(&mut self) {

    }

    // Inverts cary flag in F register
    fn CCF(&mut self) {

    }

    // Set cary flag in F register
    fn SCF(&mut self) {

    }

    fn NOP(&mut self) {

    }

    fn HALT(&mut self) {

    }

    // disable the maskable interrupt
    fn DI(&mut self) {

    }

    // enable interrupt sets
    fn EI(&mut self) {

    }

    // setup interrupt mode
    fn IM(&mut self) {

    }

    fn RLCA(&mut self) {

    }

    fn RLA(&mut self) {

    }

    fn RRCA(&mut self) {

    }

    fn RRA(&mut self) {

    }

    fn RLC(&mut self) {

    }

    fn RL(&mut self) {

    }

    fn RRC(&mut self) {

    }

    fn RR(&mut self) {

    }

    fn SLA(&mut self) {

    }

    fn SRA(&mut self) {

    }

    fn SRL(&mut self) {

    }

    fn RLD(&mut self) {

    }

    fn RRD(&mut self) {

    }

    fn BIT(&mut self) {

    }

    fn SET(&mut self) {

    }

    fn RES(&mut self) {

    }

    fn JP(&mut self) {

    }

    fn DJNZ(&mut self) {

    }

    fn CALL(&mut self) {

    }

    fn RET(&mut self) {

    }

    fn RETI(&mut self) {

    }

    fn RETN(&mut self) {

    }

    fn RST(&mut self) {

    }

    fn IN(&mut self) {

    }

    fn INI(&mut self) {

    }

    fn INIR(&mut self) {

    }

    fn IND(&mut self) {

    }

    fn INDR(&mut self) {

    }

    fn OUT(&mut self) {

    }

    fn OUTI(&mut self) {

    }

    fn OTIR(&mut self) {

    }

    fn OUTD(&mut self) {

    }

    fn OTDR(&mut self) {

    }
} 
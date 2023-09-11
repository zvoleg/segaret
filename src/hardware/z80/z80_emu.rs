use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::hardware::{sign_extend, Size};

use super::{Z80Bus, Instruction, Operand, AmType, Location, Register};

lazy_static! {
    static ref EXTENDING_TO_TWO_BYTES: HashSet<u32> = HashSet::from([0xCB, 0xDD, 0xED, 0xFDu32]);
    static ref EXTENDING_TO_FOUR_BYTES: HashSet<u32> = HashSet::from([0xDDCB, 0xFDCBu32]);

    static ref OPCODE_TABLE: Vec<Instruction> = vec![];
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
    src_operand: Option<Operand>,
    dst_operand: Option<Operand>,

    bus: *mut dyn Z80Bus,
}

impl Z80Emu {
    fn clock(&mut self) {
        self.fetch_current_opcode();
        let instruction = &OPCODE_TABLE[self.curr_opcode as u8 as usize];

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
            AmType::BitAddr => self.bit_am(),
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
        let opcode_data = self.curr_opcode as u8;
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
        let data = self.read_memory(address, Size::Byte);
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
        let data = self.read_memory(address, Size::Byte);
        Operand::memory_operand(address, data)
    }

    fn bit_am(&mut self) -> Operand {
        // TODO used for bit manipulation instructions and bit number contains in opcode,
        // may be it will be calculates in handlers
        Operand::constant_operand(0)
    }
}

/* Instruction set */
#[allow(non_snake_case)]
impl Z80Emu {
    // load data from src to dst (load 8 or 16 bits)
    fn LD(&mut self) {
        // src value u8
        // destination pointer u8
        // match self.instruction.size {
        //     Size::Byte => {
        //         let src_value = *(self.src_ptr as *const u8);
        //         unsafe {
        //             *(self.dst_ptr as *mut u8) = src_value;
        //         }
        //     },
        //     Size::Word => {
        //         let src_value = *(self.src_ptr as *const u16);
        //         unsafe {
        //             *(self.dst_ptr as *mut u16) = src_value;
        //         }
        //     },
        //     Size::Long => (),
        // }
    }

    // push data on the stack
    fn PUSH(&mut self) {
        // at first decrements sp
        // save high-order bits of selected register
        // decrements sp again
        // save low-order bits of selected register
    }

    // pop data from the stack
    fn POP(&mut self) {
        // save data from stack to low-order bits of selected register
        // increment sp
        // save data from stack to high-order bits of selected register
    }

    // exchange data between registers
    fn EX(&mut self) {

    }

    // exchange all 2-bytes registers between its pair
    fn EXX(&mut self) {

    }

    // transfer data from memory to memory
    fn LDI(&mut self) {

    }

    fn LDIR(&mut self) {

    }

    fn LDD(&mut self) {

    }

    fn LDDR(&mut self) {

    }

    fn CPI(&mut self) {

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
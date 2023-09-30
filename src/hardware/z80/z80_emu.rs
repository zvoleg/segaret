use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::hardware::{sign_extend, Size, is_negate, is_zero, msb_is_set, get_msb};

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

    fn stack_push(&mut self, data: u16, size: Size) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_memory(self.sp, data, size);
    }

    fn stack_pop(&mut self, size: Size) -> u16 {
        let data = self.read_memory(self.sp, size);
        self.sp = self.sp.wrapping_add(2);
        data
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
        let data = self.src_operand.as_ref().unwrap().data;
        self.stack_push(data, self.instruction_size)
    }

    // pop data from the stack
    fn POP(&mut self) {
        let data = self.stack_pop(self.instruction_size);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data, self.instruction_size);
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

        self.hl = self.hl.wrapping_add(1);
        self.de = self.de.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

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

        self.hl = self.hl.wrapping_sub(1);
        self.de = self.de.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

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

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);
    }

    fn CPIR(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);

        if self.bc - 1 != 0 && res != 0 {
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    fn CPD(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);
    }

    fn CPDR(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);

        if self.bc - 1 != 0 && res != 0 {
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    fn ADD(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_add(src_operand);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn ADC(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let carry_in = self.get_flag(Status::C);
        let mut carry = 0;
        if carry_in {
            carry = 1;
        }
     
        let result = dst_operand.wrapping_add(src_operand).wrapping_add(carry);
        self.write(dst_location, result, self.instruction_size);

        
        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };
        let carry = ((result >> carry_bit_offset) & 1) != 0;
        
        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = ((dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb)) || carry_in != carry;

        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SUB(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_sub(src_operand);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SBC(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let carry_in = self.get_flag(Status::C);
        let mut carry = 0;
        if carry_in {
            carry = 1;
        }
     
        let result = dst_operand.wrapping_sub(src_operand).wrapping_sub(carry);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn AND(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand & dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn OR(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand | dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn XOR(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand ^ dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn CP(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;

        let result = dst_operand.wrapping_sub(src_operand);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, true);
        self.set_flag(Status::C, carry);
    }

    fn INC(&mut self) {
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_add(1);
        self.write(dst_location, dst_operand, self.instruction_size);

        match self.instruction_size {
            Size::Byte => {
                self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
                self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
                self.set_flag(Status::H, (result >> 3) & 1 == 1);
                self.set_flag(Status::PV, dst_operand == 0x7F);
                self.set_flag(Status::N, false);
            },
            _ => panic!("Z80::INC: unexpected instruction size"),
        }
    }

    // BCD addition
    fn DEC(&mut self) {
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_sub(1);
        self.write(dst_location, dst_operand, self.instruction_size);

        match self.instruction_size {
            Size::Byte => {
                self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
                self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
                self.set_flag(Status::H, (result >> 4) & 1 == 1);
                self.set_flag(Status::PV, dst_operand == 0x80);
                self.set_flag(Status::N, true);
            },
            _ => panic!("Z80::INC: unexpected instruction size"),
        }
    }

    // Inverts accumulator (one's complement)
    fn CPL(&mut self) {
        let result = !self.get_accumulator();
        self.set_accumulator(result);

        self.set_flag(Status::H, true);
        self.set_flag(Status::N, true);
    }

    // Inverts accumulator (two's complement)
    fn NEG(&mut self) {
        let acc = self.get_accumulator();
        let result = (!acc).wrapping_add(1);
        self.set_accumulator(result);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, (result >> 4) & 1 == 1);
        self.set_flag(Status::PV, acc == 0x80);
        self.set_flag(Status::N, true);
        self.set_flag(Status::C, acc != 0);
    }

    // Inverts cary flag in F register
    fn CCF(&mut self) {
        let carry = self.get_flag(Status::C);
        self.set_flag(Status::H, carry);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, !carry);
    }

    // Set cary flag in F register
    fn SCF(&mut self) {
        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, true);
    }

    fn NOP(&mut self) {

    }

    fn HALT(&mut self) {
        self.pc = self.pc - 1;
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
        let acc = self.get_accumulator();
        
        let msb = acc >> 7;
        let carry = msb == 1;
        
        let result = (acc << 1) | msb;
        self.set_accumulator(result);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLA(&mut self) {
        let acc = self.get_accumulator();

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let msb = acc >> 7;
        let carry = msb == 1;

        let result = (acc << 1) | poped_carry;
        self.set_accumulator(acc);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRCA(&mut self) {
        let acc = self.get_accumulator();
        
        let lsb: u16 = acc & 1;
        let carry = lsb == 1;
        
        let result = (acc >> 1) | (lsb << 7);
        self.set_accumulator(result);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRA(&mut self) {
        let acc = self.get_accumulator();

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let lsb = acc & 1;
        let carry = lsb == 1;

        let result = (acc >> 1) | (poped_carry << 7);
        self.set_accumulator(acc);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLC(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        
        let msb = data >> 7;
        let carry = msb == 1;
        
        let result = (data << 1) | msb;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RL(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let msb = data >> 7;
        let carry = msb == 1;

        let result = (data << 1) | poped_carry;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRC(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let lsb: u16 = data & 1;
        let carry = lsb == 1;
        
        let result = (data >> 1) | (lsb << 7);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RR(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let lsb = data & 1;
        let carry = lsb == 1;

        let result = (data >> 1) | (poped_carry << 7);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SLA(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let msb = data >> 7;
        let carry = msb == 1;

        let result = data << 1;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SRA(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let msb = get_msb(data as u32, self.instruction_size);
        let lsb = data & 1;
        let carry = lsb == 1;

        let msb_offste = match self.instruction_size {
            Size::Byte => 7,
            Size::Word => 15,
            Size::Long => panic!("Z80::SRA: unexpected instruction size")
        };

        let result = (data >> 1) | ((msb as u16) << msb_offste);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SRL(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let lsb = data & 1;
        let carry = lsb == 1;

        let result = data >> 1;
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLD(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let acc = self.get_accumulator();
        
        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc & 0x0F;
        
        let data_res = (data_low_bits << 4) | acc_low_bits;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data_res, self.instruction_size);

        let acc_res = (acc & 0xF0) | data_high_bits;
        self.set_accumulator(acc_res);

        self.set_flag(Status::S, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::N, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::H, false);
        self.set_flag(Status::PV, acc_res % 2 == 0);
        self.set_flag(Status::N, false);
    }

    fn RRD(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let acc = self.get_accumulator();
        
        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc & 0x0F;
        
        let data_res = (acc_low_bits << 4) | data_high_bits;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data_res, self.instruction_size);

        let acc_res = (acc & 0xF0) | data_low_bits;
        self.set_accumulator(acc_res);

        self.set_flag(Status::S, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::N, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::H, false);
        self.set_flag(Status::PV, acc_res % 2 == 0);
        self.set_flag(Status::N, false);
    }

    fn BIT(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data & (1 << bit_offset) == 0;

        self.set_flag(Status::Z, result);
        self.set_flag(Status::H, true);
        self.set_flag(Status::N, false);
    }

    fn SET(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data | (1 << bit_offset);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);
    }

    fn RES(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data & !(1 << bit_offset);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);
    }

    fn JP(&mut self) {
        let address = self.dst_operand.as_ref().unwrap().data;
        self.pc = address;
    }

    fn JR(&mut self) {
        let condition = match self.curr_opcode {
            0x18 => true, // unconditional branching
            0x38 => self.get_flag(Status::C),
            0x30 => !self.get_flag(Status::C),
            0x28 => self.get_flag(Status::Z),
            0x20 => !self.get_flag(Status::Z),
            _ => panic!("Z80::JR: unsupported opcode byte for condition selecting"),
        };

        if condition {
            let address_ofset = self.dst_operand.as_ref().unwrap().data;
            self.pc = self.pc.wrapping_add(address_ofset);
        }
    }

    fn DJNZ(&mut self) {
        let mut reg_b = self.read_register(Register::B);
        reg_b = reg_b.wrapping_sub(1);
        self.write_register(Register::B, reg_b, Size::Byte);

        if reg_b != 0 {
            let address_offset = self.dst_operand.as_ref().unwrap().data;
            self.pc = self.pc.wrapping_add(address_offset);
        }
    }

    fn CALL(&mut self) {
        let condition = if let Some(condition_bits) = self.dst_operand.as_ref() {
            match condition_bits.data {
                0b000 => !self.get_flag(Status::Z),
                0b001 => self.get_flag(Status::Z),
                0b010 => !self.get_flag(Status::C),
                0b011 => self.get_flag(Status::C),
                0b100 => !self.get_flag(Status::PV),
                0b101 => self.get_flag(Status::PV),
                0b110 => !self.get_flag(Status::S),
                0b111 => self.get_flag(Status::S),
                _ => panic!("Z80::CALL: unexpected bit pattern for condition determination")
            }
        } else {
            true
        };

        if condition {
            self.stack_push(self.pc, Size::Word);
            
            let address = self.dst_operand.as_ref().unwrap().data;
            self.pc = address;
        }
    }

    fn RET(&mut self) {
        let condition = if let Some(condition_bits) = self.dst_operand.as_ref() {
            match condition_bits.data {
                0b000 => !self.get_flag(Status::Z),
                0b001 => self.get_flag(Status::Z),
                0b010 => !self.get_flag(Status::C),
                0b011 => self.get_flag(Status::C),
                0b100 => !self.get_flag(Status::PV),
                0b101 => self.get_flag(Status::PV),
                0b110 => !self.get_flag(Status::S),
                0b111 => self.get_flag(Status::S),
                _ => panic!("Z80::CALL: unexpected bit pattern for condition determination")
            }
        } else {
            true
        };

        if condition {
            let address = self.stack_pop(Size::Word);
            self.pc = address;
        }
    }

    fn RETI(&mut self) {
        let address = self.stack_pop(Size::Word);
        self.pc = address;

        // maybe there need to pop the active interrupt
    }

    fn RETN(&mut self) {
        let address = self.stack_pop(Size::Word);
        self.pc = address;
        self.iff1 = self.iff2;
    }

    fn RST(&mut self) {
        self.stack_push(self.pc, Size::Word);
        let address = self.dst_operand.as_ref().unwrap().data;
        self.pc = address;
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
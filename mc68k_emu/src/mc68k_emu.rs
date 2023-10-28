extern crate lazy_static;

use std::fmt;

use hardware::{ get_msb, is_negate, is_zero, msb_is_set, sign_extend, Size };
use disassembler::Disassembler;

use crate::instruction_set;
use crate::instruction_set::instruction_meta_data_types::*;
use crate::instruction_set::InstructionProcess;


use super::Location;
use super::LocationType;
use super::Mc68kBus;
use super::RegisterType;
use super::addressing_mode::{AddrMode, AddrModeType};
use super::vector_table::VectorTable;
use super::instruction_set::Instruction;
use super::Condition;


enum Status {
    X = 4,
    N = 3,
    Z = 2,
    V = 1,
    C = 0,
}

#[derive(PartialEq)]
enum Mode {
    Supervisor,
    UserMode,
}

pub struct Mc68k {
    opcode_table: Vec<Box<dyn InstructionProcess>>,
    vector_table: VectorTable,

    reg: [u32; 17], // idx 15 and 16 are ssp and usp
    pub(crate)  pc: u32,

    sr: u16,
    mode: Mode, // user/supervisor

    clock_counter: u32,
    
    pended_interrupt_level: Option<usize>,

    pub(in crate) instruction: Box<dyn InstructionProcess>,
    current_addr_mode: AddrMode,
    ea_location: Location,
    ea_operand: u32,

    bus: *mut dyn Mc68kBus,
    disassembler: Disassembler,
}

impl fmt::Display for Mc68k {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut cpu_condition_buffer = vec![String::from("data register:\n")];
        for r in 0..4 {
            cpu_condition_buffer.push(format!("D{}: 0x{:08X}", r, self.reg[r]));
            cpu_condition_buffer.push(String::from("\t"));
            cpu_condition_buffer.push(format!("D{}: 0x{:08X}", r + 4, self.reg[r + 4]));
            cpu_condition_buffer.push(String::from("\n"));
        }
        cpu_condition_buffer.push(String::from("address register:\n"));
        for r in 0..4 {
            let r = r + 8;
            cpu_condition_buffer.push(format!("A{}: 0x{:08X}", r - 8, self.reg[r]));
            cpu_condition_buffer.push(String::from("\t"));
            cpu_condition_buffer.push(format!("A{}: 0x{:08X}", r + 4 - 8, self.reg[r + 4]));
            cpu_condition_buffer.push(String::from("\n"));
        }
        cpu_condition_buffer.push(String::from("TTSM0III___XNZVC\n"));
        cpu_condition_buffer.push(format!("{:016b}\n", self.sr));
        write!(f, "{}", cpu_condition_buffer.join(""))
    }
}

#[allow(non_snake_case)]
impl Mc68k {
    pub fn init(bus: *mut dyn Mc68kBus, rom_ptr: *const u8, disassembler: Disassembler) -> Self {
        let vector_table = VectorTable::init(rom_ptr);

        let stack_ptr = vector_table.reset_stack_pointer();
        let pc = vector_table.reset_program_counter();
        let opcode_table = instruction_set::generate();

        let instruction: Box<dyn InstructionProcess> = opcode_table[0].clone();
        let mut reg = [0; 17];
        reg[15] = stack_ptr;

        // TODO call RESET function

        Self {
            opcode_table: opcode_table,
            vector_table: vector_table,

            reg: reg,
            pc: pc,

            sr: 0,
            mode: Mode::Supervisor,

            clock_counter: 0,

            pended_interrupt_level: None,

            instruction: instruction,
            
            current_addr_mode: AddrMode::new(AddrModeType::Immediate, 0),

            ea_location: Location::new(LocationType::Memory, 0x1000000),
            ea_operand: 0,

            bus: bus,
            disassembler: disassembler,
        }
    }

    pub fn clock(&mut self) {
        if self.clock_counter == 0 {
            if let Some(interrutp) = self.pended_interrupt_level {
                self.prepare_exception();
                self.pc = self.vector_table.interrupt_level(interrutp);
                self.pended_interrupt_level = None;
            }

            let instruction_addr = self.pc;
            let operation_word = self.read_memory(self.pc as usize, Size::Word);
            self.increment_pc();

            // get instruction from table by its opcode
            let mut instruction = self.opcode_table[operation_word as usize].clone();
            // fetch all additional instruction information from instruction opcode and from memory (extension words, decode addressing modes, etc.)
            instruction.fetch_decode_instruction_data(self);
            self.clock_counter = instruction.as_ref().clock();

            // save instruction in current cpu state for getting an instruction metadata, when cpu will execute this they will use them
            self.instruction = instruction;
            
            let disasm_str = format!("0x{:04X}\t{}", operation_word, self.instruction.as_ref().disassembly());
            self.disassembler.push_instruction(instruction_addr, disasm_str.clone());
            println!("0x{:08X}: {}", instruction_addr, disasm_str);
            
            // call instruction execution
            (self.instruction.as_ref().handler())(self);
            println!("{}", self);
            self.clock_counter -= 1;
        } else {
            self.clock_counter -= 1;
        }
    }

    pub fn interrupt(&mut self, interrupt_level: usize) {
        let cpu_priority = ((self.sr >> 8) & 0x7) as usize;
        if self.pended_interrupt_level.is_none() && interrupt_level > cpu_priority {
            self.pended_interrupt_level = Some(interrupt_level);
        }
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }

    pub fn save(&self) {
        self.disassembler.save().unwrap();
    }

    fn read_data_reg(&self, reg: usize, size: Size) -> u32 {
        match size {
            Size::Byte => self.reg[reg] & 0xFF,
            Size::Word => self.reg[reg] & 0xFFFF,
            Size::Long => self.reg[reg],
        }
    }

    fn write_data_reg(&mut self, reg: usize, data: u32, size: Size) {
        match size {
            Size::Byte => {
                self.reg[reg] &= !0xFF;
                self.reg[reg] |= data & 0xFF;
            }
            Size::Word => {
                self.reg[reg] &= !0xFFFF;
                self.reg[reg] |= data & 0xFFFF;
            }
            Size::Long => {
                self.reg[reg] = data;
            }
        }
    }

    fn read_addr_reg(&self, reg: usize, size: Size) -> u32 {
        let reg = reg + 8;
        match size {
            Size::Byte => 0, // stub, may be should panic?
            Size::Word => self.reg[reg] & 0xFFFF,
            Size::Long => self.reg[reg] & 0x00FFFFFF,
        }
    }

    fn write_addr_reg(&mut self, reg: usize, data: u32, size: Size) {
        let reg = reg + 8;
        match size {
            Size::Byte => (),
            Size::Word => {
                let data = data & 0xFFFF;
                let data = sign_extend(data, size);
                self.reg[reg] = data;
            }
            Size::Long => {
                self.reg[reg] = data;
            }
        }
    }

    fn read_memory(&self, address: usize, size: Size) -> u32 {
        unsafe {
            (*self.bus).read(address, size)
        }
    }

    fn write_memory(&mut self, address: usize, data: u32, size: Size) {
        unsafe {
            (*self.bus).write(address, data, size);
        }
    }

    pub(in crate) fn read(&mut self, location: Location, size: Size) -> u32 {
        match location.location_type {
            LocationType::DataReg => self.read_data_reg(location.address, size),
            LocationType::AddrReg => self.read_addr_reg(location.address, size),
            LocationType::Memory => self.read_memory(location.address, size),
        }
    }

    fn write(&mut self, location: Location, data: u32, size: Size) {
        match location.location_type {
            LocationType::DataReg => self.write_data_reg(location.address, data, size),
            LocationType::AddrReg => self.write_addr_reg(location.address, data, size),
            LocationType::Memory => self.write_memory(location.address, data, size),
        }
    }

    fn stack_ptr(&self) -> u32 {
        self.reg[15]
    }

    fn set_stack_ptr(&mut self, data: u32) {
        self.reg[15] = data;
    }

    fn push(&mut self, data: u32, size: Size) {
        let size = match size {
            Size::Byte => Size::Word,
            _ => size,
        };
        let offset = size as u32;
        self.set_stack_ptr(self.stack_ptr().wrapping_sub(offset));

        let location = Location::new(LocationType::Memory, self.stack_ptr() as usize);
        self.write(location, data, size);
    }

    fn pop(&mut self, size: Size) -> u32 {
        let size = match size {
            Size::Byte => Size::Word,
            _ => size,
        };
        let offset = size as u32;

        let location = Location::new(LocationType::Memory, self.stack_ptr() as usize);
        let data = self.read(location, size);

        self.set_stack_ptr(self.stack_ptr().wrapping_add(offset));

        data
    }

    fn set_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Supervisor => {
                if self.mode == Mode::UserMode {
                    self.reg.swap(15, 16);
                }
            }
            Mode::UserMode => {
                if self.mode == Mode::Supervisor {
                    self.reg.swap(15, 16);
                }
            }
        }
    }

    fn get_status(&self, status: Status) -> bool {
        let mask = 1 << status as u16;
        self.sr & mask != 0
    }

    fn set_status(&mut self, status: Status, set: bool) {
        let mask = 1 << status as u16;
        if set {
            self.sr = self.sr | mask;
        } else {
            self.sr = self.sr & !mask;
        }
    }

    pub(in super) fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(Size::Word as u32);
    }

    fn call_addressing_mode(&mut self) {
        match self.current_addr_mode.am_type {
            AddrModeType::Data => self.data_reg(),
            AddrModeType::Addr => self.addr_reg(),
            AddrModeType::AddrInd => self.addr_indirect(),
            AddrModeType::AddrIndPostInc => self.addr_indirect_post_inc(),
            AddrModeType::AddrIndPreDec => self.addr_indirect_pre_dec(),
            AddrModeType::AddrIndDips => self.addr_indirect_disp(),
            AddrModeType::AddrIndIdx => self.addr_indirect_idx(),
            AddrModeType::PcDisp => self.pc_indirect_disp(),
            AddrModeType::PcIdx => self.pc_indirect_idx(),
            AddrModeType::AbsShort => self.abs_short(),
            AddrModeType::AbsLong => self.abs_long(),
            AddrModeType::Immediate => self.immediate(),
        }
    }

    fn check_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::True => true,
            Condition::False => false,
            Condition::Higher => {
                let carry = self.get_status(Status::C);
                let zero = self.get_status(Status::Z);
                !carry & !zero
            }
            Condition::LowerOrSame => {
                let carry = self.get_status(Status::C);
                let zero = self.get_status(Status::Z);
                carry | zero
            }
            Condition::CarryClear => {
                let carry = self.get_status(Status::C);
                !carry
            }
            Condition::CarrySet => {
                let carry = self.get_status(Status::C);
                carry
            }
            Condition::NotEqual => {
                let zero = self.get_status(Status::Z);
                !zero
            }
            Condition::Equal => {
                let zero = self.get_status(Status::Z);
                zero
            }
            Condition::OverflowClear => {
                let overflow = self.get_status(Status::V);
                !overflow
            }
            Condition::OverflowSet => {
                let overflow = self.get_status(Status::V);
                overflow
            }
            Condition::Plus => {
                let negate = self.get_status(Status::N);
                !negate
            }
            Condition::Minus => {
                let negate = self.get_status(Status::N);
                negate
            }
            Condition::GreaterOrEqual => {
                let negate = self.get_status(Status::N);
                let overflow = self.get_status(Status::V);
                negate & overflow | !negate & !overflow
            }
            Condition::LessThan => {
                let negate = self.get_status(Status::N);
                let overflow = self.get_status(Status::V);
                negate & !overflow | !negate & overflow
            }
            Condition::GreaterThan => {
                let negate = self.get_status(Status::N);
                let overflow = self.get_status(Status::V);
                let zero = self.get_status(Status::Z);
                negate & overflow & !zero | !negate & !overflow & !zero
            }
            Condition::LessOrEqual => {
                let negate = self.get_status(Status::N);
                let overflow = self.get_status(Status::V);
                let zero = self.get_status(Status::Z);
                zero | negate & !overflow | !negate & overflow
            }
        }
    }

    fn prepare_exception(&mut self) {
        self.set_mode(Mode::Supervisor);

        let sr_copy = self.sr as u32;
        let pc_copy = self.pc;

        self.push(pc_copy, Size::Long); 
        self.push(sr_copy, Size::Word);
    }

    fn instruction<T: 'static + Clone>(&self) -> T {
        self.instruction.as_ref().as_any().downcast_ref::<T>().unwrap().clone()
    }

    /********************/
    /* ADDRESSING MODES */
    /********************/
    fn data_reg(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;

        self.ea_location = Location::new(LocationType::DataReg, reg_idx);
        self.ea_operand = self.read_data_reg(reg_idx, self.instruction.as_ref().size());
    }

    fn addr_reg(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;

        self.ea_location = Location::new(LocationType::AddrReg, reg_idx);
        self.ea_operand = self.read_addr_reg(reg_idx, self.instruction.as_ref().size());
    }

    fn addr_indirect(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;
        let address = self.read_addr_reg(reg_idx, Size::Long) as usize;

        self.ea_location = Location::new(LocationType::Memory, address);
        self.ea_operand = self.read_memory(address, self.instruction.as_ref().size());
    }

    fn addr_indirect_post_inc(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;
        let address = self.read_addr_reg(reg_idx, Size::Long) as usize;

        // increment address in address register
        let increment = match self.instruction.as_ref().size() {
            Size::Byte => {
                if reg_idx == 6 {
                    self.instruction.as_ref().size() as u32 + 1
                } else {
                    self.instruction.as_ref().size() as u32
                }
            }
            Size::Word => self.instruction.as_ref().size() as u32,
            Size::Long => self.instruction.as_ref().size() as u32,
        };
        self.write_addr_reg(reg_idx, address as u32 + increment, Size::Long);

        // fetch data
        self.ea_location = Location::new(LocationType::Memory, address);
        self.ea_operand = self.read_memory(address, self.instruction.as_ref().size());
    }

    fn addr_indirect_pre_dec(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;
        // decrement address in address register
        let decrement = match self.instruction.as_ref().size() {
            Size::Byte => {
                if reg_idx == 6 {
                    self.instruction.as_ref().size() as u32 + 1
                } else {
                    self.instruction.as_ref().size() as u32
                }
            }
            Size::Word => self.instruction.as_ref().size() as u32,
            Size::Long => self.instruction.as_ref().size() as u32,
        };

        let register_data = self.read_addr_reg(reg_idx, Size::Long);
        let address = register_data.wrapping_sub(decrement) as usize;
        self.write_addr_reg(reg_idx, address as u32, Size::Long);

        // fetch data
        self.ea_location = Location::new(LocationType::Memory, address);
        self.ea_operand = self.read_memory(address, self.instruction.as_ref().size());
    }

    fn addr_indirect_disp(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;

        let address = self.read_addr_reg(reg_idx, Size::Long);
        let displacement = self.current_addr_mode.ext_word.unwrap();

        let ea_addr = address.wrapping_add(displacement) as usize;

        self.ea_location = Location::new(LocationType::Memory, ea_addr);
        self.ea_operand = self.read_memory(ea_addr, self.instruction.as_ref().size());
    }

    fn addr_indirect_idx(&mut self) {
        let reg_idx = self.current_addr_mode.reg_idx;
        let address = self.read_addr_reg(reg_idx, Size::Long);

        let brief_ext_word = self.current_addr_mode.brief_ext_word.unwrap();
        let idx_reg = brief_ext_word.register;

        let idx_reg_data = match idx_reg.reg_type {
            RegisterType::Address => self.read_addr_reg(idx_reg.reg_idx, brief_ext_word.size),
            RegisterType::Data => self.read_data_reg(idx_reg.reg_idx, brief_ext_word.size),
        };

        let idx_reg_data = sign_extend(idx_reg_data, brief_ext_word.size);

        let ea_addr = address
            .wrapping_add(brief_ext_word.displacement)
            .wrapping_add(idx_reg_data) as usize;

        self.ea_location = Location::new(LocationType::Memory, ea_addr);
        self.ea_operand = self.read_memory(ea_addr, self.instruction.as_ref().size());
    }

    fn pc_indirect_disp(&mut self) {
        let address = self.current_addr_mode.ext_word_addr; // address in pc should pouint at extension word after instruction
        let displacement = self.current_addr_mode.ext_word.unwrap();

        let ea_addr = address.wrapping_add(displacement) as usize;

        self.ea_location = Location::new(LocationType::Memory, ea_addr);
        self.ea_operand = self.read_memory(ea_addr, self.instruction.as_ref().size());
    }

    fn pc_indirect_idx(&mut self) {
        let address = self.current_addr_mode.ext_word_addr; // address in pc should pouint at extension word after instruction
        let brief_ext_word = self.current_addr_mode.brief_ext_word.unwrap();
        let idx_reg = brief_ext_word.register;

        let idx_reg_data = match idx_reg.reg_type {
            RegisterType::Address => self.read_addr_reg(idx_reg.reg_idx, brief_ext_word.size),
            RegisterType::Data => self.read_data_reg(idx_reg.reg_idx, brief_ext_word.size),
        };

        let idx_reg_data = sign_extend(idx_reg_data, brief_ext_word.size);

        let ea_addr = address
            .wrapping_add(brief_ext_word.displacement)
            .wrapping_add(idx_reg_data) as usize;

        self.ea_location = Location::new(LocationType::Memory, ea_addr);
        self.ea_operand = self.read_memory(ea_addr, self.instruction.as_ref().size());
    }

    fn abs_short(&mut self) {
        let ext_word = self.current_addr_mode.ext_word.unwrap();

        let address = sign_extend(ext_word, Size::Word) as usize;
        self.ea_location = Location::new(LocationType::Memory, address);
        self.ea_operand = self.read_memory(address, self.instruction.as_ref().size());
    }

    fn abs_long(&mut self) {
        let address = self.current_addr_mode.ext_word.unwrap() as usize;

        self.ea_location = Location::new(LocationType::Memory, address);
        self.ea_operand = self.read_memory(address, self.instruction.as_ref().size());
    }

    fn immediate(&mut self) {
        self.ea_operand = self.current_addr_mode.ext_word.unwrap();
    }

    /* INSTRUCTION SET */
    // data movement MOVE, MOVE16, MOVEM, MOVEP, MOVEQ, EXG, LEA, PEA, LINK, and UNLK

    pub(crate) fn MOVE(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<MoveInstructionMetadata>>();

        let src_addr_mode = instruction.meta_data.src_addr_mode;
        self.current_addr_mode = src_addr_mode.clone();
        self.call_addressing_mode();

        let src_data = self.ea_operand;

        let dst_addr_mode = instruction.meta_data.dst_addr_mode;
        self.current_addr_mode = dst_addr_mode;
        self.call_addressing_mode();

        let dst_address = self.ea_location;
        self.write(dst_address, src_data, size);

        // set status codes
        self.set_status(Status::N, is_negate(src_data, size));
        self.set_status(Status::Z, is_zero(src_data, size));

        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn MOVEA(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let src_data = self.ea_operand;

        let location = Location::register(instruction.meta_data.reg_x);

        self.write(location, src_data, size);
    }

    pub(crate) fn MOVEM(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<AddrModeExtWordMetadata>>();

        // calculate the affected registers from register_mask
        let register_mask = instruction.meta_data.ext_word;
        let mut affected_registers = Vec::new();
        if self.current_addr_mode.am_type == AddrModeType::AddrIndPreDec {
            // D0..D7A0..A7
            for i in 0..16 {
                let bit = (register_mask >> i) & 1;
                if bit == 1 {
                    affected_registers.push(15 - i);
                }
            }
        } else {
            // A7..A0D7..D0
            for i in 0..16 {
                let bit = (register_mask >> i) & 1;
                if bit == 1 {
                    affected_registers.push(i);
                }
            }
        }

        let num_of_regs = affected_registers.len();
        match self.instruction.as_ref().size() {
            Size::Word => self.clock_counter += 4 * num_of_regs as u32,
            Size::Long => self.clock_counter += 8 * num_of_regs as u32,
            Size::Byte => panic!("MOVEM: wrong size for this instruction"),
        }

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let direction_bit = (instruction.operation_word >> 10) & 0x1;
        let current_addr_mode_type = instruction.meta_data.addr_mode.am_type;

        let operation_size_usize = size as usize;

        if direction_bit == 0 { // Register to memory
            if current_addr_mode_type == AddrModeType::AddrIndPreDec {
                let address = self.ea_location.address + operation_size_usize;
                let reg_idx = self.current_addr_mode.reg_idx;
                self.write_addr_reg(reg_idx, address as u32, Size::Long);
            }

            for reg_idx in affected_registers {
                let data = self.reg[reg_idx];
                self.write(self.ea_location, data, size);

                if current_addr_mode_type != AddrModeType::AddrIndPreDec {
                    self.ea_location = Location::memory(self.ea_location.address + operation_size_usize);
                } else {
                    self.ea_location = Location::memory(self.ea_location.address - operation_size_usize);
                }
            }

            if current_addr_mode_type == AddrModeType::AddrIndPreDec {
                let reg_idx = self.current_addr_mode.reg_idx;
                let address = self.ea_location.address;
                self.write_addr_reg(reg_idx, address as u32, Size::Long);
            }
        } else { // Memory to register
            for reg_idx in affected_registers {
                let data = self.read(self.ea_location, size);
                let data = sign_extend(data, size);

                self.reg[reg_idx] = data;

                self.ea_location = Location::memory(self.ea_location.address + operation_size_usize);

                if current_addr_mode_type == AddrModeType::AddrIndPostInc {
                    let address = self.ea_location.address;
                    let reg_idx = self.current_addr_mode.reg_idx;
                    self.write_addr_reg(reg_idx, address as u32, Size::Long);
                }
            }
        }
    }

    pub(crate) fn MOVEP(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(location, size).to_le();

        let iterations = size as usize;

        let direction_bit = (instruction.operation_word >> 7) & 0x1;
        if direction_bit == 1 { // register to memory
            for _ in 0..iterations {
                let byte_ = data as u8;
                self.write(self.ea_location, byte_ as u32, Size::Byte);

                self.ea_location.address += 2;
            }
        } else { // memory to register
            let mut data = 0;
            for i in 0..iterations {
                let byte_ = self.read(self.ea_location, Size::Byte);
                data |= (byte_ as u32) << i * 8;
            }
            data = data.to_be();

            self.write(location, data, size);
        }
    }

    pub(crate) fn MOVEQ(&mut self) {
        let instruction = self.instruction::<Instruction<RxDataMetadata>>();

        let data = sign_extend(instruction.meta_data.data, Size::Byte);

        let location = Location::register(instruction.meta_data.reg_x);
        self.write(location, data, Size::Long);

        self.set_status(Status::N, is_negate(data, Size::Long));
        self.set_status(Status::Z, is_zero(data, Size::Long));

        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn MOVE_to_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

            self.current_addr_mode = instruction.meta_data.addr_mode;
            self.call_addressing_mode();

            self.sr = self.ea_operand as u16;
            // TODO clear unused bits
        } else {
            // TODO call privilage exception
        }
    }

    pub(crate) fn MOVE_from_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

            self.current_addr_mode = instruction.meta_data.addr_mode;
            self.call_addressing_mode();

            let data = self.sr as u32;
            self.write(self.ea_location, data, Size::Word);
        } else {
            // TODO call pribilage excaption
        }
    }

    pub(crate) fn MOVE_USP(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<RyMetadata>>();

            let location = Location::register(instruction.meta_data.reg_y);
            let direction_bit = (instruction.operation_word >> 3) & 0x1;

            if direction_bit == 0 { // USP to memory
                let data = self.reg[16];
                self.write(location, data, Size::Long);
            } else { // Memory to USP
                let data = self.read(location, Size::Long);
                self.reg[16] = data;
            }
        } else {
            // TODO call privilage exception
        }
    }

    pub(crate) fn MOVE_to_CCR(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = self.ea_operand & 0xFF;

        self.sr &= !0xFF;
        self.sr |= data as u16;
    }

    pub(crate) fn EXG(&mut self) {
        let instruction = self.instruction::<Instruction<RxRyMetadata>>();
        let (a_idx, b_idx) = {
            let reg_x = instruction.meta_data.reg_x;
            let reg_y = instruction.meta_data.reg_y;

            let reg_x_idx = match reg_x.reg_type {
                RegisterType::Data => reg_x.reg_idx,
                RegisterType::Address => reg_x.reg_idx + 8,
            };

            let reg_y_idx = match reg_y.reg_type {
                RegisterType::Data => reg_y.reg_idx,
                RegisterType::Address => reg_y.reg_idx + 8,
            };
            (reg_x_idx, reg_y_idx)
        };

        self.reg.swap(a_idx, b_idx);
    }

    pub(crate) fn LEA(&mut self) {
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();
        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);

        self.write(location, self.ea_location.address as u32, Size::Long);
    }

    pub(crate) fn PEA(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();
        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        self.push(self.ea_location.address as u32, Size::Long);
    }

    pub(crate) fn LINK(&mut self) {
        let instruction = self.instruction::<Instruction<RyExtWordMetadata>>();

        let location = Location::register(instruction.meta_data.reg_y);
        let data = self.read(location, Size::Long);
        self.push(data, Size::Long);
        
        self.write(location, self.stack_ptr(), Size::Long);

        let displacement = instruction.meta_data.ext_word;
        let displacement = sign_extend(displacement, Size::Word);
        let new_stack_ptr = self.stack_ptr().wrapping_add(displacement);

        self.set_stack_ptr(new_stack_ptr);
    }

    pub(crate) fn UNLK(&mut self) {
        let instruction = self.instruction::<Instruction<RyMetadata>>();
        let location = Location::register(instruction.meta_data.reg_y);
        let data = self.read(location, Size::Long);
        self.set_stack_ptr(data);

        let data = self.pop(Size::Long);
        self.write(location, data, Size::Long);
    }

    pub(crate) fn TAS(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let negate = is_negate(self.ea_operand, Size::Byte);
        let zero = is_zero(self.ea_operand, Size::Byte);
        let overflow = false;
        let carry = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);

        self.ea_operand |= 0x80;
        self.write(self.ea_location, self.ea_operand, Size::Byte);
    }

    pub(crate) fn TST(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let is_negate = is_negate(self.ea_operand, self.instruction.as_ref().size());
        let is_zero = is_zero(self.ea_operand, self.instruction.as_ref().size());

        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn Bcc(&mut self) {
        let instruction = self.instruction::<Instruction<ConditionDisplacementMetadata>>();

        let offset = instruction.meta_data.displacement;
        let condition = instruction.meta_data.condition;

        let result = self.check_condition(condition);

        if result {
            self.pc = self.pc.wrapping_add(offset);
        } else {
            let clock_corection = match instruction.meta_data.displacement_size {
                Size::Byte => -2,
                Size::Word => {
                    self.increment_pc();
                    2
                },
                Size::Long => panic!("Bcc: unexpected displacement size"),
            };
            self.clock_counter = self.clock_counter.wrapping_add(clock_corection as u32);
        }
    }

    pub(crate) fn DBcc(&mut self) {
        let instruction = self.instruction::<Instruction<ConditionRyExtWordMetadata>>();

        let ext_word = instruction.meta_data.ext_word;
        let offset = sign_extend(ext_word, Size::Word);

        let condition = instruction.meta_data.condition;
        let result = self.check_condition(condition);


        if !result {
            let counter_location = Location::register(instruction.meta_data.reg_y);
            let mut counter = self.read(counter_location, Size::Word) as i32;
            counter -= 1;
            self.write(counter_location, counter as u32, Size::Word);
            if counter >= 0 {
                self.pc = offset.wrapping_add(self.pc - 2);
            } else {
                self.clock_counter += 4 // if loop counter expired
            }
        } else {
            self.clock_counter += 2 // if condition true
        }
    }

    pub(crate) fn Scc(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<ConditionAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let condition = instruction.meta_data.condition;
        let result = self.check_condition(condition);

        if result {
            self.write(self.ea_location, 0x00, size);
        } else {
            self.write(self.ea_location, 0xFF, size);
        }

        if result && instruction.meta_data.addr_mode.am_type == AddrModeType::Data {
            self.clock_counter += 2;
        }
    }

    pub(crate) fn BRA(&mut self) {
        let instruction = self.instruction::<Instruction<DisplacementMetadata>>();

        let offset = instruction.meta_data.displacement;

        match instruction.meta_data.displacement_size {
            Size::Word => self.pc -= 2,
            _ => (),
        }

        self.pc = self.pc.wrapping_add(offset);
    }

    pub(crate) fn BSR(&mut self) {
        let instruction = self.instruction::<Instruction<DisplacementMetadata>>();

        self.push(self.pc as u32, Size::Long);

        let offset = instruction.meta_data.displacement;
        self.pc = self.pc.wrapping_add(offset);
    }

    pub(crate) fn JMP(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        self.pc = self.ea_location.address as u32;
    }

    pub(crate) fn JSR(&mut self) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        self.push(self.pc, Size::Long);
        self.pc = self.ea_location.address as u32;
    }

    pub(crate) fn NOP(&mut self) {

    }

    pub(crate) fn RTR(&mut self) {
        let ccr = self.pop(Size::Word);
        self.sr = (self.sr & !0xFF) | ccr as u16;

        self.pc = self.pop(Size::Long);
    }
    
    pub(crate) fn RTS(&mut self) {
        self.pc = self.pop(Size::Long);
    }

    pub(crate) fn RTE(&mut self) {
        if self.mode == Mode::Supervisor {
            self.sr = self.pop(Size::Word) as u16;
            self.pc = self.pop(Size::Long);
        } else {
            // TODO call privilage exception
        }
    }

    pub(crate) fn ADD(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(location, size);

        let result = match size {
            Size::Byte => (self.ea_operand + data) & 0xFF,
            Size::Word => (self.ea_operand + data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_add(data),
        };

        let direction_bit = (instruction.operation_word >> 8) & 0x1;
        let (carry, overflow) = if direction_bit == 1 { // Memory to register
            self.write(self.ea_location, result, size);
            let sm = msb_is_set(data, size);
            let dm = msb_is_set(self.ea_operand, size);
            let rm = msb_is_set(result, size);

            let overflow = sm && dm && !rm || !sm && !dm && rm;
            let carry = sm && dm || !rm && dm || sm && !rm;
            (carry, overflow)
        } else { // Register to memory
            self.write(location, result, size);
            let sm = msb_is_set(self.ea_operand, size);
            let dm = msb_is_set(data, size);
            let rm = msb_is_set(result, size);
    
            let overflow = sm && dm && !rm || !sm && !dm && rm;
            let carry = sm && dm || !rm && dm || sm && !rm;
            (carry, overflow)
        };

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result, size);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ADDA(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);
        let addr_reg_data = self.read(location, Size::Long);
        let operand = sign_extend(self.ea_operand, size);

        let result = addr_reg_data.wrapping_add(operand);
        self.write(location, result, Size::Long);
    }

    pub(crate) fn ADDI(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;

        let result = match size {
            Size::Byte => (self.ea_operand + data) & 0xFF,
            Size::Word => (self.ea_operand + data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_add(data),
        };

        self.write(self.ea_location, result, size);
        
        let sm = msb_is_set(data, size);
        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);

        let overflow = sm && dm && !rm || !sm && !dm && rm;
        let carry = sm && dm || !rm && dm || sm && !rm;

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result, size);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ADDQ(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeDataMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.data;

        let result = match size {
            Size::Byte => (self.ea_operand + data) & 0xFF,
            Size::Word => (self.ea_operand + data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_add(data),
        };

        self.write(self.ea_location, result, size);

        if instruction.meta_data.addr_mode.am_type != AddrModeType::Addr {
            let sm = msb_is_set(data, size);
            let dm = msb_is_set(self.ea_operand, size);
            let rm = msb_is_set(result, size);

            let overflow = sm && dm && !rm || !sm && !dm && rm;
            let carry = sm && dm || !rm && dm || sm && !rm;

            let is_negate = is_negate(result, size);
            let is_zero = is_zero(result, size);

            self.set_status(Status::X, carry);
            self.set_status(Status::N, is_negate);
            self.set_status(Status::Z, is_zero);
            self.set_status(Status::V, overflow);
            self.set_status(Status::C, carry);
        }
    }

    pub(crate) fn ADDX(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxRySpecAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode_x;
        self.call_addressing_mode();

        let data_x = self.ea_operand;

        self.current_addr_mode = instruction.meta_data.addr_mode_y;
        self.call_addressing_mode();

        let data_y = self.ea_operand;

        let x_bit = if self.get_status(Status::X) {
            1
        } else {
            0
        };

        let result = match size {
            Size::Byte => (data_x + x_bit + data_y) & 0x00FF,
            Size::Word => (data_x + x_bit + data_y) & 0xFFFF,
            Size::Long => data_x.wrapping_add(x_bit).wrapping_add(data_y),
        };

        let sm = msb_is_set(data_x, size);
        let dm = msb_is_set(data_y, size);
        let rm = msb_is_set(result, size);

        let overflow = sm && dm && !rm || !sm && !dm && rm;
        let carry = sm && dm || !rm && dm || sm && !rm;
        let negate = is_negate(result, size);
        
        self.set_status(Status::X, carry);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
        
        if result != 0 {
            self.set_status(Status::Z, false);
        }
    }

    pub(crate) fn SUB(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(location, size);

        let direction_bit = (instruction.operation_word >> 8) & 0x1;
        let (result, carry, overflow) = if direction_bit == 1 { // <ea> - dn -> <ea>
            let result = match size {
                Size::Byte => self.ea_operand.wrapping_sub(data) & 0xFF,
                Size::Word => self.ea_operand.wrapping_sub(data) & 0xFFFF,
                Size::Long => self.ea_operand.wrapping_sub(data),
            };
            self.write(self.ea_location, result, size);
            let sm = msb_is_set(data, size);
            let dm = msb_is_set(self.ea_operand, size);
            let rm = msb_is_set(result, size);
            
            let overflow = !sm && dm && !rm || sm && !dm && rm;
            let carry = sm && !dm || rm && !dm || sm && rm;
            (result, carry, overflow)
        } else { // dn - <ea> -> dn
            let result = match size {
                Size::Byte => data.wrapping_sub(self.ea_operand) & 0xFF,
                Size::Word => data.wrapping_sub(self.ea_operand) & 0xFFFF,
                Size::Long => data.wrapping_sub(self.ea_operand),
            };
            self.write(location, result, size);
            let sm = msb_is_set(self.ea_operand, size);
            let dm = msb_is_set(data, size);
            let rm = msb_is_set(result, size);
    
            let overflow = !sm && dm && !rm || sm && !dm && rm;
            let carry = sm && !dm || rm && !dm || sm && rm;
            (result, carry, overflow)
        };

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result, size);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn SUBA(&mut self) {
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(location, self.instruction.as_ref().size());

        let result = self.ea_operand.wrapping_sub(data);
        self.write(location, result, self.instruction.as_ref().size());
    }

    pub(crate) fn SUBI(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;

        let result = match size {
            Size::Byte => self.ea_operand.wrapping_sub(data) & 0xFF,
            Size::Word => self.ea_operand.wrapping_sub(data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_sub(data),
        };

        self.write(self.ea_location, result, size);
        
        let sm = msb_is_set(data, size);
        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);

        let overflow = !sm && dm && !rm || sm && !dm && rm;
        let carry = sm && !dm || rm && !dm || sm && rm;

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result, size);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn SUBQ(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeDataMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.data;

        let result = match size {
            Size::Byte => self.ea_operand.wrapping_sub(data) & 0xFF,
            Size::Word => self.ea_operand.wrapping_sub(data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_sub(data),
        };

        self.write(self.ea_location, result, size);

        if instruction.meta_data.addr_mode.am_type != AddrModeType::Addr {
            let sm = msb_is_set(data, size);
            let dm = msb_is_set(self.ea_operand, size);
            let rm = msb_is_set(result, size);

            let overflow = !sm && dm && !rm || sm && !dm && rm;
            let carry = sm && !dm || rm && !dm || sm && rm;

            let is_negate = is_negate(result, size);
            let is_zero = is_zero(result, size);

            self.set_status(Status::X, carry);
            self.set_status(Status::N, is_negate);
            self.set_status(Status::Z, is_zero);
            self.set_status(Status::V, overflow);
            self.set_status(Status::C, carry);
        }
    }

    pub(crate) fn SUBX(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxRySpecAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode_x;
        self.call_addressing_mode();

        let data_x = self.ea_operand;

        self.current_addr_mode = instruction.meta_data.addr_mode_y;
        self.call_addressing_mode();

        let data_y = self.ea_operand;

        let x_bit = if self.get_status(Status::X) {
            1
        } else {
            0
        };

        let result = match size {
            Size::Byte => data_x.wrapping_sub(x_bit).wrapping_sub(data_y) & 0x00FF,
            Size::Word => data_x.wrapping_sub(x_bit).wrapping_sub(data_y) & 0xFFFF,
            Size::Long => data_x.wrapping_sub(x_bit).wrapping_sub(data_y),
        };

        let sm = msb_is_set(data_x, size);
        let dm = msb_is_set(data_y, size);
        let rm = msb_is_set(result, size);

        let overflow = !sm && dm && !rm || sm && !dm && rm;
        let carry = sm && !dm || rm && !dm || sm && rm;
        let negate = is_negate(result, size);
        
        self.set_status(Status::X, carry);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
        
        if result != 0 {
            self.set_status(Status::Z, false);
        }
    }
    
    pub(crate) fn CLR(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        self.write(self.ea_location, 0, size);

        self.set_status(Status::N, false);
        self.set_status(Status::Z, true);
        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn CMP(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = self.read(Location::register(instruction.meta_data.reg_x), size);

        let result = data.wrapping_sub(self.ea_operand);

        let sm = msb_is_set(data, size);
        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);

        let overflow = !sm && dm && !rm || sm && !dm && rm;
        let carry = sm && !dm || rm && !dm || sm && rm;
        let negate = is_negate(result, size);
        let zero = is_zero(result, size);

        self.set_status(Status::Z, zero);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn CMPA(&mut self) {
        self.CMP();
    }

    pub(crate) fn CMPI(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;

        let result = self.ea_operand.wrapping_sub(data);

        let sm = msb_is_set(data, size);
        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);

        let overflow = !sm && dm && !rm || sm && !dm && rm;
        let carry = sm && !dm || rm && !dm || sm && rm;
        let negate = is_negate(result, size);
        let zero = is_zero(result, size);

        self.set_status(Status::Z, zero);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn CMPM(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxRyMetadata>>();

        self.current_addr_mode = AddrMode::new(AddrModeType::AddrIndPostInc, instruction.meta_data.reg_x.reg_idx);
        self.call_addressing_mode();
        let data_x = self.ea_operand;

        self.current_addr_mode = AddrMode::new(AddrModeType::AddrIndPostInc, instruction.meta_data.reg_y.reg_idx);
        self.call_addressing_mode();
        let data_y = self.ea_operand;

        let result = data_y.wrapping_sub(data_x);

        let sm = msb_is_set(data_x, size);
        let dm = msb_is_set(data_y, size);
        let rm = msb_is_set(result, size);

        let overflow = !sm && dm && !rm || sm && !dm && rm;
        let carry = sm && !dm || rm && !dm || sm && rm;
        let negate = is_negate(result, size);
        let zero = is_zero(result, size);

        self.set_status(Status::Z, zero);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn EXT(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RyMetadata>>();

        let read_size = match size {
            Size::Word => {
                Size::Byte
            }, 
            Size::Long => {
                Size::Word
            },
            _ => panic!("EXT: unexpected operation size (byte)"),
        };

        let location = Location::register(instruction.meta_data.reg_y);
        let data = self.read(location, read_size);
        let result = sign_extend(data, size);
        self.write(location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }
    pub(crate) fn NEG(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let result = 0u32.wrapping_sub(self.ea_operand);
        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let carry = !zero; // в описании инструкции указано !zero, в таблице вычисления флагов dm || rm

        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);
        let overflow = dm & rm;

        self.set_status(Status::X, carry);
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn NEGX(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let x_bit = if self.get_status(Status::X) {
            1
        } else {
            0
        };

        let result = 0u32.wrapping_sub(self.ea_operand).wrapping_sub(x_bit);
        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);

        let dm = msb_is_set(self.ea_operand, size);
        let rm = msb_is_set(result, size);
        let overflow = dm & rm;
        let carry = dm | rm;

        self.set_status(Status::X, carry);
        self.set_status(Status::N, negate);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);

        if !zero {
            self.set_status(Status::Z, zero);
        }
    }

    pub(crate) fn MULS(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let dst_location = Location::register(instruction.meta_data.reg_x);
        let dst_operand = self.read(dst_location, size) as i32;

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let (result, overflow) = (self.ea_operand as i32).overflowing_mul(dst_operand);
        let result = result as u32;
        self.write(dst_location, result, Size::Long);
        
        let negate = is_negate(result, Size::Long);
        let zero = is_zero(result, size);
        let carry = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn MULU(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let dst_location = Location::register(instruction.meta_data.reg_x);
        let dst_operand = self.read(dst_location, size);

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let (result, overflow) = self.ea_operand.overflowing_mul(dst_operand);
        self.write(dst_location, result, Size::Long);
        
        let negate = is_negate(result, Size::Long);
        let zero = is_zero(result, size);
        let carry = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn DIVS(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let dst_location = Location::register(instruction.meta_data.reg_x);
        let dst_operand = self.read(dst_location, Size::Long) as i32;

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        if self.ea_operand != 0 {
            let (quotient, overflow) = dst_operand.overflowing_div(self.ea_operand as i32);
            
            if overflow {
                self.set_status(Status::V, overflow);
                return;
            }

            let remainder = dst_operand % self.ea_operand as i32;

            let result = remainder << 16 | (quotient & 0xFFFF);

            self.write(dst_location, result as u32, Size::Long);

            let negate = is_negate(quotient as u32, size);
            let zero = is_zero(quotient as u32, size);
            let carry = false;

            self.set_status(Status::N, negate);
            self.set_status(Status::Z, zero);
            self.set_status(Status::V, overflow);
            self.set_status(Status::C, carry);

        } else {
            self.prepare_exception();
            self.pc = self.vector_table.zero_division_exception();
        }
    }

    pub(crate) fn DIVU(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let dst_location = Location::register(instruction.meta_data.reg_x);
        let dst_operand = self.read(dst_location, Size::Long);

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        if self.ea_operand != 0 {
            let (quotient, overflow) = dst_operand.overflowing_div(self.ea_operand);
            
            if overflow {
                self.set_status(Status::V, overflow);
                return;
            }

            let remainder = dst_operand % self.ea_operand;

            let result = remainder << 16 | (quotient & 0xFFFF);

            self.write(dst_location, result as u32, Size::Long);

            let negate = is_negate(quotient as u32, size);
            let zero = is_zero(quotient as u32, size);
            let carry = false;

            self.set_status(Status::N, negate);
            self.set_status(Status::Z, zero);
            self.set_status(Status::V, overflow);
            self.set_status(Status::C, carry);

        } else {
            self.prepare_exception();
            self.pc = self.vector_table.zero_division_exception();
        }
    }

    pub(crate) fn AND(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let reg_location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(reg_location, size);

        let result = data & self.ea_operand;

        let direction_bit = (instruction.operation_word >> 8) & 1;

        if direction_bit == 0 { // memory to data reg
            self.write(reg_location, result, size);
        } else { // data reg to memory
            self.write(self.ea_location, result, size);
        }

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ANDI(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;
        let result = data & self.ea_operand;

        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ANDI_to_CCR(&mut self) {
        let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();

        let data = instruction.meta_data.immediate_data as u16;
        let ccr = self.sr & 0xFF;
        let result = ccr & data;

        self.sr = (self.sr & !0xFF) | result;
    }

    pub(crate) fn ANDI_to_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();
            let data = instruction.meta_data.immediate_data as u16;
            self.sr &= data;
        } else {
            // TODO call privilage exception
        }
    }

    pub(crate) fn EOR(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let reg_location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(reg_location, size);

        let result = data ^ self.ea_operand;

        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn EORI(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;

        let result = data ^ self.ea_operand;

        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn EORI_to_CCR(&mut self) {
        let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();

        let data = instruction.meta_data.immediate_data as u16;
        let ccr = self.sr & 0xFF;
        let result = ccr ^ data;

        self.sr = (self.sr & !0xFF) | result;
    }

    pub(crate) fn EORI_to_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();
            let data = instruction.meta_data.immediate_data as u16;
            self.sr ^= data;
        } else {
             // TODO call privilage exception
        }
    }

    pub(crate) fn OR(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let reg_location = Location::register(instruction.meta_data.reg_x);
        let data = self.read(reg_location, size);

        let result = data | self.ea_operand;

        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }
    
    pub(crate) fn ORI(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.meta_data.immediate_data;

        let result = data | self.ea_operand;

        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ORI_to_CCR(&mut self) {
        let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();

        let data = instruction.meta_data.immediate_data as u16;
        let ccr = self.sr & 0xFF;
        let result = ccr | data;

        self.sr = (self.sr & !0xFF) | result;
    }

    pub(crate) fn ORI_to_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction::<Instruction<ExplicitImmediateMetadata>>();
            let data = instruction.meta_data.immediate_data as u16;
            self.sr |= data;
        } else {
             // TODO call privilage exception
        }
    }
    
    pub(crate) fn NOT(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let result = !self.ea_operand;
        self.write(self.ea_location, result, size);

        let negate = is_negate(result, size);
        let zero = is_zero(result, size);
        let overflow = false;
        let carry = false;
        
        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    fn shifting_operation_data(&mut self) -> (u32, Location, u16) {
        let mode = (self.instruction.operation_word() >> 5) & 1;
        let (counter, data_register) = if mode == 0 { // immediate counter
            let instruction = self.instruction::<Instruction<RotationRyMetadata>>();
            (instruction.meta_data.counter, instruction.meta_data.reg_y)
        } else { // data register counter
            let instruction = self.instruction::<Instruction<RxRyMetadata>>();
            let location = Location::register(instruction.meta_data.reg_x);
            let data = self.read(location, Size::Long);
            (data % 64, instruction.meta_data.reg_y)
        };
        let location = Location::register(data_register);
        
        let direction = (self.instruction.operation_word() >> 8) & 1;

        (counter, location, direction)
    }

    fn shifting_operation_memory(&mut self) -> (u32, Location, u16) {
        let instruction = self.instruction::<Instruction<AddrModeMetadata>>();

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let location = self.ea_location;

        let direction = (self.instruction.operation_word() >> 8) & 1;
        (1, location, direction)
    }

    pub(crate) fn ASd_data(&mut self) {
        let (counter, location, direction) = self.shifting_operation_data();
        if direction == 0 {
            self.ASR(counter, location);
        } else {
            self.ASL(counter, location);
        }
    }

    pub(crate) fn ASd_memory(&mut self) {
        let (counter, location, direction) = self.shifting_operation_memory();
        if direction == 0 {
            self.ASR(counter, location);
        } else {
            self.ASL(counter, location);
        }
    }

    fn ASR(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);
        
        let msb = if msb_is_set(data, size) {
            1
        } else {
            0
        };
        let msb_mask = msb << ((8 * size as u32) - 1);

        (0..counter).for_each(|_| {
            let poped_bit = data & 1 == 1;

            data >>= 1;
            data |= msb_mask;

            self.set_status(Status::X, poped_bit);
            self.set_status(Status::C, poped_bit);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    fn ASL(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);

        let mut msb_changed = false;
        (0..counter).for_each(|_| {
            let poped_bit = msb_is_set(data, size);
            data <<= 1;
            let msb_after = msb_is_set(data, size);

            if !msb_changed && poped_bit != msb_after {
                msb_changed = true;
            }

            self.set_status(Status::X, poped_bit);
            self.set_status(Status::C, poped_bit);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = msb_changed;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    pub(crate) fn LSd_data(&mut self) {
        let (counter, location, direction) = self.shifting_operation_data();
        if direction == 0 {
            self.ASR(counter, location);
        } else {
            self.ASL(counter, location);
        }
    }

    pub(crate) fn LSd_memory(&mut self) {
        let (counter, location, direction) = self.shifting_operation_memory();
        if direction == 0 {
            self.LSR(counter, location);
        } else {
            self.LSL(counter, location);
        }
    }

    fn LSR(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);
        
        (0..counter).for_each(|_| {
            let poped_bit = data & 1 == 1;
            data >>= 1;

            self.set_status(Status::X, poped_bit);
            self.set_status(Status::C, poped_bit);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    fn LSL(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);

        (0..counter).for_each(|_| {
            let poped_bit = msb_is_set(data, size);
            data <<= 1;

            self.set_status(Status::X, poped_bit);
            self.set_status(Status::C, poped_bit);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    pub(crate) fn ROd_data(&mut self) {
        let (counter, location, direction) = self.shifting_operation_data();
        if direction == 0 {
            self.ROR(counter, location);
        } else {
            self.ROL(counter, location);
        }
    }

    pub(crate) fn ROd_memory(&mut self) {
        let (counter, location, direction) = self.shifting_operation_memory();
        if direction == 0 {
            self.ROR(counter, location);
        } else {
            self.ROL(counter, location);
        }
    }

    fn ROR(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);

        (0..counter).for_each(|_| {
            let lsb = data & 1;
            let msb_mask = lsb << (8 * size as u32);

            data >>= 1;
            data |= msb_mask;

            self.set_status(Status::C, lsb == 1);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    fn ROL(&mut self, counter: u32, location: Location) {
        let size = self.instruction.size();
        let mut data = self.read(location, size);

        (0..counter).for_each(|_| {
            let msb = get_msb(data, size);

            data <<= 1;
            data |= msb;

            self.set_status(Status::C, msb == 1);
        });

        self.write(location, data, size);

        let negate = is_negate(data, size);
        let zero = is_zero(data, size);
        let overflow = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
    }

    pub(crate) fn SWAP(&mut self) {
        let instruction = self.instruction::<Instruction<RyMetadata>>();

        let location = Location::register(instruction.meta_data.reg_y);
        let mut data = self.read(location, Size::Long);

        let msw = (data & 0xFFFF0000) >> 16;
        let lsw = (data & 0x0000FFFF) << 16;

        data = lsw | msw;

        self.write(location, data, Size::Long);

        let negate = is_negate(data, Size::Long);
        let zero = is_zero(data, Size::Long);
        let overflow = false;
        let carry = false;

        self.set_status(Status::N, negate);
        self.set_status(Status::Z, zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn BCHG_reg(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let bit_number_location = Location::register(instruction.meta_data.reg_x);
        let mut bit_number = self.read(bit_number_location, Size::Long);
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand ^ (1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BCHG_ext_word(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        let mut bit_number = instruction.meta_data.immediate_data;
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand ^ (1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BCLR_reg(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let bit_number_location = Location::register(instruction.meta_data.reg_x);
        let mut bit_number = self.read(bit_number_location, Size::Long);
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand & !(1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BCLR_ext_word(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        let mut bit_number = instruction.meta_data.immediate_data;
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand & !(1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BSET_reg(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let bit_number_location = Location::register(instruction.meta_data.reg_x);
        let mut bit_number = self.read(bit_number_location, Size::Long);
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand | (1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BSET_ext_word(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        let mut bit_number = instruction.meta_data.immediate_data;
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        let result = self.ea_operand | (1 << bit_number);
        self.write(self.ea_location, result, size);

        self.set_status(Status::Z, bit == 0);
    }
    
    pub(crate) fn BTST_reg(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<RxAddrModeMetadata>>();

        let bit_number_location = Location::register(instruction.meta_data.reg_x);
        let mut bit_number = self.read(bit_number_location, Size::Long);
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn BTST_ext_word(&mut self) {
        let size = self.instruction.size();
        let instruction = self.instruction::<Instruction<AddrModeImmediateMetadata>>();

        let mut bit_number = instruction.meta_data.immediate_data;
        match size {
            Size::Byte => bit_number %= 8,
            Size::Long => bit_number %= 32,
            Size::Word => panic!("bit change: wrong instruction size"),
        };

        self.current_addr_mode = instruction.meta_data.addr_mode;
        self.call_addressing_mode();

        let bit = (self.ea_operand >> bit_number) & 1;

        self.set_status(Status::Z, bit == 0);
    }

    pub(crate) fn ILLEAGL(&mut self) {
        self.prepare_exception();
        self.pc = self.vector_table.illegal_instruction();
    }
}

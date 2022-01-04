extern crate lazy_static;

use crate::hardware::cpu::instruction_set::instruction_data_types::AddrModeImmediateMetadata;
use crate::hardware::cpu::instruction_set::generators::*;
use crate::hardware::cpu::instruction_set::instruction_data_types::ConditionDisplacementMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::RyExtWordMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::RyMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::AddrModeMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::RxDataMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::RxRyMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::AddrModeExtWordMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::RxAddrModeMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::MoveInstructionMetadata;
use crate::hardware::cpu::instruction_set::instruction_data_types::ExplicitMetadata;

use crate::hardware::cpu::instruction_set::InstructionProcess;

use super::RegisterType;
use super::addressing_mode::{AddrMode, AddrModeType};
use super::vector_table::VectorTable;
use super::instruction_set::Instruction;
use super::Condition;

use crate::hardware::bus::bus::Bus;
use crate::hardware::{
    Size, Location, LocationType, sign_extend
};

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
    pub pc: u32,

    sr: u16,
    mode: Mode, // user/supervisor

    clock_counter: i32,

    instruction: Box<dyn InstructionProcess>,
    current_addr_mode: AddrMode,
    current_ext_word: Option<u16>,
    ea_location: Location,
    ea_operand: u32,

    // memory stub
    bus: Bus,
}

#[allow(non_snake_case)]
impl Mc68k {
    pub fn init(bus: Bus) -> Self {
        let ram_ptr = bus.get_rom_ptr();
        let vector_table = VectorTable::init(ram_ptr);

        let stack_ptr = vector_table.reset_stack_pointer();
        let pc = vector_table.reset_program_counter();
        let mut opcode_table: Vec<Box<dyn InstructionProcess>> = vec![
            Box::new(Instruction::new(String::new(), 0, Size::Byte, 34, Mc68k::ILLEAGL, ExplicitMetadata)); 0x10000];

        move_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        addr_mode_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        rx_addr_mode_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        addr_mode_ext_word_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        rx_data_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        rx_ry_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });
        condition_displ_generator::generate().iter().for_each(|instruction| {
            let opcode = instruction.operation_word as usize;
            opcode_table[opcode] = Box::new(instruction.clone());
        });

        let instruction: Box<dyn InstructionProcess> = opcode_table[0].clone();
        let mut reg = [0; 17];
        reg[16] = stack_ptr;

        // TODO call RESET function

        Self {
            opcode_table: opcode_table,
            vector_table: vector_table,

            reg: reg,
            pc: pc,

            sr: 0,
            mode: Mode::Supervisor,

            clock_counter: 0,

            instruction: instruction,
            
            current_addr_mode: AddrMode::new(AddrModeType::Immediate, 0, 0),
            current_ext_word: None,

            ea_location: Location::new(LocationType::Memory, 0x1000000),
            ea_operand: 0,

            bus: bus,
        }
    }

    pub fn clock(&mut self) {
        let instruction_addr = self.pc;
        let operation_word = self.read_memory(self.pc as usize, Size::Word);
        self.increment_pc();

        let mut instruction = self.opcode_table[operation_word as usize].clone();
        instruction.as_mut().fetch_data(self);

        self.instruction = instruction;

        println!("{:08X} {}", instruction_addr, self.instruction.as_ref().disassembly());

        (self.instruction.as_ref().handler())(self);
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
            Size::Long => self.reg[reg],
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
        self.bus.read(address, size)
    }

    fn write_memory(&mut self, address: usize, data: u32, size: Size) {
        self.bus.write(address, data, size);
    }

    pub fn read(&mut self, location: Location, size: Size) -> u32 {
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
        self.set_stack_ptr(self.stack_ptr() - 4);

        let location = Location::new(LocationType::Memory, self.stack_ptr() as usize);
        self.write(location, data, size);
    }

    fn pop(&mut self, size: Size) -> u32 {
        let location = Location::new(LocationType::Memory, self.stack_ptr() as usize);
        let data = self.read(location, size);

        self.set_stack_ptr(self.stack_ptr() + 4);

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
        let mask = match status {
            Status::X => 1 << Status::X as u16,
            Status::N => 1 << Status::N as u16,
            Status::Z => 1 << Status::Z as u16,
            Status::V => 1 << Status::V as u16,
            Status::C => 1 << Status::C as u16,
        };
        self.sr & mask != 0
    }

    fn set_status(&mut self, status: Status, set: bool) {
        let mask = match status {
            Status::X => 1 << Status::X as u16,
            Status::N => 1 << Status::N as u16,
            Status::Z => 1 << Status::Z as u16,
            Status::V => 1 << Status::V as u16,
            Status::C => 1 << Status::C as u16,
        };
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

        self.push(sr_copy, Size::Word);
        self.push(pc_copy, Size::Word); 
    }

    /* ADDRESSING MODES */
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

        let address = (self.read_addr_reg(reg_idx, Size::Long) - decrement) as usize;
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
        let address = self.pc;
        let displacement = self.current_addr_mode.ext_word.unwrap();
        
        self.increment_pc();

        let ea_addr = address.wrapping_add(displacement) as usize;

        self.ea_location = Location::new(LocationType::Memory, ea_addr);
        self.ea_operand = self.read_memory(ea_addr, self.instruction.as_ref().size());
    }

    fn pc_indirect_idx(&mut self) {
        let address = self.pc;
        let brief_ext_word = self.current_addr_mode.brief_ext_word.unwrap();
        let idx_reg = brief_ext_word.register;

        self.increment_pc();

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
        // contains one or two extension words
        // don't understend how to figure out its amount
    }

    /* INSTRUCTION SET */
    // data movement MOVE, MOVE16, MOVEM, MOVEP, MOVEQ, EXG, LEA, PEA, LINK, and UNLK

    pub(crate) fn MOVE(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<MoveInstructionMetadata>>().unwrap().clone();

        let src_addr_mode = instruction.data.src_addr_mode;
        self.current_addr_mode = src_addr_mode;
        self.call_addressing_mode();

        let src_data = self.ea_operand;

        let dst_addr_mode = instruction.data.dst_addr_mode;
        self.current_addr_mode = dst_addr_mode;
        self.call_addressing_mode();

        let dst_address = self.ea_location;
        self.write(dst_address, src_data, size);

        // set status codes
        self.set_status(Status::N, is_negate(src_data, size));
        self.set_status(Status::Z, is_zero(src_data));

        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn MOVEA(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxAddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let src_data = self.ea_operand;

        let location = Location::register(instruction.data.reg_x);

        self.write(location, src_data, size);
    }

    pub(crate) fn MOVEM(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeExtWordMetadata>>().unwrap().clone();

        // расчитать затрагиваемые регистры из register_mask
        let register_mask = instruction.data.ext_word;
        let mut affected_registers = Vec::new();
        if self.current_addr_mode.am_type == AddrModeType::AddrIndPreDec {
            // A7..A0D7..D0
            for i in 0..16 {
                let bit = (register_mask >> i) & 1;
                if bit == 1 {
                    affected_registers.push(i);
                }
            }
        } else {
            // D0..D7A0..A7
            for i in 0..16 {
                let bit = (register_mask >> i) & 1;
                if bit == 1 {
                    affected_registers.push(15 - i);
                }
            }
        }

        let num_of_regs = affected_registers.len();
        match self.instruction.as_ref().size() {
            Size::Word => self.clock_counter += 4 * num_of_regs as i32,
            Size::Long => self.clock_counter += 8 * num_of_regs as i32,
            Size::Byte => panic!("MOVEM: wrong size for this instruction"),
        }

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let direction_bit = (instruction.operation_word >> 9) & 0x1;
        let current_addr_mode_type = instruction.data.addr_mode.am_type;

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
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxAddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.data.reg_x);
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
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxDataMetadata>>().unwrap().clone();

        let data = sign_extend(instruction.data.data, Size::Byte);

        let location = Location::register(instruction.data.reg_x);
        self.write(location, data, Size::Long);

        self.set_status(Status::N, is_negate(data, Size::Long));
        self.set_status(Status::Z, is_zero(data));

        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn MOVE_to_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeMetadata>>().unwrap().clone();

            self.current_addr_mode = instruction.data.addr_mode;
            self.call_addressing_mode();

            self.sr = self.ea_operand as u16;
            // TODO clear unused bits
        } else {
            // TODO call privilage exception
        }
    }

    pub(crate) fn MOVE_from_SR(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeMetadata>>().unwrap().clone();

            self.current_addr_mode = instruction.data.addr_mode;
            self.call_addressing_mode();

            let data = self.sr as u32;
            self.write(self.ea_location, data, Size::Word);
        } else {
            // TODO call pribilage excaption
        }
    }

    pub(crate) fn MOVE_USP(&mut self) {
        if self.mode == Mode::Supervisor {
            let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RyMetadata>>().unwrap().clone();

            let location = Location::register(instruction.data.reg_y);
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
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let data = self.ea_operand & 0xFF;

        self.sr &= !0xFF;
        self.sr |= data as u16;
    }

    pub(crate) fn EXG(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxRyMetadata>>().unwrap().clone();
        let (a_idx, b_idx) = {
            let reg_x = instruction.data.reg_x;
            let reg_y = instruction.data.reg_y;

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
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxAddrModeMetadata>>().unwrap().clone();
        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.data.reg_x);

        self.write(location, self.ea_location.address as u32, Size::Long);
    }

    pub(crate) fn PEA(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeMetadata>>().unwrap().clone();
        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        self.push(self.ea_location.address as u32, Size::Long);
    }

    pub(crate) fn LINK(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RyExtWordMetadata>>().unwrap().clone();

        let location = Location::register(instruction.data.reg_y);
        let data = self.read(location, Size::Long);
        self.push(data, Size::Long);
        
        self.write(location, self.stack_ptr(), Size::Long);

        let displacement = instruction.data.ext_word;
        let displacement = sign_extend(displacement, Size::Word);
        let new_stack_ptr = self.stack_ptr().wrapping_add(displacement);

        self.set_stack_ptr(new_stack_ptr);
    }

    pub(crate) fn UNLK(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RyMetadata>>().unwrap().clone();
        let location = Location::register(instruction.data.reg_y);
        let data = self.read(location, Size::Long);
        self.set_stack_ptr(data);

        let data = self.pop(Size::Long);
        self.write(location, data, Size::Long);
    }

    pub(crate) fn TST(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let is_negate = is_negate(self.ea_operand, self.instruction.as_ref().size());
        let is_zero = is_zero(self.ea_operand);

        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, false);
        self.set_status(Status::C, false);
    }

    pub(crate) fn Bcc(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<ConditionDisplacementMetadata>>().unwrap().clone();

        let offset = instruction.data.displacement;
        let condition = instruction.data.condition;

        let result = self.check_condition(condition);

        if result {
            self.pc = self.pc.wrapping_add(offset);
        } else {
            let clock_corection = match instruction.data.displacement_size {
                Size::Byte => -2,
                Size::Word => 2,
                Size::Long => panic!("Bcc: unexpected displacement size"),
            };
            self.clock_counter += clock_corection;
        }
    }

    pub(crate) fn ADD(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxAddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.data.reg_x);
        let data = self.read(location, size);

        let result = match size {
            Size::Byte => (self.ea_operand + data) & 0xFF,
            Size::Word => (self.ea_operand + data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_add(data),
        };

        let direction_bit = (instruction.operation_word >> 8) & 0x1;
        let (carry, overflow) = if direction_bit == 1 { // Memory to register
            self.write(self.ea_location, result, size);
            let sm = get_msb(data, size);
            let dm = get_msb(self.ea_operand, size);
            let rm = get_msb(result, size);

            let carry = sm && dm && !rm || !sm && !dm && rm;
            let overflow = sm && dm || !rm && dm || sm && !rm;
            (carry, overflow)
        } else { // Register to memory
            self.write(location, result, size);
            let sm = get_msb(self.ea_operand, size);
            let dm = get_msb(data, size);
            let rm = get_msb(result, size);
    
            let carry = sm && dm && !rm || !sm && !dm && rm;
            let overflow = sm && dm || !rm && dm || sm && !rm;
            (carry, overflow)
        };

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ADDA(&mut self) {
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<RxAddrModeMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let location = Location::register(instruction.data.reg_x);
        let data = self.read(location, self.instruction.as_ref().size());

        let result = self.ea_operand.wrapping_add(data);
        self.write(location, result, self.instruction.as_ref().size());
    }

    pub(crate) fn ADDI(&mut self) {
        let size = self.instruction.as_ref().size();
        let instruction = self.instruction.as_ref().as_any().downcast_ref::<Instruction<AddrModeImmediateMetadata>>().unwrap().clone();

        self.current_addr_mode = instruction.data.addr_mode;
        self.call_addressing_mode();

        let data = instruction.data.immediate_data;

        let result = match size {
            Size::Byte => (self.ea_operand + data) & 0xFF,
            Size::Word => (self.ea_operand + data) & 0xFFFF,
            Size::Long => self.ea_operand.wrapping_add(data),
        };

        self.write(self.ea_location, result, size);
        let sm = get_msb(data, size);
        let dm = get_msb(self.ea_operand, size);
        let rm = get_msb(result, size);

        let carry = sm && dm && !rm || !sm && !dm && rm;
        let overflow = sm && dm || !rm && dm || sm && !rm;

        let is_negate = is_negate(result, size);
        let is_zero = is_zero(result);

        self.set_status(Status::X, carry);
        self.set_status(Status::N, is_negate);
        self.set_status(Status::Z, is_zero);
        self.set_status(Status::V, overflow);
        self.set_status(Status::C, carry);
    }

    pub(crate) fn ADDQ(&mut self) {}

    pub(crate) fn ADDX(&mut self) {}

    pub(crate) fn CLR(&mut self) {}

    pub(crate) fn CMP(&mut self) {}

    pub(crate) fn CMPA(&mut self) {}

    pub(crate) fn CMPI(&mut self) {}

    pub(crate) fn CMPM(&mut self) {}

    pub(crate) fn CMP2(&mut self) {}

    pub(crate) fn DIVS(&mut self) {}

    pub(crate) fn DIVU(&mut self) {}

    pub(crate) fn DIVSL(&mut self) {}

    pub(crate) fn DIVUL(&mut self) {}

    pub(crate) fn EXT(&mut self) {}

    pub(crate) fn EXTB(&mut self) {}

    pub(crate) fn MULS(&mut self) {}

    pub(crate) fn MULU(&mut self) {}

    pub(crate) fn NEG(&mut self) {}

    pub(crate) fn NEGX(&mut self) {}

    pub(crate) fn SUB(&mut self) {}

    pub(crate) fn SUBA(&mut self) {}

    pub(crate) fn SUBI(&mut self) {}

    pub(crate) fn SUBQ(&mut self) {}

    pub(crate) fn SUBX(&mut self) {}

    pub(crate) fn ILLEAGL(&mut self) {
        self.prepare_exception();
        self.pc = self.vector_table.illegal_instruction();
    }
}

fn is_negate(data: u32, size: Size) -> bool {
    match size {
        Size::Byte => data & 0x80 != 0,
        Size::Word => data & 0x8000 != 0,
        Size::Long => data & 0x80000000 != 0,
    }
}

fn is_zero(data: u32) -> bool {
    data == 0
}

fn get_msb(data: u32, size: Size) -> bool {
    match size {
        Size::Byte => data & 0x80 != 0,
        Size::Word => data & 0x8000 != 0,
        Size::Long => data & 0x80000000 != 0,
    }
}


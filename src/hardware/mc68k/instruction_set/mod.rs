use crate::hardware::sign_extend;
use std::any::Any;

use crate::Mc68k;

pub(in crate::hardware) mod instruction_meta_data_types;
pub(in crate::hardware) mod generators;

mod addr_mode_table;

use crate::hardware::{Size, LocationType, Location};

use instruction_meta_data_types::*;

use super::Mc68kBus;

pub(in crate::hardware) trait InstructionProcess: InstructionBoxedClone + InstructionData {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k);
    fn disassembly(&self) -> String;
}

pub(in crate::hardware) trait InstructionBoxedClone {
    fn clone_box(&self) -> Box<dyn InstructionProcess>;
}

impl<T> InstructionBoxedClone for T where T: 'static + InstructionProcess + Clone {
    fn clone_box(&self) -> Box<dyn InstructionProcess> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn InstructionProcess> {
    fn clone(&self) -> Box<dyn InstructionProcess> {
        self.clone_box()
    }
}

pub(in crate::hardware) trait InstructionData {
    fn handler(&self) -> fn(&mut Mc68k);
    fn size(&self) -> Size;
    fn operation_word(&self) -> u16;
    fn as_any(&self) -> &dyn Any;
}

impl<T> InstructionData for Instruction<T> where T: 'static {
    fn handler(&self) -> fn(&mut Mc68k) {
        self.handler
    }

    fn size(&self) -> Size {
        self.size
    }

    fn operation_word(&self) -> u16 {
        self.operation_word
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Instruction<T> {
    name: &'static str,
    pub operation_word: u16,
    size: Size,
    clock: u32,
    pub meta_data: T,
    pub handler: fn(&mut Mc68k),
}

impl<T> Instruction<T> {
    pub fn new(name: &'static str, operation_word: u16, size: Size, clock: u32, handler: fn(&mut Mc68k), meta_data: T) -> Self {
        Self {
            name: name,
            operation_word: operation_word, 
            size: size,
            clock: clock,
            handler: handler,
            meta_data: meta_data,
        } 
    }
}

impl InstructionProcess for Instruction<AddrModeMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        self.meta_data.addr_mode.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        format!("{}.{} {}", self.name, self.size, self.meta_data.addr_mode)
    }
}

impl InstructionProcess for Instruction<MoveInstructionMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        self.meta_data.src_addr_mode.fetch_ext_word(cpu, self.size);
        self.meta_data.dst_addr_mode.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.src_addr_mode, self.meta_data.dst_addr_mode)
    }
}

impl InstructionProcess for Instruction<ExplicitMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) {

    }

    fn disassembly(&self) -> String {
        String::from(self.name)
    }
}

impl InstructionProcess for Instruction<AddrModeImmediateMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        
        let size = match self.size {
            Size::Byte => Size::Word,
            _ => self.size,
        };
        let data = cpu.read(location, size);
        self.meta_data.immediate_data = data;

        cpu.increment_pc();
        match self.size {
            Size::Long => {
                cpu.increment_pc();
            },
            _ => (),
        };

        self.meta_data.addr_mode.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        match self.size {
            Size::Byte | Size::Word => format!("{}.{} 0x{:04X} {}", self.name, self.size, self.meta_data.immediate_data, self.meta_data.addr_mode),
            Size::Long => format!("{}.{} 0x{:08X} {}", self.name, self.size, self.meta_data.immediate_data, self.meta_data.addr_mode),
        }
    }
}

impl InstructionProcess for Instruction<AddrModeDataMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) {

    }

    fn disassembly(&self) -> String {
        format!("{}.{} #{} {}", self.name, self.size, self.meta_data.data, self.meta_data.addr_mode)
    }
}

impl InstructionProcess for Instruction<AddrModeExtWordMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let data = cpu.read(location, Size::Word);

        self.meta_data.ext_word = data as u16;

        cpu.increment_pc();

        self.meta_data.addr_mode.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        format!("{}.{} 0x{:04X} {}", self.name, self.size, self.meta_data.ext_word, self.meta_data.addr_mode)
    }
}

impl InstructionProcess for Instruction<RxAddrModeMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        self.meta_data.addr_mode.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.reg_x, self.meta_data.addr_mode)
    }
}

impl InstructionProcess for Instruction<RyMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { 
        
    }
    
    fn disassembly(&self) -> std::string::String {
        format!("{}.{} {}", self.name, self.size, self.meta_data.reg_y)
    }
}

impl InstructionProcess for Instruction<RyExtWordMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let data = cpu.read(location, Size::Word);

        cpu.increment_pc();

        self.meta_data.ext_word = data;
    }

    fn disassembly(&self) -> String {
        format!("{}.{} {} 0x{:04X}", self.name, self.size, self.meta_data.reg_y, self.meta_data.ext_word)
    }
}

impl InstructionProcess for Instruction<VectorMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<DataAddrModeMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionAddrModeMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionRyExtWordMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let ext_word = cpu.read(location, Size::Word);
        cpu.increment_pc();

        self.meta_data.ext_word = ext_word;
    }
    
    fn disassembly(&self) -> std::string::String {
        format!("{} {}, {} 0x{:04X}", self.name, self.meta_data.condition, self.meta_data.reg_y, self.meta_data.ext_word)
    }
}

impl InstructionProcess for Instruction<DisplacementMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionDisplacementMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let mut displacement = self.operation_word as u8 as u32;
        displacement = sign_extend(displacement, Size::Byte);
        if displacement == 0 {
            let location = Location::memory(cpu.pc as usize);
            displacement = cpu.read(location, Size::Word);
            displacement = sign_extend(displacement, Size::Word);
        }
        self.meta_data.displacement = displacement;
    }

    fn disassembly(&self) -> String {
        format!("{} {} 0x{:04X}", self.name, self.meta_data.condition, self.meta_data.displacement)
    }
}

impl InstructionProcess for Instruction<RxDataMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) {}
    fn disassembly(&self) -> String { 
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.reg_x, self.meta_data.data)
    }
}

impl InstructionProcess for Instruction<RxRyMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) { }

    fn disassembly(&self) -> std::string::String { 
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.reg_x, self.meta_data.reg_y)
    }
}

impl InstructionProcess for Instruction<RxRySpecAddrModeMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        self.meta_data.addr_mode_x.fetch_ext_word(cpu, self.size);
        self.meta_data.addr_mode_y.fetch_ext_word(cpu, self.size);
    }

    fn disassembly(&self) -> String {
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.addr_mode_x, self.meta_data.addr_mode_y)
    }
}

impl InstructionProcess for Instruction<RotationRyMetadata> {
    fn fetch_decode_instruction_data(&mut self, _: &mut Mc68k) {

    }

    fn disassembly(&self) -> String {
        format!("{}.{} {} {}", self.name, self.size, self.meta_data.counter, self.meta_data.reg_y)
    }
}

impl InstructionProcess for Instruction<ExplicitImmediateMetadata> {
    fn fetch_decode_instruction_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let mut data = cpu.read(location, Size::Word);

        match self.size {
            Size::Byte => data &= 0xFF,
            _ => (),
        }

        cpu.increment_pc();

        self.meta_data.immediate_data = data;
    }

    fn disassembly(&self) -> String {
        format!("{} 0x{:02X}", self.name, self.meta_data.immediate_data)
    }
}

pub(in crate::hardware)fn generate() -> Vec<Box<dyn InstructionProcess>> {
    let mut opcode_table: Vec<Box<dyn InstructionProcess>> = vec![
            Box::new(Instruction::new("illegal", 0, Size::Byte, 34, Mc68k::ILLEAGL, ExplicitMetadata)); 0x10000];

    generators::move_generator::generate(&mut opcode_table);
    generators::addr_mode_generator::generate(&mut opcode_table);
    generators::addr_mode_ext_word_generator::generate(&mut opcode_table);
    generators::addr_mode_data_generator::generate(&mut opcode_table);
    generators::addr_mode_immediate_generator::generate(&mut opcode_table);
    generators::explicit_generator::generate(&mut opcode_table);
    generators::explicit_immediate_generator::generate(&mut opcode_table);
    generators::displacement_generator::generate(&mut opcode_table);
    generators::rx_addr_mode_generator::generate(&mut opcode_table);
    generators::rx_data_generator::generate(&mut opcode_table);
    generators::rx_ry_generator::generate(&mut opcode_table);
    generators::rx_ry_spec_addr_mode_generator::generate(&mut opcode_table);
    generators::ry_generator::generate(&mut opcode_table);
    generators::ry_ext_word_generator::generate(&mut opcode_table);
    generators::condition_displ_generator::generate(&mut opcode_table);
    generators::condition_ry_ext_word_generator::generate(&mut opcode_table);
    generators::condition_addr_mode_generator::generate(&mut opcode_table);
    generators::shifting_rotation_generator::generate(&mut opcode_table);

    opcode_table
}
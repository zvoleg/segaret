use std::any::Any;

use crate::Mc68k;

pub(in crate::hardware) mod instruction_data_types;
pub(in crate::hardware) mod generators;

mod addr_mode_table;

use crate::hardware::{Size, LocationType, Location};

use instruction_data_types::*;

pub(in crate::hardware) trait InstructionProcess: InstructionBoxedClone + InstructionData {
    fn fetch_data(&mut self, cpu: &mut Mc68k);
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
    fn as_any(&self) -> &dyn Any;
}

impl<T> InstructionData for Instruction<T> where T: 'static {
    fn handler(&self) -> fn(&mut Mc68k) {
        self.handler
    }

    fn size(&self) -> Size {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct Instruction<T> {
    name: String,
    pub operation_word: u16,
    size: Size,
    clock: u32,
    pub data: T,
    pub handler: fn(&mut Mc68k),
}

impl<T> Instruction<T> {
    pub fn new(name: String, operation_word: u16, size: Size, clock: u32, handler: fn(&mut Mc68k), data: T) -> Self {
        Self {
            name: name,
            operation_word: operation_word, 
            size: size,
            clock: clock,
            handler: handler,
            data: data,
        } 
    }
}

impl InstructionProcess for Instruction::<AddrModeMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        self.data.addr_mode.fetch_ext_word(cpu);
    }

    fn disassembly(&self) -> String {
        String::from(format!("{}.{} {}", self.name, self.size(), self.data.addr_mode))
    }
}

impl InstructionProcess for Instruction::<MoveInstructionMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        self.data.src_addr_mode.fetch_ext_word(cpu);
        self.data.dst_addr_mode.fetch_ext_word(cpu);
    }

    fn disassembly(&self) -> String {
        String::from(format!("{}.{} {} {}", self.name, self.size, self.data.src_addr_mode, self.data.dst_addr_mode))
    }
}

impl InstructionProcess for Instruction::<ExplicitMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {

    }

    fn disassembly(&self) -> String {
        self.name.clone()
    }
}

impl InstructionProcess for Instruction<AddrModeImmediateMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::new(LocationType::Memory, cpu.pc as usize);
        
        let size = match self.size {
            Size::Byte => Size::Word,
            _ => self.size,
        };
        let data = cpu.read(location, size);
        self.data.immediate_data = data;

        cpu.increment_pc();
        match self.size {
            Size::Long => {
                cpu.increment_pc();
            },
            _ => (),
        };

        self.data.addr_mode.fetch_ext_word(cpu);
    }

    fn disassembly(&self) -> String {
        match self.size {
            Size::Byte | Size::Word => String::from(format!("{}.{} {:04X} {}", self.name, self.size, self.data.immediate_data, self.data.addr_mode)),
            Size::Long => String::from(format!("{}.{} {:08X} {}", self.name, self.size, self.data.immediate_data, self.data.addr_mode)),
        }
    }
}

impl InstructionProcess for Instruction<AddrModeExtWordMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let data = cpu.read(location, Size::Word);

        self.data.ext_word = data as u16;

        cpu.increment_pc();

        self.data.addr_mode.fetch_ext_word(cpu);
    }

    fn disassembly(&self) -> String {
        String::from(format!("{}.{} {:04X} {}", self.name, self.size, self.data.ext_word, self.data.addr_mode))
    }
}

impl InstructionProcess for Instruction<RxAddrModeMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        self.data.addr_mode.fetch_ext_word(cpu);
    }

    fn disassembly(&self) -> String {
        String::from(format!("{}.{} {} {}", self.name, self.size, self.data.reg_x, self.data.addr_mode))
    }
}

impl InstructionProcess for Instruction<RyMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<RyExtWordMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        let location = Location::memory(cpu.pc as usize);
        let data = cpu.read(location, Size::Word);

        cpu.increment_pc();

        self.data.ext_word = data;
    }

    fn disassembly(&self) -> String {
        String::from(format!("{}.{} {} {:04X}", self.name, self.size, self.data.reg_y, self.data.ext_word))
    }
}

impl InstructionProcess for Instruction<VectorMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<DataAddrModeMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionAddrModeMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionRyMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<DisplacementMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<ConditionDisplacementMetadata> {
    fn fetch_data(&mut self, cpu: &mut Mc68k) {
        let mut displacement = self.operation_word as u8 as u32;
        if displacement == 0 {
            let location = Location::memory(cpu.pc as usize);
            displacement = cpu.read(location, Size::Word);
            
            cpu.increment_pc();
        }
        self.data.displacement = displacement;
    }

    fn disassembly(&self) -> String {
        String::from(format!("{} {} {:04X}", self.name, self.data.condition, self.data.displacement))
    }
}

impl InstructionProcess for Instruction<RxDataMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) {}
    fn disassembly(&self) -> String { 
        String::from(format!("{}.{} {} {}", self.name, self.size, self.data.reg_x, self.data.data))
    }
}

impl InstructionProcess for Instruction<RxRyMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

impl InstructionProcess for Instruction<RotationRyMetadata> {
    fn fetch_data(&mut self, _: &mut Mc68k) { todo!() }
    fn disassembly(&self) -> std::string::String { todo!() }
}

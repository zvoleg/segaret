use crate::{
    addressing_mode_set::AddressingMode, bus::BusM68k, instruction_set::Instruction, primitives::{memory::MemoryPtr, Pointer, Size}
};

/// Operation is composition of an instruction and the addressing modes
/// Also an Operation contains information about cycles amount
pub(crate) struct Operation<T: BusM68k> {
    pub(crate) instruction: Box<dyn Instruction<T>>,
    pub(crate) addressing_mode_list: Vec<Box<dyn AddressingMode>>,
    pub(crate) cycles: i32,
}

impl<T> Operation<T> where T: BusM68k {
    pub(crate) fn new(
        instruction: Box<dyn Instruction<T>>,
        addressing_mode_list: Vec<Box<dyn AddressingMode>>,
        cycles: i32,
    ) -> Self {
        Self {
            instruction,
            addressing_mode_list,
            cycles,
        }
    }

    pub(crate) fn disassembly(&self, address: u32, opcode_ptr: MemoryPtr) -> String {
        let mut offset = 0;
        let mut disassembly_parts = Vec::new();
        disassembly_parts.push(format!("{:08X}:", address));

        let mut raw_bytes = Vec::new();
        let opcode = opcode_ptr.read_offset(Size::Word, offset);
        offset += Size::Word as isize;
        raw_bytes.push(format!("{:04x}", opcode));

        let mut am_disassembly_parts = Vec::new();
        for am in &self.addressing_mode_list {
            let extension_words_amount = am.extension_word_length();
            if extension_words_amount == 1 {
                let extension_word = opcode_ptr.read_offset(Size::Word, offset);
                offset += Size::Word as isize;
                raw_bytes.push(format!("{:04x}", extension_word));
                am_disassembly_parts.push(am.disassembly(extension_word))
            } else if extension_words_amount == 2 {
                let extension_word = opcode_ptr.read_offset(Size::Long, offset);
                offset += Size::Long as isize;
                raw_bytes.push(format!("{:08x}", extension_word));
                am_disassembly_parts.push(am.disassembly(extension_word));
            } else {
                am_disassembly_parts.push(am.disassembly(0));
            }
        }
        let raw_bytes_string = raw_bytes.join(" ");
        disassembly_parts.push(format!("{raw:<23}:", raw = raw_bytes_string));

        disassembly_parts.push(self.instruction.to_string());
        disassembly_parts.append(&mut am_disassembly_parts);

        String::from(disassembly_parts.join(" "))
    }
}

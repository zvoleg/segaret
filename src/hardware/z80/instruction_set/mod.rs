use super::z80_emu::Z80Emu;

pub(in crate::hardware::z80) trait InstructionDecoder {
    fn decode_and_fetch_instruction_data(&mut self, cpu: Z80Emu);
}

pub(in crate::hardware::z80) trait InstructionDisassembler {
    fn disassembly(&self) -> &'static str;
}

pub(in crate::hardware::z80) trait InstructionBoxedClone {
    fn clone_box(&self) -> Box<Self>;
}

pub(in crate::hardware::z80) trait Instruction {
    
}
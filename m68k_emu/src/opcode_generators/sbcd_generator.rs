use crate::{decoder::{Operation, InstructionData, InstructionType}, addressing_mode::AdrMode, Size};

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let base_mask: usize = 0b1000000100000000;
    for mode in 0..=1 {
        for reg_y in 0..8 {
            for reg_x in 0..8 {
                let mask = reg_x << 9 | mode << 3 | reg_y;
                let opcode = base_mask | mask;
                let inst_data = match mode {
                    0 => InstructionData::SrcDstAm(AdrMode::DataReg(reg_y), AdrMode::DataReg(reg_x)),
                    1 => InstructionData::SrcDstAm(AdrMode::AdrRegIndPreDec(reg_y), AdrMode::AdrRegIndPreDec(reg_x)),
                    _ => panic!("generator_sbcd: unexpected mode bit {}", mode),
                };
                let clocks = if mode == 0 {
                    6 + AdrMode::DataReg(reg_y).additional_clocks(Size::Byte)
                } else {
                    18 + AdrMode::AdrRegIndPreDec(reg_y).additional_clocks(Size::Byte)
                };
                let inst = Operation::new(
                    opcode as u16, 
                    "SBCD",
                    InstructionType::SBCD,
                    inst_data,
                    Size::Byte,
                    false,
                    clocks,
                );
                table[opcode] = inst;
            }
        }
    }
}

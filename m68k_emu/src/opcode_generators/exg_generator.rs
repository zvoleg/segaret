use crate::{
    addressing_mode_set::{AddressRegister, AddressingMode, DataRegister},
    bus::BusM68k,
    instruction_set::{data_movement::EXG, ExchangeMode},
    operation::Operation,
    primitives::Size,
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for EXG {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b1100000100000000;
        base_mask |= (self.mode as usize) << 3;
        base_mask
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    for reg_x in 0..8 {
        for mode in [
            ExchangeMode::DataToData,
            ExchangeMode::AddressToAddress,
            ExchangeMode::DataToAddress,
        ] {
            for reg_y in 0..8 {
                let instruction = Box::new(EXG { mode: mode });
                let src_am: Box<dyn AddressingMode>;
                let dst_am: Box<dyn AddressingMode>;
                match mode {
                    ExchangeMode::DataToData => {
                        src_am = Box::new(DataRegister {
                            reg: reg_x,
                            size: Size::Long,
                        });
                        dst_am = Box::new(DataRegister {
                            reg: reg_y,
                            size: Size::Long,
                        });
                    }
                    ExchangeMode::AddressToAddress => {
                        src_am = Box::new(AddressRegister {
                            reg: reg_x,
                            size: Size::Long,
                        });
                        dst_am = Box::new(AddressRegister {
                            reg: reg_y,
                            size: Size::Long,
                        });
                    }
                    ExchangeMode::DataToAddress => {
                        src_am = Box::new(DataRegister {
                            reg: reg_x,
                            size: Size::Long,
                        });
                        dst_am = Box::new(AddressRegister {
                            reg: reg_y,
                            size: Size::Long,
                        });
                    }
                }

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | (reg_x << 9) | reg_y;

                let operation = Operation::new(instruction, vec![src_am, dst_am], 6);
                table[opcode] = operation;
            }
        }
    }
}

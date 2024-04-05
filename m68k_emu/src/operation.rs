use crate::{
    addressing_mode_set::AddressingMode, bus::BusM68k, instruction_set::Instruction,
    operand::Operand,
};

/// Operation is composition of an instruction and the addressing modes
/// Also an Operation contains information about cycles amount
pub(crate) struct Operation<T: BusM68k> {
    pub(crate) instruction: Box<dyn Instruction<T>>,
    pub(crate) addressing_mode_list: Vec<Box<dyn AddressingMode>>,
    pub(crate) cycles: u32,
}

impl<T: BusM68k> Operation<T> {
    pub(crate) fn new(
        instruction: Box<dyn Instruction<T>>,
        addressing_mode_list: Vec<Box<dyn AddressingMode>>,
        cycles: u32,
    ) -> Self {
        Self {
            instruction,
            addressing_mode_list,
            cycles,
        }
    }
}

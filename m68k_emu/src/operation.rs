use crate::{addressing_mode_set::AddressingMode, instruction_set::Instruction};

/// Operation is composition of an instruction and the addressing modes
/// Also an Operation contains information about cycles amount
pub(crate) struct Operation {
    pub(crate) instruction: Box<dyn Instruction>,
    pub(crate) addressing_mode_list: Vec<Box<dyn AddressingMode>>,
    pub(crate) cycles: i32,
}

impl Operation {
    pub(crate) fn new(
        instruction: Box<dyn Instruction>,
        addressing_mode_list: Vec<Box<dyn AddressingMode>>,
        cycles: i32,
    ) -> Self {
        Self {
            instruction,
            addressing_mode_list,
            cycles,
        }
    }
}

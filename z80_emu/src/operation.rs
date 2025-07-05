use std::fmt::Display;

use crate::{addressing_mode::AddressingMode, bus::BusZ80, instruction_set::Instruction};

pub(crate) struct Operation<T>
where
    T: BusZ80,
{
    pub(crate) instruction: Box<dyn Instruction<T>>,
    pub(crate) dst_am: Option<Box<dyn AddressingMode<T>>>,
    pub(crate) src_am: Option<Box<dyn AddressingMode<T>>>,
}

impl<T> Operation<T>
where
    T: BusZ80,
{
    pub(crate) fn new(
        instruction: Box<dyn Instruction<T>>,
        dst_am: Option<Box<dyn AddressingMode<T>>>,
        src_am: Option<Box<dyn AddressingMode<T>>>,
    ) -> Self {
        Self {
            instruction,
            dst_am,
            src_am,
        }
    }
}

impl<T: BusZ80> Display for Operation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instruction)
    }
}

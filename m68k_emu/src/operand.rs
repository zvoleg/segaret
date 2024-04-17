use std::{collections::VecDeque, fmt::Display};

use crate::{addressing_mode_set::AddressingModeType, primitives::Pointer, Size};

/// The Operand is representation of data which handled by an instruction
///
/// `operand_ptr` is a memory location of data
///
/// `address_register_ptr` is the pointer to the address register from which an memory address was calculated. The instruction some times needs to access to this data (b.e. MOVEM, LINK)
///
/// `operand_address` is the calculated memory address
pub(crate) struct Operand {
    pub(crate) operand_ptr: Box<dyn Pointer>,
    pub(crate) address_register_ptr: Option<Box<dyn Pointer>>,
    pub(crate) operand_address: u32,
    size: Size,
    addressing_mode_type: AddressingModeType,
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.addressing_mode_type {
            AddressingModeType::DataRegister => format!("D{}", self.operand_address),
            AddressingModeType::AddressRegister => format!("A{}", self.operand_address),
            AddressingModeType::AddressRegisterIndirect => format!("(A{})", self.operand_address),
            AddressingModeType::AddressRegisterPostIncrement => {
                format!("(A{})+", self.operand_address)
            }
            AddressingModeType::AddressRegisterPreDecrement => {
                format!("-(A{})", self.operand_address)
            }
            AddressingModeType::AddressRegisterDisplacement => {
                let address = self.address_register_ptr.as_ref().unwrap().read(Size::Long);
                let displacement = self.operand_address.wrapping_sub(address);
                format!("({:04X}, A)", displacement)
            },
            AddressingModeType::AddressRegisterIndexed => todo!(),
            AddressingModeType::ProgramCounterDisplacement => todo!(),
            AddressingModeType::ProgramCounterIndexed => todo!(),
            AddressingModeType::AbsShort => format!("{:04X}", self.operand_address),
            AddressingModeType::AbsLong => format!("{:08X}", self.operand_address),
            AddressingModeType::Immediate => format!("{:08X}", self.operand_ptr.read(self.size)),
        };
        write!(f, "{}", s)
    }
}

impl Operand {
    pub(crate) fn new(
        operand_ptr: Box<dyn Pointer>,
        address_register_ptr: Option<Box<dyn Pointer>>,
        operand_address: u32,
        size: Size,
        addressing_mode_type: AddressingModeType,
    ) -> Self {
        Self {
            operand_ptr,
            address_register_ptr,
            operand_address,
            size,
            addressing_mode_type,
        }
    }

    pub(crate) fn read(&self) -> u32 {
        self.operand_ptr.read(self.size)
    }

    pub(crate) fn read_sized(&self, size: Size) -> u32 {
        self.operand_ptr.read(size)
    }

    pub(crate) fn write(&self, data: u32) {
        self.operand_ptr.write(data, self.size);
    }

    pub(crate) fn write_sized(&self, data: u32, size: Size) {
        self.operand_ptr.write(data, size);
    }
}

pub(crate) struct OperandSet {
    operands: VecDeque<Operand>,
}

impl OperandSet {
    pub(crate) fn new() -> Self {
        Self {
            operands: VecDeque::new(),
        }
    }

    pub(crate) fn add(&mut self, operand: Operand) {
        self.operands.push_front(operand);
    }

    pub(crate) fn next(&mut self) -> Operand {
        self.operands.pop_back().unwrap()
    }
}

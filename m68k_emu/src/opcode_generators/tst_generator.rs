use crate::{
    addressing_mode_set::AddressingModeType, bus::BusM68k, instruction_set::program_control::TST, operation::Operation, primitives::Size, range
};

use super::OpcodeMaskGenerator;

impl OpcodeMaskGenerator for TST {
    fn generate_mask(&self) -> usize {
        let mut base_mask = 0b0100101000000000;
        base_mask |= match self.size {
            Size::Byte => 0b00,
            Size::Word => 0b01,
            Size::Long => 0b10,
        } << 6;
        base_mask
    }
}

pub(crate) fn generate<T: BusM68k>(table: &mut [Operation<T>]) {
    let am_types = [
        AddressingModeType::DataRegister,
        AddressingModeType::AddressRegisterIndirect,
        AddressingModeType::AddressRegisterPostIncrement,
        AddressingModeType::AddressRegisterPreDecrement,
        AddressingModeType::AddressRegisterDisplacement,
        AddressingModeType::AddressRegisterIndexed,
        AddressingModeType::AbsShort,
        AddressingModeType::AbsLong,
    ];

    for size in [Size::Byte, Size::Word, Size::Long] {
        for am_type in am_types {
            for idx in range!(am_type) {
                let instruction = Box::new(TST { size: size });
                let am = am_type.addressing_mode_by_type(idx, size);

                let base_mask = instruction.generate_mask();
                let opcode = base_mask | am_type.generate_mask(idx);

                let cycles = 4 + am_type.additional_clocks(size);

                let operation = Operation::new(instruction, vec![am], cycles);
                table[opcode] = operation;
            }
        }
    }
}

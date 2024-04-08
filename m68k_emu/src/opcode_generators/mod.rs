use crate::{
    addressing_mode_set::{
        AbsLong, AbsShort, AddressRegister, AddressRegisterDisplacement, AddressRegisterIndexed,
        AddressRegisterIndirect, AddressRegisterPostIncrement, AddressRegisterPreDecrement,
        AddressingMode, AddressingModeType, DataRegister, Immediate, ProgramCounterDisplacement,
        ProgramCounterIndexed,
    },
    operation::Operation,
    primitives::Size,
};

// pub(crate) mod abcd_generator;
pub(crate) mod add_generator;
pub(crate) mod and_generator;
pub(crate) mod asd_generator;
// pub(crate) mod bcc_generator;
pub(crate) mod bchg_generator;
pub(crate) mod bclr_generator;
pub(crate) mod bra_generator;
pub(crate) mod bset_generator;
pub(crate) mod bsr_generator;
pub(crate) mod btst_generator;
pub(crate) mod chk_generator;
pub(crate) mod clr_generator;
pub(crate) mod cmp_generator;
// pub(crate) mod dbcc_generator;
pub(crate) mod div_generator;
pub(crate) mod eor_generator;
pub(crate) mod exg_generator;
pub(crate) mod ext_generator;
pub(crate) mod illegal_generator;
pub(crate) mod jmp_generator;
pub(crate) mod jsr_generator;
pub(crate) mod lea_generator;
pub(crate) mod link_generator;
pub(crate) mod lsd_generator;
pub(crate) mod move_generator;
pub(crate) mod movem_generator;
pub(crate) mod mul_generator;
// pub(crate) mod nbcd_generator;
pub(crate) mod neg_generator;
pub(crate) mod nop_generator;
pub(crate) mod not_generator;
pub(crate) mod or_generator;
pub(crate) mod pea_generator;
pub(crate) mod rod_generator;
pub(crate) mod rtr_generator;
pub(crate) mod rts_generator;
// pub(crate) mod sbcd_generator;
// pub(crate) mod scc_generator;
pub(crate) mod sub_generator;
pub(crate) mod swap_generator;
// pub(crate) mod tas_generator;
// pub(crate) mod trap_generator;
pub(crate) mod tst_generator;
pub(crate) mod unlk_generator;

trait OpcodeMaskGenerator {
    fn generate_mask(&self) -> usize;
}

impl AddressingModeType {
    fn addressing_mode_by_type(&self, reg_idx: usize, size: Size) -> Box<dyn AddressingMode> {
        match self {
            AddressingModeType::DataRegister => Box::new(DataRegister { reg: reg_idx }),
            AddressingModeType::AddressRegister => Box::new(AddressRegister { reg: reg_idx }),
            AddressingModeType::AddressRegisterIndirect => {
                Box::new(AddressRegisterIndirect { reg: reg_idx })
            }
            AddressingModeType::AddressRegisterPostIncrement => {
                Box::new(AddressRegisterPostIncrement {
                    reg: reg_idx,
                    size: size,
                })
            }
            AddressingModeType::AddressRegisterPreDecrement => {
                Box::new(AddressRegisterPreDecrement {
                    reg: reg_idx,
                    size: size,
                })
            }
            AddressingModeType::AddressRegisterDisplacement => {
                Box::new(AddressRegisterDisplacement { reg: reg_idx })
            }
            AddressingModeType::AddressRegisterIndexed => {
                Box::new(AddressRegisterIndexed { reg: reg_idx })
            }
            AddressingModeType::ProgramCounterDisplacement => {
                Box::new(ProgramCounterDisplacement())
            }
            AddressingModeType::ProgramCounterIndexed => Box::new(ProgramCounterIndexed()),
            AddressingModeType::AbsShort => Box::new(AbsShort()),
            AddressingModeType::AbsLong => Box::new(AbsLong()),
            AddressingModeType::Immediate => Box::new(Immediate { size: size }),
        }
    }

    fn additional_clocks(&self, size: Size) -> u32 {
        let mut cycles = match self {
            AddressingModeType::AddressRegisterIndirect => 4,
            AddressingModeType::AddressRegisterPostIncrement => 4,
            AddressingModeType::AddressRegisterPreDecrement => 6,
            AddressingModeType::AddressRegisterDisplacement => 8,
            AddressingModeType::AddressRegisterIndexed => 10,
            AddressingModeType::ProgramCounterDisplacement => 8,
            AddressingModeType::ProgramCounterIndexed => 10,
            AddressingModeType::AbsShort => 8,
            AddressingModeType::AbsLong => 12,
            AddressingModeType::Immediate => 4,
            _ => 0,
        };
        if size == Size::Long {
            match self {
                AddressingModeType::DataRegister | AddressingModeType::AddressRegister => (),
                _ => cycles += 4,
            }
        }
        cycles
    }

    fn generate_mask(&self, reg_idx: usize) -> usize {
        match self {
            AddressingModeType::DataRegister => 0b000000 | reg_idx,
            AddressingModeType::AddressRegister => 0b001000 | reg_idx,
            AddressingModeType::AddressRegisterIndirect => 0b010000 | reg_idx,
            AddressingModeType::AddressRegisterPostIncrement => 0b011000 | reg_idx,
            AddressingModeType::AddressRegisterPreDecrement => 0b100000 | reg_idx,
            AddressingModeType::AddressRegisterDisplacement => 0b101000 | reg_idx,
            AddressingModeType::AddressRegisterIndexed => 0b110000 | reg_idx,
            AddressingModeType::ProgramCounterDisplacement => 0b111010,
            AddressingModeType::ProgramCounterIndexed => 0b111011,
            AddressingModeType::AbsShort => 0b111000,
            AddressingModeType::AbsLong => 0b111001,
            AddressingModeType::Immediate => 0b111100,
        }
    }
}

pub(crate) fn generate_opcode_list(table: &mut [Operation]) {
    bra_generator::generate(table);
    bsr_generator::generate(table);
    jmp_generator::generate(table);
    jsr_generator::generate(table);
    rtr_generator::generate(table);
    rts_generator::generate(table);
    tst_generator::generate(table);
    asd_generator::generate(table);
    lsd_generator::generate(table);
    rod_generator::generate(table);
    swap_generator::generate(table);
    and_generator::generate(table);
    eor_generator::generate(table);
    or_generator::generate(table);
    not_generator::generate(table);
    bchg_generator::generate(table);
    bclr_generator::generate(table);
    bset_generator::generate(table);
    btst_generator::generate(table);
    add_generator::generate(table);
    sub_generator::generate(table);
    clr_generator::generate(table);
    cmp_generator::generate(table);
    ext_generator::generate(table);
    neg_generator::generate(table);
    mul_generator::generate(table);
    div_generator::generate(table);
    exg_generator::generate(table);
    lea_generator::generate(table);
    pea_generator::generate(table);
    move_generator::generate(table);
    movem_generator::generate(table);
    link_generator::generate(table);
    unlk_generator::generate(table);
    chk_generator::generate(table);
    illegal_generator::generate(table);
    nop_generator::generate(table);
}

#[macro_export]
macro_rules! range {
    ($am_type:ident) => {
        match $am_type {
            AddressingModeType::DataRegister => 0..8,
            AddressingModeType::AddressRegister => 0..8,
            AddressingModeType::AddressRegisterIndirect => 0..8,
            AddressingModeType::AddressRegisterPostIncrement => 0..8,
            AddressingModeType::AddressRegisterPreDecrement => 0..8,
            AddressingModeType::AddressRegisterDisplacement => 0..8,
            AddressingModeType::AddressRegisterIndexed => 0..8,
            AddressingModeType::ProgramCounterDisplacement => 0..1,
            AddressingModeType::ProgramCounterIndexed => 0..1,
            AddressingModeType::AbsShort => 0..1,
            AddressingModeType::AbsLong => 0..1,
            AddressingModeType::Immediate => 0..1,
        }
    };
}

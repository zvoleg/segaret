use crate::instruction_set::InstructionProcess;
use crate::instruction_set::generators::register_type_by_char;
use hardware::Size;
use crate::instruction_set::RxDataMetadata;
use crate::instruction_set::Instruction;
use crate::mc68k_emu::Mc68k;
use crate::Register;

struct RxDataPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
}

pub(in crate) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxDataPattern {
            name: "moveq", mask: 0b0111000000000000, size: Size::Long, clock: 4, rx_type_alias: 'd'
        }
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let reg_type = register_type_by_char(pattern.rx_type_alias);

        (0..8).for_each(|i| {
            (0..0x100).for_each(|d| {
                let opcode = mask | i << 9 | d;
                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    Mc68k::MOVEQ,
                    RxDataMetadata::new(Register::new(reg_type, i as usize), d as u32)
                ));
            });
        });
    }
}
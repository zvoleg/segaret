use crate::hardware::cpu::instruction_set::generators::register_type_by_char;
use crate::hardware::Size;
use crate::hardware::cpu::instruction_set::RxDataMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;
use crate::hardware::cpu::Register;

struct RxDataPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
}

pub(in crate::hardware::cpu) fn generate() -> Vec<Instruction<RxDataMetadata>> {
    let patterns = vec![
        RxDataPattern {
            name: String::from("moveq"), mask: 0b0111000000000000, size: Size::Long, clock: 4, rx_type_alias: 'd'
        }
    ];

    let mut instruction_set = Vec::new();

    for pattern in patterns {
        let mask = pattern.mask;

        let reg_type = register_type_by_char(pattern.rx_type_alias);

        (0..8).for_each(|i| {
            let mut instructions = (0..0xFF).map(|d| {
                let opcode = mask | i << 9 | d;
                Instruction::new(
                    pattern.name.clone(),
                    opcode,
                    pattern.size,
                    pattern.clock,
                    Mc68k::MOVEQ,
                    RxDataMetadata::new(Register::new(reg_type, i as usize), d as u32)
                )
            }).collect::<Vec<Instruction<RxDataMetadata>>>();

            instruction_set.append(&mut instructions);
        });
    }

    instruction_set
}
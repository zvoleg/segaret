use crate::hardware::mc68k::Mc68kBus;
use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::Mc68k;
use crate::hardware::Register;
use crate::hardware::mc68k::instruction_set::generators::register_type_by_char;
use crate::hardware::mc68k::instruction_set::RyMetadata;
use crate::hardware::mc68k::instruction_set::Instruction;
use crate::hardware::Size;

struct RyPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    ry_type_alias: char,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RyPattern {
            name: "move_to_usp", mask: 0b0100111001100000, size: Size::Long, clock: 4, ry_type_alias: 'a'
        },
        RyPattern {
            name: "move_from_usp", mask: 0b0100111001101000, size: Size::Long, clock: 4, ry_type_alias: 'a'
        },

        RyPattern {
            name: "unlk", mask: 0b0100111001011000, size: Size::Byte, clock: 12, ry_type_alias: 'a',
        },

        RyPattern {
            name: "ext", mask: 0b0100100010000000, size: Size::Word, clock: 4, ry_type_alias: 'd',
        },

        RyPattern {
            name: "ext", mask: 0b0100100011000000, size: Size::Long, clock: 4, ry_type_alias: 'd',
        },

        RyPattern {
            name: "swap", mask: 0b0100100001000000, size: Size::Word, clock: 4, ry_type_alias: 'd',
        },
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let ry_type = register_type_by_char(pattern.ry_type_alias);

        (0..8).for_each(|y| {
            let opcode = mask | y;
            opcode_table[opcode as usize] = Box::new(Instruction::new(
                pattern.name,
                opcode,
                pattern.size,
                pattern.clock,
                cpu_function_by_name(pattern.name),
                RyMetadata::new(Register::new(ry_type, y as usize)),
            ));
        });
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "move_to_usp" | "move_from_usp" => Mc68k::MOVE_USP,
        "unlk" => Mc68k::UNLK,
        "ext" => Mc68k::EXT,
        "swap" => Mc68k::SWAP,
        _ => panic!("ry_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

use crate::Mc68k;
use crate::hardware::Register;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;
use crate::hardware::cpu::instruction_set::RyMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;

struct RyPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    ry_type_alias: char,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<RyMetadata>> {
    let patterns = vec![
        RyPattern {
            name: String::from("move_to_usp"), mask: 0b0100111001100000, size: Size::Long, clock: 4, ry_type_alias: 'a'
        },
        RyPattern {
            name: String::from("move_from_usp"), mask: 0b0100111001101000, size: Size::Long, clock: 4, ry_type_alias: 'a'
        },

        RyPattern {
            name: String::from("unlk"), mask: 0b0100111001011000, size: Size::Byte, clock: 12, ry_type_alias: 'a',
        }
    ];

    let mut instruction_set = Vec::new();

    for pattern in patterns {
        let mask = pattern.mask;

        let ry_type = register_type_by_char(pattern.ry_type_alias);

        let mut instructions = (0..8).map(|y| {
            let opcode = mask | y;
            Instruction::new(
                pattern.name.clone(),
                opcode,
                pattern.size,
                pattern.clock,
                cpu_function_by_name(&pattern.name),
                RyMetadata::new(Register::new(ry_type, y as usize)),
            )
        }).collect::<Vec<Instruction<RyMetadata>>>();

        instruction_set.append(&mut instructions);
    }

    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "move_to_usp" | "move_from_usp" => Mc68k::MOVE_USP,
        "unlk" => Mc68k::UNLK,
        _ => panic!("ry_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

use crate::hardware::cpu::instruction_set::RyExtWordMetadata;
use crate::Mc68k;
use crate::hardware::Register;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;

struct RyPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    ry_type_alias: char,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<RyExtWordMetadata>> {
    let patterns = vec![
        RyPattern {
            name: String::from("link"), mask: 0b0100111001010000, size: Size::Word, clock: 16, ry_type_alias: 'a'
        },
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
                RyExtWordMetadata::new(Register::new(ry_type, y as usize)),
            )
        }).collect::<Vec<Instruction<RyExtWordMetadata>>>();

        instruction_set.append(&mut instructions);
    }

    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "link" => Mc68k::LINK,
        _ => panic!("ry_ext_word_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

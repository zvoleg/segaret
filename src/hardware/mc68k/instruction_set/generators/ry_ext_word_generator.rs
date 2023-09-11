use crate::hardware::mc68k::Register;
use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::hardware::mc68k::instruction_set::RyExtWordMetadata;
use crate::Mc68k;
use crate::hardware::mc68k::instruction_set::generators::register_type_by_char;
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
            name: "link", mask: 0b0100111001010000, size: Size::Word, clock: 16, ry_type_alias: 'a'
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
                RyExtWordMetadata::new(Register::new(ry_type, y as usize)),
            ));
        });
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "link" => Mc68k::LINK,
        _ => panic!("ry_ext_word_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

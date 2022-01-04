use crate::hardware::Register;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::RxRyMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;

struct RxRyPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
    ry_type_alias: char,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<RxRyMetadata>> {
    let patterns = vec![
        RxRyPattern {
            name: String::from("exg"), mask: 0b1100000101000000, size: Size::Long, clock: 6, rx_type_alias: 'd', ry_type_alias: 'd'
        },
        RxRyPattern {
            name: String::from("exg"), mask: 0b1100000101001000, size: Size::Long, clock: 6, rx_type_alias: 'a', ry_type_alias: 'a'
        },
        RxRyPattern {
            name: String::from("exg"), mask: 0b1100000110001000, size: Size::Long, clock: 6, rx_type_alias: 'd', ry_type_alias: 'a'
        },
    ];

    let mut instruction_set = Vec::new();

    for pattern in patterns {
        let mask = pattern.mask;

        let rx_type = register_type_by_char(pattern.rx_type_alias);
        let ry_type = register_type_by_char(pattern.ry_type_alias);

        (0..8).for_each(|x| {
            let mut instructions = (0..8).map(|y| {
                let opcode = mask | x << 9 | y;
                Instruction::new(
                    pattern.name.clone(),
                    opcode,
                    pattern.size,
                    pattern.clock,
                    cpu_function_by_name(&pattern.name),
                    RxRyMetadata::new(Register::new(rx_type, x as usize), Register::new(ry_type, y as usize)),
                )
            }).collect::<Vec<Instruction<RxRyMetadata>>>();

            instruction_set.append(&mut instructions);
        })
    }

    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "exg" => Mc68k::EXG,
        _ => panic!("rx_ry_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
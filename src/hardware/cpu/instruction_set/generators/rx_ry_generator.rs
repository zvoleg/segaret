use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::Register;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::RxRyMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;

struct RxRyPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
    ry_type_alias: char,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxRyPattern {
            name: "exg", mask: 0b1100000101000000, size: Size::Long, clock: 6, rx_type_alias: 'd', ry_type_alias: 'd'
        },
        RxRyPattern {
            name: "exg", mask: 0b1100000101001000, size: Size::Long, clock: 6, rx_type_alias: 'a', ry_type_alias: 'a'
        },
        RxRyPattern {
            name: "exg", mask: 0b1100000110001000, size: Size::Long, clock: 6, rx_type_alias: 'd', ry_type_alias: 'a'
        },

        RxRyPattern {
            name: "cmpm", mask: 0b01011000100001000, size: Size::Byte, clock: 12, rx_type_alias: 'a', ry_type_alias: 'a',
        },

        RxRyPattern {
            name: "cmpm", mask: 0b01011000101001000, size: Size::Word, clock: 12, rx_type_alias: 'a', ry_type_alias: 'a',
        },

        RxRyPattern {
            name: "cmpm", mask: 0b01011000110001000, size: Size::Long, clock: 20, rx_type_alias: 'a', ry_type_alias: 'a',
        },
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let rx_type = register_type_by_char(pattern.rx_type_alias);
        let ry_type = register_type_by_char(pattern.ry_type_alias);

        (0..8).for_each(|x| {
            (0..8).for_each(|y| {
                let opcode = mask | x << 9 | y;
                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    cpu_function_by_name(pattern.name),
                    RxRyMetadata::new(Register::new(rx_type, x as usize), Register::new(ry_type, y as usize)),
                ));
            });
        })
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "exg" => Mc68k::EXG,
        "cmpm" => Mc68k::CMPM,
        _ => panic!("rx_ry_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

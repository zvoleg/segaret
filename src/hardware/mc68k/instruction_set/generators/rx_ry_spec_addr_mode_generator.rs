use crate::hardware::mc68k::Mc68kBus;
use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::hardware::mc68k::addressing_mode::AddrMode;
use crate::hardware::mc68k::instruction_set::generators::addr_mode_type_by_char;
use crate::Mc68k;
use crate::hardware::Size;
use crate::hardware::mc68k::instruction_set::RxRySpecAddrModeMetadata;
use crate::hardware::mc68k::instruction_set::Instruction;

struct RxRySpecAddrModePattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_type_x_alias: char,
    addr_mode_type_y_alias: char,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000100000000, size: Size::Byte, clock: 4, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000100001000, size: Size::Byte, clock: 18, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },

        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000101000000, size: Size::Word, clock: 4, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000101001000, size: Size::Word, clock: 18, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },

        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000110000000, size: Size::Long, clock: 8, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "addx", mask: 0b1101000110001000, size: Size::Long, clock: 30, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },

        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000100000000, size: Size::Byte, clock: 4, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000100001000, size: Size::Byte, clock: 18, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },

        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000101000000, size: Size::Word, clock: 4, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000101001000, size: Size::Word, clock: 18, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },

        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000110000000, size: Size::Long, clock: 8, addr_mode_type_x_alias: 'D', addr_mode_type_y_alias: 'D', 
        },
        RxRySpecAddrModePattern {
            name: "subx", mask: 0b1001000110001000, size: Size::Long, clock: 30, addr_mode_type_x_alias: '-', addr_mode_type_y_alias: '-', 
        },
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let addr_mode_type_x = addr_mode_type_by_char(pattern.addr_mode_type_x_alias);
        let addr_mode_type_y = addr_mode_type_by_char(pattern.addr_mode_type_y_alias);

        (0..8).for_each(|x| {
            (0..8).for_each(|y| {
                let opcode = mask | x << 9 | y;
                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    cpu_function_by_name(pattern.name),
                    RxRySpecAddrModeMetadata::new(AddrMode::new(addr_mode_type_x, x as usize), AddrMode::new(addr_mode_type_y, y as usize)),
                ));
            });
        })
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "addx" => Mc68k::ADDX,
        "subx" => Mc68k::SUBX,
        _ => panic!("rx_ry_spec_addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

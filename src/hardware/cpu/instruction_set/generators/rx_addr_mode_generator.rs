use crate::hardware::cpu::addressing_mode::AddrMode;
use crate::hardware::Register;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::RxAddrModeMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;

struct RxAddrModeInstPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
    addr_mode_aliases: String,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<RxAddrModeMetadata>> {
    let patterns = vec![
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 4, rx_type_alias: 'a', addr_mode_aliases: String::from("DA")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: String::from("a+i")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 10, rx_type_alias: 'a', addr_mode_aliases: String::from("-")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 12, rx_type_alias: 'a', addr_mode_aliases: String::from("dWP")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 14, rx_type_alias: 'a', addr_mode_aliases: String::from("xX")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0011000001000000, size: Size::Word, clock: 16, rx_type_alias: 'a', addr_mode_aliases: String::from("L")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 4, rx_type_alias: 'a', addr_mode_aliases: String::from("DA")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: String::from("a+i")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: String::from("-")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 16, rx_type_alias: 'a', addr_mode_aliases: String::from("dWP")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 18, rx_type_alias: 'a', addr_mode_aliases: String::from("xX")
        },
        RxAddrModeInstPattern {
            name: String::from("movea"), mask: 0b0010000001000000, size: Size::Long, clock: 20, rx_type_alias: 'a', addr_mode_aliases: String::from("L")
        },

        RxAddrModeInstPattern {
            name: String::from("movep"), mask: 0b0000000100001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: String::from("a")
        },
        RxAddrModeInstPattern {
            name: String::from("movep"), mask: 0b0000000100001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: String::from("a")
        },
        RxAddrModeInstPattern {
            name: String::from("movep"), mask: 0b0000000110001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: String::from("a")
        },
        RxAddrModeInstPattern {
            name: String::from("movep"), mask: 0b0000000111001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: String::from("a")
        },

        RxAddrModeInstPattern {
            name: String::from("lea"), mask: 0b0100000111000000, size: Size::Long, clock: 4, rx_type_alias: 'a', addr_mode_aliases: String::from("a")
        },
        RxAddrModeInstPattern {
            name: String::from("lea"), mask: 0b0100000111000000, size: Size::Long, clock: 8, rx_type_alias: 'a', addr_mode_aliases: String::from("dPW")
        },
        RxAddrModeInstPattern {
            name: String::from("lea"), mask: 0b0100000111000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: String::from("xXL")
        },

        // add 
        RxAddrModeInstPattern {
            name: String::from("add"), mask: 0b1101000000000000, size: Size::Byte, clock: 4 /*calc ea addr*/, rx_type_alias: 'd', addr_mode_aliases: String::from(""),
        },
        RxAddrModeInstPattern {
            name: String::from("add"), mask: 0b1101000100000000, size: Size::Byte, clock: 8 /*calc ea addr*/, rx_type_alias: 'd', addr_mode_aliases: String::from(""),
        }
    ];

    let mut instruction_set = Vec::new();

    for pattern in patterns {
        let mask = pattern.mask;

        let rx_type = register_type_by_char(pattern.rx_type_alias);
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();

        for addr_mode_type in addr_mode_type_list {
            let addr_mode_list = get_addr_mode_table(addr_mode_type);

            (0..8).for_each(|i| {
                let mut instructions = addr_mode_list
                    .iter()
                    .map(|mode| {
                        let opcode = generate_mask(&pattern.name, mask, i, mode);
                        Instruction::new(
                            pattern.name.clone(),
                            opcode,
                            pattern.size,
                            pattern.clock,
                            cpu_function_by_name(&pattern.name),
                            RxAddrModeMetadata::new(Register::new(rx_type, i as usize), *mode),
                        )
                    })
                    .collect::<Vec<Instruction<RxAddrModeMetadata>>>();
                
                instruction_set.append(&mut instructions);
            })
        }
    }

    instruction_set
}

fn generate_mask(name: &str, mask: u16, rx_idx: u16, addr_mode: &AddrMode) -> u16 {
    match name {
        "movep" => mask | rx_idx << 9 | (*addr_mode).reg_idx as u16,
        _ => mask | rx_idx << 9 | ((*addr_mode).mode_bits as u16) << 3 | (*addr_mode).reg_idx as u16,
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "movea" => Mc68k::MOVEA,
        "movep" => Mc68k::MOVEP,
        "lea" => Mc68k::LEA,
        "add" => Mc68k::ADD,
        "adda" => Mc68k::ADDA,
        _ => panic!("rx_addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
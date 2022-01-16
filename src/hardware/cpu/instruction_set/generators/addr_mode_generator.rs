use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::AddrModeMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;
use crate::hardware::Size;

struct AddrModeInstPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_aliases: String,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<AddrModeMetadata>> {
    let patterns = vec![
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101000000000, size: Size::Byte, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101001000000, size: Size::Word, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101010000000, size: Size::Long, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },

        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 12, addr_mode_aliases: String::from("a"),
        },
        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 16, addr_mode_aliases: String::from("dWP"),
        },
        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 20, addr_mode_aliases: String::from("xXL"),
        },

        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 12, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 16, addr_mode_aliases: String::from("a+i"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 18, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 20, addr_mode_aliases: String::from("dWP"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 22, addr_mode_aliases: String::from("xX"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_sr"), mask: 0b0100011011000000, size: Size::Word, clock: 24, addr_mode_aliases: String::from("L"),
        },

        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 6, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 12, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 14, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 16, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 18, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("move_from_sr"), mask: 0b0100000011000000, size: Size::Word, clock: 20, addr_mode_aliases: String::from("L"),
        },

        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 12, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 16, addr_mode_aliases: String::from("a+i"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 18, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 20, addr_mode_aliases: String::from("dWP"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 22, addr_mode_aliases: String::from("xX"),
        },
        AddrModeInstPattern {
            name: String::from("move_to_ccr"), mask: 0b0100010011000000, size: Size::Word, clock: 24, addr_mode_aliases: String::from("L"),
        },
    ];

    let mut instruction_set = Vec::new();

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();
        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);

            instruction_set.append(&mut addr_modes
                .iter()
                .map(|mode| {
                    let opcode =  mask | ((*mode).mode_bits as u16) << 3 | (*mode).reg_idx as u16;
                    Instruction::new(
                        pattern.name.clone(),
                        opcode,
                        pattern.size,
                        pattern.clock,
                        cpu_function_by_name(&pattern.name),
                        AddrModeMetadata::new(*mode),
                    )
                })
                .collect::<Vec<Instruction<AddrModeMetadata>>>());
        }
    }
    
    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "tst" => Mc68k::TST,
        "pea" => Mc68k::PEA,
        "move_to_sr" => Mc68k::MOVE_to_SR,
        "move_from_sr" => Mc68k::MOVE_from_SR,
        "move_to_ccr" => Mc68k::MOVE_to_CCR,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
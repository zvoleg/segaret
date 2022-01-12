use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::AddrModeImmediateMetadata;
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

pub(in crate::hardware) fn generate() -> Vec<Instruction<AddrModeImmediateMetadata>> {
    let patterns = vec![
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 8, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 8, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 16, addr_mode_aliases: String::from("D"),
        },

        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 16, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 18, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 20, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 22, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011000000000, size: Size::Byte, clock: 24, addr_mode_aliases: String::from("L"),
        },

        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 16, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 18, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 20, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 22, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011001000000, size: Size::Word, clock: 24, addr_mode_aliases: String::from("L"),
        },

        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 24, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 26, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 28, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 30, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addi"), mask: 0b0000011010000000, size: Size::Long, clock: 32, addr_mode_aliases: String::from("L"),
        },

        // andi
        AddrModeInstPattern {
            name: String::from("andi"), mask: 0b0000001000000000, size: Size::Byte, clock: 0, addr_mode_aliases: String::from("D"),
        }
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
                        AddrModeImmediateMetadata::new(*mode, pattern.size),
                    )
                })
                .collect::<Vec<Instruction<AddrModeImmediateMetadata>>>());
        }
    }
    
    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "addi" => Mc68k::ADDI,
        "andi" => Mc68k::ANDI,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
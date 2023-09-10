use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::hardware::mc68k::instruction_set::addr_mode_table::get_am_bits;
use crate::hardware::mc68k::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::mc68k::addressing_mode::AddrModeType;
use crate::hardware::mc68k::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::mc68k::instruction_set::AddrModeMetadata;
use crate::hardware::mc68k::instruction_set::Instruction;
use crate::hardware::mc68k::mc68k_emu::Mc68k;
use crate::hardware::Size;

struct AddrModeInstPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_aliases: &'static str,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>){
    let patterns = vec![
        AddrModeInstPattern {
            name: "tst", mask: 0b0100101000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "DAa+-dxWLi",
        },
        AddrModeInstPattern {
            name: "tst", mask: 0b0100101001000000, size: Size::Word, clock: 4, addr_mode_aliases: "DAa+-dxWLi",
        },
        AddrModeInstPattern {
            name: "tst", mask: 0b0100101010000000, size: Size::Long, clock: 4, addr_mode_aliases: "DAa+-dxWLi",
        },

        AddrModeInstPattern {
            name: "pea", mask: 0b0100100001000000, size: Size::Long, clock: 8, addr_mode_aliases: "adWLP",
        },
        AddrModeInstPattern {
            name: "pea", mask: 0b0100100001000000, size: Size::Long, clock: 10, addr_mode_aliases: "xX",
        },

        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 12, addr_mode_aliases: "Da+-dxWLPXi",
        },

        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 6, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 12, addr_mode_aliases: "Da+-dxWLPXi",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 4, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 6, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "tas", mask: 0b0100101011000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "tas", mask: 0b0100101011000000, size: Size::Byte, clock: 14, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "jmp", mask: 0b0100111011000000, size: Size::Byte, clock: 0, addr_mode_aliases: "L"
        },
        AddrModeInstPattern {
            name: "jmp", mask: 0b0100111011000000, size: Size::Byte, clock: 2, addr_mode_aliases: "dWP"
        },
        AddrModeInstPattern {
            name: "jmp", mask: 0b0100111011000000, size: Size::Byte, clock: 4, addr_mode_aliases: "axX"
        },

        AddrModeInstPattern {
            name: "jsr", mask: 0b0100111010000000, size: Size::Byte, clock: 8, addr_mode_aliases: "L"
        },
        AddrModeInstPattern {
            name: "jsr", mask: 0b0100111010000000, size: Size::Byte, clock: 10, addr_mode_aliases: "dWP"
        },
        AddrModeInstPattern {
            name: "jsr", mask: 0b0100111010000000, size: Size::Byte, clock: 12, addr_mode_aliases: "axX"
        },

        AddrModeInstPattern {
            name: "neg", mask: 0b0100010000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "neg", mask: 0b0100010000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "neg", mask: 0b0100010001000000, size: Size::Word, clock: 4, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "neg", mask: 0b0100010001000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "neg", mask: 0b0100010010000000, size: Size::Long, clock: 6, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "neg", mask: 0b0100010010000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "negx", mask: 0b0100000000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "negx", mask: 0b0100000000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "negx", mask: 0b0100000001000000, size: Size::Word, clock: 4, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "negx", mask: 0b0100000001000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "negx", mask: 0b0100000010000000, size: Size::Long, clock: 6, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "negx", mask: 0b0100000010000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "not", mask: 0b0100011000000000, size: Size::Byte, clock: 4,  addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "not", mask: 0b0100011000000000, size: Size::Byte, clock: 8,  addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "not", mask: 0b0100011001000000, size: Size::Word, clock: 4,  addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "not", mask: 0b0100011001000000, size: Size::Word, clock: 8,  addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "not", mask: 0b0100011010000000, size: Size::Long, clock: 6,  addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "not", mask: 0b0100011010000000, size: Size::Long, clock: 12,  addr_mode_aliases: "a+-dxWL"
        },
    ];

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars()
                                    .map(|c| addr_mode_type_by_char(c))
                                    .collect::<Vec<AddrModeType>>();
        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);
            let clock_period = match pattern.size {
                Size::Byte | Size::Word => pattern.clock + addr_mode_type.get_clock_periods_short(),
                Size::Long => pattern.clock + addr_mode_type.get_clock_periods_long(),
            };

            addr_modes.iter()
                .for_each(|mode| {
                    let opcode =  mask | get_am_bits((*mode).am_type) << 3 | (*mode).reg_idx as u16;

                    opcode_table[opcode as usize] = Box::new(Instruction::new(
                        pattern.name,
                        opcode,
                        pattern.size,
                        clock_period,
                        cpu_function_by_name(pattern.name),
                        AddrModeMetadata::new(*mode),
                    ));
                });
        }
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "tst" => Mc68k::TST,
        "pea" => Mc68k::PEA,
        "move_to_sr" => Mc68k::MOVE_to_SR,
        "move_from_sr" => Mc68k::MOVE_from_SR,
        "move_to_ccr" => Mc68k::MOVE_to_CCR,
        "clr" => Mc68k::CLR,
        "tas" => Mc68k::TAS,
        "jmp" => Mc68k::JMP,
        "jsr" => Mc68k::JSR,
        "neg" => Mc68k::NEG,
        "negx" => Mc68k::NEGX,
        "not" => Mc68k::NOT,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
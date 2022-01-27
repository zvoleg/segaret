use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::AddrModeMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;
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
            name: "pea", mask: 0b0100100001000000, size: Size::Long, clock: 12, addr_mode_aliases: "a",
        },
        AddrModeInstPattern {
            name: "pea", mask: 0b0100100001000000, size: Size::Long, clock: 16, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "pea", mask: 0b0100100001000000, size: Size::Long, clock: 20, addr_mode_aliases: "xXL",
        },

        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 12, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 16, addr_mode_aliases: "a+i",
        },
        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 20, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 22, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "move_to_sr", mask: 0b0100011011000000, size: Size::Word, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 6, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 14, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 16, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 18, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "move_from_sr", mask: 0b0100000011000000, size: Size::Word, clock: 20, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 12, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 16, addr_mode_aliases: "a+i",
        },
        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 20, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 22, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "move_to_ccr", mask: 0b0100010011000000, size: Size::Word, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 4, addr_mode_aliases: "D",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 6, addr_mode_aliases: "D",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 14, addr_mode_aliases: "-",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 16, addr_mode_aliases: "dW",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 18, addr_mode_aliases: "x",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001000000000, size: Size::Byte, clock: 20, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 14, addr_mode_aliases: "-",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 16, addr_mode_aliases: "dW",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 18, addr_mode_aliases: "x",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001001000000, size: Size::Word, clock: 20, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 16, addr_mode_aliases: "a+",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 18, addr_mode_aliases: "-",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 20, addr_mode_aliases: "dW",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 22, addr_mode_aliases: "x",
        },

        AddrModeInstPattern {
            name: "clr", mask: 0b0100001010000000, size: Size::Long, clock: 24, addr_mode_aliases: "L",
        },
    ];

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();
        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);

            addr_modes.iter()
                .for_each(|mode| {
                    let opcode =  mask | ((*mode).mode_bits as u16) << 3 | (*mode).reg_idx as u16;
                    opcode_table[opcode as usize] = Box::new(Instruction::new(
                        pattern.name,
                        opcode,
                        pattern.size,
                        pattern.clock,
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
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
use hardware::Size;

use crate::instruction_set::InstructionProcess;
use crate::instruction_set::addr_mode_table::get_am_bits;
use crate::instruction_set::generators::addr_mode_type_by_char;
use crate::addressing_mode::AddrModeType;
use crate::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::instruction_set::AddrModeImmediateMetadata;
use crate::instruction_set::Instruction;
use crate::mc68k_emu::Mc68k;

struct AddrModeInstPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_aliases: &'static str,
}

pub(in crate) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "Da+-dxWLPXi",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 8, addr_mode_aliases: "Da+-dxWLPXi",
        },

        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 14, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWLPXi",
        },

        // andi
        AddrModeInstPattern {
            name: "andi", mask: 0b0000001000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "andi", mask: 0b0000001000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "andi", mask: 0b0000001001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "andi", mask: 0b0000001001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "andi", mask: 0b0000001010000000, size: Size::Long, clock: 14, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "andi", mask: 0b0000001010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+-dxWL",
        },

        //eori
        AddrModeInstPattern {
            name: "eori", mask: 0b0000101000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "eori", mask: 0b0000101000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "eori", mask: 0b0000101001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "eori", mask: 0b0000101001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "eori", mask: 0b0000101010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "eori", mask: 0b0000101010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+-dxWL",
        },

        //ori
        AddrModeInstPattern {
            name: "ori", mask: 0b0000000000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "ori", mask: 0b0000000000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "ori", mask: 0b0000000001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "ori", mask: 0b0000000001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        AddrModeInstPattern {
            name: "ori", mask: 0b0000000010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "ori", mask: 0b0000000010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+-dxWL",
        },

        
        // bit manipulation instructions
        AddrModeInstPattern {
            name: "bchg", mask: 0b0000100001000000, size: Size::Long, clock: 12, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "bchg", mask: 0b0000100001000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWLPX"
        },
        
        AddrModeInstPattern {
            name: "bclr", mask: 0b0000100010000000, size: Size::Long, clock: 14, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "bclr", mask: 0b0000100010000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWLPX"
        },

        AddrModeInstPattern {
            name: "bset", mask: 0b0000100011000000, size: Size::Long, clock: 12, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "bset", mask: 0b0000100011000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+-dxWLPX"
        },

        AddrModeInstPattern {
            name: "btst", mask: 0b0000100000000000, size: Size::Long, clock: 10, addr_mode_aliases: "D"
        },
        AddrModeInstPattern {
            name: "btst", mask: 0b0000100000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWLPX"
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
                    let opcode =  mask | get_am_bits(mode.am_type) << 3 | mode.reg_idx as u16;

                    opcode_table[opcode as usize] = Box::new(Instruction::new(
                        pattern.name,
                        opcode,
                        pattern.size,
                        clock_period,
                        cpu_function_by_name(pattern.name),
                        AddrModeImmediateMetadata::new(*mode),
                    ));
                });
        }
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "addi" => Mc68k::ADDI,
        "subi" => Mc68k::SUBI,
        "cmpi" => Mc68k::CMPI,
        "andi" => Mc68k::ANDI,
        "eori" => Mc68k::EORI,
        "ori" => Mc68k::ORI,
        "bchg" => Mc68k::BCHG_ext_word,
        "bclr" => Mc68k::BCLR_ext_word,
        "bset" => Mc68k::BSET_ext_word,
        "btst" => Mc68k::BTST_ext_word,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::Size;
use crate::hardware::cpu::instruction_set::AddrModeExtWordMetadata;
use crate::hardware::cpu::instruction_set::Instruction;

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
            name: "movem", mask: 0b0100110011000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110011000000, size: Size::Long, clock: 16, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110011000000, size: Size::Long, clock: 18, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110011000000, size: Size::Long, clock: 20, addr_mode_aliases: "L",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110010000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110010000000, size: Size::Word, clock: 16, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110010000000, size: Size::Word, clock: 18, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100110010000000, size: Size::Word, clock: 20, addr_mode_aliases: "L",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100011000000, size: Size::Long, clock: 8, addr_mode_aliases: "a-",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100011000000, size: Size::Long, clock: 12, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100011000000, size: Size::Long, clock: 14, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100011000000, size: Size::Long, clock: 16, addr_mode_aliases: "L",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100010000000, size: Size::Word, clock: 8, addr_mode_aliases: "A-",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100010000000, size: Size::Word, clock: 12, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100010000000, size: Size::Word, clock: 14, addr_mode_aliases: "X",
        },
        AddrModeInstPattern {
            name: "movem", mask: 0b0100100010000000, size: Size::Word, clock: 16, addr_mode_aliases: "L",
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
                        AddrModeExtWordMetadata::new(*mode),
                    ));
                });
        }
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "movem" => Mc68k::MOVEM,
        _ => panic!("addr_mode_ext_word_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
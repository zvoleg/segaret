use hardware::Size;

use crate::instruction_set::InstructionProcess;
use crate::instruction_set::AddrModeDataMetadata;
use crate::instruction_set::addr_mode_table::get_am_bits;
use crate::instruction_set::generators::addr_mode_type_by_char;
use crate::addressing_mode::AddrModeType;
use crate::instruction_set::addr_mode_table::get_addr_mode_table;
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
            name: "addq", mask: 0b0101000000000000, size: Size::Byte, clock: 4, addr_mode_aliases: "DA",
        },
        AddrModeInstPattern {
            name: "addq", mask: 0b0101000000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWL",
        },
        
        AddrModeInstPattern {
            name: "addq", mask: 0b0101000001000000, size: Size::Word, clock: 4, addr_mode_aliases: "DA",
        },
        AddrModeInstPattern {
            name: "addq", mask: 0b0101000001000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL",
        },
        
        AddrModeInstPattern {
            name: "addq", mask: 0b0101000010000000, size: Size::Long, clock: 8, addr_mode_aliases: "DA",
        },
        AddrModeInstPattern {
            name: "addq", mask: 0b0101000010000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWL",
        },

        //subq
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000100000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000100000000, size: Size::Byte, clock: 8, addr_mode_aliases: "Aa+-dxWL",
        },
        
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000101000000, size: Size::Word, clock: 4, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000101000000, size: Size::Word, clock: 8, addr_mode_aliases: "Aa+-dxWL",
        },
        
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000110000000, size: Size::Long, clock: 8, addr_mode_aliases: "DA",
        },
        AddrModeInstPattern {
            name: "subq", mask: 0b0101000110000000, size: Size::Long, clock: 12, addr_mode_aliases: "a+-dxWL",
        },
    ];

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();
        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);
            let clock_period = match pattern.size {
                Size::Byte | Size::Word => pattern.clock + addr_mode_type.get_clock_periods_short(),
                Size::Long => pattern.clock + addr_mode_type.get_clock_periods_long(),
            };

            (0..8).for_each(|data| {
                addr_modes.iter()
                    .for_each(|mode| {
                        let opcode =  mask | data << 9 | get_am_bits(mode.am_type) << 3 | mode.reg_idx as u16;
                        let data = if data != 0 {
                            data
                        } else {
                            8
                        };
                        
                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            pattern.name,
                            opcode,
                            pattern.size,
                            clock_period,
                            cpu_function_by_name(pattern.name),
                            AddrModeDataMetadata::new(*mode, data as u32),
                        ));
                    });
            });
        }
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "addq" => Mc68k::ADDQ,
        "subq" => Mc68k::SUBQ,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::instruction_set::AddrModeDataMetadata;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
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

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 4, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 4, addr_mode_aliases: String::from("A"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 12, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 14, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 16, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 18, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000000000000, size: Size::Byte, clock: 20, addr_mode_aliases: String::from("L"),
        },

        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 4, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 4, addr_mode_aliases: String::from("A"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 12, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 14, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 16, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 18, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000001000000, size: Size::Word, clock: 20, addr_mode_aliases: String::from("L"),
        },
        
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 8, addr_mode_aliases: String::from("D"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 8, addr_mode_aliases: String::from("A"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 16, addr_mode_aliases: String::from("a+"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 18, addr_mode_aliases: String::from("-"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 20, addr_mode_aliases: String::from("dW"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 22, addr_mode_aliases: String::from("x"),
        },
        AddrModeInstPattern {
            name: String::from("addq"), mask: 0b0101000010000000, size: Size::Long, clock: 24, addr_mode_aliases: String::from("L"),
        },
    ];

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();
        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);

            (0..8).for_each(|data| {
                addr_modes.iter()
                    .for_each(|mode| {
                        let opcode =  mask | data << 9 | ((*mode).mode_bits as u16) << 3 | (*mode).reg_idx as u16;
                        let data = if data != 0 {
                            data
                        } else {
                            8
                        };
                        
                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            pattern.name.clone(),
                            opcode,
                            pattern.size,
                            pattern.clock,
                            cpu_function_by_name(&pattern.name),
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
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::AddrModeImmediateMetadata;
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

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },

        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 16, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 20, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 22, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011000000000, size: Size::Byte, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 16, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 20, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 22, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011001000000, size: Size::Word, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 24, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 26, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 28, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 30, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "addi", mask: 0b0000011010000000, size: Size::Long, clock: 32, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 16, addr_mode_aliases: "D",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 16, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 20, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 22, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010000000000, size: Size::Byte, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 16, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 18, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 20, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 22, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010001000000, size: Size::Word, clock: 24, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 24, addr_mode_aliases: "a+",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 26, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 28, addr_mode_aliases: "dW",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 30, addr_mode_aliases: "x",
        },
        AddrModeInstPattern {
            name: "subi", mask: 0b0000010010000000, size: Size::Long, clock: 32, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 12, addr_mode_aliases: "a+i",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 14, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 16, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 18, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110000000000, size: Size::Byte, clock: 20, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 8, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 12, addr_mode_aliases: "a+i",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 14, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 16, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 18, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110001000000, size: Size::Word, clock: 20, addr_mode_aliases: "L",
        },

        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 14, addr_mode_aliases: "D",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 20, addr_mode_aliases: "a+i",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 22, addr_mode_aliases: "-",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 24, addr_mode_aliases: "dWP",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 26, addr_mode_aliases: "xX",
        },
        AddrModeInstPattern {
            name: "cmpi", mask: 0b0000110010000000, size: Size::Long, clock: 28, addr_mode_aliases: "L",
        },

        // andi
        AddrModeInstPattern {
            name: "andi", mask: 0b0000001000000000, size: Size::Byte, clock: 0, addr_mode_aliases: "D",
        }
    ];

    for pattern in patterns {
        
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars()
                                    .map(|c| addr_mode_type_by_char(c))
                                    .collect::<Vec<AddrModeType>>();
        
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
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
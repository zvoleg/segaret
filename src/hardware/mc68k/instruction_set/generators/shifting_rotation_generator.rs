use crate::hardware::mc68k::addressing_mode::AddrModeType;
use crate::hardware::mc68k::instruction_set::addr_mode_table::get_am_bits;
use crate::hardware::mc68k::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::mc68k::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::mc68k::instruction_set::AddrModeMetadata;
use crate::hardware::mc68k::instruction_set::RxRyMetadata;
use crate::Mc68k;
use crate::hardware::Register;
use crate::hardware::mc68k::instruction_set::RotationRyMetadata;
use crate::hardware::mc68k::instruction_set::Instruction;
use crate::hardware::Size;
use crate::hardware::mc68k::instruction_set::InstructionProcess;

struct AddrModeInstPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_aliases: &'static str,
} 

struct RotationRyPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}

struct RxRyPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}


pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    generate_rotation_ry_instructions(opcode_table);
    generate_rx_ry_instructions(opcode_table);
    generate_memory_instructions(opcode_table);
}

fn generate_rotation_ry_instructions(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        //asd
        RotationRyPattern {
            name: "asr", mask: 0b1110_000_0_00_0_00_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "asr", mask: 0b1110_000_0_01_0_00_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "asr", mask: 0b1110_000_0_10_0_00_000, size: Size::Long, clock: 8
        },

        RotationRyPattern {
            name: "asl", mask: 0b1110_000_1_00_0_00_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "asl", mask: 0b1110_000_1_01_0_00_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "asl", mask: 0b1110_000_1_10_0_00_000, size: Size::Long, clock: 8
        },

        //lsd
        RotationRyPattern {
            name: "lsr", mask: 0b1110_000_0_00_0_01_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "lsr", mask: 0b1110_000_0_01_0_01_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "lsr", mask: 0b1110_000_0_10_0_01_000, size: Size::Long, clock: 8
        },

        RotationRyPattern {
            name: "lsl", mask: 0b1110_000_1_00_0_01_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "lsl", mask: 0b1110_000_1_01_0_01_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "lsl", mask: 0b1110_000_1_10_0_01_000, size: Size::Long, clock: 8
        },

        //rod
        RotationRyPattern {
            name: "ror", mask: 0b1110_000_0_00_0_11_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "ror", mask: 0b1110_000_0_01_0_11_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "ror", mask: 0b1110_000_0_10_0_11_000, size: Size::Long, clock: 8
        },

        RotationRyPattern {
            name: "rol", mask: 0b1110_000_1_00_0_11_000, size: Size::Byte, clock: 6
        },
        RotationRyPattern {
            name: "rol", mask: 0b1110_000_1_01_0_11_000, size: Size::Word, clock: 6
        },
        RotationRyPattern {
            name: "rol", mask: 0b1110_000_1_10_0_11_000, size: Size::Long, clock: 8
        },
    ];

    for pattern in patterns {
        (0..0x8).for_each(|counter_mask| {
            (0..0x8).for_each(|reg_idx| {
                let opcode = pattern.mask | counter_mask << 9 | reg_idx;
                let counter = if counter_mask == 0 {
                    8
                } else {
                    counter_mask
                } as u32;

                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    data_cpu_function_by_name(pattern.name),
                    RotationRyMetadata::new(
                        counter,
                        Register::data(reg_idx as usize)
                    )
                ));
            });
        });
    }
}

fn generate_rx_ry_instructions(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxRyPattern {
            name: "asr", mask: 0b1110_000_0_00_1_00_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "asr", mask: 0b1110_000_0_01_1_00_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "asr", mask: 0b1110_000_0_10_1_00_000, size: Size::Long, clock: 8
        },

        RxRyPattern {
            name: "asl", mask: 0b1110_000_1_00_1_00_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "asl", mask: 0b1110_000_1_01_1_00_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "asl", mask: 0b1110_000_1_10_1_00_000, size: Size::Long, clock: 8
        },

        //lsd
        RxRyPattern {
            name: "lsr", mask: 0b1110_000_0_00_1_01_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "lsr", mask: 0b1110_000_0_01_1_01_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "lsr", mask: 0b1110_000_0_10_1_01_000, size: Size::Long, clock: 8
        },

        RxRyPattern {
            name: "lsl", mask: 0b1110_000_1_00_1_01_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "lsl", mask: 0b1110_000_1_01_1_01_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "lsl", mask: 0b1110_000_1_10_1_01_000, size: Size::Long, clock: 8
        },

        //rod
        RxRyPattern {
            name: "ror", mask: 0b1110_000_0_00_1_11_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "ror", mask: 0b1110_000_0_01_1_11_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "ror", mask: 0b1110_000_0_10_1_11_000, size: Size::Long, clock: 8
        },

        RxRyPattern {
            name: "rol", mask: 0b1110_000_1_00_1_11_000, size: Size::Byte, clock: 6
        },
        RxRyPattern {
            name: "rol", mask: 0b1110_000_1_01_1_11_000, size: Size::Word, clock: 6
        },
        RxRyPattern {
            name: "rol", mask: 0b1110_000_1_10_1_11_000, size: Size::Long, clock: 8
        },
    ];

    for pattern in patterns {
        (0..8).for_each(|reg_idx_x| {
            (0..8).for_each(|reg_idx_y| {
                let opcode = pattern.mask | reg_idx_x << 9 | reg_idx_y;

                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    data_cpu_function_by_name(pattern.name),
                    RxRyMetadata::new(
                        Register::data(reg_idx_x as usize),
                        Register::data(reg_idx_y as usize)
                    )
                ));
            });
        });
    }
}
    
fn generate_memory_instructions(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        //asd
        AddrModeInstPattern {
            name: "asr", mask: 0b1110000_0_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "asl", mask: 0b1110000_1_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        //lsd
        AddrModeInstPattern {
            name: "lsr", mask: 0b1110001_0_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "lsl", mask: 0b1110001_1_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        //rod
        AddrModeInstPattern {
            name: "ror", mask: 0b1110011_0_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
        },

        AddrModeInstPattern {
            name: "rol", mask: 0b1110011_1_11_000000, size: Size::Word, clock: 8, addr_mode_aliases: "a+-dxWL"
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
                        addr_mode_cpu_function_by_name(pattern.name),
                        AddrModeMetadata::new(*mode),
                    ));
                });
        }
    }
}

fn data_cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "asr" | "asl" => Mc68k::ASd_data,
        "lsr" | "lsl" => Mc68k::LSd_data,
        "ror" | "rol" => Mc68k::ROd_data,
        _ => panic!("shifting_rotation_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}


fn addr_mode_cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "asr" | "asl" => Mc68k::ASd_memory,
        "lsr" | "lsl" => Mc68k::LSd_memory,
        "ror" | "rol" => Mc68k::ROd_memory,
        _ => panic!("shifting_rotation_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
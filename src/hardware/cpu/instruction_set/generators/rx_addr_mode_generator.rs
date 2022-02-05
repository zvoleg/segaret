use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::addressing_mode::AddrMode;
use crate::hardware::Register;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::RxAddrModeMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;

struct RxAddrModeInstPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
    addr_mode_aliases: &'static str,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 4, rx_type_alias: 'a', addr_mode_aliases: "DAa+-dxWLPXi"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 4, rx_type_alias: 'a', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000100001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "d"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000100001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "d"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000110001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "d"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000111001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "d"
        },

        RxAddrModeInstPattern {
            name: "lea", mask: 0b0100000111000000, size: Size::Long, clock: 0, rx_type_alias: 'a', addr_mode_aliases: "adWLP"
        },
        RxAddrModeInstPattern {
            name: "lea", mask: 0b0100000111000000, size: Size::Long, clock: 2, rx_type_alias: 'a', addr_mode_aliases: "xX"
        },

        // add into data register
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi",
        },
        
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "DAi",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWLPX",
        },

        // add into addr_mode
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

        // adda
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DAa+-dxWLPXi",
        },

        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DAi",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 6, rx_type_alias: 'a', addr_mode_aliases: "a+-dxWLPX",
        },

        // sub from data register
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi",
        },
        
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "DAi",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWLPX",
        },

        // sub from addr_mode
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL",
        },

         // suba
         RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DAa+-dxWLPXi",
        },

        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DAi",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 6, rx_type_alias: 'a', addr_mode_aliases: "a+-dxWLPX",
        },

        // cmp
        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "muls", mask: 0b1100000111000000, size: Size::Word, clock: 70, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "mulu", mask: 0b1100000011000000, size: Size::Word, clock: 70, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "divs", mask: 0b1000000111000000, size: Size::Word, clock: 158, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "divu", mask: 0b1000000011000000, size: Size::Word, clock: 140, rx_type_alias: 'd', addr_mode_aliases: "DAa+-dxWLPXi"
        },

        // and to register
        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000010000000, size: Size::Long, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "Di"
        },
        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        //and to memory
        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000100000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000101000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "and", mask: 0b1100000110000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        //eor
        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000100000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "D"
        },
        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000100000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000101000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "D"
        },
        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000101000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000110000000, size: Size::Long, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "D"
        },
        RxAddrModeInstPattern {
            name: "eor", mask: 0b1011000110000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWL"
        },

        // or to register
        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "Da+-dxWLPXi"
        },

        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000010000000, size: Size::Long, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "Di"
        },
        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWLPX"
        },

        //or to memory
        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000100000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000101000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },

        RxAddrModeInstPattern {
            name: "or", mask: 0b1000000110000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+-dxWL"
        },
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let rx_type = register_type_by_char(pattern.rx_type_alias);
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();

        for addr_mode_type in addr_mode_type_list {
            let addr_mode_list = get_addr_mode_table(addr_mode_type);

            let clock_period = if pattern.name != "movep" {
                match pattern.size {
                    Size::Byte | Size::Word => pattern.clock + addr_mode_type.get_clock_periods_short(),
                    Size::Long => pattern.clock + addr_mode_type.get_clock_periods_long(),
                }
            } else {
                pattern.clock
            };

            (0..8).for_each(|i| {
                addr_mode_list.iter()
                    .for_each(|mode| {
                        let opcode = generate_mask(&pattern.name, mask, i, mode);
                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            pattern.name,
                            opcode,
                            pattern.size,
                            clock_period,
                            cpu_function_by_name(pattern.name),
                            RxAddrModeMetadata::new(Register::new(rx_type, i as usize), *mode),
                        ));
                    });
            })
        }
    }
}

fn generate_mask(name: &str, mask: u16, rx_idx: u16, addr_mode: &AddrMode) -> u16 {
    match name {
        "movep" => mask | rx_idx << 9 | (*addr_mode).reg_idx as u16,
        _ => mask | rx_idx << 9 | ((*addr_mode).mode_bits as u16) << 3 | (*addr_mode).reg_idx as u16,
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "movea" => Mc68k::MOVEA,
        "movep" => Mc68k::MOVEP,
        "lea" => Mc68k::LEA,
        "add" => Mc68k::ADD,
        "adda" => Mc68k::ADDA,
        "sub" => Mc68k::SUB,
        "suba" => Mc68k::SUBA,
        "cmp" => Mc68k::CMP,
        "cmpa" => Mc68k::CMPA,
        "muls" => Mc68k::MULS,
        "mulu" => Mc68k::MULU,
        "divs" => Mc68k::DIVS,
        "divu" => Mc68k::DIVU,
        "and" => Mc68k::AND,
        "eor" => Mc68k::EOR,
        "or" => Mc68k::OR,
        _ => panic!("rx_addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
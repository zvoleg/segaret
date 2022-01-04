use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::AddrModeMetadata;
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

pub(in crate::hardware) fn generate() -> Vec<Instruction<AddrModeMetadata>> {
    let patterns = vec![
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101000000000, size: Size::Byte, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101001000000, size: Size::Word, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },
        AddrModeInstPattern {
            name: String::from("tst"), mask: 0b0100101010000000, size: Size::Long, clock: 4, addr_mode_aliases: String::from("DAa+-dxWLi"),
        },

        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 12, addr_mode_aliases: String::from("a"),
        },
        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 16, addr_mode_aliases: String::from("dPW"),
        },
        AddrModeInstPattern {
            name: String::from("pea"), mask: 0b0100100001000000, size: Size::Long, clock: 20, addr_mode_aliases: String::from("xXL"),
        },
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
                        AddrModeMetadata::new(*mode),
                    )
                })
                .collect::<Vec<Instruction<AddrModeMetadata>>>());
        }
    }
    
    instruction_set
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "tst" => Mc68k::TST,
        "pea" => Mc68k::PEA,
        _ => panic!("addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
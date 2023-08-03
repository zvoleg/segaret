use crate::hardware::mc68k::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::mc68k::addressing_mode::AddrModeType;
use crate::hardware::mc68k::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::mc68k::instruction_set::generators::condition_by_bits;
use crate::hardware::mc68k::instruction_set::ConditionAddrModeMetadata;
use crate::hardware::mc68k::instruction_set::Instruction;
use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::hardware::mc68k::mc68k_emu::Mc68k;
use crate::hardware::Size;

struct ConditionAddrModePattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    addr_mode_aliases: &'static str,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        ConditionAddrModePattern {
            name: "scc", mask: 0b0101000011000000, size: Size::Byte, clock: 4, addr_mode_aliases: "D"
        },
        ConditionAddrModePattern {
            name: "scc", mask: 0b0101000011000000, size: Size::Byte, clock: 8, addr_mode_aliases: "a+-dxWL"
        },
    ];
    
    for pattern in patterns {
        let mask = pattern.mask;
        let addr_mode_type_list = pattern.addr_mode_aliases.chars()
                                    .map(|c| addr_mode_type_by_char(c))
                                    .collect::<Vec<AddrModeType>>();
                        
        for addr_mode_type in addr_mode_type_list {
            let addr_modes = get_addr_mode_table(addr_mode_type);

            addr_modes.iter().for_each(|addr_mode| {
                (0..0x10).for_each(|c| {
                    let opcode = mask | c << 8 | (addr_mode.mode_bits as u16) << 3 | addr_mode.reg_idx as u16;
                    opcode_table[opcode as usize] = Box::new(Instruction::new(
                        pattern.name,
                        opcode,
                        pattern.size,
                        pattern.clock,
                        Mc68k::Scc,
                        ConditionAddrModeMetadata::new(
                            condition_by_bits(c as u32),
                            *addr_mode
                        )
                    ));
                });
            });
        }
    }
}

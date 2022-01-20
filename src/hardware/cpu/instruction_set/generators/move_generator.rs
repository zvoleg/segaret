use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::MoveInstructionMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;
use crate::hardware::Size;

struct MoveInstructionPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
    src_addr_mode_aliases: String,
    dst_addr_mode_aliases: String,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 4, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 8, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 8, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 10, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 26, src_addr_mode_aliases: String::from("dX"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 26, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Word, mask: 0b0011000000000000, clock: 28, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 4, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 12, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 12, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 14, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 16, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 16, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 18, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 18, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: String::from("DA"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 22, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 24, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 24, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("X"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("X"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 34, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 34, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Long, mask: 0b0010000000000000, clock: 36, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 4, src_addr_mode_aliases: String::from("D"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 8, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 8, src_addr_mode_aliases: String::from("D"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 10, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: String::from("D"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: String::from("D"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("D"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: String::from("D"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("X"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: String::from("a+i"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("a+-"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: String::from("-"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: String::from("dPW"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: String::from("xX"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("dW"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 26, src_addr_mode_aliases: String::from("dX"), dst_addr_mode_aliases: String::from("L"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 26, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("x"),
        },
        MoveInstructionPattern {
            name: String::from("move"), size: Size::Byte, mask: 0b0001000000000000, clock: 28, src_addr_mode_aliases: String::from("L"), dst_addr_mode_aliases: String::from("L"),
        },
    ];

    for pattern in patterns {

        let name = pattern.name;
        let mask = pattern.mask;
        let size = pattern.size;
        let clock = pattern.clock;

        let src_addr_mode_type_list = pattern.src_addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();
        let dst_addr_mode_type_list = pattern.dst_addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();

        for src_addr_mode_type in src_addr_mode_type_list {
            for dst_addr_mode_type in &dst_addr_mode_type_list {
                let src_addr_modes = get_addr_mode_table(src_addr_mode_type);
                let dst_addr_modes = get_addr_mode_table(*dst_addr_mode_type);

                src_addr_modes.iter().for_each(|src_mode| {
                    dst_addr_modes.iter().for_each(|dst_mode| {
                        let opcode = mask | (src_mode.reg_idx as u16) << 9 | (src_mode.mode_bits as u16) << 6 | (dst_mode.mode_bits as u16) << 3 | dst_mode.reg_idx as u16;

                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            name.clone(),
                            opcode,
                            size,
                            clock,
                            Mc68k::MOVE,
                            MoveInstructionMetadata::new(*src_mode, *dst_mode)));
                    });
                });
            }
        }
    }
}

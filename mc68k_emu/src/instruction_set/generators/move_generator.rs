use crate::instruction_set::InstructionProcess;
use crate::instruction_set::addr_mode_table::get_am_bits;
use crate::instruction_set::generators::addr_mode_type_by_char;
use crate::addressing_mode::AddrModeType;
use crate::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::instruction_set::MoveInstructionMetadata;
use crate::instruction_set::Instruction;
use crate::mc68k_emu::Mc68k;
use hardware::Size;

struct MoveInstructionPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    src_addr_mode_aliases: &'static str,
    dst_addr_mode_aliases: &'static str,
}

pub(in crate) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 4, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 8, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 8, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 10, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 12, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 14, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 16, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 18, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 20, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 22, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 24, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 26, src_addr_mode_aliases: "dX", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 26, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Word, mask: 0b0011000000000000, clock: 28, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 4, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 12, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 12, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 14, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 16, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 16, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 18, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 18, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: "DA", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 20, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 22, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 24, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 24, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 26, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "X",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 28, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 30, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "X",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 32, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 34, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 34, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Long, mask: 0b0010000000000000, clock: 36, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 4, src_addr_mode_aliases: "D", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 8, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 8, src_addr_mode_aliases: "D", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 10, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: "D", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 12, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: "D", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 14, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: "D", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 16, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "D",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "X",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 18, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: "a+i", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 20, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "a+-",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: "-", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 22, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: "dPW", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: "xX", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 24, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "dW",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 26, src_addr_mode_aliases: "dX", dst_addr_mode_aliases: "L",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 26, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "x",
        },
        MoveInstructionPattern {
            name: "move", size: Size::Byte, mask: 0b0001000000000000, clock: 28, src_addr_mode_aliases: "L", dst_addr_mode_aliases: "L",
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
                        let opcode = mask | (dst_mode.reg_idx as u16) << 9 | get_am_bits(dst_mode.am_type) << 6 | get_am_bits(src_mode.am_type) << 3 | src_mode.reg_idx as u16;

                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            name,
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

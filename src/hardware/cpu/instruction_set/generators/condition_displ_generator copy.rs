use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::instruction_set::generators::condition_by_bits;
use crate::hardware::Size;
use crate::hardware::cpu::instruction_set::ConditionDisplacementMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;

struct ConditionDisplPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        ConditionDisplPattern {
            name: "bcc", mask: 0b0110000000000000, size: Size::Byte, clock: 10,
        }
    ];
    
    for pattern in patterns {
        let mask = pattern.mask;

        (0..0x10).for_each(|c| {
            (0..0x100).for_each(|d| {
                let opcode = mask | c << 8 | d;
                let displacement_size = if d == 0 {
                    Size::Word
                } else {
                    Size::Byte
                };
                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    Mc68k::Bcc,
                    ConditionDisplacementMetadata::new(
                        condition_by_bits(c as u32),
                        d as u32,
                        displacement_size,
                    )
                ));
            });
        });
    }
}

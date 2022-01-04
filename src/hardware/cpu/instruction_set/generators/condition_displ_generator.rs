use crate::hardware::cpu::instruction_set::generators::condition_by_bits;
use crate::hardware::Size;
use crate::hardware::cpu::instruction_set::ConditionDisplacementMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::mc68k_emu::Mc68k;

struct ConditionDisplPattern {
    name: String,
    mask: u16,
    size: Size,
    clock: u32,
}

pub(in crate::hardware) fn generate() -> Vec<Instruction<ConditionDisplacementMetadata>> {
    let patterns = vec![
        ConditionDisplPattern {
            name: String::from("bcc"), mask: 0b0110000000000000, size: Size::Byte, clock: 10,
        }
    ];
    
    let mut instruction_set = Vec::new();

    for pattern in patterns {
        let mask = pattern.mask;

        (0..0x10).for_each(|c| {
            let mut instructions = (0..0x100).map(|d| {
                let opcode = mask | c << 8 | d;
                let displacement_size = if d == 0 {
                    Size::Word
                } else {
                    Size::Byte
                };
                Instruction::new(
                    pattern.name.clone(),
                    opcode,
                    pattern.size,
                    pattern.clock,
                    Mc68k::Bcc,
                    ConditionDisplacementMetadata::new(
                        condition_by_bits(c as u32),
                        d as u32,
                        displacement_size,
                    )
                )
            }).collect::<Vec<Instruction<ConditionDisplacementMetadata>>>();

            instruction_set.append(&mut instructions);
        });
    }

    instruction_set
}
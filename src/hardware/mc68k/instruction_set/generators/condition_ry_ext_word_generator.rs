use crate::hardware::Register;
use crate::hardware::mc68k::instruction_set::ConditionRyExtWordMetadata;
use crate::hardware::mc68k::instruction_set::InstructionProcess;
use crate::hardware::mc68k::instruction_set::generators::condition_by_bits;
use crate::hardware::Size;
use crate::hardware::mc68k::instruction_set::Instruction;
use crate::hardware::mc68k::mc68k_emu::Mc68k;

struct ConditionRyExtWordPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        ConditionRyExtWordPattern {
            name: "dbcc", mask: 0b0101000011001000, size: Size::Word, clock: 10,
        }
    ];
    
    for pattern in patterns {
        let mask = pattern.mask;

        (0..0x10).for_each(|c| {
            (0..0x8).for_each(|d| {
                let opcode = mask | c << 8 | d;
                opcode_table[opcode as usize] = Box::new(Instruction::new(
                    pattern.name,
                    opcode,
                    pattern.size,
                    pattern.clock,
                    Mc68k::DBcc,
                    ConditionRyExtWordMetadata::new(
                        condition_by_bits(c as u32),
                        Register::data(d as usize),
                    )
                ));
            });
        });
    }
}

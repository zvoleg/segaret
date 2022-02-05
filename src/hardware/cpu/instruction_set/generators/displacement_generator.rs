use crate::hardware::cpu::instruction_set::DisplacementMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::mc68k_emu::Mc68k;
use crate::hardware::Size;

struct DisplPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        DisplPattern {
            name: "bra", mask: 0b0110000000000000, size: Size::Byte, clock: 10,
        },
        DisplPattern {
            name: "bsr", mask: 0b0110000100000000, size: Size::Byte, clock: 18,
        }
    ];
    
    for pattern in patterns {
        let mask = pattern.mask;

        (0..0x100).for_each(|d| {
            let opcode = mask | d;
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
                cpu_function_by_name(pattern.name),
                DisplacementMetadata::new(
                    d as u32,
                    displacement_size,
                )
            ));
        });
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "bra" => Mc68k::BRA,
        "bsr" => Mc68k::BSR,
        _ => panic!("displacement_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}

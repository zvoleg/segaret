use crate::hardware::cpu::instruction_set::ExplicitMetadata;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;
use crate::hardware::cpu::instruction_set::InstructionProcess;

struct ExplicitPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        ExplicitPattern {
            name: "nop", mask: 0b0100111001110001, size: Size::Byte ,clock: 4,
        },

        ExplicitPattern {
            name: "rte", mask: 0b0100111001110011, size: Size::Byte ,clock: 20,
        },

        ExplicitPattern {
            name: "rtr", mask: 0b0100111001110111, size: Size::Byte ,clock: 20,
        },

        ExplicitPattern {
            name: "rts", mask: 0b0100111001110101, size: Size::Byte ,clock: 16,
        },
    ];

    for pattern in patterns {
        let opcode = pattern.mask;
        opcode_table[opcode as usize] = Box::new(Instruction::new(
            pattern.name,
            opcode,
            pattern.size,
            pattern.clock,
            cpu_function_by_name(pattern.name),
            ExplicitMetadata,
        ))
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "nop" => Mc68k::NOP,
        "rte" => Mc68k::RTE,
        "rtr" => Mc68k::RTR,
        "rts" => Mc68k::RTS,
        _ => panic!("explicit_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
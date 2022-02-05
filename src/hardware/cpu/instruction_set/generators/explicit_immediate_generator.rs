use crate::hardware::cpu::instruction_set::ExplicitImmediateMetadata;
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
            name: "andi_to_ccr", mask: 0b0000001000111100, size: Size::Byte ,clock: 20,
        },
        ExplicitPattern {
            name: "andi_to_sr", mask: 0b0000001001111100, size: Size::Word ,clock: 20,
        },

        ExplicitPattern {
            name: "eori_to_ccr", mask: 0b0000101000111100, size: Size::Byte ,clock: 20,
        },
        ExplicitPattern {
            name: "eori_to_sr", mask: 0b0000101001111100, size: Size::Word ,clock: 20,
        },

        ExplicitPattern {
            name: "ori_to_ccr", mask: 0b0000000000111100, size: Size::Byte ,clock: 20,
        },
        ExplicitPattern {
            name: "ori_to_sr", mask: 0b0000101001111100, size: Size::Word ,clock: 20,
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
            ExplicitImmediateMetadata::new(),
        ))
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "andi_to_ccr" => Mc68k::ANDI_to_CCR,
        "andi_to_sr" => Mc68k::ANDI_to_SR,
        "eori_to_ccr" => Mc68k::EORI_to_CCR,
        "eori_to_sr" => Mc68k::EORI_to_SR,
        "ori_to_ccr" => Mc68k::ORI_to_CCR,
        "ori_to_sr" => Mc68k::ORI_to_SR,
        _ => panic!("explicit_immediate_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
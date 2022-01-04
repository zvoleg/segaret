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

pub(in crate::hardware) fn generate() -> Vec<Instruction<MoveInstructionMetadata>> {
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
        // move	w					0011000000000000		...-........	d...........	10
        // move	w					0011000000000000		da..........	.....D.W....	12
        // move	w					0011000000000000		..A+.....i..	..A+-.......	12
        // move	w					0011000000000000		.....D.W..p.	d...........	12
        // move	w					0011000000000000		da..........	......X.....	14
        // move	w					0011000000000000		....-.......	..A+-.......	14
        // move	w					0011000000000000		......X....x	d...........	14
        // move	w					0011000000000000		da..........	........L...	16
        // move	w					0011000000000000		..A+.....i..	.....D.W....	16
        // move	w					0011000000000000		.....D.W..p.	..A+-.......	16
        // move	w					0011000000000000		..A+.....i..	......X.....	18
        // move	w					0011000000000000		....-.......	.....D.W....	18
        // move	w					0011000000000000		......X....x	..A+-.......	18
        // move	w					0011000000000000		..A+.....i..	........L...	20
        // move	w					0011000000000000		....-.......	......X.....	20
        // move	w					0011000000000000		.....D.W..p.	.....D.W....	20
        // move	w					0011000000000000		........L...	..A+-.......	20
        // move	w					0011000000000000		....-.......	........L...	22
        // move	w					0011000000000000		.....D.W..p.	......X.....	22
        // move	w					0011000000000000		......X....x	.....D.W....	22
        // move	w					0011000000000000		.....D.W..p.	........L...	24
        // move	w					0011000000000000		......X....x	......X.....	24
        // move	w					0011000000000000		........L...	.....D.W....	24
        // move	w					0011000000000000		.....D.....x	........L...	26
        // move	w					0011000000000000		........L...	......X.....	26
        // move	w					0011000000000000		........L...	........L...	28
        // move	l					0010000000000000		da..........	d...........	4
        // move	l					0010000000000000		da..........	..A+-.......	12
        // move	l					0010000000000000		..A+.....i..	d...........	12
        // move	l					0010000000000000		...-........	d...........	14
        // move	l					0010000000000000		da..........	.....D.W....	16
        // move	l					0010000000000000		.....D.W..p.	d...........	16
        // move	l					0010000000000000		da..........	......X.....	18
        // move	l					0010000000000000		......X....x	d...........	18
        // move	l					0010000000000000		da..........	........L...	20
        // move	l					0010000000000000		..A+.....i..	..A+-.......	20
        // move	l					0010000000000000		........L...	d...........	20
        // move	l					0010000000000000		...-........	..A+-.......	22
        // move	l					0010000000000000		..A+.....i..	.....D.W....	24
        // move	l					0010000000000000		.....D.W..p.	..A+-.......	24
        // move	l					0010000000000000		..A+.....i..	......X.....	26
        // move	l					0010000000000000		...-........	.....D.W....	26
        // move	l					0010000000000000		......X....x	..A+-.......	26
        // move	l					0010000000000000		..A+.....i..	........L...	28
        // move	l					0010000000000000		...-........	......X.....	28
        // move	l					0010000000000000		.....D.W..p.	.....D.W....	28
        // move	l					0010000000000000		........L...	..A+-.......	28
        // move	l					0010000000000000		...-........	........L...	30
        // move	l					0010000000000000		.....D.W..p.	......X.....	30
        // move	l					0010000000000000		......X....x	.....D.W....	30
        // move	l					0010000000000000		.....D.W..p.	........L...	32
        // move	l					0010000000000000		......X....x	......X.....	32
        // move	l					0010000000000000		........L...	.....D.W....	32
        // move	l					0010000000000000		......X....x	........L...	34
        // move	l					0010000000000000		........L...	......X.....	34
        // move	l					0010000000000000		........L...	........L...	36
        // move	b					0001000000000000		d...........	d...........	4
        // move	b					0001000000000000		..A+.....i..	d...........	8
        // move	b					0001000000000000		d...........	..A+-.......	8
        // move	b					0001000000000000		...-........	d...........	10
        // move	b					0001000000000000		d...........	.....D.W....	12
        // move	b					0001000000000000		..A+.....i..	..A+-.......	12
        // move	b					0001000000000000		.....D.W..p.	d...........	12
        // move	b					0001000000000000		d...........	......X.....	14
        // move	b					0001000000000000		....-.......	..A+-.......	14
        // move	b					0001000000000000		......X....x	d...........	14
        // move	b					0001000000000000		d...........	........L...	16
        // move	b					0001000000000000		..A+.....i..	.....D.W....	16
        // move	b					0001000000000000		.....D.W..p.	..A+-.......	16
        // move	b					0001000000000000		..A+.....i..	......X.....	18
        // move	b					0001000000000000		....-.......	.....D.W....	18
        // move	b					0001000000000000		......X....x	..A+-.......	18
        // move	b					0001000000000000		..A+.....i..	........L...	20
        // move	b					0001000000000000		....-.......	......X.....	20
        // move	b					0001000000000000		.....D.W..p.	.....D.W....	20
        // move	b					0001000000000000		........L...	..A+-.......	20
        // move	b					0001000000000000		....-.......	........L...	22
        // move	b					0001000000000000		.....D.W..p.	......X.....	22
        // move	b					0001000000000000		......X....x	.....D.W....	22
        // move	b					0001000000000000		.....D.W..p.	........L...	24
        // move	b					0001000000000000		......X....x	......X.....	24
        // move	b					0001000000000000		........L...	.....D.W....	24
        // move	b					0001000000000000		.....D.....x	........L...	26
        // move	b					0001000000000000		........L...	......X.....	26
        // move	b					0001000000000000		........L...	........L...	28
    ];

    let mut instruction_set = Vec::new();
    
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
                    let mut instructions = dst_addr_modes.iter().map(|dst_mode| {
                        let opcode = mask | (src_mode.reg_idx as u16) << 9 | (src_mode.mode_bits as u16) << 6 | (dst_mode.mode_bits as u16) << 3 | dst_mode.reg_idx as u16;

                        Instruction::new(
                            name.clone(),
                            opcode,
                            size,
                            clock,
                            Mc68k::MOVE,
                            MoveInstructionMetadata::new(*src_mode, *dst_mode))
                    }).collect::<Vec<Instruction<MoveInstructionMetadata>>>();
                    
                    instruction_set.append(&mut instructions);  
                });
            }
        }
    }

    instruction_set
}

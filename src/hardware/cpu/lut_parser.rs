extern crate csv;
extern crate serde;

use crate::hardware::OperationMode;
use crate::hardware::Address;
use crate::hardware::{AddrMode, AddrModeType, AddressType, Instruction, Size};
use super::Mc68k;

use csv::Trim;
use serde::Deserialize;

use std::fs::File;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Record {
    name: String,
    size: char,
    op_mode: Option<String>,
    spec_am: Option<char>,
    spec_reg_x: Option<char>,
    spec_reg_y: Option<char>,
    mask: String,
    addr_mode: Option<String>,
    src_addr_mode: Option<String>,
    dst_addr_mode: Option<String>,
    clock: i32,
    ext_word: Option<bool>,
}

enum MaksType {
    Undefined,
    AddressingMode,
    RXAdrressingMode,
    RY,
    RXRY,
    Move,
    RxData,
    Data,
}

pub fn parse() -> Vec<Instruction> {
    let file = File::open("lut.csv").unwrap();
    let mut csv_result = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .trim(Trim::All)
        .from_reader(file);

    let records: Vec<Record> = csv_result.deserialize().map(|r| r.unwrap()).collect();

    let mut opcode_table = Vec::new();
    for _ in 0..0x10000 {
        opcode_table.push(Instruction::new(0));
    }

    for record in records {
        let mask = record.mask;
        
        let mut base_opcode = 0;
        let mut offset = 15;

        let mut gap_offset_positions = Vec::new();
        for char_ in mask.chars() {
            if char_ != '.' {
                let digit = char_.to_digit(10).unwrap() as usize;
                base_opcode |= digit << offset;
            }
            else {
                gap_offset_positions.push(offset);
            }
            offset -= 1;
        }

        let mut gap_sizes = Vec::new();
        let mut previous_offset = 0;
        for offset_position in gap_offset_positions {
            if offset_position + 1 == previous_offset {
                if let Some(last) = gap_sizes.last_mut() {
                    *last += 1;
                }
            } else {
                gap_sizes.push(1);
            }
            previous_offset = offset_position;
        }

        let mask_type = match gap_sizes.len() {
            1 => {
                let gap = gap_sizes[0];
                match gap {
                    3 => MaksType::RY,
                    6 => MaksType::AddressingMode,
                    8 => MaksType::Data,
                    12 => MaksType::Move,
                    _ => MaksType::Undefined,
                }
            },
            2 => {
                let (gap_a, gap_b) = (gap_sizes[0], gap_sizes[1]);
                match (gap_a, gap_b) {
                    (3, 3) => MaksType::RXRY,
                    (3, 6) => MaksType::RXAdrressingMode,
                    (3, 8) => MaksType::RxData,
                    _ => MaksType::Undefined,
                }
            },
            _ => MaksType::Undefined,
        };

        let size = get_size_by_char(&record.size);
        let ext_word = if let Some(_) = record.ext_word {
            Some(0)
        } else {
            None
        };
        let op_mode = if let Some(op_mode_str) = record.op_mode {
            Some(get_op_mode_by_chars(&op_mode_str))
        } else {
            None
        };

        match mask_type {
            MaksType::AddressingMode => {
                let addr_modes = collect_addr_modes(&record.addr_mode);

                for addr_mode_variants in addr_modes {
                    for addr_mode_variant in addr_mode_variants {
                        let opcode = base_opcode | addr_mode_variant.mode_bits << 3 | addr_mode_variant.reg_idx;
                        // println!("{}: {:04X}", record.name.clone(), opcode);
                        opcode_table[opcode] = Instruction {
                            operation_word: opcode as u16,
                            name: record.name.clone(),
                            func_: get_func_by_name(&record.name),
                            operation_size: size,
    
                            op_mode: op_mode,
    
                            inst_ext_word: ext_word,
                            addr_mode: Some(addr_mode_variant),
                            addr_mode_ext_word: None,
    
                            src_addr_mode: None,
                            src_ext_word: None,
    
                            dst_addr_mode: None,
                            dst_ext_word: None,
    
                            spec_reg_x: None,
                            spec_reg_y: None,
    
                            clock: record.clock,
                        }
                    }
                }
            },
            MaksType::Move => {
                let src_addr_modes = collect_addr_modes(&record.src_addr_mode);
                let dst_addr_modes = collect_addr_modes(&record.dst_addr_mode);

                for src_addr_mode_variants in src_addr_modes {
                    for src_addr_mode_variant in src_addr_mode_variants {
                        for dst_addr_mode_variants in &dst_addr_modes {
                            for dst_addr_mode_variant in dst_addr_mode_variants {
                                let opcode = base_opcode
                                    | dst_addr_mode_variant.reg_idx << 9
                                    | dst_addr_mode_variant.mode_bits << 6
                                    | src_addr_mode_variant.mode_bits << 3
                                    | src_addr_mode_variant.reg_idx;

                                opcode_table[opcode] = Instruction {
                                    operation_word: opcode as u16,
                                    name: record.name.clone(),
                                    func_: get_func_by_name(&record.name),
                                    operation_size: size,
        
                                    op_mode: op_mode,
        
                                    inst_ext_word: ext_word,
                                    addr_mode: None,
                                    addr_mode_ext_word: None,
        
                                    src_addr_mode: Some(src_addr_mode_variant),
                                    src_ext_word: None,
        
                                    dst_addr_mode: Some(*dst_addr_mode_variant),
                                    dst_ext_word: None,
        
                                    spec_reg_x: None,
                                    spec_reg_y: None,
        
                                    clock: record.clock,
                                }
                            }
                        }
                    }
                }
            },
            MaksType::RXAdrressingMode => {
                let addr_modes = collect_addr_modes(&record.addr_mode);
                let registers_x = collect_registers(&record.spec_reg_x);

                for reg_x in registers_x {
                    for addr_mode_variants in &addr_modes {
                        for addr_mode_variant in addr_mode_variants {
                            let opcode = base_opcode 
                                | reg_x.address << 9 
                                | addr_mode_variant.mode_bits << 6 
                                | addr_mode_variant.reg_idx;
                            opcode_table[opcode] = Instruction {
                                operation_word: opcode as u16,
                                name: record.name.clone(),
                                func_: get_func_by_name(&record.name),
                                operation_size: size,
    
                                op_mode: op_mode,
    
                                inst_ext_word: ext_word,
                                addr_mode: Some(*addr_mode_variant),
                                addr_mode_ext_word: None,
    
                                src_addr_mode: None,
                                src_ext_word: None,
    
                                dst_addr_mode: None,
                                dst_ext_word: None,
    
                                spec_reg_x: Some(reg_x),
                                spec_reg_y: None,
    
                                clock: record.clock,
                            }
                        }
                    }
                }
            }
            MaksType::RXRY => {
                let registers_x = collect_registers(&record.spec_reg_x);
                let registers_y = collect_registers(&record.spec_reg_y);

                for reg_x in registers_x {
                    for reg_y in &registers_y {
                        let opcode = base_opcode
                            | reg_x.address << 9
                            | reg_y.address;

                        opcode_table[opcode] = Instruction {
                            operation_word: opcode as u16,
                            name: record.name.clone(),
                            func_: get_func_by_name(&record.name),
                            operation_size: size,

                            op_mode: op_mode,

                            inst_ext_word: ext_word,
                            addr_mode: None,
                            addr_mode_ext_word: None,

                            src_addr_mode: None,
                            src_ext_word: None,

                            dst_addr_mode: None,
                            dst_ext_word: None,

                            spec_reg_x: Some(reg_x),
                            spec_reg_y: Some(*reg_y),

                            clock: record.clock,
                        }
                    }
                }
            },
            MaksType::RxData => {
                let registers_x = collect_registers(&record.spec_reg_x);
                for reg_x in registers_x {
                    for data in 0..0x100 {
                        let opcode = base_opcode
                            | reg_x.address << 9
                            | data;
                        opcode_table[opcode] = Instruction {
                            operation_word: opcode as u16,
                            name: record.name.clone(),
                            func_: get_func_by_name(&record.name),
                            operation_size: size,

                            op_mode: op_mode,

                            inst_ext_word: ext_word,
                            addr_mode: None,
                            addr_mode_ext_word: None,

                            src_addr_mode: None,
                            src_ext_word: None,

                            dst_addr_mode: None,
                            dst_ext_word: None,

                            spec_reg_x: Some(reg_x),
                            spec_reg_y: None,

                            clock: record.clock,
                        }
                    }
                }
            },
            MaksType::Data => {
                for data in 0..0x100 {
                    let opcode = base_opcode
                        | data;
                    opcode_table[opcode] = Instruction {
                        operation_word: opcode as u16,
                        name: record.name.clone(),
                        func_: get_func_by_name(&record.name),
                        operation_size: size,

                        op_mode: op_mode,

                        inst_ext_word: ext_word,
                        addr_mode: None,
                        addr_mode_ext_word: None,

                        src_addr_mode: None,
                        src_ext_word: None,

                        dst_addr_mode: None,
                        dst_ext_word: None,

                        spec_reg_x: None,
                        spec_reg_y: None,

                        clock: record.clock,
                    }
                }
            }
            _ => panic!(),
        }
    }

    opcode_table
}

fn collect_addr_modes(addr_modes: &Option<String>) -> Vec<Vec<AddrMode>> {
    if let Some(addr_mode) = addr_modes {
        addr_mode
            .chars()
            .filter(|c| *c != '.')
            .map(|c| get_addr_mode_type_by_char(&c))
            .map(|a| generate_addr_modes(&a))
            .collect::<Vec<Vec<AddrMode>>>()
    } else {
        Vec::new()
    }
}

fn collect_registers(c: &Option<char>) -> Vec<Address> {
    if let Some(c) = c {
        match c {
            'a' => {
                (0..8)
                    .map(|i| Address::new(AddressType::AddrReg, i))
                    .collect::<Vec<Address>>()
            },
            'd' => {
                (0..8)
                    .map(|i| Address::new(AddressType::DataReg, i))
                    .collect::<Vec<Address>>()
            },
            _ => panic!("Unexpected char for collect_regisetrs {}", c)
        }
    } else {
        Vec::new()
    }
}

fn get_op_mode_by_chars(s: &str) -> OperationMode {
    match s {
        "dd" => OperationMode::DataToData,
        "aa" => OperationMode::AddressToAddress,
        "da" => OperationMode::DataToAddress,
        "mr" => OperationMode::MemoryToRegister,
        "rm" => OperationMode::RegisterToMemory,
        _ => panic!("Unexpected op_mode pattern '{}'", s),
    }
}

fn get_size_by_char(c: &char) -> Size {
    match c {
        'b' => Size::Byte,
        'w' => Size::Word,
        'l' => Size::Long,
        _ => panic!("Unexpected char for Size generation"),
    }
}

fn get_addr_mode_type_by_char(c: &char) -> AddrModeType {
    match c {
        'd' => AddrModeType::Data,
        'a' => AddrModeType::Addr,
        'A' => AddrModeType::AddrInd,
        '+' => AddrModeType::AddrIndPostInc,
        '-' => AddrModeType::AddrIndPreDec,
        'D' => AddrModeType::AddrIndDips,
        'X' => AddrModeType::AddrIndIdx,
        'W' => AddrModeType::AbsShort,
        'L' => AddrModeType::AbsLong,
        'i' => AddrModeType::Immediate,
        'p' => AddrModeType::PcDisp,
        'x' => AddrModeType::PcIdx,
        _ => panic!("Unexpected char for AddrModeType generation"),
    }
}

fn generate_addr_modes(addr_mode: &AddrModeType) -> Vec<AddrMode> {
    match addr_mode {
        AddrModeType::Data => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::Data,
                mode_bits: 0b000,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::Addr => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::Addr,
                mode_bits: 0b001,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AddrInd => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::AddrInd,
                mode_bits: 0b010,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AddrIndPostInc => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::AddrIndPostInc,
                mode_bits: 0b011,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AddrIndPreDec => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::AddrIndPreDec,
                mode_bits: 0b100,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AddrIndDips => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::AddrIndDips,
                mode_bits: 0b101,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AddrIndIdx => (0..8)
            .map(|i| AddrMode {
                am_type: AddrModeType::AddrIndIdx,
                mode_bits: 0b110,
                reg_idx: i,
            })
            .collect::<Vec<AddrMode>>(),
        AddrModeType::AbsShort => vec![AddrMode {
            am_type: AddrModeType::AbsShort,
            mode_bits: 0b111,
            reg_idx: 0b000,
        }],
        AddrModeType::AbsLong => vec![AddrMode {
            am_type: AddrModeType::AbsLong,
            mode_bits: 0b111,
            reg_idx: 0b001,
        }],
        AddrModeType::Immediate => vec![AddrMode {
            am_type: AddrModeType::Immediate,
            mode_bits: 0b111,
            reg_idx: 0b100,
        }],
        AddrModeType::PcDisp => vec![AddrMode {
            am_type: AddrModeType::PcDisp,
            mode_bits: 0b111,
            reg_idx: 0b010,
        }],
        AddrModeType::PcIdx => vec![AddrMode {
            am_type: AddrModeType::PcIdx,
            mode_bits: 0b111,
            reg_idx: 0b011,
        }],
    }
}

fn get_func_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "move" => Mc68k::MOVE,
        "movea" => Mc68k::MOVEA,
        "movem" => Mc68k::MOVEM,
        "movep" => Mc68k::MOVEP,
        "moveq" => Mc68k::MOVEQ,
        "exg" => Mc68k::EXG,
        "lea" => Mc68k::LEA,
        "pea" => Mc68k::PEA,
        "tst" => Mc68k::TST,
        "bcc" => Mc68k::Bcc,
        _ => panic!("Unexpected instruction name for fetchin method from Mc68k"),
    }
}

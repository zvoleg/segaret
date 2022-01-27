use crate::hardware::cpu::instruction_set::InstructionProcess;
use crate::hardware::cpu::addressing_mode::AddrMode;
use crate::hardware::Register;
use crate::Mc68k;
use crate::hardware::cpu::instruction_set::RxAddrModeMetadata;
use crate::hardware::cpu::instruction_set::Instruction;
use crate::hardware::Size;
use crate::hardware::cpu::addressing_mode::AddrModeType;
use crate::hardware::cpu::instruction_set::generators::addr_mode_type_by_char;
use crate::hardware::cpu::instruction_set::addr_mode_table::get_addr_mode_table;
use crate::hardware::cpu::instruction_set::generators::register_type_by_char;

struct RxAddrModeInstPattern {
    name: &'static str,
    mask: u16,
    size: Size,
    clock: u32,
    rx_type_alias: char,
    addr_mode_aliases: &'static str,
}

pub(in crate::hardware) fn generate(opcode_table: &mut Vec<Box<dyn InstructionProcess>>) {
    let patterns = vec![
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 4, rx_type_alias: 'a', addr_mode_aliases: "DA"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "a+i"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 10, rx_type_alias: 'a', addr_mode_aliases: "-"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "dWP"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "xX"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0011000001000000, size: Size::Word, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "L"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 4, rx_type_alias: 'a', addr_mode_aliases: "DA"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "a+i"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "-"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "dWP"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 18, rx_type_alias: 'a', addr_mode_aliases: "xX"
        },
        RxAddrModeInstPattern {
            name: "movea", mask: 0b0010000001000000, size: Size::Long, clock: 20, rx_type_alias: 'a', addr_mode_aliases: "L"
        },

        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000100001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "a"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000100001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "a"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000110001000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "a"
        },
        RxAddrModeInstPattern {
            name: "movep", mask: 0b0000000111001000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "a"
        },

        RxAddrModeInstPattern {
            name: "lea", mask: 0b0100000111000000, size: Size::Long, clock: 4, rx_type_alias: 'a', addr_mode_aliases: "a"
        },
        RxAddrModeInstPattern {
            name: "lea", mask: 0b0100000111000000, size: Size::Long, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "dPW"
        },
        RxAddrModeInstPattern {
            name: "lea", mask: 0b0100000111000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "xXL"
        },

        // add into data register
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000000000000, size: Size::Byte, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L",
        },
        
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000001000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000010000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        // add into addr_mode
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000100000000, size: Size::Byte, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000101000000, size: Size::Word, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 22, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "add", mask: 0b1101000110000000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        // adda
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 18, rx_type_alias: 'a', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000011000000, size: Size::Word, clock: 20, rx_type_alias: 'a', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 10, rx_type_alias: 'a', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "adda", mask: 0b1101000111000000, size: Size::Long, clock: 18, rx_type_alias: 'a', addr_mode_aliases: "L",
        },

        // sub from data register
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000000000000, size: Size::Byte, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L",
        },
        
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000001000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000010000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        // sub from addr_mode
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000100000000, size: Size::Byte, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000101000000, size: Size::Word, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "a+",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "dW",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 22, rx_type_alias: 'd', addr_mode_aliases: "x",
        },
        RxAddrModeInstPattern {
            name: "sub", mask: 0b1001000110000000, size: Size::Long, clock: 24, rx_type_alias: 'd', addr_mode_aliases: "L",
        },

         // suba
         RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 8, rx_type_alias: 'a', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 18, rx_type_alias: 'a', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000011000000, size: Size::Word, clock: 20, rx_type_alias: 'a', addr_mode_aliases: "L",
        },

        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "DA",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 10, rx_type_alias: 'a', addr_mode_aliases: "a+i",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 12, rx_type_alias: 'a', addr_mode_aliases: "-",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 14, rx_type_alias: 'a', addr_mode_aliases: "dPW",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 16, rx_type_alias: 'a', addr_mode_aliases: "xX",
        },
        RxAddrModeInstPattern {
            name: "suba", mask: 0b1001000111000000, size: Size::Long, clock: 18, rx_type_alias: 'a', addr_mode_aliases: "L",
        },

        // cmp
        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "D"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dWP"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000000000000, size: Size::Byte, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 4, rx_type_alias: 'd', addr_mode_aliases: "DA"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 8, rx_type_alias: 'd', addr_mode_aliases: "a+i"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "-"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "dWP"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "xX"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000001000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "L"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DA"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "a+i"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "-"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "dWP"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "xX"
        },

        RxAddrModeInstPattern {
            name: "cmp", mask: 0b1011000010000000, size: Size::Long, clock: 22, rx_type_alias: 'd', addr_mode_aliases: "L"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DA"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 10, rx_type_alias: 'd', addr_mode_aliases: "a+i"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 12, rx_type_alias: 'd', addr_mode_aliases: "-"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "dWP"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "xX"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000011000000, size: Size::Word, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "L"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 6, rx_type_alias: 'd', addr_mode_aliases: "DA"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 14, rx_type_alias: 'd', addr_mode_aliases: "a+i"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 16, rx_type_alias: 'd', addr_mode_aliases: "-"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 18, rx_type_alias: 'd', addr_mode_aliases: "dWP"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 20, rx_type_alias: 'd', addr_mode_aliases: "xX"
        },

        RxAddrModeInstPattern {
            name: "cmpa", mask: 0b1011000111000000, size: Size::Long, clock: 22, rx_type_alias: 'd', addr_mode_aliases: "L"
        },
    ];

    for pattern in patterns {
        let mask = pattern.mask;

        let rx_type = register_type_by_char(pattern.rx_type_alias);
        let addr_mode_type_list = pattern.addr_mode_aliases.chars().map(|c| addr_mode_type_by_char(c)).collect::<Vec<AddrModeType>>();

        for addr_mode_type in addr_mode_type_list {
            let addr_mode_list = get_addr_mode_table(addr_mode_type);

            (0..8).for_each(|i| {
                addr_mode_list.iter()
                    .for_each(|mode| {
                        let opcode = generate_mask(&pattern.name, mask, i, mode);
                        opcode_table[opcode as usize] = Box::new(Instruction::new(
                            pattern.name,
                            opcode,
                            pattern.size,
                            pattern.clock,
                            cpu_function_by_name(pattern.name),
                            RxAddrModeMetadata::new(Register::new(rx_type, i as usize), *mode),
                        ));
                    });
            })
        }
    }
}

fn generate_mask(name: &str, mask: u16, rx_idx: u16, addr_mode: &AddrMode) -> u16 {
    match name {
        "movep" => mask | rx_idx << 9 | (*addr_mode).reg_idx as u16,
        _ => mask | rx_idx << 9 | ((*addr_mode).mode_bits as u16) << 3 | (*addr_mode).reg_idx as u16,
    }
}

fn cpu_function_by_name(name: &str) -> fn(&mut Mc68k) {
    match name {
        "movea" => Mc68k::MOVEA,
        "movep" => Mc68k::MOVEP,
        "lea" => Mc68k::LEA,
        "add" => Mc68k::ADD,
        "adda" => Mc68k::ADDA,
        "sub" => Mc68k::SUB,
        "suba" => Mc68k::SUBA,
        "cmp" => Mc68k::CMP,
        "cmpa" => Mc68k::CMPA,
        _ => panic!("rx_addr_mode_generator::cpu_function_by_name: unexpected function name ({})", name)
    }
}
const RESET_SP: usize = 0x00000;
const RESET_PC: usize = 0x00004;
const BUS_ERROR: usize = 0x00008;
const ADDRESS_ERROR: usize = 0x0000C;
const ILLEGAL_INSTRUCTION: usize = 0x00010;
const DIVISION_BY_ZERO: usize = 0x00014;
const CHK_INSTRUCTION: usize = 0x00018;
const TRAPV_INSTRUCTION: usize = 0x0001C;
const PRIVILEGE_VIOLATION: usize = 0x00020;
const TRACE: usize = 0x00024;
const UNIMPLEMENTED_INSTRUCTION_A: usize = 0x00028;
const UNIMPLEMENTED_INSTRUCTION_F: usize = 0x0002C;
const UNINITIALIZED_INTERRUPT: usize = 0x0003C;
const SPURIOUS_INTERRUPT: usize = 0x00060;
const LEVEL_1: usize = 0x00064;
const LEVEL_2: usize = 0x00068;
const LEVEL_3: usize = 0x0006C;
const LEVEL_4: usize = 0x00070;
const LEVEL_5: usize = 0x00074;
const LEVEL_6: usize = 0x00078;
const LEVEL_7: usize = 0x0007C;
const TRAP_0_15: usize = 0x00080;
const USER_INTERRUPT_VECTORS: usize = 0x00100; // ~0003FF
                                               // 000030	Reserved/Unused on 68000
                                               // 000034
                                               // 000038
                                               // 000040	Reserved
                                               // 000044
                                               // 000048
                                               // 00004C
                                               // 000050
                                               // 000054
                                               // 000058
                                               // 00005C
                                               // 0000C0~0000FC	RESERVED/UNUSED

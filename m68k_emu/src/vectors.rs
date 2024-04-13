pub(crate) const RESET_SP: usize = 0x00000;
pub(crate) const RESET_PC: usize = 0x00004;
pub(crate) const BUS_ERROR: usize = 0x00008;
pub(crate) const ADDRESS_ERROR: usize = 0x0000C;
pub(crate) const ILLEGAL_INSTRUCTION: usize = 0x00010;
pub(crate) const DIVISION_BY_ZERO: usize = 0x00014;
pub(crate) const CHK_INSTRUCTION: usize = 0x00018;
pub(crate) const TRAPV_INSTRUCTION: usize = 0x0001C;
pub(crate) const PRIVILEGE_VIOLATION: usize = 0x00020;
pub(crate) const TRACE: usize = 0x00024;
pub(crate) const UNIMPLEMENTED_INSTRUCTION_A: usize = 0x00028;
pub(crate) const UNIMPLEMENTED_INSTRUCTION_F: usize = 0x0002C;
pub(crate) const UNINITIALIZED_INTERRUPT: usize = 0x0003C;
pub(crate) const SPURIOUS_INTERRUPT: usize = 0x00060;
pub(crate) const LEVEL_1: usize = 0x00064;
pub(crate) const LEVEL_2: usize = 0x00068;
pub(crate) const LEVEL_3: usize = 0x0006C;
pub(crate) const LEVEL_4: usize = 0x00070;
pub(crate) const LEVEL_5: usize = 0x00074;
pub(crate) const LEVEL_6: usize = 0x00078;
pub(crate) const LEVEL_7: usize = 0x0007C;
pub(crate) const TRAP_0_15: usize = 0x00080;
pub(crate) const USER_INTERRUPT_VECTORS: usize = 0x00100; // ~0003FF
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

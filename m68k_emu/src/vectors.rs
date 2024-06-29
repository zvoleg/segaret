pub(crate) const RESET_SP: u32 = 0x00000;
pub(crate) const RESET_PC: u32 = 0x00004;
pub(crate) const BUS_ERROR: u32 = 0x00008;
pub(crate) const ADDRESS_ERROR: u32 = 0x0000C;
pub(crate) const ILLEGAL_INSTRUCTION: u32 = 0x00010;
pub(crate) const DIVISION_BY_ZERO: u32 = 0x00014;
pub(crate) const CHK_INSTRUCTION: u32 = 0x00018;
pub(crate) const TRAPV_INSTRUCTION: u32 = 0x0001C;
pub(crate) const PRIVILEGE_VIOLATION: u32 = 0x00020;
pub(crate) const TRACE: u32 = 0x00024;
pub(crate) const UNIMPLEMENTED_INSTRUCTION_A: u32 = 0x00028;
pub(crate) const UNIMPLEMENTED_INSTRUCTION_F: u32 = 0x0002C;
pub(crate) const UNINITIALIZED_INTERRUPT: u32 = 0x0003C;
pub(crate) const SPURIOUS_INTERRUPT: u32 = 0x00060;
pub(crate) const LEVEL_1: u32 = 0x00064;
pub(crate) const LEVEL_2: u32 = 0x00068;
pub(crate) const LEVEL_3: u32 = 0x0006C;
pub(crate) const LEVEL_4: u32 = 0x00070;
pub(crate) const LEVEL_5: u32 = 0x00074;
pub(crate) const LEVEL_6: u32 = 0x00078;
pub(crate) const LEVEL_7: u32 = 0x0007C;
pub(crate) const TRAP_0_15: u32 = 0x00080;
pub(crate) const USER_INTERRUPT_VECTORS: u32 = 0x00100; // ~0003FF
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

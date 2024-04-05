use std::collections::HashSet;

use lazy_static::lazy_static;

use hardware::{sign_extend, is_negate, is_zero, get_msb, Size};

use crate::{inst, sz, am};

use crate::{Z80Bus, Instruction, Operand, AmType, Location, Register};

lazy_static! {
    static ref EXTENDING_TO_TWO_BYTES: HashSet<u32> = HashSet::from([0xCB, 0xDD, 0xED, 0xFDu32]);
    static ref EXTENDING_TO_FOUR_BYTES: HashSet<u32> = HashSet::from([0xDDCB, 0xFDCBu32]);

    static ref OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: NOP.b),         inst!(0x01: LD.w bc nn),    inst!(0x02: LD.b ind_bc a), inst!(0x03: INC.w bc),       inst!(0x04: INC.b b),       inst!(0x05: DEC.b b),       inst!(0x06: LD.b b n),       inst!(0x07: RLCA.b),        inst!(0x08: EX.w af af_), inst!(0x09: ADD.w hl bc),     inst!(0x0A: LD.b a ind_bc), inst!(0x0B: DEC.w bc),   inst!(0x0C: INC.b c),   inst!(0x0D: DEC.b c),   inst!(0x0E: LD.b c n),       inst!(0x0F: RRCA.b),
        inst!(0x10: DJNZ.b D),      inst!(0x11: LD.w de nn),    inst!(0x12: LD.b ind_de a), inst!(0x13: INC.w de),       inst!(0x14: INC.b d),       inst!(0x15: DEC.b d),       inst!(0x16: LD.b d n),       inst!(0x17: RLA.b),         inst!(0x18: JR.b D),      inst!(0x19: ADD.w hl de),     inst!(0x1A: LD.b a ind_de), inst!(0x1B: DEC.w de),   inst!(0x1C: INC.b e),   inst!(0x1D: DEC.b e),   inst!(0x1E: LD.b e n),       inst!(0x1F: RRA.b),
        inst!(0x20: JR.b D),        inst!(0x21: LD.w hl nn),    inst!(0x22: LD.b DD hl),    inst!(0x23: INC.w hl),       inst!(0x24: INC.b h),       inst!(0x25: DEC.b h),       inst!(0x26: LD.b h n),       inst!(0x27: DAA.b),         inst!(0x28: JR.b D),      inst!(0x29: ADD.w hl hl),     inst!(0x2A: LD.w hl DD),    inst!(0x2B: DEC.w hl),   inst!(0x2C: INC.b l),   inst!(0x2D: DEC.b l),   inst!(0x2E: LD.b l n),       inst!(0x2F: CPL.b),
        inst!(0x30: JR.b D),        inst!(0x31: LD.w sp nn),    inst!(0x32: LD.b DD a),     inst!(0x33: INC.w sp),       inst!(0x34: INC.b ind_hl),  inst!(0x35: DEC.b ind_hl),  inst!(0x36: LD.b ind_hl n),  inst!(0x37: SCF.b),         inst!(0x38: JR.b D),      inst!(0x39: ADD.w hl sp),     inst!(0x3A: LD.b a DD),     inst!(0x3B: DEC.w sp),   inst!(0x3C: INC.b a),   inst!(0x3D: DEC.b a),   inst!(0x3E: LD.b a n),       inst!(0x3F: CCF.b),
        inst!(0x40: LD.b b b_),     inst!(0x41: LD.b b c_),     inst!(0x42: LD.b b d_),     inst!(0x43: LD.b b e_),      inst!(0x44: LD.b b h_),     inst!(0x45: LD.b b l_),     inst!(0x46: LD.b b ind_hl),  inst!(0x47: LD.b b a_),     inst!(0x48: LD.b c b_),   inst!(0x49: LD.b c c_),       inst!(0x4A: LD.b c d_),     inst!(0x4B: LD.b c e_),  inst!(0x4C: LD.b c h_), inst!(0x4D: LD.b c l_), inst!(0x4E: LD.b c ind_hl),  inst!(0x4F: LD.b c a_),
        inst!(0x50: LD.b d b_),     inst!(0x51: LD.b d c_),     inst!(0x52: LD.b d d_),     inst!(0x53: LD.b d e_),      inst!(0x54: LD.b d h_),     inst!(0x55: LD.b d l_),     inst!(0x56: LD.b d ind_hl),  inst!(0x57: LD.b d a_),     inst!(0x58: LD.b e b_),   inst!(0x59: LD.b e c_),       inst!(0x5A: LD.b e d_),     inst!(0x5B: LD.b e e_),  inst!(0x5C: LD.b e h_), inst!(0x5D: LD.b e l_), inst!(0x5E: LD.b e ind_hl),  inst!(0x5F: LD.b e a_),
        inst!(0x60: LD.b h b_),     inst!(0x61: LD.b h c_),     inst!(0x62: LD.b h d_),     inst!(0x63: LD.b h e_),      inst!(0x64: LD.b h h_),     inst!(0x65: LD.b h l_),     inst!(0x66: LD.b h ind_hl),  inst!(0x67: LD.b h a_),     inst!(0x68: LD.b l b_),   inst!(0x69: LD.b l c_),       inst!(0x6A: LD.b l d_),     inst!(0x6B: LD.b l e_),  inst!(0x6C: LD.b l h_), inst!(0x6D: LD.b l l_), inst!(0x6E: LD.b l ind_hl),  inst!(0x6F: LD.b l a_),
        inst!(0x70: LD.b ind_hl b), inst!(0x71: LD.b ind_hl c), inst!(0x72: LD.b ind_hl d), inst!(0x73: LD.b ind_hl e),  inst!(0x74: LD.b ind_hl h), inst!(0x75: LD.b ind_hl l), inst!(0x76: HALT.b),         inst!(0x77: LD.b ind_hl a), inst!(0x78: LD.b a b_),   inst!(0x79: LD.b a c_),       inst!(0x7A: LD.b a d_),     inst!(0x7B: LD.b a e_),  inst!(0x7C: LD.b a h_), inst!(0x7D: LD.b a l_), inst!(0x7E: LD.b a ind_hl),  inst!(0x7F: LD.b a a_),
        inst!(0x80: ADD.b a b),     inst!(0x81: ADD.b a c),     inst!(0x82: ADD.b a d),     inst!(0x83: ADD.b a e),      inst!(0x84: ADD.b a h),     inst!(0x85: ADD.b a l),     inst!(0x86: ADD.b a ind_hl), inst!(0x87: ADD.b a a),     inst!(0x88: ADC.b a b),   inst!(0x89: ADC.b a c),       inst!(0x8A: ADC.b a d),     inst!(0x8B: ADC.b a e),  inst!(0x8C: ADC.b a h), inst!(0x8D: ADC.b a l), inst!(0x8E: ADC.b a ind_hl), inst!(0x8F: ADC.b a a),
        inst!(0x90: SUB.b a b),     inst!(0x91: SUB.b a c),     inst!(0x92: SUB.b a d),     inst!(0x93: SUB.b a e),      inst!(0x94: SUB.b a h),     inst!(0x95: SUB.b a l),     inst!(0x96: SUB.b a ind_hl), inst!(0x97: SUB.b a a),     inst!(0x98: SBC.b a b),   inst!(0x99: SBC.b a c),       inst!(0x9A: SBC.b a d),     inst!(0x9B: SBC.b a e),  inst!(0x9C: SBC.b a h), inst!(0x9D: SBC.b a l), inst!(0x9E: SBC.b a ind_hl), inst!(0x9F: SBC.b a a),
        inst!(0xA0: AND.b a b),     inst!(0xA1: AND.b a c),     inst!(0xA2: AND.b a d),     inst!(0xA3: AND.b a e),      inst!(0xA4: AND.b a h),     inst!(0xA5: AND.b a h),     inst!(0xA6: AND.b a ind_hl), inst!(0xA7: AND.b a a),     inst!(0xA8: XOR.b a b),   inst!(0xA9: XOR.b a c),       inst!(0xAA: XOR.b a d),     inst!(0xAB: XOR.b a e),  inst!(0xAC: XOR.b a h), inst!(0xAD: XOR.b a l), inst!(0xAE: XOR.b a ind_hl), inst!(0xAF: XOR.b a a),
        inst!(0xB0: OR.b a b),      inst!(0xB1: OR.b a c),      inst!(0xB2: OR.b a d),      inst!(0xB3: OR.b a e),       inst!(0xB4: OR.b a h),      inst!(0xB5: OR.b a l),      inst!(0xB6: OR.b a ind_hl),  inst!(0xB7: OR.b a a),      inst!(0xB8: CP.b a b),    inst!(0xB9: CP.b a c),        inst!(0xBA: CP.b a d),      inst!(0xBB: CP.b a e),   inst!(0xBC: CP.b a h),  inst!(0xBD: CP.b a l),  inst!(0xBE: CP.b a ind_hl),  inst!(0xBF: CP.b a a),
        inst!(0xC0: RET.b),         inst!(0xC1: POP.w bc),      inst!(0xC2: JP.b DD),       inst!(0xC3: JP.b DD),        inst!(0xC4: CALL.b DD),     inst!(0xC5: PUSH.w bc),     inst!(0xC6: ADD.b a n),      inst!(0xC7: RST.b),         inst!(0xC8: RET.b),       inst!(0xC9: RET.b),           inst!(0xCA: JP.b DD),       inst!(0xCB: NOP.b),      inst!(0xCC: CALL.b DD), inst!(0xCD: CALL.b DD), inst!(0xCE: ADC.b a n),      inst!(0xCF: RST.b),
        inst!(0xD0: RET.b),         inst!(0xD1: POP.w de),      inst!(0xD2: JP.b DD),       inst!(0xD3: OUT.b n a),      inst!(0xD4: CALL.b DD),     inst!(0xD5: PUSH.w de),     inst!(0xD6: SUB.b a n),      inst!(0xD7: RST.b),         inst!(0xD8: RET.b),       inst!(0xD9: EXX.b),           inst!(0xDA: JP.b DD),       inst!(0xDB: IN.b a n),   inst!(0xDC: CALL.b DD), inst!(0xDD: NOP.b),     inst!(0xDE: SBC.b a n),      inst!(0xDF: RST.b),
        inst!(0xE0: RET.b),         inst!(0xE1: POP.w hl),      inst!(0xE2: JP.b DD),       inst!(0xE3: EX.w ind_sp hl), inst!(0xE4: CALL.b DD),     inst!(0xE5: PUSH.w hl),     inst!(0xE6: AND.b a n),      inst!(0xE7: RST.b),         inst!(0xE8: RET.b),       inst!(0xE9: JP_ind.b hl), inst!(0xEA: JP.b DD),       inst!(0xEB: EX.w de hl), inst!(0xEC: CALL.b DD), inst!(0xED: NOP.b),     inst!(0xEE: XOR.b a n),      inst!(0xEF: RST.b),
        inst!(0xF0: RET.b),         inst!(0xF1: POP.w af),      inst!(0xF2: JP.b DD),       inst!(0xF3: DI.b),           inst!(0xF4: CALL.b DD),     inst!(0xF5: PUSH.w af),     inst!(0xF6: OR.b a n),       inst!(0xF7: RST.b),         inst!(0xF8: RET.b),       inst!(0xF9: LD.w sp hl),      inst!(0xFA: JP.b DD),       inst!(0xFB: EI.b),       inst!(0xFC: CALL.b DD), inst!(0xFD: NOP.b),     inst!(0xFE: CP.b a n),       inst!(0xFF: RST.b),
    ];

    static ref CB_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: RLC.b b),   inst!(0x01: RLC.b c),   inst!(0x02: RLC.b d),   inst!(0x03: RLC.b e),   inst!(0x04: RLC.b h),   inst!(0x05: RLC.b l),   inst!(0x06: RLC.b ind_hl),   inst!(0x07: RLC.b a),   inst!(0x08: RRC.b b),   inst!(0x09: RRC.b c),   inst!(0x0A: RRC.b d),   inst!(0x0B: RRC.b e),   inst!(0x0C: RRC.b h),   inst!(0x0D: RRC.b l),   inst!(0x0E: RRC.b ind_hl),   inst!(0x0F: RRC.b a),
        inst!(0x10: RL.b b),    inst!(0x11: RL.b c),    inst!(0x12: RL.b d),    inst!(0x13: RL.b e),    inst!(0x14: RL.b h),    inst!(0x15: RL.b l),    inst!(0x16: RL.b ind_hl),    inst!(0x17: RL.b a),    inst!(0x18: RR.b b),    inst!(0x19: RR.b c),    inst!(0x1A: RR.b a d),  inst!(0x1B: RR.b e),    inst!(0x1C: RR.b h),    inst!(0x1D: RR.b l),    inst!(0x1E: RR.b ind_hl),    inst!(0x1F: RR.b a),
        inst!(0x20: SLA.b b),   inst!(0x21: SLA.b c),   inst!(0x22: SLA.b d),   inst!(0x23: SLA.b e),   inst!(0x24: SLA.b h),   inst!(0x25: SLA.b l),   inst!(0x26: SLA.b ind_hl),   inst!(0x27: SLA.b a),   inst!(0x28: SRA.b b),   inst!(0x29: SRA.b c),   inst!(0x2A: SRA.b d),   inst!(0x2B: SRA.b e),   inst!(0x2C: SRA.b h),   inst!(0x2D: SRA.b l),   inst!(0x2E: SRA.b ind_hl),   inst!(0x2F: SRA.b a),
        inst!(0x30: NOP.b),     inst!(0x31: NOP.b),     inst!(0x32: NOP.b),     inst!(0x33: NOP.b),     inst!(0x34: NOP.b),     inst!(0x35: NOP.b),     inst!(0x36: NOP.b),          inst!(0x37: NOP.b),     inst!(0x38: SRL.b b),   inst!(0x39: SRL.b c),   inst!(0x3A: SRL.b d),   inst!(0x3B: SRL.b e),   inst!(0x3C: SRL.b h),   inst!(0x3D: SRL.b l),   inst!(0x3E: SRL.b ind_hl),   inst!(0x3F: SRL.b a),
        inst!(0x40: BIT.b 0 b), inst!(0x41: BIT.b 0 c), inst!(0x42: BIT.b 0 d), inst!(0x43: BIT.b 0 e), inst!(0x44: BIT.b 0 h), inst!(0x45: BIT.b 0 l), inst!(0x46: BIT.b 0 ind_hl), inst!(0x47: BIT.b 0 a), inst!(0x48: BIT.b 1 b), inst!(0x49: BIT.b 1 c), inst!(0x4A: BIT.b 1 d), inst!(0x4B: BIT.b 1 e), inst!(0x4C: BIT.b 1 h), inst!(0x4D: BIT.b 1 l), inst!(0x4E: BIT.b 1 ind_hl), inst!(0x4F: BIT.b 1 a),
        inst!(0x50: BIT.b 2 b), inst!(0x51: BIT.b 2 c), inst!(0x52: BIT.b 2 d), inst!(0x53: BIT.b 2 e), inst!(0x54: BIT.b 2 h), inst!(0x55: BIT.b 2 l), inst!(0x56: BIT.b 2 ind_hl), inst!(0x57: BIT.b 2 a), inst!(0x58: BIT.b 3 b), inst!(0x59: BIT.b 3 c), inst!(0x5A: BIT.b 3 d), inst!(0x5B: BIT.b 3 e), inst!(0x5C: BIT.b 3 h), inst!(0x5D: BIT.b 3 l), inst!(0x5E: BIT.b 3 ind_hl), inst!(0x5F: BIT.b 3 a),
        inst!(0x60: BIT.b 4 b), inst!(0x61: BIT.b 4 c), inst!(0x62: BIT.b 4 d), inst!(0x63: BIT.b 4 e), inst!(0x64: BIT.b 4 h), inst!(0x65: BIT.b 4 l), inst!(0x66: BIT.b 4 ind_hl), inst!(0x67: BIT.b 4 a), inst!(0x68: BIT.b 5 b), inst!(0x69: BIT.b 5 c), inst!(0x6A: BIT.b 5 d), inst!(0x6B: BIT.b 5 e), inst!(0x6C: BIT.b 5 h), inst!(0x6D: BIT.b 5 l), inst!(0x6E: BIT.b 5 ind_hl), inst!(0x6F: BIT.b 5 a),
        inst!(0x70: BIT.b 6 b), inst!(0x71: BIT.b 6 c), inst!(0x72: BIT.b 6 d), inst!(0x73: BIT.b 6 e), inst!(0x74: BIT.b 6 h), inst!(0x75: BIT.b 6 l), inst!(0x76: BIT.b 6 ind_hl), inst!(0x77: BIT.b 6 a), inst!(0x78: BIT.b 7 b), inst!(0x79: BIT.b 7 c), inst!(0x7A: BIT.b 7 d), inst!(0x7B: BIT.b 7 e), inst!(0x7C: BIT.b 7 h), inst!(0x7D: BIT.b 7 l), inst!(0x7E: BIT.b 7 ind_hl), inst!(0x7F: BIT.b 7 a),
        inst!(0x80: RES.b 0 b), inst!(0x81: RES.b 0 c), inst!(0x82: RES.b 0 d), inst!(0x83: RES.b 0 e), inst!(0x84: RES.b 0 h), inst!(0x85: RES.b 0 l), inst!(0x86: RES.b 0 ind_hl), inst!(0x87: RES.b 0 a), inst!(0x88: RES.b 1 b), inst!(0x89: RES.b 1 c), inst!(0x8A: RES.b 1 d), inst!(0x8B: RES.b 1 e), inst!(0x8C: RES.b 1 h), inst!(0x8D: RES.b 1 l), inst!(0x8E: RES.b 1 ind_hl), inst!(0x8F: RES.b 1 a),
        inst!(0x90: RES.b 2 b), inst!(0x91: RES.b 2 c), inst!(0x92: RES.b 2 d), inst!(0x93: RES.b 2 e), inst!(0x94: RES.b 2 h), inst!(0x95: RES.b 2 l), inst!(0x96: RES.b 2 ind_hl), inst!(0x97: RES.b 2 a), inst!(0x98: RES.b 3 b), inst!(0x99: RES.b 3 c), inst!(0x9A: RES.b 3 d), inst!(0x9B: RES.b 3 e), inst!(0x9C: RES.b 3 h), inst!(0x9D: RES.b 3 l), inst!(0x9E: RES.b 3 ind_hl), inst!(0x9F: RES.b 3 a),
        inst!(0xA0: RES.b 4 b), inst!(0xA1: RES.b 4 c), inst!(0xA2: RES.b 4 d), inst!(0xA3: RES.b 4 e), inst!(0xA4: RES.b 4 h), inst!(0xA5: RES.b 4 l), inst!(0xA6: RES.b 4 ind_hl), inst!(0xA7: RES.b 4 a), inst!(0xA8: RES.b 5 b), inst!(0xA9: RES.b 5 c), inst!(0xAA: RES.b 5 d), inst!(0xAB: RES.b 5 e), inst!(0xAC: RES.b 5 h), inst!(0xAD: RES.b 5 l), inst!(0xAE: RES.b 5 ind_hl), inst!(0xAF: RES.b 5 a),
        inst!(0xB0: RES.b 6 b), inst!(0xB1: RES.b 6 c), inst!(0xB2: RES.b 6 d), inst!(0xB3: RES.b 6 e), inst!(0xB4: RES.b 6 h), inst!(0xB5: RES.b 6 l), inst!(0xB6: RES.b 6 ind_hl), inst!(0xB7: RES.b 6 a), inst!(0xB8: RES.b 7 b), inst!(0xB9: RES.b 7 c), inst!(0xBA: RES.b 7 d), inst!(0xBB: RES.b 7 e), inst!(0xBC: RES.b 7 h), inst!(0xBD: RES.b 7 l), inst!(0xBE: RES.b 7 ind_hl), inst!(0xBF: RES.b 7 a),
        inst!(0xC0: SET.b 0 b), inst!(0xC1: SET.b 0 c), inst!(0xC2: SET.b 0 d), inst!(0xC3: SET.b 0 e), inst!(0xC4: SET.b 0 h), inst!(0xC5: SET.b 0 l), inst!(0xC6: SET.b 0 ind_hl), inst!(0xC7: SET.b 0 a), inst!(0xC8: SET.b 1 b), inst!(0xC9: SET.b 1 c), inst!(0xCA: SET.b 1 d), inst!(0xCB: SET.b 1 e), inst!(0xCC: SET.b 1 h), inst!(0xCD: SET.b 1 l), inst!(0xCE: SET.b 1 ind_hl), inst!(0xCF: SET.b 1 a),
        inst!(0xD0: SET.b 2 b), inst!(0xD1: SET.b 2 c), inst!(0xD2: SET.b 2 d), inst!(0xD3: SET.b 2 e), inst!(0xD4: SET.b 2 h), inst!(0xD5: SET.b 2 l), inst!(0xD6: SET.b 2 ind_hl), inst!(0xD7: SET.b 2 a), inst!(0xD8: SET.b 3 b), inst!(0xD9: SET.b 3 c), inst!(0xDA: SET.b 3 d), inst!(0xDB: SET.b 3 e), inst!(0xDC: SET.b 3 h), inst!(0xDD: SET.b 3 l), inst!(0xDE: SET.b 3 ind_hl), inst!(0xDF: SET.b 3 a),
        inst!(0xE0: SET.b 4 b), inst!(0xE1: SET.b 4 c), inst!(0xE2: SET.b 4 d), inst!(0xE3: SET.b 4 e), inst!(0xE4: SET.b 4 h), inst!(0xE5: SET.b 4 l), inst!(0xE6: SET.b 4 ind_hl), inst!(0xE7: SET.b 4 a), inst!(0xE8: SET.b 5 b), inst!(0xE9: SET.b 5 c), inst!(0xEA: SET.b 5 d), inst!(0xEB: SET.b 5 e), inst!(0xEC: SET.b 5 h), inst!(0xED: SET.b 5 l), inst!(0xEE: SET.b 5 ind_hl), inst!(0xEF: SET.b 5 a),
        inst!(0xF0: SET.b 6 b), inst!(0xF1: SET.b 6 c), inst!(0xF2: SET.b 6 d), inst!(0xF3: SET.b 6 e), inst!(0xF4: SET.b 6 h), inst!(0xF5: SET.b 6 l), inst!(0xF6: SET.b 6 ind_hl), inst!(0xF7: SET.b 6 a), inst!(0xF8: SET.b 7 b), inst!(0xF9: SET.b 7 c), inst!(0xFA: SET.b 7 d), inst!(0xFB: SET.b 7 e), inst!(0xFC: SET.b 7 h), inst!(0xFD: SET.b 7 l), inst!(0xFE: SET.b 7 ind_hl), inst!(0xFF: SET.b 7 a),
    ];

    static ref DD_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: XEP.b),         inst!(0x01: XEP.b),         inst!(0x02: XEP.b),         inst!(0x03: XEP.b),          inst!(0x04: XEP.b),         inst!(0x05: XEP.b),         inst!(0x06: XEP.b),          inst!(0x07: XEP.b),         inst!(0x08: XEP.b), inst!(0x09: ADD.w ix bc), inst!(0x0A: XEP.b),      inst!(0x0B: XEP.b),    inst!(0x0C: XEP.b), inst!(0x0D: XEP.b), inst!(0x0E: XEP.b),          inst!(0x0F: XEP.b),
        inst!(0x10: XEP.b),         inst!(0x11: XEP.b),         inst!(0x12: XEP.b),         inst!(0x13: XEP.b),          inst!(0x14: XEP.b),         inst!(0x15: XEP.b),         inst!(0x16: XEP.b),          inst!(0x17: XEP.b),         inst!(0x18: XEP.b), inst!(0x19: ADD.w ix de), inst!(0x1A: XEP.b),      inst!(0x1B: XEP.b),    inst!(0x1C: XEP.b), inst!(0x1D: XEP.b), inst!(0x1E: XEP.b),          inst!(0x1F: XEP.b),
        inst!(0x20: XEP.b),         inst!(0x21: LD.w ix nn),    inst!(0x22: LD.w DD ix),    inst!(0x23: INC.w ix),       inst!(0x24: XEP.b),         inst!(0x25: XEP.b),         inst!(0x26: XEP.b),          inst!(0x27: XEP.b),         inst!(0x28: XEP.b), inst!(0x29: ADD.w ix ix), inst!(0x2A: LD.w ix DD), inst!(0x2B: DEC.w ix), inst!(0x2C: XEP.b), inst!(0x2D: XEP.b), inst!(0x2E: XEP.b),          inst!(0x2F: XEP.b),
        inst!(0x30: XEP.b),         inst!(0x31: XEP.b),         inst!(0x32: XEP.b),         inst!(0x33: XEP.b),          inst!(0x34: INC.b idx_ix),  inst!(0x35: DEC.b idx_ix),  inst!(0x36: LD.b idx_ix n),  inst!(0x37: XEP.b),         inst!(0x38: XEP.b), inst!(0x39: ADD.w ix sp), inst!(0x3A: XEP.b),      inst!(0x3B: XEP.b),    inst!(0x3C: XEP.b), inst!(0x3D: XEP.b), inst!(0x3E: XEP.b),          inst!(0x3F: XEP.b),
        inst!(0x40: XEP.b),         inst!(0x41: XEP.b),         inst!(0x42: XEP.b),         inst!(0x43: XEP.b),          inst!(0x44: XEP.b),         inst!(0x45: XEP.b),         inst!(0x46: LD.b b idx_ix),  inst!(0x47: XEP.b),         inst!(0x48: XEP.b), inst!(0x49: XEP.b),       inst!(0x4A: XEP.b),      inst!(0x4B: XEP.b),    inst!(0x4C: XEP.b), inst!(0x4D: XEP.b), inst!(0x4E: LD.b c idx_ix),  inst!(0x4F: XEP.b),
        inst!(0x50: XEP.b),         inst!(0x51: XEP.b),         inst!(0x52: XEP.b),         inst!(0x53: XEP.b),          inst!(0x54: XEP.b),         inst!(0x55: XEP.b),         inst!(0x56: LD.b d idx_ix),  inst!(0x57: XEP.b),         inst!(0x58: XEP.b), inst!(0x59: XEP.b),       inst!(0x5A: XEP.b),      inst!(0x5B: XEP.b),    inst!(0x5C: XEP.b), inst!(0x5D: XEP.b), inst!(0x5E: LD.b e idx_ix),  inst!(0x5F: XEP.b),
        inst!(0x60: XEP.b),         inst!(0x61: XEP.b),         inst!(0x62: XEP.b),         inst!(0x63: XEP.b),          inst!(0x64: XEP.b),         inst!(0x65: XEP.b),         inst!(0x66: LD.b h idx_ix),  inst!(0x67: XEP.b),         inst!(0x68: XEP.b), inst!(0x69: XEP.b),       inst!(0x6A: XEP.b),      inst!(0x6B: XEP.b),    inst!(0x6C: XEP.b), inst!(0x6D: XEP.b), inst!(0x6E: LD.b l idx_ix),  inst!(0x6F: XEP.b),
        inst!(0x70: LD.b idx_ix b), inst!(0x71: LD.b idx_ix c), inst!(0x72: LD.b idx_ix d), inst!(0x73: LD.b idx_ix e),  inst!(0x74: LD.b idx_ix h), inst!(0x75: LD.b idx_ix l), inst!(0x76: XEP.b),          inst!(0x77: LD.b idx_ix a), inst!(0x78: XEP.b), inst!(0x79: XEP.b),       inst!(0x7A: XEP.b),      inst!(0x7B: XEP.b),    inst!(0x7C: XEP.b), inst!(0x7D: XEP.b), inst!(0x7E: LD.b a idx_ix),  inst!(0x7F: XEP.b),
        inst!(0x80: XEP.b),         inst!(0x81: XEP.b),         inst!(0x82: XEP.b),         inst!(0x83: XEP.b),          inst!(0x84: XEP.b),         inst!(0x85: XEP.b),         inst!(0x86: ADD.b a idx_ix), inst!(0x87: XEP.b),         inst!(0x88: XEP.b), inst!(0x89: XEP.b),       inst!(0x8A: XEP.b),      inst!(0x8B: XEP.b),    inst!(0x8C: XEP.b), inst!(0x8D: XEP.b), inst!(0x8E: ADC.b a idx_ix), inst!(0x8F: XEP.b),
        inst!(0x90: XEP.b),         inst!(0x91: XEP.b),         inst!(0x92: XEP.b),         inst!(0x93: XEP.b),          inst!(0x94: XEP.b),         inst!(0x95: XEP.b),         inst!(0x96: SUB.b a idx_ix), inst!(0x97: XEP.b),         inst!(0x98: XEP.b), inst!(0x99: XEP.b),       inst!(0x9A: XEP.b),      inst!(0x9B: XEP.b),    inst!(0x9C: XEP.b), inst!(0x9D: XEP.b), inst!(0x9E: SBC.b a idx_ix), inst!(0x9F: XEP.b),
        inst!(0xA0: XEP.b),         inst!(0xA1: XEP.b),         inst!(0xA2: XEP.b),         inst!(0xA3: XEP.b),          inst!(0xA4: XEP.b),         inst!(0xA5: XEP.b),         inst!(0xA6: AND.b a idx_ix), inst!(0xA7: XEP.b),         inst!(0xA8: XEP.b), inst!(0xA9: XEP.b),       inst!(0xAA: XEP.b),      inst!(0xAB: XEP.b),    inst!(0xAC: XEP.b), inst!(0xAD: XEP.b), inst!(0xAE: XOR.b a idx_ix), inst!(0xAF: XEP.b),
        inst!(0xB0: XEP.b),         inst!(0xB1: XEP.b),         inst!(0xB2: XEP.b),         inst!(0xB3: XEP.b),          inst!(0xB4: XEP.b),         inst!(0xB5: XEP.b),         inst!(0xB6: OR.b a idx_ix),  inst!(0xB7: XEP.b),         inst!(0xB8: XEP.b), inst!(0xB9: XEP.b),       inst!(0xBA: XEP.b),      inst!(0xBB: XEP.b),    inst!(0xBC: XEP.b), inst!(0xBD: XEP.b), inst!(0xBE: CP.b a idx_ix),  inst!(0xBF: XEP.b),
        inst!(0xC0: XEP.b),         inst!(0xC1: XEP.b),         inst!(0xC2: XEP.b),         inst!(0xC3: XEP.b),          inst!(0xC4: XEP.b),         inst!(0xC5: XEP.b),         inst!(0xC6: XEP.b),          inst!(0xC7: XEP.b),         inst!(0xC8: XEP.b), inst!(0xC9: XEP.b),       inst!(0xCA: XEP.b),      inst!(0xCB: XEP.b),    inst!(0xCC: XEP.b), inst!(0xCD: XEP.b), inst!(0xCE: XEP.b),          inst!(0xCF: XEP.b),
        inst!(0xD0: XEP.b),         inst!(0xD1: XEP.b),         inst!(0xD2: XEP.b),         inst!(0xD3: XEP.b),          inst!(0xD4: XEP.b),         inst!(0xD5: XEP.b),         inst!(0xD6: XEP.b),          inst!(0xD7: XEP.b),         inst!(0xD8: XEP.b), inst!(0xD9: XEP.b),       inst!(0xDA: XEP.b),      inst!(0xDB: XEP.b),    inst!(0xDC: XEP.b), inst!(0xDD: XEP.b), inst!(0xDE: XEP.b),          inst!(0xDF: XEP.b),
        inst!(0xE0: XEP.b),         inst!(0xE1: POP.w ix),      inst!(0xE2: XEP.b),         inst!(0xE3: EX.w ind_sp ix), inst!(0xE4: XEP.b),         inst!(0xE5: PUSH.w ix),     inst!(0xE6: XEP.b),          inst!(0xE7: XEP.b),         inst!(0xE8: XEP.b), inst!(0xE9: JP_ind.b ix), inst!(0xEA: XEP.b),      inst!(0xEB: XEP.b),    inst!(0xEC: XEP.b), inst!(0xED: XEP.b), inst!(0xEE: XEP.b),          inst!(0xEF: XEP.b),
        inst!(0xF0: XEP.b),         inst!(0xF1: XEP.b),         inst!(0xF2: XEP.b),         inst!(0xF3: XEP.b),          inst!(0xF4: XEP.b),         inst!(0xF5: XEP.b),         inst!(0xF6: XEP.b),          inst!(0xF7: XEP.b),         inst!(0xF8: XEP.b), inst!(0xF9: LD.w sp ix),  inst!(0xFA: XEP.b),      inst!(0xFB: XEP.b),    inst!(0xFC: XEP.b), inst!(0xFD: XEP.b), inst!(0xFE: XEP.b),          inst!(0xFF: XEP.b),
    ];

    static ref ED_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: XEP.b),        inst!(0x01: XEP.b),         inst!(0x02: XEP.b),       inst!(0x03: XEP.b),      inst!(0x04: XEP.b),   inst!(0x05: XEP.b),  inst!(0x06: XEP.b),         inst!(0x07: XEP.b),        inst!(0x08: XEP.b),        inst!(0x09: XEP.b),         inst!(0x0A: XEP.b),       inst!(0x0B: XEP.b),      inst!(0x0C: XEP.b), inst!(0x0D: XEP.b),  inst!(0x0E: XEP.b),          inst!(0x0F: XEP.b),
        inst!(0x10: XEP.b),        inst!(0x11: XEP.b),         inst!(0x12: XEP.b),       inst!(0x13: XEP.b),      inst!(0x14: XEP.b),   inst!(0x15: XEP.b),  inst!(0x16: XEP.b),         inst!(0x17: XEP.b),        inst!(0x18: XEP.b),        inst!(0x19: XEP.b),         inst!(0x1A: XEP.b),       inst!(0x1B: XEP.b),      inst!(0x1C: XEP.b), inst!(0x1D: XEP.b),  inst!(0x1E: XEP.b),          inst!(0x1F: XEP.b),
        inst!(0x20: XEP.b),        inst!(0x21: XEP.b),         inst!(0x22: XEP.b),       inst!(0x23: XEP.b),      inst!(0x24: XEP.b),   inst!(0x25: XEP.b),  inst!(0x26: XEP.b),         inst!(0x27: XEP.b),        inst!(0x28: XEP.b),        inst!(0x29: XEP.b),         inst!(0x2A: XEP.b),       inst!(0x2B: XEP.b),      inst!(0x2C: XEP.b), inst!(0x2D: XEP.b),  inst!(0x2E: XEP.b),          inst!(0x2F: XEP.b),
        inst!(0x30: XEP.b),        inst!(0x31: XEP.b),         inst!(0x32: XEP.b),       inst!(0x33: XEP.b),      inst!(0x34: XEP.b),   inst!(0x35: XEP.b),  inst!(0x36: XEP.b),         inst!(0x37: XEP.b),        inst!(0x38: XEP.b),        inst!(0x39: XEP.b),         inst!(0x3A: XEP.b),       inst!(0x3B: XEP.b),      inst!(0x3C: XEP.b), inst!(0x3D: XEP.b),  inst!(0x3E: XEP.b),          inst!(0x3F: XEP.b),
        inst!(0x40: IN.b b ind_c), inst!(0x41: OUT.b ind_c b), inst!(0x42: SBC.w hl bc), inst!(0x43: LD.w DD bc), inst!(0x44: NEG.b a), inst!(0x45: RETN.b), inst!(0x46: IM.b 0),        inst!(0x47: LD.b i a),     inst!(0x48: IN.b c ind_c), inst!(0x49: OUT.b ind_c c), inst!(0x4A: ADC.w hl bc), inst!(0x4B: LD.w bc DD), inst!(0x4C: XEP.b), inst!(0x4D: RETI.b), inst!(0x4E: LD.b c idx_ix),  inst!(0x4F: LD.b r a),
        inst!(0x50: IN.b d ind_c), inst!(0x51: OUT.b ind_c d), inst!(0x52: SBC.w hl de), inst!(0x53: LD.w DD de), inst!(0x54: XEP.b),   inst!(0x55: XEP.b),  inst!(0x56: IM.b 1),        inst!(0x57: LD.b a i),     inst!(0x58: IN.b e ind_c), inst!(0x59: OUT.b ind_c e), inst!(0x5A: ADC.w hl de), inst!(0x5B: LD.w de DD), inst!(0x5C: XEP.b), inst!(0x5D: XEP.b),  inst!(0x5E: IM.b 2),         inst!(0x5F: LD.b a r),
        inst!(0x60: IN.b h ind_c), inst!(0x61: OUT.b ind_c h), inst!(0x62: SBC.w hl hl), inst!(0x63: XEP.b),      inst!(0x64: XEP.b),   inst!(0x65: XEP.b),  inst!(0x66: LD.b h idx_ix), inst!(0x67: RRD.b ind_hl), inst!(0x68: IN.b l ind_c), inst!(0x69: OUT.b ind_c l), inst!(0x6A: ADC.w hl hl), inst!(0x6B: XEP.b),      inst!(0x6C: XEP.b), inst!(0x6D: XEP.b),  inst!(0x6E: LD.b l idx_ix),  inst!(0x6F: RLD.b ind_hl),
        inst!(0x70: XEP.b),        inst!(0x71: XEP.b),         inst!(0x72: SBC.w hl sp), inst!(0x73: LD.w DD sp), inst!(0x74: XEP.b),   inst!(0x75: XEP.b),  inst!(0x76: XEP.b),         inst!(0x77: XEP.b),        inst!(0x78: IN.b a ind_c), inst!(0x79: OUT.b ind_c a), inst!(0x7A: ADC.w hl sp), inst!(0x7B: LD.w sp DD), inst!(0x7C: XEP.b), inst!(0x7D: XEP.b),  inst!(0x7E: XEP.b),          inst!(0x7F: XEP.b),
        inst!(0x80: XEP.b),        inst!(0x81: XEP.b),         inst!(0x82: XEP.b),       inst!(0x83: XEP.b),      inst!(0x84: XEP.b),   inst!(0x85: XEP.b),  inst!(0x86: XEP.b),         inst!(0x87: XEP.b),        inst!(0x88: XEP.b),        inst!(0x89: XEP.b),         inst!(0x8A: XEP.b),       inst!(0x8B: XEP.b),      inst!(0x8C: XEP.b), inst!(0x8D: XEP.b),  inst!(0x8E: XEP.b),          inst!(0x8F: XEP.b),
        inst!(0x90: XEP.b),        inst!(0x91: XEP.b),         inst!(0x92: XEP.b),       inst!(0x93: XEP.b),      inst!(0x94: XEP.b),   inst!(0x95: XEP.b),  inst!(0x96: XEP.b),         inst!(0x97: XEP.b),        inst!(0x98: XEP.b),        inst!(0x99: XEP.b),         inst!(0x9A: XEP.b),       inst!(0x9B: XEP.b),      inst!(0x9C: XEP.b), inst!(0x9D: XEP.b),  inst!(0x9E: XEP.b),          inst!(0x9F: XEP.b),
        inst!(0xA0: LDI.b),        inst!(0xA1: CPI.b),         inst!(0xA2: INI.b),       inst!(0xA3: OUTI.b),     inst!(0xA4: XEP.b),   inst!(0xA5: XEP.b),  inst!(0xA6: XEP.b),         inst!(0xA7: XEP.b),        inst!(0xA8: LDD.b),        inst!(0xA9: CPD.b),         inst!(0xAA: IND.b),       inst!(0xAB: OUTD.b),     inst!(0xAC: XEP.b), inst!(0xAD: XEP.b),  inst!(0xAE: XOR.b a idx_ix), inst!(0xAF: XEP.b),
        inst!(0xB0: LDIR.b),       inst!(0xB1: CPIR.b),        inst!(0xB2: INIR.b),      inst!(0xB3: OTIR.b),     inst!(0xB4: XEP.b),   inst!(0xB5: XEP.b),  inst!(0xB6: XEP.b),         inst!(0xB7: XEP.b),        inst!(0xB8: LDDR.b),       inst!(0xB9: CPDR.b),        inst!(0xBA: INDR.b),      inst!(0xBB: OTDR.b),     inst!(0xBC: XEP.b), inst!(0xBD: XEP.b),  inst!(0xBE: XEP.b),          inst!(0xBF: XEP.b),
        inst!(0xC0: XEP.b),        inst!(0xC1: XEP.b),         inst!(0xC2: XEP.b),       inst!(0xC3: XEP.b),      inst!(0xC4: XEP.b),   inst!(0xC5: XEP.b),  inst!(0xC6: XEP.b),         inst!(0xC7: XEP.b),        inst!(0xC8: XEP.b),        inst!(0xC9: XEP.b),         inst!(0xCA: XEP.b),       inst!(0xCB: XEP.b),      inst!(0xCC: XEP.b), inst!(0xCD: XEP.b),  inst!(0xCE: XEP.b),          inst!(0xCF: XEP.b),
        inst!(0xD0: XEP.b),        inst!(0xD1: XEP.b),         inst!(0xD2: XEP.b),       inst!(0xD3: XEP.b),      inst!(0xD4: XEP.b),   inst!(0xD5: XEP.b),  inst!(0xD6: XEP.b),         inst!(0xD7: XEP.b),        inst!(0xD8: XEP.b),        inst!(0xD9: XEP.b),         inst!(0xDA: XEP.b),       inst!(0xDB: XEP.b),      inst!(0xDC: XEP.b), inst!(0xDD: XEP.b),  inst!(0xDE: XEP.b),          inst!(0xDF: XEP.b),
        inst!(0xE0: XEP.b),        inst!(0xE1: XEP.b),         inst!(0xE2: XEP.b),       inst!(0xE3: XEP.b),      inst!(0xE4: XEP.b),   inst!(0xE5: XEP.b),  inst!(0xE6: XEP.b),         inst!(0xE7: XEP.b),        inst!(0xE8: XEP.b),        inst!(0xE9: XEP.b),         inst!(0xEA: XEP.b),       inst!(0xEB: XEP.b),      inst!(0xEC: XEP.b), inst!(0xED: XEP.b),  inst!(0xEE: XEP.b),          inst!(0xEF: XEP.b),
        inst!(0xF0: XEP.b),        inst!(0xF1: XEP.b),         inst!(0xF2: XEP.b),       inst!(0xF3: XEP.b),      inst!(0xF4: XEP.b),   inst!(0xF5: XEP.b),  inst!(0xF6: XEP.b),         inst!(0xF7: XEP.b),        inst!(0xF8: XEP.b),        inst!(0xF9: XEP.b),         inst!(0xFA: XEP.b),       inst!(0xFB: XEP.b),      inst!(0xFC: XEP.b), inst!(0xFD: XEP.b),  inst!(0xFE: XEP.b),          inst!(0xFF: XEP.b),
    ];

    static ref FD_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: XEP.b),         inst!(0x01: XEP.b),         inst!(0x02: XEP.b),         inst!(0x03: XEP.b),          inst!(0x04: XEP.b),         inst!(0x05: XEP.b),         inst!(0x06: XEP.b),          inst!(0x07: XEP.b),         inst!(0x08: XEP.b), inst!(0x09: ADD.w iy bc), inst!(0x0A: XEP.b),      inst!(0x0B: XEP.b),    inst!(0x0C: XEP.b), inst!(0x0D: XEP.b), inst!(0x0E: XEP.b),          inst!(0x0F: XEP.b),
        inst!(0x10: XEP.b),         inst!(0x11: XEP.b),         inst!(0x12: XEP.b),         inst!(0x13: XEP.b),          inst!(0x14: XEP.b),         inst!(0x15: XEP.b),         inst!(0x16: XEP.b),          inst!(0x17: XEP.b),         inst!(0x18: XEP.b), inst!(0x19: ADD.w iy de), inst!(0x1A: XEP.b),      inst!(0x1B: XEP.b),    inst!(0x1C: XEP.b), inst!(0x1D: XEP.b), inst!(0x1E: XEP.b),          inst!(0x1F: XEP.b),
        inst!(0x20: XEP.b),         inst!(0x21: LD.w iy nn),    inst!(0x22: LD.w DD iy),    inst!(0x23: INC.w iy),       inst!(0x24: XEP.b),         inst!(0x25: XEP.b),         inst!(0x26: XEP.b),          inst!(0x27: XEP.b),         inst!(0x28: XEP.b), inst!(0x29: ADD.w iy iy), inst!(0x2A: LD.w iy DD), inst!(0x2B: DEC.w iy), inst!(0x2C: XEP.b), inst!(0x2D: XEP.b), inst!(0x2E: XEP.b),          inst!(0x2F: XEP.b),
        inst!(0x30: XEP.b),         inst!(0x31: XEP.b),         inst!(0x32: XEP.b),         inst!(0x33: XEP.b),          inst!(0x34: INC.b idx_iy),  inst!(0x35: DEC.b idx_iy),  inst!(0x36: LD.b idx_iy n),  inst!(0x37: XEP.b),         inst!(0x38: XEP.b), inst!(0x39: ADD.w iy sp), inst!(0x3A: XEP.b),      inst!(0x3B: XEP.b),    inst!(0x3C: XEP.b), inst!(0x3D: XEP.b), inst!(0x3E: XEP.b),          inst!(0x3F: XEP.b),
        inst!(0x40: XEP.b),         inst!(0x41: XEP.b),         inst!(0x42: XEP.b),         inst!(0x43: XEP.b),          inst!(0x44: XEP.b),         inst!(0x45: XEP.b),         inst!(0x46: LD.b b idx_iy),  inst!(0x47: XEP.b),         inst!(0x48: XEP.b), inst!(0x49: XEP.b),       inst!(0x4A: XEP.b),      inst!(0x4B: XEP.b),    inst!(0x4C: XEP.b), inst!(0x4D: XEP.b), inst!(0x4E: LD.b c idx_iy),  inst!(0x4F: XEP.b),
        inst!(0x50: XEP.b),         inst!(0x51: XEP.b),         inst!(0x52: XEP.b),         inst!(0x53: XEP.b),          inst!(0x54: XEP.b),         inst!(0x55: XEP.b),         inst!(0x56: LD.b d idx_iy),  inst!(0x57: XEP.b),         inst!(0x58: XEP.b), inst!(0x59: XEP.b),       inst!(0x5A: XEP.b),      inst!(0x5B: XEP.b),    inst!(0x5C: XEP.b), inst!(0x5D: XEP.b), inst!(0x5E: LD.b e idx_iy),  inst!(0x5F: XEP.b),
        inst!(0x60: XEP.b),         inst!(0x61: XEP.b),         inst!(0x62: XEP.b),         inst!(0x63: XEP.b),          inst!(0x64: XEP.b),         inst!(0x65: XEP.b),         inst!(0x66: LD.b h idx_iy),  inst!(0x67: XEP.b),         inst!(0x68: XEP.b), inst!(0x69: XEP.b),       inst!(0x6A: XEP.b),      inst!(0x6B: XEP.b),    inst!(0x6C: XEP.b), inst!(0x6D: XEP.b), inst!(0x6E: LD.b l idx_iy),  inst!(0x6F: XEP.b),
        inst!(0x70: LD.b idx_iy b), inst!(0x71: LD.b idx_iy c), inst!(0x72: LD.b idx_iy d), inst!(0x73: LD.b idx_iy e),  inst!(0x74: LD.b idx_iy h), inst!(0x75: LD.b idx_iy l), inst!(0x76: XEP.b),          inst!(0x77: LD.b idx_iy a), inst!(0x78: XEP.b), inst!(0x79: XEP.b),       inst!(0x7A: XEP.b),      inst!(0x7B: XEP.b),    inst!(0x7C: XEP.b), inst!(0x7D: XEP.b), inst!(0x7E: LD.b a idx_iy),  inst!(0x7F: XEP.b),
        inst!(0x80: XEP.b),         inst!(0x81: XEP.b),         inst!(0x82: XEP.b),         inst!(0x83: XEP.b),          inst!(0x84: XEP.b),         inst!(0x85: XEP.b),         inst!(0x86: ADD.b a idx_iy), inst!(0x87: XEP.b),         inst!(0x88: XEP.b), inst!(0x89: XEP.b),       inst!(0x8A: XEP.b),      inst!(0x8B: XEP.b),    inst!(0x8C: XEP.b), inst!(0x8D: XEP.b), inst!(0x8E: ADC.b a idx_iy), inst!(0x8F: XEP.b),
        inst!(0x90: XEP.b),         inst!(0x91: XEP.b),         inst!(0x92: XEP.b),         inst!(0x93: XEP.b),          inst!(0x94: XEP.b),         inst!(0x95: XEP.b),         inst!(0x96: SUB.b a idx_iy), inst!(0x97: XEP.b),         inst!(0x98: XEP.b), inst!(0x99: XEP.b),       inst!(0x9A: XEP.b),      inst!(0x9B: XEP.b),    inst!(0x9C: XEP.b), inst!(0x9D: XEP.b), inst!(0x9E: SBC.b a idx_iy), inst!(0x9F: XEP.b),
        inst!(0xA0: XEP.b),         inst!(0xA1: XEP.b),         inst!(0xA2: XEP.b),         inst!(0xA3: XEP.b),          inst!(0xA4: XEP.b),         inst!(0xA5: XEP.b),         inst!(0xA6: AND.b a idx_iy), inst!(0xA7: XEP.b),         inst!(0xA8: XEP.b), inst!(0xA9: XEP.b),       inst!(0xAA: XEP.b),      inst!(0xAB: XEP.b),    inst!(0xAC: XEP.b), inst!(0xAD: XEP.b), inst!(0xAE: XOR.b a idx_iy), inst!(0xAF: XEP.b),
        inst!(0xB0: XEP.b),         inst!(0xB1: XEP.b),         inst!(0xB2: XEP.b),         inst!(0xB3: XEP.b),          inst!(0xB4: XEP.b),         inst!(0xB5: XEP.b),         inst!(0xB6: OR.b a idx_iy),  inst!(0xB7: XEP.b),         inst!(0xB8: XEP.b), inst!(0xB9: XEP.b),       inst!(0xBA: XEP.b),      inst!(0xBB: XEP.b),    inst!(0xBC: XEP.b), inst!(0xBD: XEP.b), inst!(0xBE: CP.b a idx_iy),  inst!(0xBF: XEP.b),
        inst!(0xC0: XEP.b),         inst!(0xC1: XEP.b),         inst!(0xC2: XEP.b),         inst!(0xC3: XEP.b),          inst!(0xC4: XEP.b),         inst!(0xC5: XEP.b),         inst!(0xC6: XEP.b),          inst!(0xC7: XEP.b),         inst!(0xC8: XEP.b), inst!(0xC9: XEP.b),       inst!(0xCA: XEP.b),      inst!(0xCB: XEP.b),    inst!(0xCC: XEP.b), inst!(0xCD: XEP.b), inst!(0xCE: XEP.b),          inst!(0xCF: XEP.b),
        inst!(0xD0: XEP.b),         inst!(0xD1: XEP.b),         inst!(0xD2: XEP.b),         inst!(0xD3: XEP.b),          inst!(0xD4: XEP.b),         inst!(0xD5: XEP.b),         inst!(0xD6: XEP.b),          inst!(0xD7: XEP.b),         inst!(0xD8: XEP.b), inst!(0xD9: XEP.b),       inst!(0xDA: XEP.b),      inst!(0xDB: XEP.b),    inst!(0xDC: XEP.b), inst!(0xDD: XEP.b), inst!(0xDE: XEP.b),          inst!(0xDF: XEP.b),
        inst!(0xE0: XEP.b),         inst!(0xE1: POP.w iy),      inst!(0xE2: XEP.b),         inst!(0xE3: EX.w ind_sp iy), inst!(0xE4: XEP.b),         inst!(0xE5: PUSH.w iy),     inst!(0xE6: XEP.b),          inst!(0xE7: XEP.b),         inst!(0xE8: XEP.b), inst!(0xE9: JP_ind.b iy), inst!(0xEA: XEP.b),      inst!(0xEB: XEP.b),    inst!(0xEC: XEP.b), inst!(0xED: XEP.b), inst!(0xEE: XEP.b),          inst!(0xEF: XEP.b),
        inst!(0xF0: XEP.b),         inst!(0xF1: XEP.b),         inst!(0xF2: XEP.b),         inst!(0xF3: XEP.b),          inst!(0xF4: XEP.b),         inst!(0xF5: XEP.b),         inst!(0xF6: XEP.b),          inst!(0xF7: XEP.b),         inst!(0xF8: XEP.b), inst!(0xF9: LD.w sp iy),  inst!(0xFA: XEP.b),      inst!(0xFB: XEP.b),    inst!(0xFC: XEP.b), inst!(0xFD: XEP.b), inst!(0xFE: XEP.b),          inst!(0xFF: XEP.b),
    ];

    static ref DDCB_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: XEP.b), inst!(0x01: XEP.b), inst!(0x02: XEP.b), inst!(0x03: XEP.b), inst!(0x04: XEP.b), inst!(0x05: XEP.b), inst!(0x06: RLC.b idx_ix),   inst!(0x07: XEP.b), inst!(0x08: XEP.b), inst!(0x09: XEP.b), inst!(0x0A: XEP.b), inst!(0x0B: XEP.b), inst!(0x0C: XEP.b), inst!(0x0D: XEP.b), inst!(0x0E: RRC.b idx_ix), inst!(0x0F: XEP.b),
        inst!(0x10: XEP.b), inst!(0x11: XEP.b), inst!(0x12: XEP.b), inst!(0x13: XEP.b), inst!(0x14: XEP.b), inst!(0x15: XEP.b), inst!(0x16: RL.b idx_ix),    inst!(0x17: XEP.b), inst!(0x18: XEP.b), inst!(0x19: XEP.b), inst!(0x1A: XEP.b), inst!(0x1B: XEP.b), inst!(0x1C: XEP.b), inst!(0x1D: XEP.b), inst!(0x1E: RR.b idx_ix), inst!(0x1F: XEP.b),
        inst!(0x20: XEP.b), inst!(0x21: XEP.b), inst!(0x22: XEP.b), inst!(0x23: XEP.b), inst!(0x24: XEP.b), inst!(0x25: XEP.b), inst!(0x26: SLA.b idx_ix),   inst!(0x27: XEP.b), inst!(0x28: XEP.b), inst!(0x29: XEP.b), inst!(0x2A: XEP.b), inst!(0x2B: XEP.b), inst!(0x2C: XEP.b), inst!(0x2D: XEP.b), inst!(0x2E: SRA.b idx_ix), inst!(0x2F: XEP.b),
        inst!(0x30: XEP.b), inst!(0x31: XEP.b), inst!(0x32: XEP.b), inst!(0x33: XEP.b), inst!(0x34: XEP.b), inst!(0x35: XEP.b), inst!(0x36: XEP.b),          inst!(0x37: XEP.b), inst!(0x38: XEP.b), inst!(0x39: XEP.b), inst!(0x3A: XEP.b), inst!(0x3B: XEP.b), inst!(0x3C: XEP.b), inst!(0x3D: XEP.b), inst!(0x3E: SRL.b idx_ix), inst!(0x3F: XEP.b),
        inst!(0x40: XEP.b), inst!(0x41: XEP.b), inst!(0x42: XEP.b), inst!(0x43: XEP.b), inst!(0x44: XEP.b), inst!(0x45: XEP.b), inst!(0x46: BIT.b 0 idx_ix), inst!(0x47: XEP.b), inst!(0x48: XEP.b), inst!(0x49: XEP.b), inst!(0x4A: XEP.b), inst!(0x4B: XEP.b), inst!(0x4C: XEP.b), inst!(0x4D: XEP.b), inst!(0x4E: BIT.b 1 idx_ix), inst!(0x4F: XEP.b),
        inst!(0x50: XEP.b), inst!(0x51: XEP.b), inst!(0x52: XEP.b), inst!(0x53: XEP.b), inst!(0x54: XEP.b), inst!(0x55: XEP.b), inst!(0x56: BIT.b 2 idx_ix), inst!(0x57: XEP.b), inst!(0x58: XEP.b), inst!(0x59: XEP.b), inst!(0x5A: XEP.b), inst!(0x5B: XEP.b), inst!(0x5C: XEP.b), inst!(0x5D: XEP.b), inst!(0x5E: BIT.b 3 idx_ix), inst!(0x5F: XEP.b),
        inst!(0x60: XEP.b), inst!(0x61: XEP.b), inst!(0x62: XEP.b), inst!(0x63: XEP.b), inst!(0x64: XEP.b), inst!(0x65: XEP.b), inst!(0x66: BIT.b 4 idx_ix), inst!(0x67: XEP.b), inst!(0x68: XEP.b), inst!(0x69: XEP.b), inst!(0x6A: XEP.b), inst!(0x6B: XEP.b), inst!(0x6C: XEP.b), inst!(0x6D: XEP.b), inst!(0x6E: BIT.b 5 idx_ix), inst!(0x6F: XEP.b),
        inst!(0x70: XEP.b), inst!(0x71: XEP.b), inst!(0x72: XEP.b), inst!(0x73: XEP.b), inst!(0x74: XEP.b), inst!(0x75: XEP.b), inst!(0x76: BIT.b 6 idx_ix), inst!(0x77: XEP.b), inst!(0x78: XEP.b), inst!(0x79: XEP.b), inst!(0x7A: XEP.b), inst!(0x7B: XEP.b), inst!(0x7C: XEP.b), inst!(0x7D: XEP.b), inst!(0x7E: BIT.b 7 idx_ix), inst!(0x7F: XEP.b),
        inst!(0x80: XEP.b), inst!(0x81: XEP.b), inst!(0x82: XEP.b), inst!(0x83: XEP.b), inst!(0x84: XEP.b), inst!(0x85: XEP.b), inst!(0x86: RES.b 0 idx_ix), inst!(0x87: XEP.b), inst!(0x88: XEP.b), inst!(0x89: XEP.b), inst!(0x8A: XEP.b), inst!(0x8B: XEP.b), inst!(0x8C: XEP.b), inst!(0x8D: XEP.b), inst!(0x8E: RES.b 1 idx_ix), inst!(0x8F: XEP.b),
        inst!(0x90: XEP.b), inst!(0x91: XEP.b), inst!(0x92: XEP.b), inst!(0x93: XEP.b), inst!(0x94: XEP.b), inst!(0x95: XEP.b), inst!(0x96: RES.b 2 idx_ix), inst!(0x97: XEP.b), inst!(0x98: XEP.b), inst!(0x99: XEP.b), inst!(0x9A: XEP.b), inst!(0x9B: XEP.b), inst!(0x9C: XEP.b), inst!(0x9D: XEP.b), inst!(0x9E: RES.b 3 idx_ix), inst!(0x9F: XEP.b),
        inst!(0xA0: XEP.b), inst!(0xA1: XEP.b), inst!(0xA2: XEP.b), inst!(0xA3: XEP.b), inst!(0xA4: XEP.b), inst!(0xA5: XEP.b), inst!(0xA6: RES.b 4 idx_ix), inst!(0xA7: XEP.b), inst!(0xA8: XEP.b), inst!(0xA9: XEP.b), inst!(0xAA: XEP.b), inst!(0xAB: XEP.b), inst!(0xAC: XEP.b), inst!(0xAD: XEP.b), inst!(0xAE: RES.b 5 idx_ix), inst!(0xAF: XEP.b),
        inst!(0xB0: XEP.b), inst!(0xB1: XEP.b), inst!(0xB2: XEP.b), inst!(0xB3: XEP.b), inst!(0xB4: XEP.b), inst!(0xB5: XEP.b), inst!(0xB6: RES.b 6 idx_ix), inst!(0xB7: XEP.b), inst!(0xB8: XEP.b), inst!(0xB9: XEP.b), inst!(0xBA: XEP.b), inst!(0xBB: XEP.b), inst!(0xBC: XEP.b), inst!(0xBD: XEP.b), inst!(0xBE: RES.b 7 idx_ix), inst!(0xBF: XEP.b),
        inst!(0xC0: XEP.b), inst!(0xC1: XEP.b), inst!(0xC2: XEP.b), inst!(0xC3: XEP.b), inst!(0xC4: XEP.b), inst!(0xC5: XEP.b), inst!(0xC6: SET.b 0 idx_ix), inst!(0xC7: XEP.b), inst!(0xC8: XEP.b), inst!(0xC9: XEP.b), inst!(0xCA: XEP.b), inst!(0xCB: XEP.b), inst!(0xCC: XEP.b), inst!(0xCD: XEP.b), inst!(0xCE: SET.b 1 idx_ix), inst!(0xCF: XEP.b),
        inst!(0xD0: XEP.b), inst!(0xD1: XEP.b), inst!(0xD2: XEP.b), inst!(0xD3: XEP.b), inst!(0xD4: XEP.b), inst!(0xD5: XEP.b), inst!(0xD6: SET.b 2 idx_ix), inst!(0xD7: XEP.b), inst!(0xD8: XEP.b), inst!(0xD9: XEP.b), inst!(0xDA: XEP.b), inst!(0xDB: XEP.b), inst!(0xDC: XEP.b), inst!(0xDD: XEP.b), inst!(0xDE: SET.b 3 idx_ix), inst!(0xDF: XEP.b),
        inst!(0xE0: XEP.b), inst!(0xE1: XEP.b), inst!(0xE2: XEP.b), inst!(0xE3: XEP.b), inst!(0xE4: XEP.b), inst!(0xE5: XEP.b), inst!(0xE6: SET.b 4 idx_ix), inst!(0xE7: XEP.b), inst!(0xE8: XEP.b), inst!(0xE9: XEP.b), inst!(0xEA: XEP.b), inst!(0xEB: XEP.b), inst!(0xEC: XEP.b), inst!(0xED: XEP.b), inst!(0xEE: SET.b 5 idx_ix), inst!(0xEF: XEP.b),
        inst!(0xF0: XEP.b), inst!(0xF1: XEP.b), inst!(0xF2: XEP.b), inst!(0xF3: XEP.b), inst!(0xF4: XEP.b), inst!(0xF5: XEP.b), inst!(0xF6: SET.b 6 idx_ix), inst!(0xF7: XEP.b), inst!(0xF8: XEP.b), inst!(0xF9: XEP.b), inst!(0xFA: XEP.b), inst!(0xFB: XEP.b), inst!(0xFC: XEP.b), inst!(0xFD: XEP.b), inst!(0xFE: SET.b 7 idx_ix), inst!(0xFF: XEP.b),
    ];

    static ref FDCB_OPCODE_TABLE: [Instruction; 0x100] = [
        inst!(0x00: XEP.b), inst!(0x01: XEP.b), inst!(0x02: XEP.b), inst!(0x03: XEP.b), inst!(0x04: XEP.b), inst!(0x05: XEP.b), inst!(0x06: RLC.b idx_iy),   inst!(0x07: XEP.b), inst!(0x08: XEP.b), inst!(0x09: XEP.b), inst!(0x0A: XEP.b), inst!(0x0B: XEP.b), inst!(0x0C: XEP.b), inst!(0x0D: XEP.b), inst!(0x0E: RRC.b idx_iy), inst!(0x0F: XEP.b),
        inst!(0x10: XEP.b), inst!(0x11: XEP.b), inst!(0x12: XEP.b), inst!(0x13: XEP.b), inst!(0x14: XEP.b), inst!(0x15: XEP.b), inst!(0x16: RL.b idx_iy),    inst!(0x17: XEP.b), inst!(0x18: XEP.b), inst!(0x19: XEP.b), inst!(0x1A: XEP.b), inst!(0x1B: XEP.b), inst!(0x1C: XEP.b), inst!(0x1D: XEP.b), inst!(0x1E: RR.b idx_iy), inst!(0x1F: XEP.b),
        inst!(0x20: XEP.b), inst!(0x21: XEP.b), inst!(0x22: XEP.b), inst!(0x23: XEP.b), inst!(0x24: XEP.b), inst!(0x25: XEP.b), inst!(0x26: SLA.b idx_iy),   inst!(0x27: XEP.b), inst!(0x28: XEP.b), inst!(0x29: XEP.b), inst!(0x2A: XEP.b), inst!(0x2B: XEP.b), inst!(0x2C: XEP.b), inst!(0x2D: XEP.b), inst!(0x2E: SRA.b idx_iy), inst!(0x2F: XEP.b),
        inst!(0x30: XEP.b), inst!(0x31: XEP.b), inst!(0x32: XEP.b), inst!(0x33: XEP.b), inst!(0x34: XEP.b), inst!(0x35: XEP.b), inst!(0x36: XEP.b),          inst!(0x37: XEP.b), inst!(0x38: XEP.b), inst!(0x39: XEP.b), inst!(0x3A: XEP.b), inst!(0x3B: XEP.b), inst!(0x3C: XEP.b), inst!(0x3D: XEP.b), inst!(0x3E: SRL.b idx_iy), inst!(0x3F: XEP.b),
        inst!(0x40: XEP.b), inst!(0x41: XEP.b), inst!(0x42: XEP.b), inst!(0x43: XEP.b), inst!(0x44: XEP.b), inst!(0x45: XEP.b), inst!(0x46: BIT.b 0 idx_iy), inst!(0x47: XEP.b), inst!(0x48: XEP.b), inst!(0x49: XEP.b), inst!(0x4A: XEP.b), inst!(0x4B: XEP.b), inst!(0x4C: XEP.b), inst!(0x4D: XEP.b), inst!(0x4E: BIT.b 1 idx_iy), inst!(0x4F: XEP.b),
        inst!(0x50: XEP.b), inst!(0x51: XEP.b), inst!(0x52: XEP.b), inst!(0x53: XEP.b), inst!(0x54: XEP.b), inst!(0x55: XEP.b), inst!(0x56: BIT.b 2 idx_iy), inst!(0x57: XEP.b), inst!(0x58: XEP.b), inst!(0x59: XEP.b), inst!(0x5A: XEP.b), inst!(0x5B: XEP.b), inst!(0x5C: XEP.b), inst!(0x5D: XEP.b), inst!(0x5E: BIT.b 3 idx_iy), inst!(0x5F: XEP.b),
        inst!(0x60: XEP.b), inst!(0x61: XEP.b), inst!(0x62: XEP.b), inst!(0x63: XEP.b), inst!(0x64: XEP.b), inst!(0x65: XEP.b), inst!(0x66: BIT.b 4 idx_iy), inst!(0x67: XEP.b), inst!(0x68: XEP.b), inst!(0x69: XEP.b), inst!(0x6A: XEP.b), inst!(0x6B: XEP.b), inst!(0x6C: XEP.b), inst!(0x6D: XEP.b), inst!(0x6E: BIT.b 5 idx_iy), inst!(0x6F: XEP.b),
        inst!(0x70: XEP.b), inst!(0x71: XEP.b), inst!(0x72: XEP.b), inst!(0x73: XEP.b), inst!(0x74: XEP.b), inst!(0x75: XEP.b), inst!(0x76: BIT.b 6 idx_iy), inst!(0x77: XEP.b), inst!(0x78: XEP.b), inst!(0x79: XEP.b), inst!(0x7A: XEP.b), inst!(0x7B: XEP.b), inst!(0x7C: XEP.b), inst!(0x7D: XEP.b), inst!(0x7E: BIT.b 7 idx_iy), inst!(0x7F: XEP.b),
        inst!(0x80: XEP.b), inst!(0x81: XEP.b), inst!(0x82: XEP.b), inst!(0x83: XEP.b), inst!(0x84: XEP.b), inst!(0x85: XEP.b), inst!(0x86: RES.b 0 idx_iy), inst!(0x87: XEP.b), inst!(0x88: XEP.b), inst!(0x89: XEP.b), inst!(0x8A: XEP.b), inst!(0x8B: XEP.b), inst!(0x8C: XEP.b), inst!(0x8D: XEP.b), inst!(0x8E: RES.b 1 idx_iy), inst!(0x8F: XEP.b),
        inst!(0x90: XEP.b), inst!(0x91: XEP.b), inst!(0x92: XEP.b), inst!(0x93: XEP.b), inst!(0x94: XEP.b), inst!(0x95: XEP.b), inst!(0x96: RES.b 2 idx_iy), inst!(0x97: XEP.b), inst!(0x98: XEP.b), inst!(0x99: XEP.b), inst!(0x9A: XEP.b), inst!(0x9B: XEP.b), inst!(0x9C: XEP.b), inst!(0x9D: XEP.b), inst!(0x9E: RES.b 3 idx_iy), inst!(0x9F: XEP.b),
        inst!(0xA0: XEP.b), inst!(0xA1: XEP.b), inst!(0xA2: XEP.b), inst!(0xA3: XEP.b), inst!(0xA4: XEP.b), inst!(0xA5: XEP.b), inst!(0xA6: RES.b 4 idx_iy), inst!(0xA7: XEP.b), inst!(0xA8: XEP.b), inst!(0xA9: XEP.b), inst!(0xAA: XEP.b), inst!(0xAB: XEP.b), inst!(0xAC: XEP.b), inst!(0xAD: XEP.b), inst!(0xAE: RES.b 5 idx_iy), inst!(0xAF: XEP.b),
        inst!(0xB0: XEP.b), inst!(0xB1: XEP.b), inst!(0xB2: XEP.b), inst!(0xB3: XEP.b), inst!(0xB4: XEP.b), inst!(0xB5: XEP.b), inst!(0xB6: RES.b 6 idx_iy), inst!(0xB7: XEP.b), inst!(0xB8: XEP.b), inst!(0xB9: XEP.b), inst!(0xBA: XEP.b), inst!(0xBB: XEP.b), inst!(0xBC: XEP.b), inst!(0xBD: XEP.b), inst!(0xBE: RES.b 7 idx_iy), inst!(0xBF: XEP.b),
        inst!(0xC0: XEP.b), inst!(0xC1: XEP.b), inst!(0xC2: XEP.b), inst!(0xC3: XEP.b), inst!(0xC4: XEP.b), inst!(0xC5: XEP.b), inst!(0xC6: SET.b 0 idx_iy), inst!(0xC7: XEP.b), inst!(0xC8: XEP.b), inst!(0xC9: XEP.b), inst!(0xCA: XEP.b), inst!(0xCB: XEP.b), inst!(0xCC: XEP.b), inst!(0xCD: XEP.b), inst!(0xCE: SET.b 1 idx_iy), inst!(0xCF: XEP.b),
        inst!(0xD0: XEP.b), inst!(0xD1: XEP.b), inst!(0xD2: XEP.b), inst!(0xD3: XEP.b), inst!(0xD4: XEP.b), inst!(0xD5: XEP.b), inst!(0xD6: SET.b 2 idx_iy), inst!(0xD7: XEP.b), inst!(0xD8: XEP.b), inst!(0xD9: XEP.b), inst!(0xDA: XEP.b), inst!(0xDB: XEP.b), inst!(0xDC: XEP.b), inst!(0xDD: XEP.b), inst!(0xDE: SET.b 3 idx_iy), inst!(0xDF: XEP.b),
        inst!(0xE0: XEP.b), inst!(0xE1: XEP.b), inst!(0xE2: XEP.b), inst!(0xE3: XEP.b), inst!(0xE4: XEP.b), inst!(0xE5: XEP.b), inst!(0xE6: SET.b 4 idx_iy), inst!(0xE7: XEP.b), inst!(0xE8: XEP.b), inst!(0xE9: XEP.b), inst!(0xEA: XEP.b), inst!(0xEB: XEP.b), inst!(0xEC: XEP.b), inst!(0xED: XEP.b), inst!(0xEE: SET.b 5 idx_iy), inst!(0xEF: XEP.b),
        inst!(0xF0: XEP.b), inst!(0xF1: XEP.b), inst!(0xF2: XEP.b), inst!(0xF3: XEP.b), inst!(0xF4: XEP.b), inst!(0xF5: XEP.b), inst!(0xF6: SET.b 6 idx_iy), inst!(0xF7: XEP.b), inst!(0xF8: XEP.b), inst!(0xF9: XEP.b), inst!(0xFA: XEP.b), inst!(0xFB: XEP.b), inst!(0xFC: XEP.b), inst!(0xFD: XEP.b), inst!(0xFE: SET.b 7 idx_iy), inst!(0xFF: XEP.b),
    ];
}

enum Status {
    S = 7,
    Z = 6,
    H = 4,
    PV = 2,
    N = 1,
    C = 0,
}

pub struct Z80Emu {
    pc: u16,
    sp: u16,

    i: u8, // interrupt page address, stores high-order 8 bits of interrupt address
    r: u8, // memory refresh register (counter), 7 bits increments after each instruction fetch, 8 bit is programmed, resulting from an LD R, A instruction

    af: u16, // acc and flag
    af_: u16, // acc' and flag'
    
    bc: u16,
    de: u16,
    hl: u16,

    bc_: u16,
    de_: u16,
    hl_: u16,

    ix: u16, // index register X
    iy: u16, // index register Y

    iff1: bool,
    iff2: bool,

    //internal
    curr_opcode: u32,
    curr_opcode_size: i32,
    instruction_size: Size,
    src_operand: Option<Operand>,
    dst_operand: Option<Operand>,

    bus: *mut dyn Z80Bus,
}

impl Z80Emu {
    pub fn new(bus: *mut dyn Z80Bus) -> Self {
        Self {
            pc: 0,
            sp: 0,
            i: 0,
            r: 0,
            af: 0,
            af_: 0,
            bc: 0,
            de: 0,
            hl: 0,
            bc_: 0,
            de_: 0,
            hl_: 0,
            ix: 0,
            iy: 0,
            iff1: false,
            iff2: false,
            curr_opcode: 0,
            curr_opcode_size: 0,
            instruction_size: Size::Byte,
            src_operand: None,
            dst_operand: None,
            bus: bus,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.i = 0;
        self.r = 0;
        self.af = 0;
        self.af_ = 0;
        self.bc = 0;
        self.de = 0;
        self.hl = 0;
        self.bc_ = 0;
        self.de_ = 0;
        self.hl_ = 0;
        self.ix = 0;
        self.iy = 0;
        self.iff1 = false;
        self.iff2 = false;
        self.curr_opcode = 0;
        self.curr_opcode_size = 0;
        self.instruction_size = Size::Byte;
        self.src_operand = None;
        self.dst_operand = None;
    }

    pub fn clock(&mut self) {
        self.fetch_current_opcode();
        let opcode_idx = self.curr_opcode as u8 as usize;
        let instruction = match self.curr_opcode_size {
            1 => &OPCODE_TABLE[opcode_idx],
            2 => {
                match self.curr_opcode >> 8 {
                    0xCB => &CB_OPCODE_TABLE[opcode_idx],
                    0xDD => &DD_OPCODE_TABLE[opcode_idx],
                    0xED => &ED_OPCODE_TABLE[opcode_idx],
                    0xFD => &FD_OPCODE_TABLE[opcode_idx],
                    _ => panic!("Z80::clock: unexpected extension bytes: 0x{:0X}", self.curr_opcode >> 8),
                }
            },
            4 => {
                match self.curr_opcode >> 16 {
                    0xDDCB => &DDCB_OPCODE_TABLE[opcode_idx],
                    0xFDCB => &FDCB_OPCODE_TABLE[opcode_idx],
                    _ => panic!("Z80::clock: unexpected extension bytes: 0x{:0X}", self.curr_opcode >> 16),
                }
            }
            _ => panic!("Z80::clock: unexpected opcode size"),
        };

        self.instruction_size = instruction.size;
        self.src_operand = match &instruction.src_am {
            Some(am_type) => Some(self.call_am(am_type)),
            None => None,
        };
        self.dst_operand = match &instruction.dst_am {
            Some(am_type) => Some(self.call_am(am_type)),
            None => None,
        };
        (instruction.handler)(self);
    }

    fn fetch_current_opcode(&mut self) {
        // excluded bytes CB, DD, ED, FD, DDCB, FDCB
        let mut opcode = self.read_pc_and_increment() as u32;
        let mut byte_counter = 1;
        if EXTENDING_TO_TWO_BYTES.contains(&opcode) {
            let additional_byte = self.read_pc_and_increment();
            opcode = (opcode << 8) | additional_byte as u32;
            byte_counter += 1;
            if EXTENDING_TO_FOUR_BYTES.contains(&opcode) {
                opcode = (opcode << 8) | self.read_pc_and_increment() as u32;
                opcode = (opcode << 8) | self.read_pc_and_increment() as u32;
                byte_counter += 2;
            }
        }
        self.curr_opcode = opcode;
        self.curr_opcode_size = byte_counter;
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    fn read_pc_and_increment(&mut self) -> u8 {
        let data = self.read_memory(self.pc, Size::Byte) as u8;
        self.increment_pc();
        data
    }

    fn call_am(&mut self, am_type: &AmType) -> Operand {
        match am_type {
            AmType::Imm => self.immediate_am(),
            AmType::ImmExt => self.immediate_extended_am(),
            AmType::PageZero(addr) => self.modified_page_zero_am(*addr),
            AmType::Relative => self.relative_am(),
            AmType::Extended => self.extended_am(),
            AmType::Indexed(reg) => self.indexed_am(*reg),
            AmType::Register(reg) => self.register_am(*reg),
            AmType::Implied => self.implied_am(),
            AmType::RegIndirect(reg) => self.register_indirect_am(*reg),
            AmType::BitAddr(offset) => self.bit_am(*offset),
        }
    }

    fn write(&mut self, location: Location, data: u16, size: Size) {
        match location {
            Location::Register(reg) => self.write_register(reg, data, size),
            Location::Memory(addr) => self.write_memory(addr, data, size),
            Location::Const => panic!("Z80::write: can't write into Location::Const"),
        }
    }

    fn read_memory(&mut self, address: u16, size: Size) -> u16 {
        unsafe {
            (*self.bus).read(address, size)
        }
    }

    fn write_memory(&mut self, address: u16, data: u16, size: Size) {
        unsafe {
            (*self.bus).write(address, data, size);
        }
    }

    fn read_register(&self, register: Register) -> u16 {
        match register {
            Register::A => self.af >> 8,
            Register::A_ => self.af_ >> 8,
            Register::B => self.bc >> 8,
            Register::B_ => self.bc_ >> 8,
            Register::C => self.bc & 0xFF,
            Register::C_ => self.bc_ & 0xFF,
            Register::D => self.de >> 8,
            Register::D_ => self.de_ >> 8,
            Register::E => self.de & 0xFF,
            Register::E_ => self.de_ & 0xFF,
            Register::H => self.hl >> 8,
            Register::H_ => self.hl_ >> 8,
            Register::L => self.hl & 0xFF,
            Register::L_ => self.hl_ & 0xFF,
            Register::AF => self.af,
            Register::AF_ => self.af_,
            Register::BC => self.bc,
            Register::BC_ => self.bc_,
            Register::DE => self.de,
            Register::DE_ => self.de_,
            Register::HL => self.hl,
            Register::HL_ => self.hl_,
            Register::I => self.i as u16,
            Register::R => self.r as u16,
            Register::IX => self.ix,
            Register::IY => self.iy,
            Register::SP => self.sp,
        }
    }

    fn write_register(&mut self, register: Register, data: u16, size: Size) {
        let write_high = |reg, data| -> u16 {
            let reg_data = reg & 0x00FF;
            (data << 8) | reg_data
        };
        let write_low = |reg, data| -> u16 {
            let reg_data = reg & 0xFF00;
            data | reg_data
        };
        match register {
            Register::A => self.af = write_high(self.af, data),
            Register::A_ => self.af_ = write_high(self.af, data),
            Register::B => self.bc = write_high(self.bc, data),
            Register::B_ => self.bc_ = write_high(self.bc, data),
            Register::C => self.bc = write_low(self.bc, data),
            Register::C_ => self.bc_ = write_low(self.bc, data),
            Register::D => self.de = write_high(self.de, data),
            Register::D_ => self.de_ = write_high(self.de, data),
            Register::E => self.de = write_low(self.de, data),
            Register::E_ => self.de_ = write_low(self.de, data),
            Register::H => self.hl = write_high(self.hl, data),
            Register::H_ => self.hl_ = write_high(self.hl, data),
            Register::L => self.hl = write_low(self.hl, data),
            Register::L_ => self.hl_ = write_low(self.hl, data),
            Register::AF => self.af = data,
            Register::AF_ => self.af_ = data,
            Register::BC => self.bc = data,
            Register::BC_ => self.bc_ = data,
            Register::DE => self.de = data,
            Register::DE_ => self.de_ = data,
            Register::HL => self.hl = data,
            Register::HL_ => self.hl_ = data,
            Register::I => self.i = data as u8,
            Register::R => self.r = data as u8,
            Register::IX => self.ix = data,
            Register::IY => self.iy = data,
            Register::SP => self.sp = data,
        }
    }

    fn get_flag(&self, status: Status) -> bool {
        let mask = 1 << status as u16;
        self.af & mask != 0
    }

    fn set_flag(&mut self, status: Status, set: bool) {
        let mask = 1 << status as u16;
        if set {
            self.af = self.af | mask;
        } else {
            self.af = self.af & !mask;
        }
    }

    fn get_accumulator(&self) -> u16 {
        self.af >> 8
    }

    fn set_accumulator(&mut self, data: u16) {
        self.af &= 0x00FF;
        self.af |= data << 8;
    }

    fn stack_push(&mut self, data: u16, size: Size) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_memory(self.sp, data, size);
    }

    fn stack_pop(&mut self, size: Size) -> u16 {
        let data = self.read_memory(self.sp, size);
        self.sp = self.sp.wrapping_add(2);
        data
    }
}

/* Addressing modes */
impl Z80Emu {
    fn immediate_am(&mut self) -> Operand {
        let operand = self.read_pc_and_increment();
        Operand::constant_operand(operand as u16)
    }

    fn immediate_extended_am(&mut self) -> Operand {
        let low_order_bits = self.read_pc_and_increment();
        let mut data = low_order_bits as u16;

        let high_order_bits = self.read_pc_and_increment();
        data |= (high_order_bits as u16) << 8;

        Operand::constant_operand(data)
    }

    fn modified_page_zero_am(&mut self, address: u16) -> Operand {
        Operand::memory_operand(address, 0)
    }

    fn relative_am(&mut self) -> Operand {
        // addressing mode for the jump instructions, do not need to read data by the calculated offset
        let byte = self.read_pc_and_increment();
        let offset = sign_extend(byte as u32, Size::Byte) as u16;
        let address = self.pc.wrapping_add(offset);
        
        Operand::memory_operand(address, 0)
    }

    // fetched data is address of operand or address for jump instruction
    fn extended_am(&mut self) -> Operand {
        let address = self.read_memory(self.pc, Size::Word);
        self.increment_pc();
        self.increment_pc();
        
        let data = self.read_memory(address, Size::Byte);
        Operand::memory_operand(address, data)
    }

    fn indexed_am(&mut self, register: Register) -> Operand {
        let byte= if self.curr_opcode_size == 4 {
            (self.curr_opcode >> 8) as u8
        } else {
            self.read_pc_and_increment()
        };

        let register_value = self.read_register(register);
        let displacement = sign_extend(byte as u32, Size::Byte) as u16;
        let address = register_value.wrapping_add(displacement);
        let data = self.read_memory(address, self.instruction_size);
        Operand::memory_operand(address, data)
    }

    fn register_am(&mut self, register: Register) -> Operand {
        let data = self.read_register(register);
        Operand::register_operand(register, data)
    }

    fn implied_am(&mut self) -> Operand {
        Operand::constant_operand(0)
    }

    fn register_indirect_am(&mut self, register: Register) -> Operand {
        let address = self.read_register(register);
        let data = self.read_memory(address, self.instruction_size);
        Operand::memory_operand(address, data)
    }

    fn bit_am(&mut self, bit_offseet: u16) -> Operand {
        Operand::constant_operand(bit_offseet)
    }
}

/* Instruction set */
#[allow(non_snake_case)]
impl Z80Emu {
    // load data from src to dst (load 8 or 16 bits)
    fn LD(&mut self) {
        let src_data = self.src_operand.as_ref().unwrap().data;
        let dst_location = self.dst_operand.as_ref().unwrap().location;

        self.write(dst_location, src_data, self.instruction_size)
    }

    // push data on the stack
    fn PUSH(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        self.stack_push(data, self.instruction_size)
    }

    // pop data from the stack
    fn POP(&mut self) {
        let data = self.stack_pop(self.instruction_size);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data, self.instruction_size);
    }

    // exchange data between registers
    fn EX(&mut self) {
        let reg_a = self.src_operand.as_ref().unwrap().location;
        let data_a = self.src_operand.as_ref().unwrap().data;
        let reg_b = self.dst_operand.as_ref().unwrap().location;
        let data_b = self.dst_operand.as_ref().unwrap().data;

        self.write(reg_a, data_b, self.instruction_size);
        self.write(reg_b, data_a, self.instruction_size);
    }

    // exchange all 2-bytes registers between its pair
    fn EXX(&mut self) {
        std::mem::swap(&mut self.bc, &mut self.bc_);
        std::mem::swap(&mut self.de, &mut self.de_);
        std::mem::swap(&mut self.hl, &mut self.hl_);
    }

    // transfer data from memory to memory
    fn LDI(&mut self) {
        let data = self.read_memory(self.hl, Size::Byte);
        self.write_memory(self.de, data, Size::Byte);

        self.hl = self.hl.wrapping_add(1);
        self.de = self.de.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::PV, self.bc - 1 != 0);
    }

    fn LDIR(&mut self) {
        self.LDI();

        if self.bc - 1 != 0 {
            self.pc -= 2;
        }
    }

    fn LDD(&mut self) {
        let data = self.read_memory(self.hl, Size::Byte);
        self.write_memory(self.de, data, Size::Byte);

        self.hl = self.hl.wrapping_sub(1);
        self.de = self.de.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::PV, self.bc - 1 != 0);
    }

    fn LDDR(&mut self) {
        self.LDD();

        if self.bc - 1 != 0 {
            self.pc -= 2;
        }
    }

    fn CPI(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);
    }

    fn CPIR(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_add(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);

        if self.bc - 1 != 0 && res != 0 {
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    fn CPD(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);
    }

    fn CPDR(&mut self) {
        let acc = self.get_accumulator();
        let data = self.read_memory(self.hl, Size::Byte);

        let res = acc.wrapping_sub(data);

        self.hl = self.hl.wrapping_sub(1);
        self.bc = self.bc.wrapping_sub(1);

        self.set_flag(Status::S, is_negate(res as u32, Size::Byte));
        self.set_flag(Status::Z, res == 0);
        self.set_flag(Status::H, res & 0x4 != 0);
        self.set_flag(Status::PV, self.bc - 1 != 0);
        self.set_flag(Status::N, true);

        if self.bc - 1 != 0 && res != 0 {
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    fn ADD(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_add(src_operand);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn ADC(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let carry_in = self.get_flag(Status::C);
        let mut carry = 0;
        if carry_in {
            carry = 1;
        }
     
        let result = dst_operand.wrapping_add(src_operand).wrapping_add(carry);
        self.write(dst_location, result, self.instruction_size);

        
        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };
        let carry = ((result >> carry_bit_offset) & 1) != 0;
        
        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = ((dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb)) || carry_in != carry;

        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SUB(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_sub(src_operand);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SBC(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let carry_in = self.get_flag(Status::C);
        let mut carry = 0;
        if carry_in {
            carry = 1;
        }
     
        let result = dst_operand.wrapping_sub(src_operand).wrapping_sub(carry);
        self.write(dst_location, result, self.instruction_size);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn AND(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand & dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn OR(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand | dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn XOR(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = src_operand ^ dst_operand;
        self.write(dst_location, result, self.instruction_size);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, true);
        self.set_flag(Status::PV, result & 1 == 0);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, false);
    }

    fn CP(&mut self) {
        let src_operand = self.src_operand.as_ref().unwrap().data;
        let dst_operand = self.dst_operand.as_ref().unwrap().data;

        let result = dst_operand.wrapping_sub(src_operand);

        let dst_msb = get_msb(dst_operand as u32, self.instruction_size);
        let src_msb = get_msb(src_operand as u32, self.instruction_size);
        let result_msb = get_msb(result as u32, self.instruction_size);

        let overflow = (dst_msb != src_msb) && (dst_operand == 0 && result_msb == 1);

        let (carry_bit_offset, h_bit_offset) = match self.instruction_size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
            Size::Long => panic!("Z80::ADD: unsuported command size")
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, half_carry);
        self.set_flag(Status::PV, overflow);
        self.set_flag(Status::N, true);
        self.set_flag(Status::C, carry);
    }

    fn INC(&mut self) {
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_add(1);
        self.write(dst_location, dst_operand, self.instruction_size);

        match self.instruction_size {
            Size::Byte => {
                self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
                self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
                self.set_flag(Status::H, (result >> 3) & 1 == 1);
                self.set_flag(Status::PV, dst_operand == 0x7F);
                self.set_flag(Status::N, false);
            },
            Size::Word => (),
            _ => panic!("Z80::INC: unexpected instruction size"),
        }
    }

    // BCD addition
    fn DEC(&mut self) {
        let dst_operand = self.dst_operand.as_ref().unwrap().data;
        let dst_location =self.dst_operand.as_ref().unwrap().location;

        let result = dst_operand.wrapping_sub(1);
        self.write(dst_location, dst_operand, self.instruction_size);

        match self.instruction_size {
            Size::Byte => {
                self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
                self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
                self.set_flag(Status::H, (result >> 4) & 1 == 1);
                self.set_flag(Status::PV, dst_operand == 0x80);
                self.set_flag(Status::N, true);
            },
            Size::Word => (),
            _ => panic!("Z80::INC: unexpected instruction size"),
        }
    }

    // Inverts accumulator (one's complement)
    fn CPL(&mut self) {
        let result = !self.get_accumulator();
        self.set_accumulator(result);

        self.set_flag(Status::H, true);
        self.set_flag(Status::N, true);
    }

    // Inverts accumulator (two's complement)
    fn NEG(&mut self) {
        let acc = self.get_accumulator();
        let result = (!acc).wrapping_add(1);
        self.set_accumulator(result);

        self.set_flag(Status::S, is_negate(result as u32, self.instruction_size));
        self.set_flag(Status::Z, is_zero(result as u32, self.instruction_size));
        self.set_flag(Status::H, (result >> 4) & 1 == 1);
        self.set_flag(Status::PV, acc == 0x80);
        self.set_flag(Status::N, true);
        self.set_flag(Status::C, acc != 0);
    }

    // Inverts cary flag in F register
    fn CCF(&mut self) {
        let carry = self.get_flag(Status::C);
        self.set_flag(Status::H, carry);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, !carry);
    }

    // Set cary flag in F register
    fn SCF(&mut self) {
        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, true);
    }

    fn NOP(&mut self) {

    }

    fn HALT(&mut self) {
        self.pc = self.pc - 1;
    }

    // disable the maskable interrupt
    fn DI(&mut self) {

    }

    // enable interrupt sets
    fn EI(&mut self) {

    }

    // setup interrupt mode
    fn IM(&mut self) {

    }

    fn RLCA(&mut self) {
        let acc = self.get_accumulator();
        
        let msb = acc >> 7;
        let carry = msb == 1;
        
        let result = (acc << 1) | msb;
        self.set_accumulator(result);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLA(&mut self) {
        let acc = self.get_accumulator();

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let msb = acc >> 7;
        let carry = msb == 1;

        let result = (acc << 1) | poped_carry;
        self.set_accumulator(acc);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRCA(&mut self) {
        let acc = self.get_accumulator();
        
        let lsb: u16 = acc & 1;
        let carry = lsb == 1;
        
        let result = (acc >> 1) | (lsb << 7);
        self.set_accumulator(result);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRA(&mut self) {
        let acc = self.get_accumulator();

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let lsb = acc & 1;
        let carry = lsb == 1;

        let result = (acc >> 1) | (poped_carry << 7);
        self.set_accumulator(acc);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLC(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        
        let msb = data >> 7;
        let carry = msb == 1;
        
        let result = (data << 1) | msb;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RL(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let msb = data >> 7;
        let carry = msb == 1;

        let result = (data << 1) | poped_carry;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RRC(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let lsb: u16 = data & 1;
        let carry = lsb == 1;
        
        let result = (data >> 1) | (lsb << 7);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RR(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let poped_carry = match self.get_flag(Status::C) {
            true => 1,
            false => 0,
        };
        let lsb = data & 1;
        let carry = lsb == 1;

        let result = (data >> 1) | (poped_carry << 7);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SLA(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let msb = data >> 7;
        let carry = msb == 1;

        let result = data << 1;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SRA(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let msb = get_msb(data as u32, self.instruction_size);
        let lsb = data & 1;
        let carry = lsb == 1;

        let msb_offste = match self.instruction_size {
            Size::Byte => 7,
            Size::Word => 15,
            Size::Long => panic!("Z80::SRA: unexpected instruction size")
        };

        let result = (data >> 1) | ((msb as u16) << msb_offste);
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn SRL(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;

        let lsb = data & 1;
        let carry = lsb == 1;

        let result = data >> 1;
        
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);

        self.set_flag(Status::H, false);
        self.set_flag(Status::N, false);
        self.set_flag(Status::C, carry);
    }

    fn RLD(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let acc = self.get_accumulator();
        
        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc & 0x0F;
        
        let data_res = (data_low_bits << 4) | acc_low_bits;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data_res, self.instruction_size);

        let acc_res = (acc & 0xF0) | data_high_bits;
        self.set_accumulator(acc_res);

        self.set_flag(Status::S, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::N, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::H, false);
        self.set_flag(Status::PV, acc_res % 2 == 0);
        self.set_flag(Status::N, false);
    }

    fn RRD(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let acc = self.get_accumulator();
        
        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc & 0x0F;
        
        let data_res = (acc_low_bits << 4) | data_high_bits;

        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, data_res, self.instruction_size);

        let acc_res = (acc & 0xF0) | data_low_bits;
        self.set_accumulator(acc_res);

        self.set_flag(Status::S, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::N, is_negate(acc_res as u32, Size::Byte));
        self.set_flag(Status::H, false);
        self.set_flag(Status::PV, acc_res % 2 == 0);
        self.set_flag(Status::N, false);
    }

    fn BIT(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data & (1 << bit_offset) == 0;

        self.set_flag(Status::Z, result);
        self.set_flag(Status::H, true);
        self.set_flag(Status::N, false);
    }

    fn SET(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data | (1 << bit_offset);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);
    }

    fn RES(&mut self) {
        let data = self.dst_operand.as_ref().unwrap().data;
        let bit_offset = self.src_operand.as_ref().unwrap().data;

        let result = data & !(1 << bit_offset);
        let location = self.dst_operand.as_ref().unwrap().location;
        self.write(location, result, self.instruction_size);
    }

    fn JP(&mut self) {
        let condition = match self.curr_opcode {
            0xC3 => true,
            0xC2 => !self.get_flag(Status::Z),
            0xCA => self.get_flag(Status::Z),
            0xD2 => !self.get_flag(Status::C),
            0xDA => self.get_flag(Status::C),
            0xE2 => !self.get_flag(Status::PV),
            0xEA => self.get_flag(Status::PV),
            0xF2 => !self.get_flag(Status::S),
            0xFA => self.get_flag(Status::S),
            _ => panic!("Z80::JP: unexpected bit pattern for condition determination")
        };
        if condition {
            let address = self.dst_operand.as_ref().unwrap().data;
            self.pc = address;
        }
    }

    fn JP_ind(&mut self) {
        let address = self.dst_operand.as_ref().unwrap().data;
        self.pc = address;
    }

    fn JR(&mut self) {
        let condition = match self.curr_opcode {
            0x18 => true, // unconditional branching
            0x38 => self.get_flag(Status::C),
            0x30 => !self.get_flag(Status::C),
            0x28 => self.get_flag(Status::Z),
            0x20 => !self.get_flag(Status::Z),
            _ => panic!("Z80::JR: unsupported opcode byte for condition selecting"),
        };

        if condition {
            let address_ofset = self.dst_operand.as_ref().unwrap().data;
            self.pc = self.pc.wrapping_add(address_ofset);
        }
    }

    fn DJNZ(&mut self) {
        let mut reg_b = self.read_register(Register::B);
        reg_b = reg_b.wrapping_sub(1);
        self.write_register(Register::B, reg_b, Size::Byte);

        if reg_b != 0 {
            let address_offset = self.dst_operand.as_ref().unwrap().data;
            self.pc = self.pc.wrapping_add(address_offset);
        }
    }

    fn CALL(&mut self) {
        let condition = match self.curr_opcode {
            0xCD => true,
            0xC4 => !self.get_flag(Status::Z),
            0xCC => self.get_flag(Status::Z),
            0xD4 => !self.get_flag(Status::C),
            0xDC => self.get_flag(Status::C),
            0xE4 => !self.get_flag(Status::PV),
            0xEC => self.get_flag(Status::PV),
            0xF4 => !self.get_flag(Status::S),
            0xFC => self.get_flag(Status::S),
            _ => panic!("Z80::CALL: unexpected bit pattern for condition determination")
        };

        if condition {
            self.stack_push(self.pc, Size::Word);
            
            let address = self.dst_operand.as_ref().unwrap().data;
            self.pc = address;
        }
    }

    fn RET(&mut self) {
        let condition = match self.curr_opcode {
            0xC9 => true,
            0xC0 => !self.get_flag(Status::Z),
            0xC8 => self.get_flag(Status::Z),
            0xD0 => !self.get_flag(Status::C),
            0xD8 => self.get_flag(Status::C),
            0xE0 => !self.get_flag(Status::PV),
            0xE8 => self.get_flag(Status::PV),
            0xF0 => !self.get_flag(Status::S),
            0xF8 => self.get_flag(Status::S),
            _ => panic!("Z80::RET: unexpected bit pattern for condition determination")
        };

        if condition {
            let address = self.stack_pop(Size::Word);
            self.pc = address;
        }
    }

    fn RETI(&mut self) {
        let address = self.stack_pop(Size::Word);
        self.pc = address;

        // maybe there need to pop the active interrupt
    }

    fn RETN(&mut self) {
        let address = self.stack_pop(Size::Word);
        self.pc = address;
        self.iff1 = self.iff2;
    }

    fn RST(&mut self) {
        let address = match self.curr_opcode {
            0xC7 => 0x00,
            0xCF => 0x08,
            0xD7 => 0x10,
            0xDF => 0x18,
            0xE7 => 0x20,
            0xEF => 0x28,
            0xF7 => 0x30,
            0xFF => 0x38,
            _ => panic!("Z80::RST: unexpected bit pattern for the zero page address determination")
        };

        self.stack_push(self.pc, Size::Word);
        self.pc = address;
    }

    fn IN(&mut self) {

    }

    fn INI(&mut self) {

    }

    fn INIR(&mut self) {

    }

    fn IND(&mut self) {

    }

    fn INDR(&mut self) {

    }

    fn OUT(&mut self) {

    }

    fn OUTI(&mut self) {

    }

    fn OTIR(&mut self) {

    }

    fn OUTD(&mut self) {

    }

    fn OTDR(&mut self) {

    }

    fn DAA(&mut self) {

    }

    fn XEP(&mut self) {
        println!("Z80::XEP: cpu fetched XEP function")
    }
}

#[cfg(test)]
mod tests {
    use hardware::Size;

    use crate::Z80Bus;

    use super::Z80Emu;

    struct TestBus {
        memory: [u8; 10],
    }

    impl Z80Bus for TestBus {
        fn read(&self, address: u16, size: Size) -> u16 {
            self.memory[address as usize] as u16
        }

        fn write(&mut self, address: u16, data: u16, size: Size) {
            self.memory[address as usize] = data as u8;
        }
    }

    #[test]
    fn call_first_table_of_opcodes() {
        let mut bus = TestBus{ memory: [0; 10]};
        let mut  z80 = Z80Emu::new(&mut bus);
        for op in 0..=0xFF {
            bus.memory[0] = op;
            println!("test opcode: 0x{:02X}", op);
            z80.pc = 0;
            z80.sp = 9;
            z80.clock();
        }
    }

    #[test]
    fn call_one_extended_tablea_of_opcodes() {
        let mut bus = TestBus{ memory: [0; 10]};
        let mut  z80 = Z80Emu::new(&mut bus);
        
        for e in [0xCB, 0xDD, 0xED, 0xFD] {
            bus.memory[0] = e;
            for op in 0..=0xFF {
                bus.memory[1] = op;
                println!("test opcode: 0x{:02X}{:02X}", e, op);
                z80.pc = 0;
                z80.sp = 9;
                z80.clock();
            }
        }
    }

    #[test]
    fn call_two_extended_tablea_of_opcodes() {
        let mut bus = TestBus{ memory: [0; 10]};
        let mut  z80 = Z80Emu::new(&mut bus);
        
        for e in [0xDDCB, 0xFDCB] {
            bus.memory[0] = (e >> 8) as u8;
            bus.memory[1] = e as u8;
            for op in 0..=0xFF {
                bus.memory[2] = op;
                println!("test opcode: 0x{:04X}{:02X}", e, op);
                z80.pc = 0;
                z80.sp = 9;
                z80.clock();
            }
        }
    }
}
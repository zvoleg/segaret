use super::Z80Bus;

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

    //internal
    operand: u16,

    bus: *mut dyn Z80Bus,
}

impl Z80Emu {
    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    fn read_memory(&mut self, address: u16) -> u8 {
        unsafe {
            (*self.bus).read(address)
        }
    }

    fn write_memory(&mut self, address: u16, data: u8) {
        unsafe {
            (*self.bus).write(address, data);
        }
    }
}

/* Addressing modes */
impl Z80Emu {
    fn immediate_am(&mut self) {
        let operand = self.read_memory(self.pc);
        self.operand = operand as u16;
        self.increment_pc();
    }

    fn immediate_extended_am(&mut self) {
        let high_order_bits = self.read_memory(self.pc);
        self.operand = (high_order_bits as u16) << 8;
        self.increment_pc();

        let low_order_bits = self.read_memory(self.pc);
        self.operand |= low_order_bits as u16;
        self.increment_pc();
    }

    fn modified_page_zero_am(&mut self) {

    }

    fn relative_am(&mut self) {

    }

    fn extended_am(&mut self) {

    }

    fn indexed_am(&mut self) {

    }

    fn register_am(&mut self) {

    }

    fn implied_am(&mut self) {

    }

    fn register_indirect_am(&mut self) {

    }

    fn bit_am(&mut self) {

    }
}

/* Instruction set */
#[allow(non_snake_case)]
impl Z80Emu {
    // load data from src to dst (load 8 or 16 bits)
    fn LD(&mut self) {
        // src value u8
        // destination pointer u8
    }

    // push data on the stack
    fn PUSH(&mut self) {
        // at first decrements sp
        // save high-order bits of selected register
        // decrements sp again
        // save low-order bits of selected register
    }

    // pop data from the stack
    fn POP(&mut self) {
        // save data from stack to low-order bits of selected register
        // increment sp
        // save data from stack to high-order bits of selected register
    }

    // exchange data between registers
    fn EX(&mut self) {

    }

    // exchange all 2-bytes registers between its pair
    fn EXX(&mut self) {

    }

    // transfer data from memory to memory
    fn LDI(&mut self) {

    }

    fn LDIR(&mut self) {

    }

    fn LDD(&mut self) {

    }

    fn LDDR(&mut self) {

    }

    fn CPI(&mut self) {

    }

    fn CPIR(&mut self) {

    }

    fn CPD(&mut self) {

    }

    fn CPDR(&mut self) {

    }

    fn ADD(&mut self) {

    }

    fn ADC(&mut self) {

    }

    fn SUB(&mut self) {

    }

    fn SBC(&mut self) {

    }

    fn AND(&mut self) {

    }

    fn OR(&mut self) {

    }

    fn XOR(&mut self) {

    }

    fn CP(&mut self) {

    }

    fn INC(&mut self) {

    }

    // BCD addition
    fn DEC(&mut self) {

    }

    // Inverts accumulator (one's complement)
    fn CPL(&mut self) {

    }

    // Inverts accumulator (two's complement)
    fn NEG(&mut self) {

    }

    // Inverts cary flag in F register
    fn CCF(&mut self) {

    }

    // Set cary flag in F register
    fn SCF(&mut self) {

    }

    fn NOP(&mut self) {

    }

    fn HALT(&mut self) {

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

    }

    fn RLA(&mut self) {

    }

    fn RRCA(&mut self) {

    }

    fn RRA(&mut self) {

    }

    fn RLC(&mut self) {

    }

    fn RL(&mut self) {

    }

    fn RRC(&mut self) {

    }

    fn RR(&mut self) {

    }

    fn SLA(&mut self) {

    }

    fn SRA(&mut self) {

    }

    fn SRL(&mut self) {

    }

    fn RLD(&mut self) {

    }

    fn RRD(&mut self) {

    }

    fn BIT(&mut self) {

    }

    fn SET(&mut self) {

    }

    fn RES(&mut self) {

    }

    fn JP(&mut self) {

    }

    fn DJNZ(&mut self) {

    }

    fn CALL(&mut self) {

    }

    fn RET(&mut self) {

    }

    fn RETI(&mut self) {

    }

    fn RETN(&mut self) {

    }

    fn RST(&mut self) {

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
} 
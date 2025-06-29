use crate::{
    bus::BusZ80,
    cpu::Z80,
    primitives::Operand,
    register_set::{Register, RegisterType, Status},
    IsNegate, MostSignificantBit, SignExtending, Size,
};

pub(crate) trait Instruction<T>
where
    T: BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>);
}

pub(crate) enum Condition {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
    UNC,
}

// load data from src to dst (load 8 or 16 bits)
pub(crate) struct LD();

impl<T> Instruction<T> for LD
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, operands: Vec<Operand>) {
        let src_ptr = &operands[0];
        let dst_ptr = &operands[1];

        dst_ptr.write(src_ptr.read().unwrap()).unwrap();
    }
}

pub(crate) struct PUSH();

impl<T> Instruction<T> for PUSH
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_ptr = &operands[0];
        let data = src_ptr.read().unwrap();
        cpu.push(data, src_ptr.size).unwrap();
    }
}

pub(crate) struct POP();

impl<T> Instruction<T> for POP
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let dst_ptr = &operands[0];
        let data = cpu.pop(dst_ptr.size).unwrap();
        dst_ptr.write(data).unwrap();
    }
}

// // exchange data between registers
pub(crate) struct EX();

impl<T> Instruction<T> for EX
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        src_operand.write(dst_data).unwrap();
        dst_operand.write(src_data).unwrap();
    }
}

pub(crate) struct EXX();

impl<T> Instruction<T> for EXX
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        cpu.register_set.exchange_general_registers();
    }
}

pub(crate) struct LDI();

impl<T> Instruction<T> for LDI
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        // transfer data from memory to memory
        let data = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Byte);
        cpu.register_set
            .write_register(data, Register::General(RegisterType::DE), Size::Byte);

        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        cpu.register_set.write_register(
            hl.wrapping_add(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );

        let de = cpu
            .register_set
            .read_register(Register::General(RegisterType::DE), Size::Word);
        cpu.register_set.write_register(
            de.wrapping_add(1),
            Register::General(RegisterType::DE),
            Size::Word,
        );

        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::PV, bc != 0);
    }
}

pub(crate) struct LDIR();

impl<T> Instruction<T> for LDIR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        LDI().execute(cpu, operands);

        let bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        if bc.wrapping_sub(1) != 0 {
            cpu.program_counter = cpu.program_counter.wrapping_sub(2);
        }
    }
}

pub(crate) struct LDD();

impl<T> Instruction<T> for LDD
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let data = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Byte);
        cpu.register_set
            .write_register(data, Register::General(RegisterType::DE), Size::Byte);

        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        cpu.register_set.write_register(
            hl.wrapping_sub(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );

        let de = cpu
            .register_set
            .read_register(Register::General(RegisterType::DE), Size::Word);
        cpu.register_set.write_register(
            de.wrapping_sub(1),
            Register::General(RegisterType::DE),
            Size::Word,
        );

        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::PV, bc != 0);
    }
}

pub(crate) struct LDDR();

impl<T> Instruction<T> for LDDR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        LDD().execute(cpu, operands);

        let bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        if bc.wrapping_sub(1) != 0 {
            cpu.program_counter = cpu.program_counter.wrapping_sub(2);
        }
    }
}

pub(crate) struct CPI();

impl<T> Instruction<T> for CPI
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = cpu
            .register_set
            .read_register(Register::General(RegisterType::A), Size::Byte);
        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        let data = cpu.bus_share().read(hl, Size::Byte).unwrap();

        let res = acc.wrapping_sub(data);

        cpu.register_set.write_register(
            hl.wrapping_add(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );
        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set
            .set_flag(Status::S, res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, res == 0);
        cpu.register_set.set_flag(Status::H, res & 0x4 != 0);
        cpu.register_set.set_flag(Status::PV, bc != 0);
        cpu.register_set.set_flag(Status::N, true);
    }
}

pub(crate) struct CPIR();

impl<T> Instruction<T> for CPIR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = cpu
            .register_set
            .read_register(Register::General(RegisterType::A), Size::Byte);
        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        let data = cpu.bus_share().read(hl, Size::Byte).unwrap();

        let res = acc.wrapping_sub(data);

        cpu.register_set.write_register(
            hl.wrapping_add(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );
        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set
            .set_flag(Status::S, res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, res == 0);
        cpu.register_set.set_flag(Status::H, res & 0x4 != 0);
        cpu.register_set.set_flag(Status::PV, bc != 0);
        cpu.register_set.set_flag(Status::N, true);

        if bc != 0 && res != 0 {
            cpu.program_counter = cpu.program_counter.wrapping_sub(2);
        }
    }
}

pub(crate) struct CPD();

impl<T> Instruction<T> for CPD
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = cpu
            .register_set
            .read_register(Register::General(RegisterType::A), Size::Byte);
        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        let data = cpu.bus_share().read(hl, Size::Byte).unwrap();

        let res = acc.wrapping_sub(data);

        cpu.register_set.write_register(
            hl.wrapping_sub(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );
        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set
            .set_flag(Status::S, res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, res == 0);
        cpu.register_set.set_flag(Status::H, res & 0x4 != 0);
        cpu.register_set.set_flag(Status::PV, bc != 0);
        cpu.register_set.set_flag(Status::N, true);
    }
}

pub(crate) struct CPDR();

impl<T> Instruction<T> for CPDR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = cpu
            .register_set
            .read_register(Register::General(RegisterType::A), Size::Byte);
        let hl = cpu
            .register_set
            .read_register(Register::General(RegisterType::HL), Size::Word);
        let data = cpu.bus_share().read(hl, Size::Byte).unwrap();

        let res = acc.wrapping_sub(data);

        cpu.register_set.write_register(
            hl.wrapping_sub(1),
            Register::General(RegisterType::HL),
            Size::Word,
        );
        let mut bc = cpu
            .register_set
            .read_register(Register::General(RegisterType::BC), Size::Word);
        bc = bc.wrapping_sub(1);
        cpu.register_set
            .write_register(bc, Register::General(RegisterType::BC), Size::Word);

        cpu.register_set
            .set_flag(Status::S, res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, res == 0);
        cpu.register_set.set_flag(Status::H, res & 0x4 != 0);
        cpu.register_set.set_flag(Status::PV, bc != 0);
        cpu.register_set.set_flag(Status::N, true);

        if bc != 0 && res != 0 {
            cpu.program_counter = cpu.program_counter.wrapping_sub(2);
        }
    }
}

pub(crate) struct ADD();

impl<T> Instruction<T> for ADD
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = dst_data.wrapping_add(src_data);
        dst_operand.write(result).unwrap();

        let size = dst_operand.size;
        let dst_msb = dst_data.get_msb(size);
        let src_msb = src_data.get_msb(size);
        let result_msb = result.get_msb(size);

        let overflow = (dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb);

        let (carry_bit_offset, h_bit_offset) = match size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        cpu.register_set.set_flag(Status::S, result.is_negate(size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, half_carry);
        cpu.register_set.set_flag(Status::PV, overflow);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct ADC();

impl<T> Instruction<T> for ADC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let carry_set = cpu.register_set.get_flag(Status::C);
        let carry = if carry_set { 1 } else { 0 };

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = dst_data.wrapping_add(src_data).wrapping_add(carry);
        dst_operand.write(dst_data).unwrap();

        let size = dst_operand.size;
        let dst_msb = dst_data.get_msb(size);
        let src_msb = src_data.get_msb(size);
        let result_msb = result.get_msb(size);

        let overflow = (dst_msb == src_msb) && (dst_msb != result_msb || src_msb != result_msb);

        let (carry_bit_offset, h_bit_offset) = match size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        cpu.register_set.set_flag(Status::S, result.is_negate(size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, half_carry);
        cpu.register_set.set_flag(Status::PV, overflow);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct SUB();

impl<T> Instruction<T> for SUB
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = dst_data.wrapping_sub(dst_data);
        dst_operand.write(result).unwrap();

        let size = dst_operand.size;
        let dst_msb = dst_data.get_msb(size);
        let src_msb = src_data.get_msb(size);
        let result_msb = result.get_msb(size);

        let overflow = (dst_msb != src_msb) && (dst_data == 0 && result_msb);

        let (carry_bit_offset, h_bit_offset) = match size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        cpu.register_set.set_flag(Status::S, result.is_negate(size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, half_carry);
        cpu.register_set.set_flag(Status::PV, overflow);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct SBC();

impl<T> Instruction<T> for SBC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let carry_set = cpu.register_set.get_flag(Status::C);
        let carry = if carry_set { 1 } else { 0 };

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = dst_data.wrapping_sub(src_data).wrapping_sub(carry);
        dst_operand.write(result).unwrap();

        let size = dst_operand.size;
        let dst_msb = dst_data.get_msb(size);
        let src_msb = src_data.get_msb(size);
        let result_msb = result.get_msb(size);

        let overflow = (dst_msb != src_msb) && (dst_data == 0 && result_msb);

        let (carry_bit_offset, h_bit_offset) = match size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        cpu.register_set.set_flag(Status::S, result.is_negate(size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, half_carry);
        cpu.register_set.set_flag(Status::PV, overflow);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct AND();

impl<T> Instruction<T> for AND
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = src_data & dst_data;
        dst_operand.write(result).unwrap();

        cpu.register_set
            .set_flag(Status::S, result.is_negate(dst_operand.size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, true);
        cpu.register_set.set_flag(Status::PV, result & 1 == 0);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, false);
    }
}

pub(crate) struct OR();

impl<T> Instruction<T> for OR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = src_data | dst_data;
        dst_operand.write(result).unwrap();

        cpu.register_set
            .set_flag(Status::S, result.is_negate(dst_operand.size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, true);
        cpu.register_set.set_flag(Status::PV, result & 1 == 0);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, false);
    }
}

pub(crate) struct XOR();

impl<T> Instruction<T> for XOR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = src_data ^ dst_data;
        dst_operand.write(result).unwrap();

        cpu.register_set
            .set_flag(Status::S, result.is_negate(dst_operand.size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, true);
        cpu.register_set.set_flag(Status::PV, result & 1 == 0);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, false);
    }
}

pub(crate) struct CP();

impl<T> Instruction<T> for CP
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let dst_operand = &operands[1];

        let src_data = src_operand.read().unwrap();
        let dst_data = dst_operand.read().unwrap();

        let result = dst_data.wrapping_sub(src_data);

        let size = dst_operand.size;
        let dst_msb = dst_data.get_msb(size);
        let src_msb = src_data.get_msb(size);
        let result_msb = result.get_msb(size);

        let overflow = (dst_msb != src_msb) && (dst_data == 0 && result_msb);

        let (carry_bit_offset, h_bit_offset) = match size {
            Size::Byte => (7, 3),
            Size::Word => (15, 11),
        };

        let carry = ((result >> carry_bit_offset) & 1) != 0;
        let half_carry = ((result >> h_bit_offset) & 1) != 0;

        cpu.register_set.set_flag(Status::S, result.is_negate(size));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, half_carry);
        cpu.register_set.set_flag(Status::PV, overflow);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct INC();

impl<T> Instruction<T> for INC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let result = data.wrapping_add(1);
        operand.write(result).unwrap();

        match operand.size {
            Size::Byte => {
                cpu.register_set
                    .set_flag(Status::S, result.is_negate(operand.size));
                cpu.register_set.set_flag(Status::Z, result == 0);
                cpu.register_set.set_flag(Status::H, (result >> 3) & 1 == 1);
                cpu.register_set.set_flag(Status::PV, data == 0x7F);
                cpu.register_set.set_flag(Status::N, false);
            }
            Size::Word => panic!("Z80::INC: unexpected instruction size"),
        }
    }
}

pub(crate) struct DEC();

impl<T> Instruction<T> for DEC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let result = data.wrapping_sub(1);
        operand.write(result).unwrap();

        match operand.size {
            Size::Byte => {
                cpu.register_set
                    .set_flag(Status::S, result.is_negate(operand.size));
                cpu.register_set.set_flag(Status::Z, result == 0);
                cpu.register_set.set_flag(Status::H, (result >> 3) & 1 == 1);
                cpu.register_set.set_flag(Status::PV, data == 0x80);
                cpu.register_set.set_flag(Status::N, false);
            }
            Size::Word => panic!("Z80::DEC: unexpected instruction size"),
        }
    }
}

pub(crate) struct DAA();

impl<T> Instruction<T> for DAA
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        // This instruction conditionally adjusts the Accumulator
        // for BCD addition and subtraction operations
        ()
    }
}

pub(crate) struct CPL();

impl<T> Instruction<T> for CPL
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let result = !cpu.register_set.read_register(acc, Size::Byte);
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set.set_flag(Status::H, true);
        cpu.register_set.set_flag(Status::N, true);
    }
}

pub(crate) struct NEG();

impl<T> Instruction<T> for NEG
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let data = cpu.register_set.read_register(acc, Size::Byte);
        let result = (!data).wrapping_add(1);
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set
            .set_flag(Status::S, result.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, result == 0);
        cpu.register_set.set_flag(Status::H, (result >> 4) & 1 == 1);
        cpu.register_set.set_flag(Status::PV, data == 0x80);
        cpu.register_set.set_flag(Status::N, true);
        cpu.register_set.set_flag(Status::C, data != 0);
    }
}

pub(crate) struct CCF();

impl<T> Instruction<T> for CCF
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let carry = cpu.register_set.get_flag(Status::C);
        cpu.register_set.set_flag(Status::H, carry);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, !carry);
    }
}

pub(crate) struct SCF();

impl<T> Instruction<T> for SCF
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, true);
    }
}

pub(crate) struct NOP();

impl<T> Instruction<T> for NOP
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        ()
    }
}

pub(crate) struct HALT();

impl<T> Instruction<T> for HALT
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        cpu.program_counter = cpu.program_counter - 1;
    }
}

// // disable the maskable interrupt
pub(crate) struct DI();

impl<T> Instruction<T> for DI
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        todo!()
    }
}

// // enable interrupt sets
pub(crate) struct EI();

impl<T> Instruction<T> for EI
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        todo!()
    }
}

// // setup interrupt mode
pub(crate) struct IM();

impl<T> Instruction<T> for IM
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        todo!()
    }
}

pub(crate) struct RLCA();

impl<T> Instruction<T> for RLCA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let data = cpu.register_set.read_register(acc, Size::Byte);

        let poped_bit = if data.get_msb(Size::Byte) { 1 } else { 0 };
        let carry = poped_bit == 1;

        let result = (data << 1) | poped_bit;
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RLA();

impl<T> Instruction<T> for RLA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let data = cpu.register_set.read_register(acc, Size::Byte);

        let cary_bit = if cpu.register_set.get_flag(Status::C) {
            1
        } else {
            0
        };
        let carry = data.get_msb(Size::Byte);

        let result = (data << 1) | cary_bit;
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RRCA();

impl<T> Instruction<T> for RRCA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let data = cpu.register_set.read_register(acc, Size::Byte);

        let lsb: u16 = data & 1;
        let carry = lsb == 1;

        let result = (data >> 1) | (lsb << 7);
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RRA();

impl<T> Instruction<T> for RRA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let acc = Register::General(RegisterType::A);
        let data = cpu.register_set.read_register(acc, Size::Byte);

        let cary_bit = if cpu.register_set.get_flag(Status::C) {
            1
        } else {
            0
        };
        let lsb = data & 1;
        let carry = lsb == 1;

        let result = (data >> 1) | (cary_bit << 7);
        cpu.register_set.write_register(result, acc, Size::Byte);

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RLC();

impl<T> Instruction<T> for RLC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let msb = if data.get_msb(Size::Byte) { 1 } else { 0 };
        let carry = msb == 1;

        let result = (data << 1) | msb;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RL();

impl<T> Instruction<T> for RL
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let cary_bit = if cpu.register_set.get_flag(Status::C) {
            1
        } else {
            0
        };
        let msb = if data.get_msb(Size::Byte) { 1 } else { 0 };
        let carry = msb == 1;

        let result = (data << 1) | cary_bit;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RRC();

impl<T> Instruction<T> for RRC
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let lsb = data & 1;
        let carry = lsb == 1;

        let result = (data >> 1) | lsb << 7;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RR();

impl<T> Instruction<T> for RR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let cary_bit = if cpu.register_set.get_flag(Status::C) {
            1
        } else {
            0
        };
        let carry = data & 1 == 1;

        let result = (data >> 1) | cary_bit << 7;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct SLA();

impl<T> Instruction<T> for SLA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let msb = if data.get_msb(Size::Byte) { 1 } else { 0 };
        let carry = msb == 1;

        let result = data << 1;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct SRA();

impl<T> Instruction<T> for SRA
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let msb = if data.get_msb(Size::Byte) { 1 } else { 0 };
        let carry = data & 1 == 1;

        let result = data >> 1 | msb << 7;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct SRL();

impl<T> Instruction<T> for SRL
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let carry = data & 1 == 1;

        let result = data >> 1;
        operand.write(result).unwrap();

        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::N, false);
        cpu.register_set.set_flag(Status::C, carry);
    }
}

pub(crate) struct RLD();

impl<T> Instruction<T> for RLD
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let acc = Register::General(RegisterType::A);
        let acc_data = cpu.register_set.read_register(acc, Size::Byte);

        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc_data & 0x0F;

        let data_res = (data_low_bits << 4) | acc_low_bits;

        operand.write(data_res).unwrap();

        let acc_res = (acc_data & 0xF0) | data_high_bits;
        cpu.register_set.write_register(acc_res, acc, Size::Byte);

        cpu.register_set
            .set_flag(Status::S, acc_res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, acc_res == 0);
        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::PV, acc_res % 2 == 0);
        cpu.register_set.set_flag(Status::N, false);
    }
}

pub(crate) struct RRD();

impl<T> Instruction<T> for RRD
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let operand = &operands[0];
        let data = operand.read().unwrap();

        let acc = Register::General(RegisterType::A);
        let acc_data = cpu.register_set.read_register(acc, Size::Byte);

        let data_low_bits = data & 0x0F;
        let data_high_bits = (data & 0xF0) >> 4;
        let acc_low_bits = acc_data & 0x0F;

        let data_res = (acc_low_bits << 4) | data_high_bits;

        operand.write(data_res).unwrap();

        let acc_res = (acc_data & 0xF0) | data_low_bits;
        cpu.register_set.write_register(acc_res, acc, Size::Byte);

        cpu.register_set
            .set_flag(Status::S, acc_res.is_negate(Size::Byte));
        cpu.register_set.set_flag(Status::Z, acc_res == 0);
        cpu.register_set.set_flag(Status::H, false);
        cpu.register_set.set_flag(Status::PV, acc_res % 2 == 0);
        cpu.register_set.set_flag(Status::N, false);
    }
}

pub(crate) struct BIT();

impl<T> Instruction<T> for BIT
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let bit_offset = src_operand.read().unwrap();

        let dst_operand = &operands[1];
        let data = dst_operand.read().unwrap();

        let result = data & (1 << bit_offset) == 0;

        cpu.register_set.set_flag(Status::Z, result);
        cpu.register_set.set_flag(Status::H, true);
        cpu.register_set.set_flag(Status::N, false);
    }
}

pub(crate) struct SET();

impl<T> Instruction<T> for SET
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let bit_offset = src_operand.read().unwrap();

        let dst_operand = &operands[1];
        let data = dst_operand.read().unwrap();

        let result = data | (1 << bit_offset);
        dst_operand.write(result).unwrap();
    }
}

pub(crate) struct RES();

impl<T> Instruction<T> for RES
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, operands: Vec<Operand>) {
        let src_operand = &operands[0];
        let bit_offset = src_operand.read().unwrap();

        let dst_operand = &operands[1];
        let data = dst_operand.read().unwrap();

        let result = data & !(1 << bit_offset);
        dst_operand.write(result).unwrap();
    }
}

pub(crate) struct JP {
    condition: Condition,
}

impl JP {
    pub(crate) fn new(condition: Condition) -> Self {
        Self { condition }
    }
}

impl<T> Instruction<T> for JP
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let condition_test = match self.condition {
            Condition::NZ => !cpu.register_set.get_flag(Status::Z),
            Condition::Z => cpu.register_set.get_flag(Status::Z),
            Condition::NC => !cpu.register_set.get_flag(Status::C),
            Condition::C => cpu.register_set.get_flag(Status::C),
            Condition::PO => !cpu.register_set.get_flag(Status::PV),
            Condition::PE => cpu.register_set.get_flag(Status::PV),
            Condition::P => !cpu.register_set.get_flag(Status::S),
            Condition::M => cpu.register_set.get_flag(Status::S),
            Condition::UNC => true,
        };
        if condition_test {
            let oprand = &operands[0];
            let address = oprand.read().unwrap();
            cpu.program_counter = address;
        }
    }
}

pub(crate) struct JR {
    condition: Condition,
}

impl JR {
    pub(crate) fn new(condition: Condition) -> Self {
        Self { condition }
    }
}

impl<T> Instruction<T> for JR
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let condition_test = match self.condition {
            Condition::NZ => !cpu.register_set.get_flag(Status::Z),
            Condition::Z => cpu.register_set.get_flag(Status::Z),
            Condition::NC => !cpu.register_set.get_flag(Status::C),
            Condition::C => cpu.register_set.get_flag(Status::C),
            Condition::UNC => true,
            _ => panic!("Z80::instruction_set::JR: unexpected condition"),
        };
        if condition_test {
            let operand = &operands[0];
            let offset = operand.read().unwrap().sign_extend(Size::Byte);
            cpu.program_counter = cpu.program_counter.wrapping_add(offset);
        }
    }
}

pub(crate) struct DJNZ();

impl<T> Instruction<T> for DJNZ
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let mut reg_b = cpu
            .register_set
            .read_register(Register::General(RegisterType::B), Size::Byte);
        reg_b = reg_b.wrapping_sub(1);
        cpu.register_set
            .write_register(reg_b, Register::General(RegisterType::B), Size::Byte);

        if reg_b != 0 {
            let operand = &operands[0];
            let offset = operand.read().unwrap().sign_extend(Size::Byte);
            cpu.program_counter = cpu.program_counter.wrapping_add(offset);
        }
    }
}

pub(crate) struct CALL {
    condition: Condition,
}

impl CALL {
    pub(crate) fn new(condition: Condition) -> Self {
        Self { condition }
    }
}

impl<T> Instruction<T> for CALL
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, operands: Vec<Operand>) {
        let condition_test = match self.condition {
            Condition::NZ => !cpu.register_set.get_flag(Status::Z),
            Condition::Z => cpu.register_set.get_flag(Status::Z),
            Condition::NC => !cpu.register_set.get_flag(Status::C),
            Condition::C => cpu.register_set.get_flag(Status::C),
            Condition::PO => !cpu.register_set.get_flag(Status::PV),
            Condition::PE => cpu.register_set.get_flag(Status::PV),
            Condition::P => !cpu.register_set.get_flag(Status::S),
            Condition::M => cpu.register_set.get_flag(Status::S),
            Condition::UNC => true,
        };
        if condition_test {
            let program_counter = cpu.program_counter;
            cpu.push(program_counter, Size::Word).unwrap();

            let operand = &operands[0];
            let address = operand.read().unwrap();
            cpu.program_counter = address;
        }
    }
}

pub(crate) struct RET {
    condition: Condition,
}

impl RET {
    pub(crate) fn new(condition: Condition) -> Self {
        Self {condition}
    }
}

impl<T> Instruction<T> for RET
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let condition_test = match self.condition {
            Condition::NZ => !cpu.register_set.get_flag(Status::Z),
            Condition::Z => cpu.register_set.get_flag(Status::Z),
            Condition::NC => !cpu.register_set.get_flag(Status::C),
            Condition::C => cpu.register_set.get_flag(Status::C),
            Condition::PO => !cpu.register_set.get_flag(Status::PV),
            Condition::PE => cpu.register_set.get_flag(Status::PV),
            Condition::P => !cpu.register_set.get_flag(Status::S),
            Condition::M => cpu.register_set.get_flag(Status::S),
            Condition::UNC => true,
        };
        if condition_test {
            let address = cpu.pop(Size::Word).unwrap();
            cpu.program_counter = address;
        }
    }
}

pub(crate) struct RETI {
    condition: Condition,
}

impl<T> Instruction<T> for RETI
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let address = cpu.pop(Size::Word).unwrap();
        cpu.program_counter = address;

        // TODO set an 'interrupt complete flag'
    }
}

pub(crate) struct RETN {
    condition: Condition,
}

impl<T> Instruction<T> for RETN
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let address = cpu.pop(Size::Word).unwrap();
        cpu.program_counter = address;

        // TODO set an 'self.iff1 = self.iff2;'
    }
}

pub(crate) struct RST {
    address: u16,
}

impl RST {
    pub(crate) fn new(address: u16) -> Self {
        Self { address }
    }
}

impl<T> Instruction<T> for RST
where
    T: 'static + BusZ80,
{
    fn execute(&self, cpu: &mut Z80<T>, _: Vec<Operand>) {
        let address = cpu.program_counter;
        cpu.push(address, Size::Word).unwrap();

        cpu.program_counter = self.address;
    }
}

pub(crate) struct IN();

impl<T> Instruction<T> for IN
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct INI();

impl<T> Instruction<T> for INI
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct INIR();

impl<T> Instruction<T> for INIR
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct IND();

impl<T> Instruction<T> for IND
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct INDR();

impl<T> Instruction<T> for INDR
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct OUT();

impl<T> Instruction<T> for OUT
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct OUTI();

impl<T> Instruction<T> for OUTI
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct OTIR();

impl<T> Instruction<T> for OTIR
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct OUTD();

impl<T> Instruction<T> for OUTD
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct OTDR();

impl<T> Instruction<T> for OTDR
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
    }
}

pub(crate) struct XEP();

impl<T> Instruction<T> for XEP
where
    T: 'static + BusZ80,
{
    fn execute(&self, _: &mut Z80<T>, _: Vec<Operand>) {
        println!("Z80::XEP: cpu fetched XEP function");
    }
}

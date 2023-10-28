#[macro_export]
macro_rules! am {
    (n) => { AmType::Imm };
    (nn) => { AmType::ImmExt };
    (D) => { AmType::Relative };
    (DD) => { AmType::Extended };
    (rel) => { AmType::Relative };
    (ext) => { AmType::Extended };
    (imp) => { AmType::Implied };
    
    (a) => { AmType::Register(Register::A) };
    (a_) => { AmType::Register(Register::A_) };
    (b) => { AmType::Register(Register::B) };
    (b_) => { AmType::Register(Register::B_) };
    (c) => { AmType::Register(Register::C) };
    (c_) => { AmType::Register(Register::C_) };
    (d) => { AmType::Register(Register::D) };
    (d_) => { AmType::Register(Register::D_) };
    (e) => { AmType::Register(Register::E) };
    (e_) => { AmType::Register(Register::E_) };
    (h) => { AmType::Register(Register::H) };
    (h_) => { AmType::Register(Register::H_) };
    (l) => { AmType::Register(Register::L) };
    (l_) => { AmType::Register(Register::L_) };
    (af) => { AmType::Register(Register::AF) };
    (af_) => { AmType::Register(Register::AF_) };
    (bc) => { AmType::Register(Register::BC) };
    (bc_) => { AmType::Register(Register::BC_) };
    (de) => { AmType::Register(Register::DE) };
    (de_) => { AmType::Register(Register::DE_) };
    (hl) => { AmType::Register(Register::HL) };
    (hl_) => { AmType::Register(Register::HL_) };
    (i) => { AmType::Register(Register::I) };
    (r) => { AmType::Register(Register::R) };
    (ix) => { AmType::Register(Register::IX) };
    (iy) => { AmType::Register(Register::IY) };
    (sp) => { AmType::Register(Register::SP) };

    (ind_b) => { AmType::RegIndirect(Register::B) };
    (ind_c) => { AmType::RegIndirect(Register::C) };
    (ind_d) => { AmType::RegIndirect(Register::D) };
    (ind_e) => { AmType::RegIndirect(Register::E) };
    (ind_h) => { AmType::RegIndirect(Register::H) };
    (ind_l) => { AmType::RegIndirect(Register::L) };
    (ind_bc) => { AmType::RegIndirect(Register::BC) };
    (ind_de) => { AmType::RegIndirect(Register::DE) };
    (ind_hl) => { AmType::RegIndirect(Register::HL) };
    (ind_ix) => { AmType::RegIndirect(Register::IX) };
    (ind_iy) => { AmType::RegIndirect(Register::IY) };
    (ind_sp) => { AmType::RegIndirect(Register::SP) };

    (idx_ix) => { AmType::Indexed(Register::IX) };
    (idx_iy) => { AmType::Indexed(Register::IY) };

    ($offset:literal) => { AmType::BitAddr($offset) };
}

#[macro_export]
macro_rules! sz {
    (b) => { Size::Byte };
    (w) => { Size::Word };
}

#[macro_export]
macro_rules! inst {
    ($idx:literal: $handler:ident.$s:ident) => {
        Instruction {
            src_am: None,
            dst_am: Some(am!(imp)),
            size: sz!($s),
            handler: Z80Emu::$handler
        }
    };

    ($idx:literal: $handler:ident.$s:ident $dst_am:tt) => {
        Instruction {
            src_am: None,
            dst_am: Some(am!($dst_am)),
            size: sz!($s),
            handler: Z80Emu::$handler
        }
    };
    
    ($idx:literal: $handler:ident.$s:ident $dst_am:tt $src_am:tt) => {
        Instruction {
            src_am: Some(am!($src_am)),
            dst_am: Some(am!($dst_am)),
            size: sz!($s),
            handler: Z80Emu::$handler
        }
    };
}
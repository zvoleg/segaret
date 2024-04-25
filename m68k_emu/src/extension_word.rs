use crate::{
    cpu_internals::{RegisterSet, RegisterType},
    primitives::Pointer,
    SignExtending, Size,
};

pub(crate) struct BriefExtensionWord {
    pub(crate) index_register_ptr: Box<dyn Pointer>,
    pub(crate) size: Size,
    pub(crate) scale: u32,
    pub(crate) displacement: u32,
}

impl BriefExtensionWord {
    pub(crate) fn disassembly(address_reg_str: &str, word: u32) -> String {
        let size = if word & 0x0800 != 0 {
            Size::Long
        } else {
            Size::Word
        };
        let index_reg_idx = ((word >> 11) & 0x07) as usize;
        let mut scale = ((word >> 9) & 0b11) * 2;
        if scale == 0 {
            scale = 1
        }
        let index_reg_str = if word & 0x8000 != 0 {
            format!("A{}.{}*{}", index_reg_idx, size, scale)
        } else {
            format!("D{}.{}*{}", index_reg_idx, size, scale)
        };
        let displacement = (word & 0xFF) as u32;
        format!("({}, {}, {})", displacement, address_reg_str, index_reg_str)
    }

    pub(crate) fn new(word: u16, rs: &mut RegisterSet) -> Self {
        let reg_idx = ((word >> 12) & 0b111) as usize;
        let index_register_ptr = if word & 0x8000 != 0 {
            rs.get_register_ptr(reg_idx, RegisterType::Address)
        } else {
            rs.get_register_ptr(reg_idx, RegisterType::Data)
        };
        let size = if word & 0x0800 != 0 {
            Size::Long
        } else {
            Size::Word
        };
        let mut scale = (((word >> 9) & 0b11) * 2) as u32;
        if scale == 0 {
            scale = 1
        }
        let mut displacement = (word & 0xFF) as u32;
        displacement = displacement.sign_extend(Size::Byte);
        Self {
            index_register_ptr,
            size: size,
            scale: scale,
            displacement: displacement,
        }
    }
}

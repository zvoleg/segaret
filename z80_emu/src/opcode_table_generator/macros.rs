// #[macro_export]
// macro_rules! specifiers {
//     (nz) => {
//         Condition::NZ
//     };

//     (nc) => {
//         Condition::NC
//     };

//     (po) => {
//         Condition::PO
//     };

//     (p) => {
//         Condition::P
//     };

//     (z) => {
//         Condition::Z
//     };

//     (c) => {
//         Condition::C
//     };

//     (pe) => {
//         Condition::PE
//     };

//     (m) => {
//         Condition::M
//     };

//     (unc) => {
//         Condition::UNCONDITIONAL
//     };

//     (D) => {
//         Immediate()
//     };

//     (DD) => {
//         ImmediateExt()
//     };

//     (_hl) => {
//         RegisterIndirect {
//             register: Register::General(RegisterType::HL),
//             size: Size::Word,
//         }
//     };
// }

// #[macro_export]
// macro_rules! am {
//     (a.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::A),
//             size: sz!($s),
//         }
//     };

//     (f.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::F),
//             size: sz!($s),
//         }
//     };

//     (af.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::AF),
//             size: sz!($s),
//         }
//     };

//     (b.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::B),
//             size: sz!($s),
//         }
//     };

//     (c.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::C),
//             size: sz!($s),
//         }
//     };

//     (bc.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::BC),
//             size: sz!($s),
//         }
//     };

//     (d.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::D),
//             size: sz!($s),
//         }
//     };

//     (e.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::E),
//             size: sz!($s),
//         }
//     };

//     (de.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::DE),
//             size: sz!($s),
//         }
//     };

//     (h.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::H),
//             size: sz!($s),
//         }
//     };

//     (l.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::L),
//             size: sz!($s),
//         }
//     };

//     (hl.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::HL),
//             size: sz!($s),
//         }
//     };

//     (a_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::A_),
//             size: sz!($s),
//         }
//     };

//     (f_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::F_),
//             size: sz!($s),
//         }
//     };

//     (af_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::AF_),
//             size: sz!($s),
//         }
//     };

//     (b_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::B_),
//             size: sz!($s),
//         }
//     };

//     (c_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::C_),
//             size: sz!($s),
//         }
//     };

//     (bc_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::BC_),
//             size: sz!($s),
//         }
//     };

//     (d_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::D_),
//             size: sz!($s),
//         }
//     };

//     (e_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::E_),
//             size: sz!($s),
//         }
//     };

//     (de_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::DE_),
//             size: sz!($s),
//         }
//     };

//     (h_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::H_),
//             size: sz!($s),
//         }
//     };

//     (l_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::L_),
//             size: sz!($s),
//         }
//     };

//     (hl_.$s:ident) => {
//         RegisterAddressing {
//             register: Register::General(RegisterType::HL_),
//             size: sz!($s),
//         }
//     };

//     (sp.$s:ident) => {
//         RegisterAddressing {
//             register: Register::StackPointer,
//             size: sz!($s),
//         }
//     };

//     (_bc.$s:ident) => {
//         RegisterIndirect {
//             register: Register::General(RegisterType::BC),
//             size: sz!($s),
//         }
//     };

//     (_de.$s:ident) => {
//         RegisterIndirect {
//             register: Register::General(RegisterType::DE),
//             size: sz!($s),
//         }
//     };

//     (_hl.$s:ident) => {
//         RegisterIndirect {
//             register: Register::General(RegisterType::HL),
//             size: sz!($s),
//         }
//     };

//     (_sp.$s:ident) => {
//         RegisterIndirect {
//             register: Register::StackPointer,
//             size: sz!($s),
//         }
//     };

//     (D.$s:ident) => {
//         Immediate()
//     };

//     (DD.$s:ident) => {
//         ImmediateExt()
//     };
// }

// #[macro_export]
// macro_rules! inst {
//     ($idx:literal: $inst:ident) => {
//         Operation::new(Box::new($inst()), None, None)
//     };

//     ($idx:literal: $inst:ident-$spec:ident) => {
//         Operation::new(Box::new($inst::new($crate::specifiers!($spec))), None, None)
//     };

//     ($idx:literal: $inst:ident-$spec:ident $am:ident) => {
//         Operation::new(
//             Box::new($inst::new($crate::specifiers!($spec))),
//             Some(Box::new($crate::specifiers!($am))),
//             None,
//         )
//     };

//     ($idx:literal: $inst:ident $spec:literal) => {
//         Operation::new(Box::new($inst::new($spec)), None, None)
//     };

//     ($idx:literal: $inst:ident $am:ident) => {
//         Operation::new(
//             Box::new($inst()),
//             Some(Box::new($crate::specifiers!($am))),
//             None,
//         )
//     };

//     ($idx:literal: $inst:ident.$s:ident $dst_am:ident $src_am:ident) => {
//         Operation::new(
//             Box::new($inst()),
//             Some(Box::new($crate::am!($dst_am.$s))),
//             Some(Box::new($crate::am!($src_am.$s))),
//         )
//     };

//     ($idx:literal: $inst:ident.$s:ident $dst_am:ident) => {
//         Operation::new(
//             Box::new($inst()),
//             Some(Box::new($crate::am!($dst_am.$s))),
//             None,
//         )
//     };
// }

#[macro_export]
macro_rules! sz {
    (b) => {
        Size::Byte
    };

    (w) => {
        Size::Word
    };
}

#[macro_export]
macro_rules! op {
    ($idx:literal: $inst:expr) => {
        Operation::new($inst, None, None)
    };

    ($idx:literal: $inst:expr, $dst_am:expr) => {
        Operation::new($inst, Some($dst_am), None)
    };

    ($idx:literal: $inst:expr, $dst_am:expr, $src_am:expr) => {
        Operation::new($inst, Some($dst_am), Some($src_am))
    };
}

#[macro_export]
macro_rules! b {
    ($inst:ident) => {
        Box::new($inst())
    };

    ($inst:ident $cond:ident) => {
        Box::new($inst::new(Condition::$cond))
    };

    ($inst:ident $lit:literal) => {
        Box::new($inst::new($lit))
    }
}

#[macro_export]
macro_rules! rg {
    ($reg_type:ident) => {
        {
            let register = Register::General(RegisterType::$reg_type); 
            let size = register.size();
            Box::new(RegisterAddressing {
                register: register,
                size: size,
            })
        }
    };
}

#[macro_export]
macro_rules! sp {
    () => {
        Box::new(RegisterAddressing {
            register: Register::StackPointer,
            size: Size::Word,
        })
    };

    ($s:ident) => {
        Box::new(RegisterIndirect {
            register: Register::StackPointer,
            size: sz!($s),
        })
    }
}

#[macro_export]
macro_rules! rgi {
    ($reg_type:ident.$s:ident) => {
        Box::new(RegisterIndirect {
            register: Register::General(RegisterType::$reg_type),
            size: sz!($s)
        })
    };
}

#[macro_export]
macro_rules! am {
    (D) => {
        Box::new(Immediate())
    };

    (DD) => {
        Box::new(ImmediateExt())
    };

    (R) => {
        Box::new(Relative())
    };

    (RR) => {
        Box::new(Extended { size: Size::Word } )
    };
}

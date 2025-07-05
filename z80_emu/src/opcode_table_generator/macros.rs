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

    ($reg_type:ident.$s:ident) => {
        Box::new(RegisterIndirect {
            register: Register::General(RegisterType::$reg_type),
            size: sz!($s)
        })
    };
}

#[macro_export]
macro_rules! rx {
    ($idx:ident) => {
        {
            let register = Register::Index(IndexRegister::$idx);
            Box::new(RegisterAddressing {
                register: register,
                size: Size::Word,
            })
        }
    };

    ($idx:ident.$s:ident) => {
        {
            let register = IndexRegister::$idx;
            Box::new(Indexed {
                index_reg: register,
                size: sz!($s),
            })
        }
    };
}

#[macro_export]
macro_rules! ri {
    () => {
        Box::new(RegisterAddressing {
            register: Register::InterruptVector,
            size: Size::Byte,
        })
    };
}

#[macro_export]
macro_rules! rr {
    () => {
        Box::new(RegisterAddressing {
            register: Register::MemoryRefresh,
            size: Size::Byte,
        })
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

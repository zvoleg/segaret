use std::{cell::RefCell, rc::Rc};

use crate::memory_space::MemorySpace;

pub struct VdpBus {
    memory_space: Rc<RefCell<MemorySpace>>,
}

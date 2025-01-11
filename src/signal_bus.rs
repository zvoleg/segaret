pub enum Signal {
    V_INTERRUPT,
    H_INTERRUPT,
    CPU_HALT,
}

pub struct SignalBus {
    signal_que: Vec<Signal>,
}

impl SignalBus {
    pub fn new() -> Self {
        Self { signal_que: vec![] }
    }

    pub fn push_siganal(&mut self, signal: Signal) {}

    pub fn handle_signal(&mut self, signal: Signal) -> bool {
        false
    }
}

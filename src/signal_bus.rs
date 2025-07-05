#[derive(PartialEq)]
pub enum Signal {
    VInterrupt,
    HInterrupt,
    CpuHalt,
    Z80BusRequest,
    Z80BusFree,
    Z80Reset,
}

pub struct SignalBus {
    signal_que: Vec<Signal>,
}

impl SignalBus {
    pub fn new() -> Self {
        Self { signal_que: vec![] }
    }

    pub fn push_siganal(&mut self, signal: Signal) {
        self.signal_que.push(signal);
    }

    pub fn handle_signal(&mut self, signal: Signal) -> bool {
        for i in 0..self.signal_que.len() {
            if signal == self.signal_que[i] {
                self.signal_que.swap_remove(i);
                return true;
            }
        }
        false
    }
}

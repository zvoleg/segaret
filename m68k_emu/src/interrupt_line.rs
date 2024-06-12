pub struct InterruptLine {
    interrupt_level: usize,
}

impl InterruptLine {
    pub fn new() -> Self {
        Self { interrupt_level: 0 }
    }

    pub fn send(&mut self, interrupt_level: usize) {
        self.interrupt_level = interrupt_level;
    }

    pub(crate) fn receive(&mut self) -> usize {
        let signal = self.interrupt_level;
        self.interrupt_level = 0;
        signal
    }
}

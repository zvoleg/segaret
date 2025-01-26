pub enum Button {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    B = 4,
    C = 5,
    A = 6,
    Start = 7,
}

pub struct Controller {
    button_selector: bool,
    data: u8,
}

impl Controller {
    pub(crate) fn new() -> Self {
        Self {
            button_selector: false,
            data: 0xFF,
        }
    }

    pub(crate) fn read(&self) -> u8 {
        if self.button_selector {
            self.data & 0x3F
        } else {
            let down_up = self.data & 0x3;
            let start_a = self.data >> 6 & 0x3;
            start_a << 4 | down_up
        }
    }

    pub(crate) fn write(&mut self, data: u8) {
        self.button_selector = data & 0x40 != 0;
    }

    pub(crate) fn press_button(&mut self, button: Button) {
        self.data &= match button {
            Button::Up => 0xFE,
            Button::Down => 0xFD,
            Button::Left => 0xFB,
            Button::Right => 0xF7,
            Button::B => 0xEF,
            Button::C => 0xDF,
            Button::A => 0xBF,
            Button::Start => 0x7F,
        }
    }

    pub(crate) fn release_button(&mut self, button: Button) {
        self.data |= match button {
            Button::Up => 0x01,
            Button::Down => 0x02,
            Button::Left => 0x04,
            Button::Right => 0x08,
            Button::B => 0x10,
            Button::C => 0x20,
            Button::A => 0x40,
            Button::Start => 0x80,
        }
    }
}
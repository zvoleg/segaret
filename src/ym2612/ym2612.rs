use log::warn;

use crate::ym2612::{channel::Channel, RegisterPart, Ym2612Ports};

pub struct Ym2612 {
    channels: Vec<Channel>,

    register_fm1: u8,
    register_fm2: u8,

    dac_data: u8,
    dac_enabled: bool,

    lfo_enable: bool,
    lfo_freq: f32, // Hz

    timer_a: u16, // 18 * (1024 - timer_a) ms (0x3FF=0.108 ms | 0x000=18.4ms)
    timer_b: u16, // 288 * (256 - timer_b) ms (0xFF=0.288 ms | 0x00=73.44ms)
}

impl Ym2612 {
    pub fn new() -> Self {
        Self {
            channels: vec![],

            register_fm1: 0,
            register_fm2: 0,

            dac_data: 0,
            dac_enabled: false,

            lfo_enable: false,
            lfo_freq: 3.98,

            timer_a: 0,
            timer_b: 0,
        }
    }

    fn setup_lfo(&mut self, data: u8) {
        self.lfo_enable = data & 0x08 != 0;
        self.lfo_freq = match data & 0x07 {
            0 => 3.98,
            1 => 5.56,
            2 => 6.02,
            3 => 6.37,
            4 => 6.88,
            5 => 9.63,
            6 => 48.1,
            7 => 72.2,
            _ => panic!("Ym2612: unexpected bit mask for setting the LFO frequency ({:02X})", data)
        }
    }

    fn set_channel_msb_frequency(&mut self, part: RegisterPart, register: u8, data: u8) {
        let offset = match part {
            RegisterPart::Fm1 => 0,
            RegisterPart::Fm2 => 3,
        };
        let channel_num = (register & 0x03) + 1 + offset; // select channel bits
        let channel = &mut self.channels[channel_num as usize];
        let octave = (data as u16 >> 3) & 0x07;
        channel.set_octave(octave);
        let frequency = (data as u16 & 0x07) << 8;
        channel.set_frequency(frequency);
    }

    fn set_channel_lsb_frequency(&mut self, part: RegisterPart, register: u8, data: u8) {
        let offset = match part {
            RegisterPart::Fm1 => 0,
            RegisterPart::Fm2 => 3,
        };
        let channel_num = (register & 0x03) + 1 + offset; // select channel bits
        let channel = &mut self.channels[channel_num as usize];
        let frequency = channel.frequency | data as u16;
        channel.set_frequency(frequency);
    }
}

impl Ym2612Ports for Ym2612 {
    fn register_set(&mut self, part: RegisterPart, data: u8) {
        let register = match part {
            RegisterPart::Fm1 => &mut self.register_fm1,
            RegisterPart::Fm2 => &mut self.register_fm2,
        };
        *register = data;
    }

    fn register_data(&mut self, part: RegisterPart, data: u8) {
        let register = match part {
            RegisterPart::Fm1 => self.register_fm1,
            RegisterPart::Fm2 => self.register_fm2,
        };
        match register {
            0x22 => self.setup_lfo(data),
            0x24 => self.timer_a = (data as u16) << 2,
            0x25 => self.timer_a |= (data as u16) & 0x03,
            0x26 => self.timer_b = data as u16,
            0x27 => {}, // ch36_mode, timer settings
            0x28 => {}, // setup pressed 'keys'
            0x2A => self.dac_data = data,
            0x2B => self.dac_enabled = data & 0x80 != 0,
            // 0x30-0x90 // setups channels and its operators first part - channels 1-3, second part - channels 4-6 
            0x30 => {} // 0x30+ setup detune (DT1) and multiple (MUL)
            0x40 => {} // 0x40+ setup total level (TL)
            0x50 => {} // 0x50+ setup rate scaling (RS) and atack rate (AR)
            0xA0 | 0xA1 | 0xA2  => self.set_channel_lsb_frequency(part, register, data),
            0xA4 | 0xA5 | 0xA6  => self.set_channel_msb_frequency(part, register, data),
            _ => warn!("Ym2612: set value to register number: {:02X}", register)
        }
    }

    fn read_status(&self) -> u8 {
        0x03 // 0x3 -> both timers are overflowed
    }
}

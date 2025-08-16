pub(crate) struct Channel {
    pub(crate) octave: u16,
    pub(crate) frequency: u16,
    
    pub(crate) operators: Vec<Opperator>,
}

impl Channel {
    pub(crate) fn set_octave(&mut self, octave: u16) {
        self.octave = octave;
    }

    pub(crate) fn set_frequency(&mut self, frequency: u16) {
        self.frequency = frequency;
    }
}

pub(crate) struct Opperator {

}
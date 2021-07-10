use crate::settings::CHANNEL_COUNT;

pub struct Music {
    sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

impl Music {
    pub fn new() -> Music {
        Music {
            sequences: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }
}

impl Music {
    pub fn sequence(&mut self, channel: u32) -> &Vec<u32> {
        &self.sequences[channel as usize]
    }

    pub fn set_sequence(&mut self, channel: u32, sounds: &[u32]) {
        let sequence = &mut self.sequences[channel as usize];

        sequence.clear();

        for sound in sounds {
            sequence.push(*sound);
        }
    }

    pub fn set(
        &mut self,
        sequence0: &[u32],
        sequence1: &[u32],
        sequence2: &[u32],
        sequence3: &[u32],
    ) {
        self.set_sequence(0, sequence0);
        self.set_sequence(1, sequence1);
        self.set_sequence(2, sequence2);
        self.set_sequence(3, sequence3);
    }
}

use crate::settings::CHANNEL_COUNT;

pub struct Music {
    sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

impl Music {
    pub fn new() -> Music {
        Music {
            sequences: Default::default(),
        }
    }

    pub fn set(&mut self, sequences: &[&[u32]]) {
        for i in 0..CHANNEL_COUNT {
            self.set_sequence(i, sequences[i as usize]);
        }
    }

    pub fn sequence(&self, channel: u32) -> &Vec<u32> {
        &self.sequences[channel as usize]
    }

    pub fn set_sequence(&mut self, channel: u32, sounds: &[u32]) {
        let sequence = &mut self.sequences[channel as usize];

        sequence.clear();

        for sound in sounds {
            sequence.push(*sound);
        }
    }
}

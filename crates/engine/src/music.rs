use crate::settings::CHANNEL_COUNT;

#[derive(Clone)]
pub struct Music {
    sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

impl Music {
    pub fn new() -> Music {
        Music {
            sequences: Default::default(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let music = Music::new();

        for i in 0..CHANNEL_COUNT {
            assert_eq!(music.sequence(i).len(), 0);
        }
    }

    #[test]
    fn set() {
        let mut music = Music::new();

        music.set(&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]);

        for i in 0..CHANNEL_COUNT {
            assert_eq!(music.sequence(i), &vec![i, i + 1, i + 2]);
        }
    }

    #[test]
    fn set_sequence() {
        let mut music = Music::new();

        for i in 0..CHANNEL_COUNT {
            music.set_sequence(i, &[0, 1, 2, 3]);
            assert_eq!(music.sequence(0), &vec![0, 1, 2, 3]);
        }
    }
}

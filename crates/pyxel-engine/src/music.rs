use parking_lot::Mutex;
use std::sync::Arc;

use crate::settings::CHANNEL_COUNT;

#[derive(Clone)]
pub struct Music {
    pub sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

impl Music {
    pub fn new() -> Music {
        Music {
            sequences: Default::default(),
        }
    }

    pub fn with_arc_mutex() -> Arc<Mutex<Music>> {
        Arc::new(Mutex::new(Music::new()))
    }

    pub fn set(&mut self, sequences: &[Vec<u32>]) {
        for i in 0..CHANNEL_COUNT {
            self.sequences[i as usize] = sequences[i as usize].clone();
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
            assert_eq!(music.sequences[i as usize].len(), 0);
        }
    }

    #[test]
    fn set() {
        let mut music = Music::new();

        music.set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);

        for i in 0..CHANNEL_COUNT {
            assert_eq!(&music.sequences[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

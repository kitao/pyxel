use std::cell::RefCell;
use std::rc::Rc;

use crate::settings::CHANNEL_COUNT;

#[derive(Clone)]
pub struct Music {
    pub seq: [Vec<u32>; CHANNEL_COUNT as usize],
}

impl Music {
    pub fn new() -> Rc<RefCell<Music>> {
        Rc::new(RefCell::new(Music {
            seq: Default::default(),
        }))
    }

    pub fn set(&mut self, seq0: &[u32], seq1: &[u32], seq2: &[u32], seq3: &[u32]) {
        self.seq[0] = seq0.to_vec();
        self.seq[1] = seq1.to_vec();
        self.seq[2] = seq2.to_vec();
        self.seq[3] = seq3.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let music = Music::new();

        for i in 0..CHANNEL_COUNT {
            assert_eq!(music.borrow().seq[i as usize].len(), 0);
        }
    }

    #[test]
    fn set() {
        let mut music = Music::new();

        music
            .borrow_mut()
            .set(&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]);

        for i in 0..CHANNEL_COUNT {
            assert_eq!(&music.borrow().seq[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

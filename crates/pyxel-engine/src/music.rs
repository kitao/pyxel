use crate::settings::NUM_CHANNELS;

pub type SharedSeq = shared_type!(Vec<u32>);

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<SharedSeq>,
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self {
            seqs: (0..NUM_CHANNELS)
                .map(|_| new_shared_type!(Vec::new()))
                .collect()
        })
    }

    pub fn set(&mut self, seqs: &[Vec<u32>]) {
        self.seqs = seqs
            .iter()
            .map(|seq| new_shared_type!(seq.clone()))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let music = Music::new();
        for i in 0..NUM_CHANNELS {
            assert_eq!(music.lock().seqs[i as usize].lock().len(), 0);
        }
    }

    #[test]
    fn set() {
        let music = Music::new();
        music
            .lock()
            .set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        for i in 0..NUM_CHANNELS {
            assert_eq!(
                &*music.lock().seqs[i as usize].lock(),
                &vec![i, i + 1, i + 2]
            );
        }
    }
}

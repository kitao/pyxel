use crate::pyxel::CHANNELS;

pub type SharedSeq = shared_type!(Vec<u32>);

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<SharedSeq>,
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self { seqs: Vec::new() })
    }

    pub fn set(&mut self, seqs: &[Vec<u32>]) {
        self.seqs = seqs
            .iter()
            .map(|seq| new_shared_type!(seq.clone()))
            .collect();
        let num_channels = CHANNELS.lock().len();
        while self.seqs.len() < num_channels {
            self.seqs.push(new_shared_type!(Vec::new()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_music_new() {
        let music = Music::new();
        assert_eq!(music.lock().seqs.len(), 0);
    }

    #[test]
    fn test_music_set() {
        let music = Music::new();
        music
            .lock()
            .set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]]);
        for i in 0..3 {
            assert_eq!(
                &*music.lock().seqs[i as usize].lock(),
                &vec![i, i + 1, i + 2]
            );
        }
    }
}

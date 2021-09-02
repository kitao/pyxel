use parking_lot::Mutex;
use std::sync::Arc;

use crate::resource::ResourceItem;
use crate::settings::{CHANNEL_COUNT, RESOURCE_ARCHIVE_DIRNAME};
use crate::utils::parse_hex_string;

#[derive(Clone)]
pub struct Music {
    pub sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

pub type SharedMusic = Arc<Mutex<Music>>;

impl Music {
    pub fn new() -> SharedMusic {
        Arc::new(Mutex::new(Music {
            sequences: Default::default(),
        }))
    }

    pub fn set(&mut self, sequences: &[Vec<u32>]) {
        for i in 0..CHANNEL_COUNT {
            self.sequences[i as usize] = sequences[i as usize].clone();
        }
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for sequence in self.sequences.iter() {
            if !sequence.is_empty() {
                return true;
            }
        }

        false
    }

    fn clear(&mut self) {
        self.sequences = Default::default();
    }

    fn serialize(&self) -> String {
        let mut output = String::new();

        for sequence in self.sequences.iter() {
            if !sequence.is_empty() {
                for sound_no in sequence {
                    output += &format!("{:02x}", sound_no);
                }
            } else {
                output += "none";
            }

            output += "\n";
        }

        output
    }

    fn deserialize(&mut self, input: &str) {
        self.clear();

        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }

            string_loop!(j, value, line, 2, {
                self.sequences[i].push(parse_hex_string(&value).unwrap());
            });
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
            assert_eq!(music.lock().sequences[i as usize].len(), 0);
        }
    }

    #[test]
    fn set() {
        let music = Music::new();

        music
            .lock()
            .set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);

        for i in 0..CHANNEL_COUNT {
            assert_eq!(&music.lock().sequences[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

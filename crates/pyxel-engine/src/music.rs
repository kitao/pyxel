use std::fmt::Write as _;

use crate::resource::ResourceItem;
use crate::settings::NUM_CHANNELS;
use crate::settings::RESOURCE_ARCHIVE_DIRNAME;
use crate::utils::parse_hex_string;

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

impl ResourceItem for Music {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_index.to_string()
    }

    fn is_modified(&self) -> bool {
        self.seqs.iter().any(|seq| !seq.lock().is_empty())
    }

    fn clear(&mut self) {
        self.seqs = (0..NUM_CHANNELS)
            .map(|_| new_shared_type!(Vec::new()))
            .collect();
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        for seq in &self.seqs {
            if seq.lock().is_empty() {
                output += "none";
            } else {
                for sound_index in &*seq.lock() {
                    let _guard = write!(output, "{sound_index:02x}");
                }
            }
            output += "\n";
        }
        output
    }

    fn deserialize(&mut self, _version: u32, input: &str) {
        self.clear();
        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }
            string_loop!(j, value, line, 2, {
                self.seqs[i].lock().push(parse_hex_string(&value).unwrap());
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

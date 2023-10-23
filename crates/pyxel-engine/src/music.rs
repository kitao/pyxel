use std::fmt::Write as _;

use crate::pyxel::Pyxel;
use crate::resource::ResourceItem;
use crate::settings::NUM_CHANNELS;
use crate::settings::RESOURCE_ARCHIVE_DIRNAME;
use crate::utils::parse_hex_string;

pub type SharedChain = shared_type!(Vec<u32>);

#[derive(Clone)]
pub struct Music {
    pub chains: Vec<SharedChain>,
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self {
            chains: (0..NUM_CHANNELS)
                .map(|_| new_shared_type!(Vec::new()))
                .collect()
        })
    }

    pub fn set(&mut self, sounds0: &[u32], sounds1: &[u32], sounds2: &[u32], sounds3: &[u32]) {
        self.chains[0] = new_shared_type!(sounds0.to_vec());
        self.chains[1] = new_shared_type!(sounds1.to_vec());
        self.chains[2] = new_shared_type!(sounds2.to_vec());
        self.chains[3] = new_shared_type!(sounds3.to_vec());
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        self.chains
            .iter()
            .any(|sound_nums| !sound_nums.lock().is_empty())
    }

    fn clear(&mut self) {
        self.chains = (0..NUM_CHANNELS)
            .map(|_| new_shared_type!(Vec::new()))
            .collect();
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();
        for sound_nums in &self.chains {
            if sound_nums.lock().is_empty() {
                output += "none";
            } else {
                for sound_num in &*sound_nums.lock() {
                    let _guard = write!(output, "{sound_num:02x}");
                }
            }
            output += "\n";
        }
        output
    }

    fn deserialize(&mut self, _pyxel: &Pyxel, _version: u32, input: &str) {
        self.clear();
        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }
            string_loop!(j, value, line, 2, {
                self.chains[i]
                    .lock()
                    .push(parse_hex_string(&value).unwrap());
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
            assert_eq!(music.lock().chains[i as usize].lock().len(), 0);
        }
    }

    #[test]
    fn set() {
        let music = Music::new();
        music
            .lock()
            .set(&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]);
        for i in 0..NUM_CHANNELS {
            assert_eq!(
                &*music.lock().chains[i as usize].lock(),
                &vec![i, i + 1, i + 2]
            );
        }
    }
}

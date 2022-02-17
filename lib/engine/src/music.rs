use crate::resource::ResourceItem;
use crate::settings::{NUM_CHANNELS, RESOURCE_ARCHIVE_DIRNAME};
use crate::utils::parse_hex_string;
use crate::Pyxel;

#[derive(Clone)]
pub struct Music {
    pub sounds: [Vec<u32>; NUM_CHANNELS as usize],
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self {
            sounds: Default::default(),
        })
    }

    pub fn set(&mut self, sounds0: &[u32], sounds1: &[u32], sounds2: &[u32], sounds3: &[u32]) {
        self.sounds[0] = sounds0.to_vec();
        self.sounds[1] = sounds1.to_vec();
        self.sounds[2] = sounds2.to_vec();
        self.sounds[3] = sounds3.to_vec();
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for sndseq in &self.sounds {
            if !sndseq.is_empty() {
                return true;
            }
        }
        false
    }

    fn clear(&mut self) {
        self.sounds = Default::default();
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();
        for sequence in &self.sounds {
            if sequence.is_empty() {
                output += "none";
            } else {
                for sound_no in sequence {
                    output += &format!("{:02x}", sound_no);
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
                self.sounds[i].push(parse_hex_string(&value).unwrap());
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
            assert_eq!(music.lock().sounds[i as usize].len(), 0);
        }
    }

    #[test]
    fn set() {
        let music = Music::new();
        music
            .lock()
            .set(&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]);
        for i in 0..NUM_CHANNELS {
            assert_eq!(&music.lock().sounds[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

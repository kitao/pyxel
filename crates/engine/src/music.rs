use crate::resource::ResourceItem;
use crate::settings::{CHANNEL_COUNT, RESOURCE_ARCHIVE_DIRNAME};
use crate::utils::parse_hex_string;
use crate::Pyxel;

#[derive(Clone)]
pub struct Music {
    pub sequences: [Vec<u32>; CHANNEL_COUNT as usize],
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self {
            sequences: Default::default(),
        })
    }

    pub fn set(
        &mut self,
        sequence0: &[u32],
        sequence1: &[u32],
        sequence2: &[u32],
        sequence3: &[u32],
    ) {
        self.sequences[0] = sequence0.to_vec();
        self.sequences[1] = sequence1.to_vec();
        self.sequences[2] = sequence2.to_vec();
        self.sequences[3] = sequence3.to_vec();
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for sequence in &self.sequences {
            if !sequence.is_empty() {
                return true;
            }
        }

        false
    }

    fn clear(&mut self) {
        self.sequences = Default::default();
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();

        for sequence in &self.sequences {
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
            .set(&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]);

        for i in 0..CHANNEL_COUNT {
            assert_eq!(&music.lock().sequences[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

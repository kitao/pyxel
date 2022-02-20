use crate::resource::ResourceItem;
use crate::settings::{NUM_CHANNELS, RESOURCE_ARCHIVE_DIRNAME};
use crate::utils::parse_hex_string;
use crate::Pyxel;

#[derive(Clone)]
pub struct Music {
    pub sounds_list: [Vec<u32>; NUM_CHANNELS as usize],
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self {
            sounds_list: Default::default(),
        })
    }

    pub fn set(&mut self, sounds0: &[u32], sounds1: &[u32], sounds2: &[u32], sounds3: &[u32]) {
        self.sounds_list[0] = sounds0.to_vec();
        self.sounds_list[1] = sounds1.to_vec();
        self.sounds_list[2] = sounds2.to_vec();
        self.sounds_list[3] = sounds3.to_vec();
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        self.sounds_list.iter().any(|sounds| !sounds.is_empty())
    }

    fn clear(&mut self) {
        self.sounds_list = Default::default();
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();
        for sounds in &self.sounds_list {
            if sounds.is_empty() {
                output += "none";
            } else {
                for sound_no in sounds {
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
                self.sounds_list[i].push(parse_hex_string(&value).unwrap());
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
            assert_eq!(music.lock().sounds_list[i as usize].len(), 0);
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
                &music.lock().sounds_list[i as usize],
                &vec![i, i + 1, i + 2]
            );
        }
    }
}

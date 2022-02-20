use crate::resource::ResourceItem;
use crate::settings::RESOURCE_ARCHIVE_DIRNAME;
use crate::utils::parse_hex_string;
use crate::Pyxel;

#[derive(Clone)]
pub struct Music {
    pub sounds: Vec<Vec<u32>>,
}

pub type SharedMusic = shared_type!(Music);

impl Music {
    pub fn new() -> SharedMusic {
        new_shared_type!(Self { sounds: Vec::new() })
    }

    pub fn set(&mut self, sounds: &[Vec<u32>]) {
        self.sounds = sounds.to_vec();
    }
}

impl ResourceItem for Music {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        self.sounds.iter().any(|sounds| !sounds.is_empty())
    }

    fn clear(&mut self) {
        self.sounds.clear();
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();
        for sounds in &self.sounds {
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
            self.sounds.push(Vec::new());
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
        assert!(music.lock().sounds.is_empty());
    }

    #[test]
    fn set() {
        let music = Music::new();
        music
            .lock()
            .set(&[vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        for i in 0..4 {
            assert_eq!(&music.lock().sounds[i as usize], &vec![i, i + 1, i + 2]);
        }
    }
}

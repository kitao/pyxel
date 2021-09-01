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

    fn clear(&mut self) {
        //
    }

    fn serialize(&self) -> String {
        /*
        Music* music = audio_->GetMusicBank(music_index);

        if (music->Channel0().size() == 0 && music->Channel1().size() == 0 &&
            music->Channel2().size() == 0 && music->Channel3().size() == 0) {
          return "";
        }

        std::stringstream ss;

        ss << std::hex;

        if (music->Channel0().size() > 0) {
          for (int32_t v : music->Channel0()) {
            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (music->Channel1().size() > 0) {
          for (int32_t v : music->Channel1()) {
            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (music->Channel2().size() > 0) {
          for (int32_t v : music->Channel2()) {
            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (music->Channel3().size() > 0) {
          for (int32_t v : music->Channel3()) {
            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        return ss.str();
        */
        "TODO".to_string()
    }

    fn deserialize(&mut self, input: &str) {
        self.clear();

        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }

            for j in 0..line.len() / 2 {
                let index = j * 2;
                let value = parse_hex_string(&line[index..index + 2].to_string()).unwrap();
                self.sequences[i].push(value);
            }
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

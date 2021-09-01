use parking_lot::Mutex;
use std::sync::Arc;

use crate::settings::{CHANNEL_COUNT, RESOURCE_ARCHIVE_DIRNAME};

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

    pub(crate) fn resource_name(music_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &music_no.to_string()
    }

    pub(crate) fn clear(&mut self) {
        //
    }

    pub(crate) fn serialize(&self) -> String {
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

    pub(crate) fn deserialize(&mut self, input: &str) {
        /*
        Music* music = audio_->GetMusicBank(music_index);
        std::stringstream ss(str);

        PARSE_CHANNEL(ss, music, Channel0);
        PARSE_CHANNEL(ss, music, Channel1);
        PARSE_CHANNEL(ss, music, Channel2);
        PARSE_CHANNEL(ss, music, Channel3);

        #define PARSE_CHANNEL(ss, music, channel)                          \
          do {                                                             \
            SoundIndexList& data = music->channel();                       \
            data.clear();                                                  \
                                                                           \
            std::string line = GetTrimmedLine(ss);                         \
                                                                           \
            if (line != "none") {                                          \
              for (int32_t i = 0; i < line.size() / 2; i++) {              \
                int32_t v = std::stoi(line.substr(i * 2, 2), nullptr, 16); \
                                                                           \
                data.push_back(v);                                         \
              }                                                            \
            }                                                              \
          } while (false)
        */
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

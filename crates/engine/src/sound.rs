use parking_lot::Mutex;
use std::sync::Arc;

use crate::resource::ResourceItem;
use crate::settings::{
    EFFECT_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_SPEED,
    RESOURCE_ARCHIVE_DIRNAME, TONE_NOISE, TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE,
};
use crate::types::{Effect, Note, Speed, Tone, Volume};
use crate::utils::{parse_hex_string, simplify_string};

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<Note>,
    pub tones: Vec<Tone>,
    pub volumes: Vec<Volume>,
    pub effects: Vec<Effect>,
    pub speed: Speed,
}

pub type SharedSound = Arc<Mutex<Sound>>;

impl Sound {
    pub fn new() -> SharedSound {
        Arc::new(Mutex::new(Sound {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: INITIAL_SPEED,
        }))
    }

    pub fn set(
        &mut self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: Speed,
    ) {
        self.set_note(note_str);
        self.set_tone(tone_str);
        self.set_volume(volume_str);
        self.set_effect(effect_str);
        self.speed = speed;
    }

    pub fn set_note(&mut self, note_str: &str) {
        let note_str = simplify_string(note_str);
        let mut chars = note_str.chars();

        self.notes.clear();

        while let Some(c) = chars.next() {
            let mut note: Note;

            if c >= 'a' && c <= 'g' {
                note = match c {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => panic!("invalid sound note '{}'", c),
                };
                let mut c = chars.next().unwrap_or(0 as char);

                if c == '#' {
                    note += 1;
                    c = chars.next().unwrap_or(0 as char);
                } else if c == '-' {
                    note -= 1;
                    c = chars.next().unwrap_or(0 as char);
                }

                if c >= '0' && c <= '4' {
                    note += (c as Note - '0' as Note) * 12;
                } else {
                    panic!("invalid sound note '{}'", c);
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!("invalid sound note '{}'", c);
            }

            self.notes.push(note);
        }
    }

    pub fn set_tone(&mut self, tone_str: &str) {
        let tone_str = simplify_string(tone_str);
        let mut chars = tone_str.chars();

        self.tones.clear();

        while let Some(c) = chars.next() {
            let tone = match c {
                't' => TONE_TRIANGLE,
                's' => TONE_SQUARE,
                'p' => TONE_PULSE,
                'n' => TONE_NOISE,
                _ => panic!("invalid sound tone '{}'", c),
            };

            self.tones.push(tone);
        }
    }

    pub fn set_volume(&mut self, volume_str: &str) {
        let volume_str = simplify_string(volume_str);
        let mut chars = volume_str.chars();

        self.volumes.clear();

        while let Some(c) = chars.next() {
            if c >= '0' && c <= '7' {
                self.volumes.push((c as u32 - '0' as u32) as Volume);
            } else {
                panic!("invalid sound volume '{}'", c);
            }
        }
    }

    pub fn set_effect(&mut self, effect_str: &str) {
        let effect_str = simplify_string(effect_str);
        let mut chars = effect_str.chars();

        self.effects.clear();

        while let Some(c) = chars.next() {
            let effect = match c {
                'n' => EFFECT_NONE,
                's' => EFFECT_SLIDE,
                'v' => EFFECT_VIBRATO,
                'f' => EFFECT_FADEOUT,
                _ => panic!("invalid sound effect '{}'", c),
            };

            self.effects.push(effect);
        }
    }
}

impl ResourceItem for Sound {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "sound" + &format!("{:02}", item_no)
    }

    fn clear(&mut self) {
        //
    }

    fn serialize(&self) -> String {
        /*
        Sound* sound = audio_->GetSoundBank(sound_index);

        if (sound->Note().size() == 0 && sound->Tone().size() == 0 &&
            sound->Volume().size() == 0 && sound->Effect().size() == 0) {
          return "";
        }

        std::stringstream ss;

        ss << std::hex;

        if (sound->Note().size() > 0) {
          for (int32_t v : sound->Note()) {
            if (v < 0) {
              v = 0xff;
            }

            ss << std::setw(2) << std::setfill('0') << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Tone().size() > 0) {
          for (int32_t v : sound->Tone()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Volume().size() > 0) {
          for (int32_t v : sound->Volume()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        if (sound->Effect().size() > 0) {
          for (int32_t v : sound->Effect()) {
            ss << v;
          }
          ss << std::endl;
        } else {
          ss << "none" << std::endl;
        }

        ss << std::dec << sound->Speed() << std::endl;

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

            if i == 0 {
                string_loop!(j, value, line, 2, {
                    self.notes.push(parse_hex_string(&value).unwrap() as i8);
                });

                continue;
            } else if i == 4 {
                self.speed = line.parse().unwrap();

                continue;
            }

            let data = match i {
                1 => &mut self.tones,
                2 => &mut self.volumes,
                3 => &mut self.effects,
                _ => panic!(),
            };

            string_loop!(j, value, line, 1, {
                data.push(parse_hex_string(&value).unwrap() as u8);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sound = Sound::new();
        assert_eq!(sound.lock().notes.len(), 0);
        assert_eq!(sound.lock().tones.len(), 0);
        assert_eq!(sound.lock().volumes.len(), 0);
        assert_eq!(sound.lock().effects.len(), 0);
        assert_eq!(sound.lock().speed, INITIAL_SPEED);
    }

    #[test]
    fn set() {
        let sound = Sound::new();

        sound.lock().set("c0d-0d0d#0", "tspn", "0123", "nsvf", 123);
        assert_eq!(&sound.lock().notes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
        assert_eq!(sound.lock().speed, 123);
    }

    #[test]
    fn set_note() {
        let sound = Sound::new();

        sound
            .lock()
            .set_note(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.lock().notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn set_tone() {
        let sound = Sound::new();

        sound.lock().set_tone(" t s p n ");
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn set_volume() {
        let sound = Sound::new();

        sound.lock().set_volume(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn set_effect() {
        let sound = Sound::new();

        sound.lock().set_effect(" n s v f ");
        assert_eq!(
            &sound.lock().effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
    }
}

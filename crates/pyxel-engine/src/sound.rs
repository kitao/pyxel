use std::fmt::Write as _;

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

pub type SharedSound = shared_type!(Sound);

impl Sound {
    pub fn new() -> SharedSound {
        new_shared_type!(Self {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: INITIAL_SPEED,
        })
    }

    pub fn set(
        &mut self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: Speed,
    ) {
        self.set_notes(note_str);
        self.set_tones(tone_str);
        self.set_volumes(volume_str);
        self.set_effects(effect_str);
        self.speed = speed;
    }

    pub fn set_notes(&mut self, note_str: &str) {
        let note_str = simplify_string(note_str);
        let mut chars = note_str.chars();
        self.notes.clear();
        while let Some(c) = chars.next() {
            let mut note: Note;
            if ('a'..='g').contains(&c) {
                note = match c {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => panic!("Invalid sound note '{}'", c),
                };
                let mut c = chars.next().unwrap_or(0 as char);
                if c == '#' {
                    note += 1;
                    c = chars.next().unwrap_or(0 as char);
                } else if c == '-' {
                    note -= 1;
                    c = chars.next().unwrap_or(0 as char);
                }
                if ('0'..='4').contains(&c) {
                    note += (c as Note - '0' as Note) * 12;
                } else {
                    panic!("Invalid sound note '{}'", c);
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!("Invalid sound note '{}'", c);
            }
            self.notes.push(note);
        }
    }

    pub fn set_tones(&mut self, tone_str: &str) {
        self.tones.clear();
        for c in simplify_string(tone_str).chars() {
            let tone = match c {
                't' => TONE_TRIANGLE,
                's' => TONE_SQUARE,
                'p' => TONE_PULSE,
                'n' => TONE_NOISE,
                _ => panic!("Invalid sound tone '{}'", c),
            };
            self.tones.push(tone);
        }
    }

    pub fn set_volumes(&mut self, volume_str: &str) {
        self.volumes.clear();
        for c in simplify_string(volume_str).chars() {
            if ('0'..='7').contains(&c) {
                self.volumes.push((c as u32 - '0' as u32) as Volume);
            } else {
                panic!("Invalid sound volume '{}'", c);
            }
        }
    }

    pub fn set_effects(&mut self, effect_str: &str) {
        self.effects.clear();
        for c in simplify_string(effect_str).chars() {
            let effect = match c {
                'n' => EFFECT_NONE,
                's' => EFFECT_SLIDE,
                'v' => EFFECT_VIBRATO,
                'f' => EFFECT_FADEOUT,
                _ => panic!("Invalid sound effect '{}'", c),
            };
            self.effects.push(effect);
        }
    }
}

impl ResourceItem for Sound {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "sound" + &format!("{:02}", item_no)
    }

    fn is_modified(&self) -> bool {
        !self.notes.is_empty()
            || !self.tones.is_empty()
            || !self.volumes.is_empty()
            || !self.effects.is_empty()
    }

    fn clear(&mut self) {
        self.notes.clear();
        self.tones.clear();
        self.volumes.clear();
        self.effects.clear();
        self.speed = INITIAL_SPEED;
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        if self.notes.is_empty() {
            output += "none\n";
        } else {
            for note in &self.notes {
                if *note < 0 {
                    output += "ff";
                } else {
                    let _ = write!(output, "{:02x}", *note);
                }
            }
            output += "\n";
        }

        macro_rules! stringify_data {
            ($name: ident) => {
                if self.$name.is_empty() {
                    output += "none\n";
                } else {
                    for value in &self.$name {
                        let _ = write!(output, "{:1x}", *value);
                    }
                    output += "\n";
                }
            };
        }

        stringify_data!(tones);
        stringify_data!(volumes);
        stringify_data!(effects);
        let _ = write!(output, "{}", self.speed);
        output
    }

    fn deserialize(&mut self, _version: u32, input: &str) {
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
            .set_notes(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.lock().notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn set_tone() {
        let sound = Sound::new();
        sound.lock().set_tones(" t s p n ");
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn set_volume() {
        let sound = Sound::new();
        sound.lock().set_volumes(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn set_effect() {
        let sound = Sound::new();
        sound.lock().set_effects(" n s v f ");
        assert_eq!(
            &sound.lock().effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
    }
}

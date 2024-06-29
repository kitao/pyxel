use crate::channel::{Note, Speed, Volume};
use crate::oscillator::Effect;
use crate::settings::{
    EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE,
    EFFECT_VIBRATO, INITIAL_SOUND_SPEED, TONE_NOISE, TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE,
};
use crate::utils::simplify_string;

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<Note>,
    pub tones: Vec<u32>,
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
            speed: INITIAL_SOUND_SPEED,
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
                    _ => panic!("Invalid sound note '{c}'"),
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
                    note += (c.to_digit(10).unwrap() as Note) * 12;
                } else {
                    panic!("Invalid sound note '{c}'");
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!("Invalid sound note '{c}'");
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
                '0'..='9' => c.to_digit(10).unwrap(),
                _ => panic!("Invalid sound tone '{c}'"),
            };
            self.tones.push(tone);
        }
    }

    pub fn set_volumes(&mut self, volume_str: &str) {
        self.volumes.clear();
        for c in simplify_string(volume_str).chars() {
            if ('0'..='7').contains(&c) {
                self.volumes.push(c.to_digit(10).unwrap() as Volume);
            } else {
                panic!("Invalid sound volume '{c}'");
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
                'h' => EFFECT_HALF_FADEOUT,
                'q' => EFFECT_QUARTER_FADEOUT,
                _ => panic!("Invalid sound effect '{c}'"),
            };
            self.effects.push(effect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_new() {
        let sound = Sound::new();
        assert_eq!(sound.lock().notes.len(), 0);
        assert_eq!(sound.lock().tones.len(), 0);
        assert_eq!(sound.lock().volumes.len(), 0);
        assert_eq!(sound.lock().effects.len(), 0);
        assert_eq!(sound.lock().speed, INITIAL_SOUND_SPEED);
    }

    #[test]
    fn test_sound_set() {
        let sound = Sound::new();
        sound.lock().set("c0d-0d0d#0", "tspn", "0123", "nsvf", 123);
        assert_eq!(&sound.lock().notes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(
            &sound.lock().effects,
            &vec![
                EFFECT_NONE,
                EFFECT_SLIDE,
                EFFECT_VIBRATO,
                EFFECT_FADEOUT,
                EFFECT_HALF_FADEOUT,
                EFFECT_QUARTER_FADEOUT
            ]
        );
        assert_eq!(sound.lock().speed, 123);
    }

    #[test]
    fn test_sound_set_note() {
        let sound = Sound::new();
        sound
            .lock()
            .set_notes(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.lock().notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn test_sound_set_tone() {
        let sound = Sound::new();
        sound.lock().set_tones(" t s p n ");
        assert_eq!(
            &sound.lock().tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn test_sound_set_volume() {
        let sound = Sound::new();
        sound.lock().set_volumes(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.lock().volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_sound_set_effect() {
        let sound = Sound::new();
        sound.lock().set_effects(" n s v f h q");
        assert_eq!(
            &sound.lock().effects,
            &vec![
                EFFECT_NONE,
                EFFECT_SLIDE,
                EFFECT_VIBRATO,
                EFFECT_FADEOUT,
                EFFECT_HALF_FADEOUT,
                EFFECT_QUARTER_FADEOUT
            ]
        );
    }
}

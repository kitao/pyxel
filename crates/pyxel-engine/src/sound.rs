use crate::settings::{
    EFFECT_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_SPEED, TONE_NOISE,
    TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE,
};
use crate::types::{Effect, Note, Speed, Tone, Volume};
use crate::utility::simplify_string;

#[derive(Clone)]
pub struct Sound {
    pub notes: Vec<Note>,
    pub tones: Vec<Tone>,
    pub volumes: Vec<Volume>,
    pub effects: Vec<Effect>,
    pub speed: Speed,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: INITIAL_SPEED,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sound = Sound::new();
        assert_eq!(sound.notes.len(), 0);
        assert_eq!(sound.tones.len(), 0);
        assert_eq!(sound.volumes.len(), 0);
        assert_eq!(sound.effects.len(), 0);
        assert_eq!(sound.speed, INITIAL_SPEED);
    }

    #[test]
    fn set() {
        let mut sound = Sound::new();

        sound.set("c0d-0d0d#0", "tspn", "0123", "nsvf", 123);
        assert_eq!(&sound.notes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
        assert_eq!(&sound.volumes, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
        assert_eq!(sound.speed, 123);
    }

    #[test]
    fn set_note() {
        let mut sound = Sound::new();

        sound.set_note(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.notes, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn set_tone() {
        let mut sound = Sound::new();

        sound.set_tone(" t s p n ");
        assert_eq!(
            &sound.tones,
            &vec![TONE_TRIANGLE, TONE_SQUARE, TONE_PULSE, TONE_NOISE]
        );
    }

    #[test]
    fn set_volume() {
        let mut sound = Sound::new();

        sound.set_volume(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(&sound.volumes, &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn set_effect() {
        let mut sound = Sound::new();

        sound.set_effect(" n s v f ");
        assert_eq!(
            &sound.effects,
            &vec![EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, EFFECT_FADEOUT]
        );
    }
}

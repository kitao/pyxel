use crate::settings::DEFAULT_SOUND_SPEED;
use crate::types::{Effect, Note, Speed, Tone, Volume};
use crate::utility::simplify_string;

#[derive(Clone)]
pub struct Sound {
    pub note: Vec<Note>,
    pub tone: Vec<Tone>,
    pub volume: Vec<Volume>,
    pub effect: Vec<Effect>,
    pub speed: Speed,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            note: Vec::new(),
            tone: Vec::new(),
            volume: Vec::new(),
            effect: Vec::new(),
            speed: DEFAULT_SOUND_SPEED,
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

        self.note.clear();

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

            self.note.push(note);
        }
    }

    pub fn set_tone(&mut self, tone_str: &str) {
        let tone_str = simplify_string(tone_str);
        let mut chars = tone_str.chars();

        self.tone.clear();

        while let Some(c) = chars.next() {
            let tone = match c {
                't' => Tone::Triangle,
                's' => Tone::Square,
                'p' => Tone::Pulse,
                'n' => Tone::Noise,
                _ => panic!("invalid sound tone '{}'", c),
            };

            self.tone.push(tone);
        }
    }

    pub fn set_volume(&mut self, volume_str: &str) {
        let volume_str = simplify_string(volume_str);
        let mut chars = volume_str.chars();

        self.volume.clear();

        while let Some(c) = chars.next() {
            let volume = match c {
                '0' => Volume::Level0,
                '1' => Volume::Level1,
                '2' => Volume::Level2,
                '3' => Volume::Level3,
                '4' => Volume::Level4,
                '5' => Volume::Level5,
                '6' => Volume::Level6,
                '7' => Volume::Level7,
                _ => panic!("invalid sound volume '{}'", c),
            };

            self.volume.push(volume);
        }
    }

    pub fn set_effect(&mut self, effect_str: &str) {
        let effect_str = simplify_string(effect_str);
        let mut chars = effect_str.chars();

        self.effect.clear();

        while let Some(c) = chars.next() {
            let effect = match c {
                'n' => Effect::None,
                's' => Effect::Slide,
                'v' => Effect::Vibrato,
                'f' => Effect::FadeOut,
                _ => panic!("invalid sound effect '{}'", c),
            };

            self.effect.push(effect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sound = Sound::new();
        assert_eq!(sound.note.len(), 0);
        assert_eq!(sound.tone.len(), 0);
        assert_eq!(sound.volume.len(), 0);
        assert_eq!(sound.effect.len(), 0);
        assert_eq!(sound.speed, DEFAULT_SOUND_SPEED);
    }

    #[test]
    fn set() {
        let mut sound = Sound::new();

        sound.set("c0d-0d0d#0", "tspn", "0123", "nsvf", 123);
        assert_eq!(&sound.note, &vec![0, 1, 2, 3]);
        assert_eq!(
            &sound.tone,
            &vec![Tone::Triangle, Tone::Square, Tone::Pulse, Tone::Noise]
        );
        assert_eq!(
            &sound.volume,
            &vec![
                Volume::Level0,
                Volume::Level1,
                Volume::Level2,
                Volume::Level3,
            ]
        );
        assert_eq!(
            &sound.effect,
            &vec![
                Effect::None,
                Effect::Slide,
                Effect::Vibrato,
                Effect::FadeOut
            ]
        );
        assert_eq!(sound.speed, 123);
    }

    #[test]
    fn set_note() {
        let mut sound = Sound::new();

        sound.set_note(" c 0 d # 1 r e 2 f 3 g 4 r a - 0 b 1 ");
        assert_eq!(&sound.note, &vec![0, 15, -1, 28, 41, 55, -1, 8, 23]);
    }

    #[test]
    fn set_tone() {
        let mut sound = Sound::new();

        sound.set_tone(" t s p n ");
        assert_eq!(
            &sound.tone,
            &vec![Tone::Triangle, Tone::Square, Tone::Pulse, Tone::Noise]
        );
    }

    #[test]
    fn set_volume() {
        let mut sound = Sound::new();

        sound.set_volume(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(
            &sound.volume,
            &vec![
                Volume::Level0,
                Volume::Level1,
                Volume::Level2,
                Volume::Level3,
                Volume::Level4,
                Volume::Level5,
                Volume::Level6,
                Volume::Level7,
            ]
        );
    }

    #[test]
    fn set_effect() {
        let mut sound = Sound::new();

        sound.set_effect(" n s v f ");
        assert_eq!(
            &sound.effect,
            &vec![
                Effect::None,
                Effect::Slide,
                Effect::Vibrato,
                Effect::FadeOut
            ]
        );
    }
}

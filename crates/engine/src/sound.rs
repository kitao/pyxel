use std::collections::HashMap;

use crate::oscillator::{Effect, Tone};
use crate::settings::{DEFAULT_SOUND_SPEED, MAX_SOUND_VOLUME};
use crate::utility::remove_whitespace;

pub type Note = i32;
pub type Volume = u32;
pub type Speed = u32;

#[derive(Clone)]
pub struct Sound {
    notes: Vec<Note>,
    tones: Vec<Tone>,
    volumes: Vec<Volume>,
    effects: Vec<Effect>,
    speed: Speed,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: DEFAULT_SOUND_SPEED,
        }
    }

    pub fn set(&mut self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: Speed) {
        self.set_notes(notes);
        self.set_tones(tones);
        self.set_volumes(volumes);
        self.set_effects(effects);
        self.set_speed(speed);
    }

    pub fn note(&self, index: u32) -> Note {
        let len = self.notes.len();

        if len > 0 {
            self.notes[index as usize % len]
        } else {
            0
        }
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn set_notes(&mut self, notes: &str) {
        let notes = remove_whitespace(notes);
        let mut chars = notes.chars();

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

    pub fn tone(&self, index: u32) -> Tone {
        let len = self.tones.len();

        if len > 0 {
            self.tones[index as usize % len]
        } else {
            Tone::Triangle
        }
    }

    pub fn tones(&self) -> &Vec<Tone> {
        &self.tones
    }

    pub fn set_tones(&mut self, tones: &str) {
        let tones = remove_whitespace(tones);
        let mut chars = tones.chars();

        self.tones.clear();

        while let Some(c) = chars.next() {
            let tone = match c {
                't' => Tone::Triangle,
                's' => Tone::Square,
                'p' => Tone::Pulse,
                'n' => Tone::Noise,
                _ => panic!("invalid sound tone '{}'", c),
            };

            self.tones.push(tone);
        }
    }

    pub fn volume(&self, index: u32) -> Volume {
        let len = self.volumes.len();

        if len > 0 {
            self.volumes[index as usize % len]
        } else {
            MAX_SOUND_VOLUME
        }
    }

    pub fn volumes(&self) -> &Vec<Volume> {
        &self.volumes
    }

    pub fn set_volumes(&mut self, volumes: &str) {
        let volumes = remove_whitespace(volumes);
        let mut chars = volumes.chars();

        self.volumes.clear();

        while let Some(c) = chars.next() {
            if c >= '0' && c <= '7' {
                self.volumes.push(c as Volume - '0' as Volume);
            } else {
                panic!("invalid sound volume '{}'", c);
            }
        }
    }

    pub fn effect(&self, index: u32) -> Effect {
        let len = self.effects.len();

        if len > 0 {
            self.effects[index as usize % len]
        } else {
            Effect::None
        }
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn set_effects(&mut self, effects: &str) {
        let effects = remove_whitespace(effects);
        let mut chars = effects.chars();

        self.effects.clear();

        while let Some(c) = chars.next() {
            let effect = match c {
                'n' => Effect::None,
                's' => Effect::Slide,
                'v' => Effect::Vibrato,
                'f' => Effect::FadeOut,
                _ => panic!("invalid sound effect '{}'", c),
            };

            self.effects.push(effect);
        }
    }

    pub fn speed(&self) -> Speed {
        self.speed
    }

    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sound = Sound::new();
        assert_eq!(sound.note(0), 0);
        assert_eq!(sound.notes().len(), 0);
        assert_eq!(sound.tone(0), Tone::Triangle);
        assert_eq!(sound.tones().len(), 0);
        assert_eq!(sound.volume(0), 7);
        assert_eq!(sound.volumes().len(), 0);
        assert_eq!(sound.effect(0), Effect::None);
        assert_eq!(sound.effects().len(), 0);
        assert_eq!(sound.speed(), 30);
    }

    #[test]
    fn set_notes() {
        let mut sound = Sound::new();

        sound.set_notes(" c 0 d 1 r e 2 f 3 g 4 r a 0 b 1 ");
        assert_eq!(sound.note(1), 14);
        assert_eq!(sound.note(11), -1);
        assert_eq!(sound.notes(), &vec![0, 14, -1, 28, 41, 55, -1, 9, 23]);
    }

    #[test]
    fn set_tones() {
        let mut sound = Sound::new();

        sound.set_tones(" t s p n ");
        assert_eq!(sound.tone(1), Tone::Square);
        assert_eq!(sound.tone(6), Tone::Pulse);
        assert_eq!(
            sound.tones(),
            &vec![Tone::Triangle, Tone::Square, Tone::Pulse, Tone::Noise]
        );
    }

    #[test]
    fn set_volumes() {
        let mut sound = Sound::new();

        sound.set_volumes(" 0 1 2 3 4 5 6 7 ");
        assert_eq!(sound.volume(1), 1);
        assert_eq!(sound.volume(10), 2);
        assert_eq!(sound.volumes(), &vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn set_effects() {
        let mut sound = Sound::new();

        sound.set_effects(" n s v f ");
        assert_eq!(sound.effect(1), Effect::Slide);
        assert_eq!(sound.effect(6), Effect::Vibrato);
        assert_eq!(
            sound.effects(),
            &vec![
                Effect::None,
                Effect::Slide,
                Effect::Vibrato,
                Effect::FadeOut
            ]
        );
    }

    #[test]
    fn set_speed() {
        let mut sound = Sound::new();

        assert_eq!(sound.speed(), 30);
        sound.set_speed(100);
        assert_eq!(sound.speed(), 100);
    }
}

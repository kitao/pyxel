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

    note_table: HashMap<char, Note>,
    tone_table: HashMap<char, Tone>,
    effect_table: HashMap<char, Effect>,
}

impl Sound {
    pub fn new() -> Sound {
        let mut note_table = HashMap::new();
        note_table.insert('c', 0);
        note_table.insert('d', 2);
        note_table.insert('e', 4);
        note_table.insert('f', 5);
        note_table.insert('g', 7);
        note_table.insert('a', 9);
        note_table.insert('b', 11);

        let mut tone_table = HashMap::new();
        tone_table.insert('t', Tone::Triangle);
        tone_table.insert('q', Tone::Square);
        tone_table.insert('p', Tone::Pulse);
        tone_table.insert('n', Tone::Noise);

        let mut effect_table = HashMap::new();
        effect_table.insert('n', Effect::None);
        effect_table.insert('s', Effect::Slide);
        effect_table.insert('v', Effect::Vibrato);
        effect_table.insert('f', Effect::FadeOut);

        Sound {
            notes: Vec::new(),
            tones: Vec::new(),
            volumes: Vec::new(),
            effects: Vec::new(),
            speed: DEFAULT_SOUND_SPEED,

            note_table: note_table,
            tone_table: tone_table,
            effect_table: effect_table,
        }
    }

    pub fn set(&mut self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: Speed) {
        self.set_notes(notes);
        self.set_tones(tones);
        self.set_volumes(volumes);
        self.set_effects(volumes);
        self.set_speed(speed);
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn note(&self, index: u32) -> Note {
        self.notes[index as usize]
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn set_notes(&mut self, notes: &str) {
        let notes = remove_whitespace(notes);
        let mut chars = notes.chars();

        self.notes.clear();

        while let Some(c) = chars.next() {
            let mut note;

            if let Some(_note) = self.note_table.get(&c) {
                note = *_note;
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
                    chars.next();
                } else {
                    panic!();
                    //PYXEL_ERROR("invalid sound note '" + s + "'");
                }
            } else if c == 'r' {
                note = -1;
            } else {
                panic!();
                //PYXEL_ERROR("invalid sound note '" + s + "'");
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
            if let Some(tone) = self.tone_table.get(&c) {
                self.tones.push(*tone);
            } else {
                panic!();
                //PYXEL_ERROR("invalid sound tone '" + s + "'");
            }
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

    pub fn volumes(&mut self) -> &Vec<Volume> {
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
                panic!();
                //PYXEL_ERROR("invalid sound volume '" + s + "'");
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

    pub fn effects(&mut self) -> &Vec<Effect> {
        &mut self.effects
    }

    pub fn set_effects(&mut self, effects: &str) {
        let effects = remove_whitespace(effects);
        let mut chars = effects.chars();

        self.effects.clear();

        while let Some(c) = chars.next() {
            if let Some(effect) = self.effect_table.get(&c) {
                self.effects.push(*effect);
            } else {
                panic!();
                //PYXEL_ERROR("invalid sound effect '" + s + "'");
            }
            //
        }
    }

    pub fn speed(&self) -> Speed {
        self.speed
    }

    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }
}

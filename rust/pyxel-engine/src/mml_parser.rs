use std::iter::{repeat, Peekable};

use crate::channel::{Note, Volume};
use crate::oscillator::ToneIndex;
use crate::settings::{
    EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO,
};
use crate::sound::Sound;
use crate::EFFECT_QUARTER_FADEOUT;

#[derive(Copy, Clone)]
struct NoteInfo {
    length: u32,
    quantize: u32,
    tone: ToneIndex,
    volume: Volume,
    vibrato: bool,
    note: Note,
}

impl Sound {
    pub fn mml(&mut self, mml_str: &str) {
        let mut chars = mml_str.chars().peekable();
        let mut length = 4;
        let mut quantize = 7;
        let mut octave = 2;
        let mut tone = 0;
        let mut volume = 7;
        let mut vibrato = false;
        let mut note_info = NoteInfo {
            length: 0,
            quantize: 0,
            tone: 0,
            volume: 0,
            vibrato: false,
            note: 0,
        };
        self.speed = 9; // T=100
        while chars.peek().is_some() {
            if let Some(value) = Self::parse_command(&mut chars, 't') {
                self.speed = (900 / value).max(1);
            } else if let Some(value) = Self::parse_command(&mut chars, 'l') {
                if value <= 32 && 32 % value == 0 {
                    length = 32 / value;
                } else {
                    panic!("Invalid note length '{value}' in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, 'q') {
                if (1..=8).contains(&value) {
                    quantize = value;
                } else {
                    panic!("Invalid quantize value '{value}' in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, 'o') {
                if value <= 4 {
                    octave = value as Note;
                } else {
                    panic!("Invalid octave value '{value}' in MML");
                }
            } else if Self::parse_char(&mut chars, '>') {
                if octave < 4 {
                    octave += 1;
                } else {
                    panic!("Octave exceeded maximum in MML");
                }
            } else if Self::parse_char(&mut chars, '<') {
                if octave > 0 {
                    octave -= 1;
                } else {
                    panic!("Octave exceeded minimum in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, '@') {
                if value <= 3 {
                    tone = value as ToneIndex;
                } else {
                    panic!("Invalid tone value '{value}' in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, 'v') {
                if value <= 7 {
                    volume = value as Volume;
                } else {
                    panic!("Invalid volume value '{value}' in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, 'w') {
                if value == 0 {
                    vibrato = false;
                } else if value == 1 {
                    vibrato = true;
                } else {
                    panic!("Invalid vibrato value '{value}' in MML");
                }
            } else if Self::parse_char(&mut chars, '&') {
                note_info.quantize = 8;
            } else if let Some((note, length)) = Self::parse_note(&mut chars, length) {
                self.add_note_info(note_info);
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volume,
                    vibrato,
                    note: note + octave * 12,
                };
            } else if let Some(length) = Self::parse_rest(&mut chars, length) {
                self.add_note_info(note_info);
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volume,
                    vibrato,
                    note: -1,
                };
            } else {
                let c = chars.peek().unwrap();
                panic!("Invalid command '{c}' in MML");
            }
        }
        self.add_note_info(note_info);
    }

    fn skip_whitespace<T: Iterator<Item = char>>(chars: &mut Peekable<T>) {
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_number<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Option<u32> {
        Self::skip_whitespace(chars);
        let mut number_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                number_str.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        if number_str.is_empty() {
            None
        } else {
            number_str.parse().ok()
        }
    }

    fn parse_char<T: Iterator<Item = char>>(chars: &mut Peekable<T>, target: char) -> bool {
        Self::skip_whitespace(chars);
        if let Some(&c) = chars.peek() {
            if c.eq_ignore_ascii_case(&target) {
                chars.next();
                return true;
            }
        }
        false
    }

    fn parse_command<T: Iterator<Item = char>>(
        chars: &mut Peekable<T>,
        target: char,
    ) -> Option<u32> {
        if Self::parse_char(chars, target) {
            if let Some(number) = Self::parse_number(chars) {
                return Some(number);
            }
            panic!("Missing value after '{target}' in MML");
        }
        None
    }

    fn parse_note<T: Iterator<Item = char>>(
        chars: &mut Peekable<T>,
        length: u32,
    ) -> Option<(Note, u32)> {
        // Parse note
        Self::skip_whitespace(chars);
        let mut note = match chars.peek()?.to_ascii_lowercase() {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => return None,
        };
        chars.next();

        // Parse modifiers
        if Self::parse_char(chars, '#') || Self::parse_char(chars, '+') {
            note += 1;
        } else if Self::parse_char(chars, '-') {
            note -= 1;
        }

        // Parse length
        let mut length = if let Some(note_division) = Self::parse_number(chars) {
            32 / note_division
        } else {
            length
        };

        // Parse dot
        if Self::parse_char(chars, '.') {
            if length >= 2 {
                length += length / 2;
            } else {
                panic!("Length added by dot is too short in MML");
            }
        }

        Some((note, length))
    }

    fn parse_rest<T: Iterator<Item = char>>(chars: &mut Peekable<T>, length: u32) -> Option<u32> {
        // Prase rest
        if !Self::parse_char(chars, 'r') {
            return None;
        }

        // Parse length
        let mut length = if let Some(note_division) = Self::parse_number(chars) {
            32 / note_division
        } else {
            length
        };

        // Parse dot
        if Self::parse_char(chars, '.') {
            if length >= 2 {
                length += length / 2;
            } else {
                panic!("Length added by dot is too short in MML");
            }
        }

        Some(length)
    }

    fn add_note_info(&mut self, note_info: NoteInfo) {
        if note_info.length == 0 {
            return;
        }

        let mut duration = note_info.length * note_info.quantize;
        for i in 0..note_info.length {
            if duration == 0 {
                self.notes.push(-1);
                self.effects.push(EFFECT_NONE);
            } else if duration >= 8 {
                self.notes.push(note_info.note);
                self.effects.push(if note_info.vibrato {
                    EFFECT_VIBRATO
                } else {
                    EFFECT_NONE
                });
                duration -= 8;
            } else if duration >= 7 {
                self.notes.push(note_info.note);
                self.effects.push(EFFECT_QUARTER_FADEOUT);
                duration = 0;
            } else if duration >= 5 {
                self.notes.push(note_info.note);
                self.effects.push(EFFECT_HALF_FADEOUT);
                duration = 0;
            } else if duration >= 3 || note_info.length == 1 {
                self.notes.push(note_info.note);
                self.effects.push(EFFECT_FADEOUT);
                duration = 0;
            } else {
                self.notes.push(-1);
                self.effects.push(EFFECT_NONE);
                duration = 0;
            }
        }
        self.tones
            .extend(repeat(note_info.tone).take(note_info.length as usize));
        self.volumes
            .extend(repeat(note_info.volume).take(note_info.length as usize));
    }
}

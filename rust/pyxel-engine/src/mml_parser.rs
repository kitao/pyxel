use std::iter::Peekable;

use crate::channel::{Note, Volume};
use crate::oscillator::ToneIndex;
use crate::settings::{EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_VIBRATO};
use crate::sound::Sound;

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
            } else if let Some((note, length)) = Self::parse_note(&mut chars, length) {
                self.add_note(note_info);
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volume,
                    vibrato: false,
                    note: note + octave * 12,
                };
            } else if let Some(length) = Self::parse_rest(&mut chars, length) {
                self.add_note(note_info);
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volume,
                    vibrato: false,
                    note: -1,
                };
            } else if Self::parse_char(&mut chars, '!') {
                note_info.vibrato = true;
            } else if Self::parse_char(&mut chars, '&') {
                note_info.quantize = 4;
            } else {
                let c = chars.peek().unwrap();
                panic!("Invalid command '{c}' in MML");
            }
        }
        self.add_note(note_info);
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

        // Parse modifier
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

    fn add_note(&mut self, note_info: NoteInfo) {
        // Hnadle empty note
        if note_info.length == 0 {
            return;
        }

        // Add tones and volumes
        repeat_extend!(&mut self.tones, note_info.tone, note_info.length);
        repeat_extend!(&mut self.volumes, note_info.volume, note_info.length);

        // Handle rest note
        if note_info.note == -1 {
            repeat_extend!(&mut self.notes, -1, note_info.length);
            repeat_extend!(&mut self.effects, EFFECT_NONE, note_info.length);
            return;
        }

        // Add full-length notes
        let num_notes = if note_info.quantize == 8 {
            note_info.length
        } else {
            (note_info.length * note_info.quantize / 8).saturating_sub(1)
        };
        let note_effect = if note_info.vibrato {
            EFFECT_VIBRATO
        } else {
            EFFECT_NONE
        };
        repeat_extend!(&mut self.notes, note_info.note, num_notes);
        repeat_extend!(&mut self.effects, note_effect, num_notes);
        if num_notes == note_info.length {
            return;
        }

        // Add fade-out note
        self.notes.push(note_info.note);
        self.effects.push(EFFECT_HALF_FADEOUT);

        // Add rests
        let num_rests = note_info.length - num_notes - 1;
        repeat_extend!(&mut self.notes, -1, num_rests);
        repeat_extend!(&mut self.effects, EFFECT_NONE, num_rests);
    }
}

use core::panic;
use std::array;
use std::iter::Peekable;

use crate::channel::{Note, Volume};
use crate::oscillator::ToneIndex;
use crate::settings::{
    EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_VIBRATO,
};
use crate::sound::Sound;

type Envelope = u32;

enum ExtendeVolume {
    Constant(Volume),
    Envelope(Envelope),
}

struct NoteInfo {
    length: u32,
    quantize: u32,
    tone: ToneIndex,
    volumes: Vec<Volume>,
    vol_index: u32,
    vibrato: bool,
    note: Note,
    is_tied: bool,
}

impl Sound {
    pub fn mml(&mut self, mml_str: &str) {
        let mut chars = mml_str.chars().peekable();
        let mut length = 4;
        let mut quantize = 7;
        let mut octave = 2;
        let mut tone = 0;
        let mut volenv = ExtendeVolume::Constant(7);
        let mut vol_data: [Vec<Volume>; 8] = array::from_fn(|_| vec![7]);
        let mut note_info = NoteInfo {
            length: 0,
            quantize: 0,
            tone: 0,
            volumes: vol_data[0].clone(),
            vol_index: 0,
            vibrato: false,
            note: 0,
            is_tied: false,
        };
        self.speed = 9; // T=100
        while chars.peek().is_some() {
            if let Some(value) = Self::parse_command(&mut chars, 't') {
                self.speed = (900 / value).max(1);
            } else if Self::parse_char(&mut chars, 'l') {
                if let Some(local_length) = Self::parse_note_length(&mut chars) {
                    length = local_length;
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
            } else if let Some(value) = Self::parse_command(&mut chars, 's') {
                if value <= 3 {
                    tone = value as ToneIndex;
                } else {
                    panic!("Invalid tone value '{value}' in MML");
                }
            } else if let Some(value) = Self::parse_command(&mut chars, 'v') {
                if value <= 7 {
                    volenv = ExtendeVolume::Constant(value as Volume);
                } else {
                    panic!("Invalid volume value '{value}' in MML");
                }
            } else if let Some((envelope, volumes)) = Self::parse_envelope(&mut chars) {
                volenv = ExtendeVolume::Envelope(envelope);
                if !volumes.is_empty() {
                    vol_data[envelope as usize] = volumes;
                }
            } else if let Some((note, length)) = Self::parse_note(&mut chars, length) {
                self.add_note(&note_info);
                let note = note + octave * 12;
                let volumes = match volenv {
                    ExtendeVolume::Constant(volume) => vec![volume],
                    ExtendeVolume::Envelope(envelope) => vol_data[envelope as usize].clone(),
                };
                let vol_index = if note_info.is_tied && note_info.note == note {
                    note_info.length + note_info.vol_index
                } else {
                    0
                };
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volumes,
                    vol_index,
                    vibrato: false,
                    note,
                    is_tied: false,
                };
            } else if let Some(length) = Self::parse_rest(&mut chars, length) {
                self.add_note(&note_info);
                note_info = NoteInfo {
                    length,
                    quantize,
                    tone,
                    volumes: vec![0],
                    vol_index: 0,
                    vibrato: false,
                    note: -1,
                    is_tied: false,
                };
            } else if Self::parse_char(&mut chars, '~') {
                note_info.vibrato = true;
            } else if Self::parse_char(&mut chars, '&') {
                note_info.quantize = 8;
                note_info.is_tied = true;
            } else {
                let c = chars.peek().unwrap();
                panic!("Invalid command '{c}' in MML");
            }
        }
        self.add_note(&note_info);
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

    fn parse_envelope<T: Iterator<Item = char>>(
        chars: &mut Peekable<T>,
    ) -> Option<(Envelope, Vec<Volume>)> {
        let envelope = Self::parse_command(chars, 'm');
        envelope?;
        let envelope = envelope.unwrap();
        assert!(envelope <= 7, "Invalid envelope value '{envelope}' in MML");
        let mut volumes = Vec::new();
        if !Self::parse_char(chars, ':') {
            return Some((envelope, volumes));
        }
        Self::skip_whitespace(chars);
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                let volume = chars.next().unwrap().to_string().parse().unwrap();
                if volume <= 7 {
                    volumes.push(volume);
                } else {
                    panic!("Invalid envlope volume '{volume}' in MML");
                }
            } else {
                break;
            }
            Self::skip_whitespace(chars);
        }
        assert!(!volumes.is_empty(), "Missing envelope volumes in MML");
        Some((envelope, volumes))
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
        let mut length = if let Some(local_length) = Self::parse_note_length(chars) {
            local_length
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

    fn parse_note_length<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Option<u32> {
        if let Some(length) = Self::parse_number(chars) {
            if length <= 32 && 32 % length == 0 {
                Some(32 / length)
            } else {
                panic!("Invalid note length '{length}' in MML");
            }
        } else {
            None
        }
    }

    fn parse_rest<T: Iterator<Item = char>>(chars: &mut Peekable<T>, length: u32) -> Option<u32> {
        // Prase rest
        if !Self::parse_char(chars, 'r') {
            return None;
        }

        // Parse length
        let mut length = if let Some(local_length) = Self::parse_note_length(chars) {
            local_length
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

    fn add_note(&mut self, note_info: &NoteInfo) {
        // Hnadle empty note
        if note_info.length == 0 {
            return;
        }

        // Add tones and volumes
        repeat_extend!(&mut self.tones, note_info.tone, note_info.length);
        for i in 0..note_info.length {
            let vol_index = ((note_info.vol_index + i) as usize).min(note_info.volumes.len() - 1);
            self.volumes.push(note_info.volumes[vol_index]);
        }

        // Handle rest note
        if note_info.note == -1 {
            repeat_extend!(&mut self.notes, -1, note_info.length);
            repeat_extend!(&mut self.effects, EFFECT_NONE, note_info.length);
            return;
        }

        // Add full-length notes
        let duration = note_info.length * note_info.quantize;
        let num_notes = duration / 8;
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
        if num_notes > 0 {
            self.effects.push(EFFECT_FADEOUT);
        } else if duration >= 6 {
            self.effects.push(EFFECT_QUARTER_FADEOUT);
        } else if duration >= 4 {
            self.effects.push(EFFECT_HALF_FADEOUT);
        } else {
            self.effects.push(EFFECT_FADEOUT);
        }

        // Add rests
        let num_rests = note_info.length - num_notes - 1;
        repeat_extend!(&mut self.notes, -1, num_rests);
        repeat_extend!(&mut self.effects, EFFECT_NONE, num_rests);
    }
}

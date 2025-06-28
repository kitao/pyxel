use std::iter::Peekable;

use crate::mml_command::MmlCommand;
use crate::TICKS_PER_QUARTER_NOTE;

pub fn parse_mml(mml: &str) -> Vec<MmlCommand> {
    let mml = mml
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_uppercase();

    let mut mml_commands = Vec::new();

    let mut chars = mml.chars().peekable();
    let mut length = 4;
    let mut octave = 2;

    while chars.peek().is_some() {
        if let Some(bpm) = parse_command(&mut chars, "T", (1, i16::MAX as i32)) {
            // T[bpm]
            mml_commands.push(MmlCommand::Tempo {
                clocks_per_tick: MmlCommand::bpm_to_cpt(bpm),
            });
        } else if let Some(gate_time) = parse_command(&mut chars, "Q", (1, 8)) {
            // Q[gate:1-8]
            mml_commands.push(MmlCommand::Quantize {
                gate_ratio: gate_time as f64 / 8.0,
            });
        } else if let Some(tone_index) = parse_command(&mut chars, "@", (0, i16::MAX as i32)) {
            // @[tone_index]
            mml_commands.push(MmlCommand::Tone {
                tone_index: tone_index as u8,
            });
        } else if let Some(volume) = parse_command(&mut chars, "V", (0, 15)) {
            // V[volume:0-15]
            mml_commands.push(MmlCommand::Volume {
                level: volume as f64,
            });
        } else if let Some(key_offset) = parse_command(&mut chars, "K", (0, 15)) {
            // K[key_offset]
            mml_commands.push(MmlCommand::Transpose {
                semitone_offset: key_offset as f64,
            });
        } else if let Some(semitone_centes) = parse_command(&mut chars, "Y", (0, 15)) {
            // Y[offset_cents]
            mml_commands.push(MmlCommand::Detune {
                semitone_offset: MmlCommand::cents_to_semitones(semitone_centes),
            });
        } else if let Some(slot) = parse_command(&mut chars, "@ENV", (0, i16::MAX as i32)) {
            // @ENV[slot]
            // @ENV[slot] { initial_volume, duration_ticks1, target_volume1, ... }
            //
        } else if let Some(slot) = parse_command(&mut chars, "@VIB", (0, i16::MAX as i32)) {
            // @VIB[slot]
            // @VIB[slot] { delay_ticks, frequency_chz, depth_cents }
            //
        } else if let Some(slot) = parse_command(&mut chars, "@GLI", (0, i16::MAX as i32)) {
            // @GLI[slot]
            // @GLI[slot] { offset_cents, duration_ticks }
            //
        } else if let Some(octave_value) = parse_command(&mut chars, "O", (0, 4)) {
            // O[octave]
            octave = octave_value as u8;
        } else if parse_keyword(&mut chars, ">") {
            // >
            if octave < 4 {
                octave += 1;
            } else {
                panic!("Octave exceeded maximum in MML");
            }
        } else if parse_keyword(&mut chars, "<") {
            // <
            if octave > 0 {
                octave -= 1;
            } else {
                panic!("Octave exceeded minimum in MML");
            }
        } else if parse_keyword(&mut chars, "L") {
            // L[length]
            length = parse_note_length(&mut chars, length);
        }
    }

    /*while chars.peek().is_some() {
        } else if let Some((note, length)) = parse_note(&mut chars, length) {
            self.add_note(&note_info);

            let note = note + octave * 12;
            let env_data = match vol_env {
                VolEnv::Constant(volume) => vec![volume],
                VolEnv::Envelope(envelope) => envelopes[envelope as usize].clone(),
            };
            let env_start = if note_info.is_tied && note_info.note == note {
                note_info.length + note_info.env_start
            } else {
                0
            };

            note_info = NoteInfo {
                length,
                quantize,
                tone,
                env_start,
                env_data,
                vibrato: false,
                note,
                is_tied: false,
            };
        } else if let Some(length) = parse_rest(&mut chars, length) {
            self.add_note(&note_info);

            note_info = NoteInfo {
                length,
                quantize,
                tone,
                env_start: 0,
                env_data: vec![0],
                vibrato: false,
                note: -1,
                is_tied: false,
            };
        } else if parse_char(&mut chars, '~') {
            note_info.vibrato = true;
        } else if parse_char(&mut chars, '&') {
            note_info.quantize = 8;
            note_info.is_tied = true;
        } else {
            let c = chars.peek().unwrap();
            panic!("Invalid command '{c}' in MML");
        }
    }

    self.add_note(&note_info);
    */

    mml_commands
}

fn parse_number<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    range: (i32, i32),
) -> Option<i32> {
    let mut number_str = String::new();

    if let Some(&c) = chars.peek() {
        if c == '-' {
            number_str.push(chars.next().unwrap());
        }
    }

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            number_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    if number_str.is_empty() {
        return None;
    }

    let value = number_str.parse::<i32>().ok()?;

    let (min, max) = range;
    if value < min || value > max {
        panic!("'{value}' out of range in MML");
    }

    Some(value)
}

fn parse_keyword<T: Iterator<Item = char>>(chars: &mut Peekable<T>, target: &str) -> bool {
    for expected in target.chars() {
        match chars.peek() {
            Some(&c) if c.eq_ignore_ascii_case(&expected) => {
                chars.next();
            }
            _ => return false,
        }
    }

    true
}

fn parse_command<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    target: &str,
    range: (i32, i32),
) -> Option<i32> {
    if parse_keyword(chars, target) {
        if let Some(number) = parse_number(chars, range) {
            return Some(number);
        }

        panic!("Missing value after '{target}' in MML");
    }

    None
}

fn parse_params<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Option<Vec<i32>> {
    if !parse_keyword(chars, "{") {
        return None;
    }

    let mut params = Vec::new();

    loop {
        if let Some(value) = parse_number(chars, (i32::MIN, i32::MAX)) {
            params.push(value);
        } else if parse_keyword(chars, ",") {
            continue;
        } else if parse_keyword(chars, "}") {
            break;
        } else {
            panic!("Invalid parameter format in MML");
        }
    }

    Some(params)
}

fn parse_note<T: Iterator<Item = char>>(chars: &mut Peekable<T>, length: u16) -> Option<(u8, u16)> {
    let mut note = match chars.peek()?.to_ascii_lowercase() {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };
    chars.next();

    if parse_keyword(chars, "#") || parse_keyword(chars, "+") {
        note += 1;
    } else if parse_keyword(chars, "-") {
        note -= 1;
    }

    Some((note, parse_note_length(chars, length)))
}

fn parse_note_length<T: Iterator<Item = char>>(chars: &mut Peekable<T>, length: u16) -> u16 {
    let mut length = length;

    if let Some(len) = parse_number(chars, (0, TICKS_PER_QUARTER_NOTE as i32 * 4)) {
        if (TICKS_PER_QUARTER_NOTE * 4) % len as u32 == 0 {
            length = TICKS_PER_QUARTER_NOTE as u16 * 4 / len as u16;
        } else {
            panic!("Invalid note length '{len}' in MML");
        }
    }

    while parse_keyword(chars, ".") {
        if length % 2 == 0 {
            length += length / 2;
        } else {
            panic!("Dotted note length not divisible in MML");
        }
    }

    length
}

fn parse_rest<T: Iterator<Item = char>>(chars: &mut Peekable<T>, length: u16) -> Option<u16> {
    if !parse_keyword(chars, "r") {
        return None;
    }

    Some(parse_note_length(chars, length))
}

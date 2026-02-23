use std::array;
use std::iter::Peekable;

use crate::mml_command::MmlCommand;
use crate::settings::{
    EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE, EFFECT_QUARTER_FADEOUT, EFFECT_VIBRATO,
};
use crate::sound::{Sound, SoundNote, SoundSpeed, SoundTone, SoundVolume};

type EnvIndex = u32;
type EnvData = Vec<SoundVolume>;

enum VolEnv {
    Constant(SoundVolume),
    Envelope(EnvIndex),
}

#[derive(Default)]
struct NoteInfo {
    length: u32,
    quantize: u32,
    tone: SoundTone,
    env_start: u32,
    env_data: EnvData,
    vibrato: bool,
    note: SoundNote,
    is_tied: bool,
}

pub fn parse_old_mml(mml: &str) -> Result<Vec<MmlCommand>, String> {
    let sound_ptr = Sound::new();
    let sound = unsafe { &mut *sound_ptr };
    let mut chars = mml.chars().peekable();
    let mut length = 4;
    let mut quantize = 7;
    let mut octave = 2;
    let mut tone = 0;
    let mut vol_env = VolEnv::Constant(7);
    let mut envelopes: [EnvData; 8] = array::from_fn(|_| vec![7]);
    let mut note_info = NoteInfo::default();
    sound.speed = 9; // T=100

    while chars.peek().is_some() {
        if let Some(value) = parse_command(&mut chars, 't')? {
            sound.speed = (900 / value).max(1) as SoundSpeed;
        } else if parse_char(&mut chars, 'l') {
            length = parse_note_length(&mut chars, length)?;
        } else if let Some(value) = parse_command(&mut chars, '@')? {
            if value <= 3 {
                tone = value as SoundTone;
            } else {
                return Err(format!("Invalid tone value '{value}' in MML"));
            }
        } else if let Some(value) = parse_command(&mut chars, 'o')? {
            if value <= 4 {
                octave = value as SoundNote;
            } else {
                return Err(format!("Invalid octave value '{value}' in MML"));
            }
        } else if parse_char(&mut chars, '>') {
            if octave < 4 {
                octave += 1;
            } else {
                return Err("Octave exceeded maximum in MML".to_string());
            }
        } else if parse_char(&mut chars, '<') {
            if octave > 0 {
                octave -= 1;
            } else {
                return Err("Octave exceeded minimum in MML".to_string());
            }
        } else if let Some(value) = parse_command(&mut chars, 'q')? {
            if (1..=8).contains(&value) {
                quantize = value;
            } else {
                return Err(format!("Invalid quantize value '{value}' in MML"));
            }
        } else if let Some(value) = parse_command(&mut chars, 'v')? {
            if value <= 7 {
                vol_env = VolEnv::Constant(value as SoundVolume);
            } else {
                return Err(format!("Invalid volume value '{value}' in MML"));
            }
        } else if let Some((env_index, env_data)) = parse_envelope(&mut chars)? {
            vol_env = VolEnv::Envelope(env_index);
            if !env_data.is_empty() {
                envelopes[env_index as usize] = env_data;
            }
        } else if let Some((note, length)) = parse_note(&mut chars, length)? {
            add_note(sound, &note_info);

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
        } else if let Some(length) = parse_rest(&mut chars, length)? {
            add_note(sound, &note_info);

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
            return Err(format!("Invalid command '{c}' in MML"));
        }
    }

    add_note(sound, &note_info);

    Ok(sound.to_commands())
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
    skip_whitespace(chars);

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
    skip_whitespace(chars);

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
) -> Result<Option<u32>, String> {
    if parse_char(chars, target) {
        if let Some(number) = parse_number(chars) {
            return Ok(Some(number));
        }

        return Err(format!("Missing value after '{target}' in MML"));
    }

    Ok(None)
}

fn parse_envelope<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
) -> Result<Option<(EnvIndex, EnvData)>, String> {
    let envelope = parse_command(chars, 'x')?;
    let Some(envelope) = envelope else {
        return Ok(None);
    };

    if envelope > 7 {
        return Err(format!("Invalid envelope value '{envelope}' in MML"));
    }

    let mut env_data = Vec::new();
    if !parse_char(chars, ':') {
        return Ok(Some((envelope, env_data)));
    }

    skip_whitespace(chars);
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            let volume = chars.next().unwrap().to_string().parse().unwrap();
            if volume <= 7 {
                env_data.push(volume);
            } else {
                return Err(format!("Invalid envlope volume '{volume}' in MML"));
            }
        } else {
            break;
        }

        skip_whitespace(chars);
    }

    if env_data.is_empty() {
        return Err("Missing envelope volumes in MML".to_string());
    }
    Ok(Some((envelope, env_data)))
}

fn parse_note<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    length: u32,
) -> Result<Option<(SoundNote, u32)>, String> {
    skip_whitespace(chars);

    let mut note = match chars.peek().map(char::to_ascii_lowercase) {
        Some('c') => 0,
        Some('d') => 2,
        Some('e') => 4,
        Some('f') => 5,
        Some('g') => 7,
        Some('a') => 9,
        Some('b') => 11,
        _ => return Ok(None),
    };
    chars.next();

    if parse_char(chars, '#') || parse_char(chars, '+') {
        note += 1;
    } else if parse_char(chars, '-') {
        note -= 1;
    }

    Ok(Some((note, parse_note_length(chars, length)?)))
}

fn parse_note_length<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    cur_length: u32,
) -> Result<u32, String> {
    let mut length = cur_length;

    if let Some(temp_length) = parse_number(chars) {
        if temp_length <= 32 && 32 % temp_length == 0 {
            length = 32 / temp_length;
        } else {
            return Err(format!("Invalid note length '{temp_length}' in MML"));
        }
    }

    let mut target_length = length;
    while parse_char(chars, '.') {
        if target_length >= 2 {
            target_length /= 2;
            length += target_length;
        } else {
            return Err("Length added by dot is too short in MML".to_string());
        }
    }

    Ok(length)
}

fn parse_rest<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    cur_length: u32,
) -> Result<Option<u32>, String> {
    if !parse_char(chars, 'r') {
        return Ok(None);
    }

    Ok(Some(parse_note_length(chars, cur_length)?))
}

fn add_note(sound: &mut Sound, note_info: &NoteInfo) {
    // Hnadle empty note
    if note_info.length == 0 {
        return;
    }

    // Add tones and volumes
    repeat_extend!(&mut sound.tones, note_info.tone, note_info.length);
    for i in 0..note_info.length {
        let env_start = ((note_info.env_start + i) as usize).min(note_info.env_data.len() - 1);
        sound.volumes.push(note_info.env_data[env_start]);
    }

    // Handle rest note
    if note_info.note == -1 {
        repeat_extend!(&mut sound.notes, -1, note_info.length);
        repeat_extend!(&mut sound.effects, EFFECT_NONE, note_info.length);
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

    repeat_extend!(&mut sound.notes, note_info.note, num_notes);
    repeat_extend!(&mut sound.effects, note_effect, num_notes);
    if num_notes == note_info.length {
        return;
    }

    // Add fade-out note
    sound.notes.push(note_info.note);
    if num_notes > 0 {
        sound.effects.push(EFFECT_FADEOUT);
    } else if duration >= 6 {
        sound.effects.push(EFFECT_QUARTER_FADEOUT);
    } else if duration >= 4 {
        sound.effects.push(EFFECT_HALF_FADEOUT);
    } else {
        sound.effects.push(EFFECT_FADEOUT);
    }

    // Add rests
    let num_rests = note_info.length - num_notes - 1;
    repeat_extend!(&mut sound.notes, -1, num_rests);
    repeat_extend!(&mut sound.effects, EFFECT_NONE, num_rests);
}

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

    let commands = sound.to_commands();
    unsafe {
        drop(Box::from_raw(sound_ptr));
    }
    Ok(commands)
}

fn skip_whitespace<T: Iterator<Item = char>>(chars: &mut Peekable<T>) {
    while chars.peek().is_some_and(|c| c.is_whitespace()) {
        chars.next();
    }
}

fn parse_number<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> Option<u32> {
    skip_whitespace(chars);

    let mut number_str = String::new();
    while chars.peek().is_some_and(char::is_ascii_digit) {
        number_str.push(chars.next().unwrap());
    }

    if number_str.is_empty() {
        None
    } else {
        number_str.parse().ok()
    }
}

fn parse_char<T: Iterator<Item = char>>(chars: &mut Peekable<T>, target: char) -> bool {
    skip_whitespace(chars);
    if chars
        .peek()
        .is_some_and(|c| c.eq_ignore_ascii_case(&target))
    {
        chars.next();
        return true;
    }
    false
}

fn parse_command<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
    target: char,
) -> Result<Option<u32>, String> {
    if !parse_char(chars, target) {
        return Ok(None);
    }
    parse_number(chars)
        .map(Some)
        .ok_or_else(|| format!("Missing value after '{target}' in MML"))
}

fn parse_envelope<T: Iterator<Item = char>>(
    chars: &mut Peekable<T>,
) -> Result<Option<(EnvIndex, EnvData)>, String> {
    let Some(envelope) = parse_command(chars, 'x')? else {
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
        if !c.is_ascii_digit() {
            break;
        }
        let volume = (chars.next().unwrap() as u32) - ('0' as u32);
        if volume > 7 {
            return Err(format!("Invalid envelope volume '{volume}' in MML"));
        }
        env_data.push(volume as SoundVolume);
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

    let Some(mut note) = chars.peek().and_then(|c| match c.to_ascii_lowercase() {
        'c' => Some(0),
        'd' => Some(2),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(7),
        'a' => Some(9),
        'b' => Some(11),
        _ => None,
    }) else {
        return Ok(None);
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
    parse_note_length(chars, cur_length).map(Some)
}

fn add_note(sound: &mut Sound, note_info: &NoteInfo) {
    // Handle empty note
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

#[cfg(test)]
mod tests {
    use super::*;

    fn first_midi_note(commands: &[MmlCommand]) -> Option<u32> {
        commands.iter().find_map(|c| match c {
            MmlCommand::Note { midi_note, .. } => Some(*midi_note),
            _ => None,
        })
    }

    fn total_ticks(commands: &[MmlCommand]) -> u32 {
        commands
            .iter()
            .map(|c| match c {
                MmlCommand::Note { duration_ticks, .. } => *duration_ticks,
                MmlCommand::Rest { duration_ticks } => *duration_ticks,
                _ => 0,
            })
            .sum()
    }

    fn note_count(commands: &[MmlCommand]) -> usize {
        commands
            .iter()
            .filter(|c| matches!(c, MmlCommand::Note { .. }))
            .count()
    }

    #[test]
    fn test_parse_errors() {
        let cases = [
            ("z", "Invalid command"),
            ("o5", "Invalid octave"),
            ("o4>", "Octave exceeded maximum"),
            ("o0<", "Octave exceeded minimum"),
            ("@4", "Invalid tone"),
            ("v8", "Invalid volume"),
            ("q0", "Invalid quantize"),
            ("c3", "Invalid note length"),
            ("tc", "Missing value"),
            ("x8c", "Invalid envelope"),
        ];
        for (input, expected) in cases {
            let result = parse_old_mml(input);
            assert!(result.is_err(), "Expected error for '{input}'");
            let err = result.unwrap_err();
            assert!(
                err.contains(expected),
                "For '{input}': expected '{expected}' in '{err}'"
            );
        }
    }

    #[test]
    fn test_parse_basic() {
        // Inputs that should parse successfully and produce notes
        let inputs = [
            "c",
            "cde",
            "c#",
            "c-",
            "r",
            "t120c",
            "v3c",
            "@1c",
            "x0:7654321c",
            "c&c",
        ];
        for input in inputs {
            let commands = parse_old_mml(input).unwrap_or_else(|e| panic!("'{input}' failed: {e}"));
            assert!(total_ticks(&commands) > 0, "'{input}' produced no ticks");
        }
    }

    #[test]
    fn test_empty_string() {
        let commands = parse_old_mml("").unwrap();
        assert_eq!(total_ticks(&commands), 0);
    }

    #[test]
    fn test_note_midi_values() {
        // Default octave=2, Wavetable base_note=36: midi = semitone + 24 + 36
        let cases = [
            ("c", 60),
            ("c#", 61),
            ("c-", 59),
            ("d", 62),
            ("e", 64),
            ("f", 65),
            ("g", 67),
            ("a", 69),
            ("b", 71),
        ];
        for (input, expected) in cases {
            let commands = parse_old_mml(input).unwrap();
            assert_eq!(
                first_midi_note(&commands),
                Some(expected),
                "midi_note mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn test_all_note_names() {
        let commands = parse_old_mml("cdefgab").unwrap();
        assert!(
            note_count(&commands) >= 7,
            "Expected at least 7 Note commands, got {}",
            note_count(&commands)
        );
    }

    #[test]
    fn test_octave_command() {
        // Wavetable base=36: o2 C = 0+24+36 = 60, o3 C = 0+36+36 = 72
        let midi_o2 = first_midi_note(&parse_old_mml("o2c").unwrap()).unwrap();
        let midi_o3 = first_midi_note(&parse_old_mml("o3c").unwrap()).unwrap();
        assert_eq!(midi_o2, 60);
        assert_eq!(midi_o3, 72);
        assert_eq!(midi_o3 - midi_o2, 12);
    }

    #[test]
    fn test_octave_up_down() {
        // o2> = octave 3 → C = 72, o2>>< = octave 3 → C = 72
        let midi_up = first_midi_note(&parse_old_mml("o2>c").unwrap()).unwrap();
        let midi_cancel = first_midi_note(&parse_old_mml("o2>><c").unwrap()).unwrap();
        assert_eq!(midi_up, 72);
        assert_eq!(midi_up, midi_cancel);
    }

    #[test]
    fn test_length_command() {
        // L8 = 32/8 = 4 ticks, L4 = 32/4 = 8 ticks
        let ticks_l8 = total_ticks(&parse_old_mml("l8c").unwrap());
        let ticks_l4 = total_ticks(&parse_old_mml("l4c").unwrap());
        assert!(
            ticks_l4 > ticks_l8,
            "L4 ({ticks_l4}) should be longer than L8 ({ticks_l8})"
        );
    }

    #[test]
    fn test_dotted_note() {
        // c4. = quarter + eighth = 12 ticks vs c4 = 8 ticks
        let ticks_dotted = total_ticks(&parse_old_mml("c4.").unwrap());
        let ticks_plain = total_ticks(&parse_old_mml("c4").unwrap());
        assert!(
            ticks_dotted > ticks_plain,
            "Dotted ({ticks_dotted}) should be longer than plain ({ticks_plain})"
        );
    }

    #[test]
    fn test_rest_between_notes() {
        let commands = parse_old_mml("crc").unwrap();
        let has_note = commands
            .iter()
            .any(|c| matches!(c, MmlCommand::Note { .. }));
        let has_rest = commands
            .iter()
            .any(|c| matches!(c, MmlCommand::Rest { .. }));
        assert!(has_note);
        assert!(has_rest);
    }

    #[test]
    fn test_vibrato_command() {
        let commands = parse_old_mml("c~d").unwrap();
        let has_vibrato = commands
            .iter()
            .any(|c| matches!(c, MmlCommand::VibratoSet { .. }));
        assert!(has_vibrato);
    }

    #[test]
    fn test_whitespace_and_case() {
        // Whitespace is ignored, commands are case-insensitive
        let midi_lower = first_midi_note(&parse_old_mml("o2c").unwrap());
        let midi_upper = first_midi_note(&parse_old_mml("O2C").unwrap());
        let midi_spaced = first_midi_note(&parse_old_mml(" o2 c ").unwrap());
        assert_eq!(midi_lower, midi_upper);
        assert_eq!(midi_lower, midi_spaced);

        let spaced_count = note_count(&parse_old_mml(" c d e ").unwrap());
        assert!(spaced_count >= 3);
    }
}

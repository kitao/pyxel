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
    let rc = Sound::new();
    let sound = rc_mut!(rc);
    let mut chars = mml.chars().peekable();
    let mut length = 4;
    let mut quantize = 7;
    let mut octave = 2;
    let mut tone = 0;
    let mut vol_env = VolEnv::Constant(7);
    let mut envelopes: [EnvData; 8] = array::from_fn(|_| vec![7]);
    let mut note_info = NoteInfo::default();
    // Old MML T=100 maps to Pyxel speed 9 by default.
    sound.speed = 9;

    // Parse old MML commands
    while chars.peek().is_some() {
        if let Some(value) = parse_command(&mut chars, 't')? {
            if value == 0 {
                return Err("Invalid tempo value '0' in MML".to_string());
            }
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
            // The matchers above consume trailing whitespace; end cleanly
            // when nothing but whitespace remained
            let Some(c) = chars.peek() else { break };
            return Err(format!("Invalid command '{c}' in MML"));
        }
    }

    add_note(sound, &note_info);

    let commands = sound.to_commands();
    Ok(commands)
}

// Parser helpers

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
        if (1..=32).contains(&temp_length) && 32 % temp_length == 0 {
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

// Note emission

fn add_note(sound: &mut Sound, note_info: &NoteInfo) {
    // Skip the initial empty accumulator before the first parsed note.
    if note_info.length == 0 {
        return;
    }

    // Expand tone and envelope data over the note length.
    repeat_extend!(&mut sound.tones, note_info.tone, note_info.length);
    for i in 0..note_info.length {
        let env_start = ((note_info.env_start + i) as usize).min(note_info.env_data.len() - 1);
        sound.volumes.push(note_info.env_data[env_start]);
    }

    // Encode rests as silent note/effect spans.
    if note_info.note == -1 {
        repeat_extend!(&mut sound.notes, -1, note_info.length);
        repeat_extend!(&mut sound.effects, EFFECT_NONE, note_info.length);
        return;
    }

    // Emit gated full-length note steps.
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

    // Select a fade-out tier for the remaining gated step.
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

    // Pad the ungated tail with rests.
    let num_rests = note_info.length - num_notes - 1;
    repeat_extend!(&mut sound.notes, -1, num_rests);
    repeat_extend!(&mut sound.effects, EFFECT_NONE, num_rests);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(mml: &str) -> Vec<MmlCommand> {
        parse_old_mml(mml).unwrap()
    }

    fn note_commands(commands: &[MmlCommand]) -> Vec<(u32, u32)> {
        commands
            .iter()
            .filter_map(|cmd| match cmd {
                MmlCommand::Note {
                    midi_note,
                    duration_ticks,
                } => Some((*midi_note, *duration_ticks)),
                _ => None,
            })
            .collect()
    }

    fn rest_commands(commands: &[MmlCommand]) -> Vec<u32> {
        commands
            .iter()
            .filter_map(|cmd| match cmd {
                MmlCommand::Rest { duration_ticks } => Some(*duration_ticks),
                _ => None,
            })
            .collect()
    }

    fn volume_commands(commands: &[MmlCommand]) -> Vec<f32> {
        commands
            .iter()
            .filter_map(|cmd| match cmd {
                MmlCommand::Volume { level } => Some(*level),
                _ => None,
            })
            .collect()
    }

    fn envelope_slots(commands: &[MmlCommand]) -> Vec<u32> {
        commands
            .iter()
            .filter_map(|cmd| match cmd {
                MmlCommand::Envelope { slot } => Some(*slot),
                _ => None,
            })
            .collect()
    }

    // Notes and octaves

    #[test]
    fn test_default_note() {
        // Default l8 at q7 yields 3 full steps plus 1 fade-out step of c at octave 2
        let cmds = parse("c");
        assert_eq!(note_commands(&cmds), [(60, 9); 4]);
        assert!(rest_commands(&cmds).is_empty());
    }

    #[test]
    fn test_octave_commands() {
        assert_eq!(note_commands(&parse("o3c"))[0].0, 72);
        assert_eq!(note_commands(&parse(">c"))[0].0, 72);
        assert_eq!(note_commands(&parse("<c"))[0].0, 48);
    }

    #[test]
    fn test_sharp_and_flat() {
        assert_eq!(note_commands(&parse("c#"))[0].0, 61);
        assert_eq!(note_commands(&parse("c+"))[0].0, 61);
        assert_eq!(note_commands(&parse("c-"))[0].0, 59);
    }

    // Lengths and tempo

    #[test]
    fn test_tempo_scales_step_ticks() {
        // t100 (default) -> 9 ticks per step; t60 -> 900/60 = 15
        assert_eq!(note_commands(&parse("t60c")), [(60, 15); 4]);
        // Extreme tempo clamps to the 1-tick minimum step
        assert_eq!(note_commands(&parse("t9000c")), [(60, 1); 4]);
    }

    #[test]
    fn test_length_command() {
        // l16 -> 2 steps: 1 full + 1 fade-out
        assert_eq!(note_commands(&parse("l16c")), [(60, 9); 2]);
    }

    #[test]
    fn test_dotted_length() {
        // c4. -> 8 + 4 = 12 steps: 10 full + 1 fade-out + 1 rest at q7
        let cmds = parse("c4.");
        assert_eq!(note_commands(&cmds), [(60, 9); 11]);
        assert_eq!(rest_commands(&cmds), [9]);
    }

    #[test]
    fn test_rest() {
        // r4 -> 8 rest steps
        assert_eq!(rest_commands(&parse("r4")), [9; 8]);
    }

    // Quantize and ties

    #[test]
    fn test_full_quantize_skips_fadeout() {
        // q8 fills the whole length, so no fade-out step and no trailing rest
        let cmds = parse("q8c");
        assert_eq!(note_commands(&cmds), [(60, 9); 4]);
        assert!(rest_commands(&cmds).is_empty());
        // Every note step stays on the constant-volume envelope slot 0
        assert_eq!(envelope_slots(&cmds), [0]);
    }

    #[test]
    fn test_tie_merges_same_note() {
        // c&c spans 8 steps: 7 full + 1 fade-out, with no rest between
        let cmds = parse("c&c");
        assert_eq!(note_commands(&cmds), [(60, 9); 8]);
        assert!(rest_commands(&cmds).is_empty());
    }

    // Envelopes and vibrato

    #[test]
    fn test_envelope_set_and_use() {
        // x1:7531 defines per-step volumes 7,5,3,1 applied from the note head
        let cmds = parse("x1:7531 x1 c");
        let volumes = volume_commands(&cmds);
        let expected = [1.0, 5.0 / 7.0, 3.0 / 7.0, 1.0 / 7.0];
        assert_eq!(volumes.len(), expected.len());
        for (volume, expected) in volumes.iter().zip(expected) {
            assert!((volume - expected).abs() < 1e-4, "volumes: {volumes:?}");
        }
    }

    #[test]
    fn test_vibrato_applies_to_pending_note() {
        // '~' after a note plays that note's full-length steps on vibrato slot 1
        let cmds = parse("c~");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Vibrato { slot: 1 })));
    }

    #[test]
    fn test_short_note_fadeout_tiers() {
        // A note shorter than one full step picks the fade tier from its gated duration:
        // q7 -> 7 ticks -> quarter fade (slot 3); q4 -> half fade (slot 2); q2 -> full fade (slot 1)
        assert_eq!(envelope_slots(&parse("l32q7c")), [3]);
        assert_eq!(envelope_slots(&parse("l32q4c")), [2]);
        assert_eq!(envelope_slots(&parse("l32q2c")), [1]);
    }

    // Errors

    #[test]
    fn test_err_zero_tempo() {
        assert_eq!(
            parse_old_mml("t0c").unwrap_err(),
            "Invalid tempo value '0' in MML"
        );
    }

    #[test]
    fn test_err_zero_length() {
        assert_eq!(
            parse_old_mml("l0c").unwrap_err(),
            "Invalid note length '0' in MML"
        );
        assert_eq!(
            parse_old_mml("c0").unwrap_err(),
            "Invalid note length '0' in MML"
        );
        assert_eq!(
            parse_old_mml("r0").unwrap_err(),
            "Invalid note length '0' in MML"
        );
    }

    #[test]
    fn test_err_out_of_range_values() {
        assert!(parse_old_mml("@4c").is_err());
        assert!(parse_old_mml("o5c").is_err());
        assert!(parse_old_mml("q0c").is_err());
        assert!(parse_old_mml("q9c").is_err());
        assert!(parse_old_mml("v8c").is_err());
        assert!(parse_old_mml("x8c").is_err());
        assert!(parse_old_mml("x1:8c").is_err());
        assert!(parse_old_mml("l3c").is_err());
        assert!(parse_old_mml("c32.").is_err());
    }

    #[test]
    fn test_err_octave_shift_limits() {
        assert!(parse_old_mml("o4>c").is_err());
        assert!(parse_old_mml("o0<c").is_err());
    }

    #[test]
    fn test_trailing_whitespace_ends_parse() {
        // Branch matchers consume trailing whitespace; the parser must end
        // cleanly instead of panicking on the exhausted stream
        assert!(parse_old_mml("c~\n").is_ok());
        assert!(parse_old_mml("v7 ").is_ok());
        assert!(parse_old_mml(" ").is_ok());
    }

    #[test]
    fn test_err_missing_values() {
        assert_eq!(
            parse_old_mml("t").unwrap_err(),
            "Missing value after 't' in MML"
        );
        assert_eq!(
            parse_old_mml("x1:").unwrap_err(),
            "Missing envelope volumes in MML"
        );
        assert!(parse_old_mml("z").is_err());
    }
}

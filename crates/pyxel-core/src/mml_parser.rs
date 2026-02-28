use crate::mml_command::MmlCommand;
use crate::settings::{AUDIO_CLOCK_RATE, TICKS_PER_QUARTER_NOTE};

const RANGE_ALL: (i32, i32) = (i32::MIN, i32::MAX);
const RANGE_GE0: (i32, i32) = (0, i32::MAX);
const RANGE_GE1: (i32, i32) = (1, i32::MAX);

const RANGE_QUANTIZE: (i32, i32) = (0, 100);
const RANGE_VOLUME: (i32, i32) = (0, 127);
const RANGE_OCTAVE: (i32, i32) = (-1, 9);
const RANGE_LENGTH: (i32, i32) = (1, 192);

const DEFAULT_TEMPO: u32 = 120;
const DEFAULT_QUANTIZE: u32 = 80;
const DEFAULT_VOLUME: u32 = 100;
const DEFAULT_OCTAVE: i32 = 4;
const DEFAULT_LENGTH: u32 = 4;

struct CharStream {
    chars: Vec<char>,
    pos: usize,
}

impl CharStream {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn error(&self, message: &str) -> String {
        format!("MML:{}: {}", self.pos, message)
    }
}

macro_rules! parse_error {
    ($stream:expr, $fmt:literal $(, $arg:expr)*) => {
        return Err($stream.error(&format!($fmt $(, $arg)*)))
    };
}

pub fn parse_mml(mml: &str) -> Result<Vec<MmlCommand>, String> {
    let mut stream = CharStream::new(mml);
    let mut commands = Vec::new();

    let mut octave: i32 = DEFAULT_OCTAVE;
    let mut note_ticks: u32 = TICKS_PER_QUARTER_NOTE * 4 / DEFAULT_LENGTH;
    let mut quantize: u32 = DEFAULT_QUANTIZE;

    let mut is_tempo_set = false;
    let mut is_quantize_set = false;
    let mut is_tone_set = false;
    let mut is_volume_set = false;
    let mut is_transpose_set = false;
    let mut is_detune_set = false;
    let mut is_envelope_set = false;
    let mut is_vibrato_set = false;
    let mut is_glide_set = false;

    let mut is_connected;
    let mut connected_note: Option<u32> = None;
    let mut last_note_index: Option<usize> = None;

    // Parse MML commands
    while stream.peek().is_some() {
        is_connected = false;
        if let Some(bpm) = parse_command(&mut stream, "T", RANGE_GE1)? {
            //
            // T<bpm> - Set tempo (bpm >= 1)
            //
            is_tempo_set = true;
            commands.push(MmlCommand::Tempo {
                clocks_per_tick: bpm_to_cpt(bpm),
            });
        } else if let Some(gate_time) = parse_command(&mut stream, "Q", RANGE_QUANTIZE)? {
            //
            // Q<gate_percent> - Set quantize gate time (0 <= gate_percent <= 100)
            //
            is_quantize_set = true;
            quantize = gate_time;
            commands.push(MmlCommand::Quantize {
                gate_ratio: gate_time_to_gate_ratio(gate_time),
            });
        } else if let Some(vol) = parse_command(&mut stream, "V", RANGE_VOLUME)? {
            //
            // V<vol> - Set volume level (0 <= vol <= 127)
            //
            is_volume_set = true;
            commands.push(MmlCommand::Volume {
                level: volume_to_level(vol),
            });
        } else if let Some(key_offset) = parse_command::<i32>(&mut stream, "K", RANGE_ALL)? {
            //
            // K<key_offset> - Transpose key in semitones
            //
            is_transpose_set = true;
            commands.push(MmlCommand::Transpose {
                semitone_offset: key_offset as f32,
            });
        } else if let Some(offset_cents) = parse_command(&mut stream, "Y", RANGE_ALL)? {
            //
            // Y<offset_cents> - Set detune in cents
            //
            is_detune_set = true;
            commands.push(MmlCommand::Detune {
                semitone_offset: cents_to_semitones(offset_cents),
            });
        } else if let Some(command) = parse_envelope(&mut stream)? {
            //
            // @ENV<slot> - Switch to envelope slot (slot >= 0, 0 = off)
            // @ENV<slot> { init_vol, dur_ticks1, vol1, ... } - Define envelope and switch to slot
            //
            is_envelope_set = true;
            commands.push(command);
        } else if let Some(command) = parse_vibrato(&mut stream)? {
            //
            // @VIB<slot> - Switch to vibrato slot (slot >= 0, 0 = off)
            // @VIB<slot> { delay_ticks, period_ticks, depth_cents } - Define vibrato and switch to slot
            //
            is_vibrato_set = true;
            commands.push(command);
        } else if let Some(command) = parse_glide(&mut stream)? {
            //
            // @GLI<slot> - Switch to glide slot (slot >= 0, 0 = off)
            // @GLI<slot> { offset_cents, dur_ticks } - Define glide and switch to slot
            //
            is_glide_set = true;
            commands.push(command);
        } else if let Some(tone) = parse_command(&mut stream, "@", RANGE_GE0)? {
            //
            // @<tone> - Set tone (tone >= 0)
            //
            is_tone_set = true;
            commands.push(MmlCommand::Tone { tone });
        } else if let Some(oct) = parse_command(&mut stream, "O", RANGE_OCTAVE)? {
            //
            // O<oct> - Set octave (-1 <= oct <= 9)
            //
            octave = oct;
        } else if parse_string(&mut stream, ">").is_ok() {
            //
            // > - Octave up
            //
            if octave < RANGE_OCTAVE.1 {
                octave += 1;
            } else {
                parse_error!(stream, "Octave exceeds maximum {}", octave);
            }
        } else if parse_string(&mut stream, "<").is_ok() {
            //
            // < - Octave down
            //
            if octave > RANGE_OCTAVE.0 {
                octave -= 1;
            } else {
                parse_error!(stream, "Octave is below minimum {}", octave);
            }
        } else if parse_string(&mut stream, "L").is_ok() {
            //
            // L<len> - Set default note length (1 <= len <= 192)
            //
            note_ticks = parse_length_as_ticks(&mut stream, note_ticks)?;
        } else if let Some((command, connected)) = parse_note(&mut stream, octave, note_ticks)? {
            //
            // C/D/E/F/G/A/B[#+-][<len>][.][&] - Play note (1 <= len <= 192)
            //
            is_connected = connected;

            // Combine durations if this note is tied to the previous one.
            if let Some(prev_note) = connected_note.take() {
                if let MmlCommand::Note {
                    midi_note,
                    duration_ticks,
                } = &command
                {
                    if *midi_note == prev_note {
                        if let Some(index) = last_note_index {
                            if let MmlCommand::Note {
                                duration_ticks: prev_ticks,
                                ..
                            } = &mut commands[index]
                            {
                                *prev_ticks += *duration_ticks;
                            }
                        }
                        if is_connected {
                            connected_note = Some(prev_note);
                        } else if is_connected && quantize != 100 {
                            commands.push(MmlCommand::Quantize {
                                gate_ratio: gate_time_to_gate_ratio(quantize),
                            });
                        }
                        continue;
                    }
                }
            }

            if !is_tempo_set {
                is_tempo_set = true;
                commands.push(MmlCommand::Tempo {
                    clocks_per_tick: bpm_to_cpt(DEFAULT_TEMPO),
                });
            }
            if !is_quantize_set {
                is_quantize_set = true;
                commands.push(MmlCommand::Quantize {
                    gate_ratio: gate_time_to_gate_ratio(DEFAULT_QUANTIZE),
                });
            }
            if !is_tone_set {
                is_tone_set = true;
                commands.push(MmlCommand::Tone { tone: 0 });
            }
            if !is_volume_set {
                is_volume_set = true;
                commands.push(MmlCommand::Volume {
                    level: volume_to_level(DEFAULT_VOLUME),
                });
            }
            if !is_transpose_set {
                is_transpose_set = true;
                commands.push(MmlCommand::Transpose {
                    semitone_offset: 0.0,
                });
            }
            if !is_detune_set {
                is_detune_set = true;
                commands.push(MmlCommand::Detune {
                    semitone_offset: 0.0,
                });
            }
            if !is_envelope_set {
                is_envelope_set = true;
                commands.push(MmlCommand::Envelope { slot: 0 });
            }
            if !is_vibrato_set {
                is_vibrato_set = true;
                commands.push(MmlCommand::Vibrato { slot: 0 });
            }
            if !is_glide_set {
                is_glide_set = true;
                commands.push(MmlCommand::Glide { slot: 0 });
            }

            if is_connected && quantize != 100 {
                commands.push(MmlCommand::Quantize { gate_ratio: 1.0 });
            } else if !is_connected && quantize != 100 {
                commands.push(MmlCommand::Quantize {
                    gate_ratio: gate_time_to_gate_ratio(quantize),
                });
            }

            if is_connected {
                if let MmlCommand::Note { midi_note, .. } = &command {
                    connected_note = Some(*midi_note);
                }
            }

            last_note_index = Some(commands.len());
            commands.push(command);
        } else if let Some(command) = parse_rest(&mut stream, note_ticks)? {
            //
            // R[<len>][.] - Rest (1 <= len <= 192)
            //
            if !is_tempo_set {
                is_tempo_set = true;
                commands.push(MmlCommand::Tempo {
                    clocks_per_tick: bpm_to_cpt(DEFAULT_TEMPO),
                });
            }

            if is_connected && quantize != 100 {
                commands.push(MmlCommand::Quantize {
                    gate_ratio: gate_time_to_gate_ratio(quantize),
                });
            }
            connected_note = None;

            commands.push(command);
            last_note_index = None;
        } else if parse_string(&mut stream, "[").is_ok() {
            //
            // [ - Repeat start marker
            //
            commands.push(MmlCommand::RepeatStart);
        } else if parse_string(&mut stream, "]").is_ok() {
            //
            // ]<count> - Repeat end (count >= 1, 0 = infinite)
            //
            let count = parse_number(&mut stream, "count", RANGE_GE1).unwrap_or(0);
            commands.push(MmlCommand::RepeatEnd { play_count: count });
        } else {
            let c = *stream.peek().unwrap();
            parse_error!(stream, "Unexpected character '{c}'");
        }
    }
    Ok(commands)
}

pub fn calc_commands_sec(commands: &[MmlCommand]) -> Option<f32> {
    let mut total_clocks = 0;
    let mut command_index: u32 = 0;
    let mut repeat_points: Vec<(u32, u32)> = Vec::new();
    let mut clocks_per_tick = bpm_to_cpt(DEFAULT_TEMPO);

    while command_index < commands.len() as u32 {
        let command = &commands[command_index as usize];
        command_index += 1;
        match command {
            MmlCommand::Tempo {
                clocks_per_tick: cpt,
            } => {
                clocks_per_tick = *cpt;
            }
            MmlCommand::Note { duration_ticks, .. } | MmlCommand::Rest { duration_ticks } => {
                total_clocks += clocks_per_tick * *duration_ticks;
            }
            MmlCommand::RepeatStart => {
                repeat_points.push((command_index, 0)); // Index after RepeatStart
            }
            MmlCommand::RepeatEnd { play_count } => {
                if *play_count == 0 {
                    return None;
                }
                if let Some((index, count)) = repeat_points.pop() {
                    if count + 1 < *play_count {
                        repeat_points.push((index, count + 1));
                        command_index = index;
                    }
                }
            }
            _ => {}
        }
    }
    Some(total_clocks as f32 / AUDIO_CLOCK_RATE as f32)
}

fn skip_whitespace(stream: &mut CharStream) {
    while let Some(&c) = stream.peek() {
        if c.is_whitespace() {
            stream.next();
        } else {
            break;
        }
    }
}

fn parse_number<T: TryFrom<i32>>(
    stream: &mut CharStream,
    name: &str,
    range: (i32, i32),
) -> Result<T, String> {
    skip_whitespace(stream);
    let pos = stream.pos;
    let mut parsed_str = String::new();

    if let Some(&c) = stream.peek() {
        if c == '-' {
            parsed_str.push(stream.next().unwrap());
        }
    }
    while let Some(&c) = stream.peek() {
        if c.is_ascii_digit() {
            parsed_str.push(stream.next().unwrap());
        } else {
            break;
        }
    }

    if parsed_str.is_empty() {
        if let Some(&c) = stream.peek() {
            parsed_str.push(c);
        }
        stream.pos = pos;
        return Err(parsed_str);
    }

    let Ok(value) = parsed_str.parse::<i32>() else {
        stream.pos = pos;
        return Err(parsed_str);
    };
    if value < range.0 {
        parse_error!(stream, "'{name}' is below minimum {}", range.0);
    }
    if value > range.1 {
        parse_error!(stream, "'{name}' exceeds maximum {}", range.1);
    }

    T::try_from(value).map_err(|_| stream.error(&format!("Invalid value for '{name}'")))
}

fn expect_number<T: TryFrom<i32>>(
    stream: &mut CharStream,
    name: &str,
    range: (i32, i32),
) -> Result<T, String> {
    match parse_number(stream, name, range) {
        Ok(value) => Ok(value),
        Err(actual) => parse_error!(stream, "Expected value for '{name}' but found '{actual}'"),
    }
}

fn parse_string(stream: &mut CharStream, literal: &str) -> Result<String, String> {
    skip_whitespace(stream);
    let pos = stream.pos;
    let mut parsed_str = String::new();

    for expected in literal.chars() {
        match stream.peek() {
            Some(&c) if c.eq_ignore_ascii_case(&expected) => {
                parsed_str.push(stream.next().unwrap());
            }
            Some(&c) => {
                parsed_str.push(c);
                stream.pos = pos;
                return Err(parsed_str);
            }
            None => {
                stream.pos = pos;
                return Err(parsed_str);
            }
        }
    }
    Ok(parsed_str)
}

fn expect_string(stream: &mut CharStream, literal: &str) -> Result<(), String> {
    if let Err(actual) = parse_string(stream, literal) {
        parse_error!(stream, "Expected '{literal}' but found '{actual}'");
    }
    Ok(())
}

fn parse_command<T: TryFrom<i32>>(
    stream: &mut CharStream,
    name: &str,
    range: (i32, i32),
) -> Result<Option<T>, String> {
    if parse_string(stream, name).is_ok() {
        return Ok(Some(expect_number(stream, name, range)?));
    }
    Ok(None)
}

fn parse_length_as_ticks(stream: &mut CharStream, note_ticks: u32) -> Result<u32, String> {
    const WHOLE_NOTE_TICKS: u32 = TICKS_PER_QUARTER_NOTE * 4;
    let mut note_ticks = note_ticks;

    if let Ok(len) = parse_number::<u32>(stream, "Note length", RANGE_LENGTH) {
        if WHOLE_NOTE_TICKS.is_multiple_of(len) {
            note_ticks = WHOLE_NOTE_TICKS / len;
        } else {
            parse_error!(stream, "Invalid note length '{len}'");
        }
    }

    let mut dot_ticks = note_ticks;
    while parse_string(stream, ".").is_ok() {
        if dot_ticks.is_multiple_of(2) {
            dot_ticks /= 2;
            note_ticks += dot_ticks;
        } else {
            parse_error!(stream, "Cannot apply dot to odd note length");
        }
    }
    Ok(note_ticks)
}

fn parse_note(
    stream: &mut CharStream,
    octave: i32,
    note_ticks: u32,
) -> Result<Option<(MmlCommand, bool)>, String> {
    skip_whitespace(stream);

    let semitone = match stream.peek() {
        Some(c) => match c.to_ascii_uppercase() {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => return Ok(None),
        },
        None => return Ok(None),
    };
    stream.next();

    let midi_note = ((octave + 1) * 12 + semitone) as u32;
    let midi_note = if parse_string(stream, "#").is_ok() || parse_string(stream, "+").is_ok() {
        midi_note + 1
    } else if parse_string(stream, "-").is_ok() {
        midi_note.saturating_sub(1)
    } else {
        midi_note
    };

    let mut duration_ticks = parse_length_as_ticks(stream, note_ticks)?;

    // Extend duration with '&<len>'
    let mut is_connected = false;
    while parse_string(stream, "&").is_ok() {
        skip_whitespace(stream);
        if stream.peek().is_some_and(char::is_ascii_digit) {
            duration_ticks += parse_length_as_ticks(stream, note_ticks)?;
        } else {
            // Set connection flag if '&' followed by non-digit
            is_connected = true;
            break;
        }
    }

    Ok(Some((
        MmlCommand::Note {
            midi_note,
            duration_ticks,
        },
        is_connected,
    )))
}

fn parse_rest(stream: &mut CharStream, note_ticks: u32) -> Result<Option<MmlCommand>, String> {
    if parse_string(stream, "R").is_err() {
        return Ok(None);
    }

    let mut duration_ticks = parse_length_as_ticks(stream, note_ticks)?;
    while parse_string(stream, "&").is_ok() {
        duration_ticks += parse_length_as_ticks(stream, note_ticks)?;
    }

    Ok(Some(MmlCommand::Rest { duration_ticks }))
}

fn parse_envelope(stream: &mut CharStream) -> Result<Option<MmlCommand>, String> {
    let Some(slot) = parse_command(stream, "@ENV", RANGE_GE0)? else {
        return Ok(None);
    };
    if parse_string(stream, "{").is_err() {
        return Ok(Some(MmlCommand::Envelope { slot }));
    }
    if slot == 0 {
        parse_error!(stream, "Envelope slot 0 is reserved for disable");
    }

    let init_vol = expect_number(stream, "init_vol", RANGE_VOLUME)?;
    let mut segments = Vec::new();
    while parse_string(stream, "}").is_err() {
        expect_string(stream, ",")?;
        let dur_ticks = expect_number(stream, "dur_ticks", RANGE_GE0)?;
        expect_string(stream, ",")?;
        let vol = expect_number(stream, "vol", RANGE_VOLUME)?;
        segments.push((dur_ticks, volume_to_level(vol)));
    }

    Ok(Some(MmlCommand::EnvelopeSet {
        slot,
        initial_level: volume_to_level(init_vol),
        segments,
    }))
}

fn parse_vibrato(stream: &mut CharStream) -> Result<Option<MmlCommand>, String> {
    let Some(slot) = parse_command(stream, "@VIB", RANGE_GE0)? else {
        return Ok(None);
    };
    if parse_string(stream, "{").is_err() {
        return Ok(Some(MmlCommand::Vibrato { slot }));
    }
    if slot == 0 {
        parse_error!(stream, "Vibrato slot 0 is reserved for disable");
    }

    let delay_ticks = expect_number(stream, "delay_ticks", RANGE_GE0)?;
    expect_string(stream, ",")?;
    let period_ticks = expect_number(stream, "period_ticks", RANGE_GE0)?;
    expect_string(stream, ",")?;
    let depth_cents = expect_number(stream, "depth_cents", RANGE_ALL)?;
    expect_string(stream, "}")?;

    Ok(Some(MmlCommand::VibratoSet {
        slot,
        delay_ticks,
        period_ticks,
        semitone_depth: cents_to_semitones(depth_cents),
    }))
}

fn parse_glide(stream: &mut CharStream) -> Result<Option<MmlCommand>, String> {
    let Some(slot) = parse_command(stream, "@GLI", RANGE_GE0)? else {
        return Ok(None);
    };
    if parse_string(stream, "{").is_err() {
        return Ok(Some(MmlCommand::Glide { slot }));
    }
    if slot == 0 {
        parse_error!(stream, "Glide slot 0 is reserved for disable");
    }

    let semitone_offset = if parse_string(stream, "*").is_ok() {
        None
    } else {
        Some(cents_to_semitones(expect_number(
            stream,
            "offset_cents",
            RANGE_ALL,
        )?))
    };
    expect_string(stream, ",")?;
    let duration_ticks = if parse_string(stream, "*").is_ok() {
        None
    } else {
        Some(expect_number(stream, "dur_ticks", RANGE_GE0)?)
    };
    expect_string(stream, "}")?;

    Ok(Some(MmlCommand::GlideSet {
        slot,
        semitone_offset,
        duration_ticks,
    }))
}

fn bpm_to_cpt(bpm: u32) -> u32 {
    (AUDIO_CLOCK_RATE as f32 * 60.0 / (bpm as f32 * TICKS_PER_QUARTER_NOTE as f32)).round() as u32
}

fn gate_time_to_gate_ratio(gate_time: u32) -> f32 {
    gate_time as f32 / RANGE_QUANTIZE.1 as f32
}

fn volume_to_level(volume: u32) -> f32 {
    volume as f32 / RANGE_VOLUME.1 as f32
}

fn cents_to_semitones(cents: i32) -> f32 {
    cents as f32 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to extract specific commands from parsed MML
    fn parse(mml: &str) -> Vec<MmlCommand> {
        parse_mml(mml).unwrap()
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

    // Basic notes

    #[test]
    fn test_single_note() {
        let cmds = parse("C");
        let notes = note_commands(&cmds);
        // C4 = MIDI 60, default length = quarter note = 48 ticks
        assert_eq!(notes, [(60, 48)]);
    }

    #[test]
    fn test_note_names() {
        let cmds = parse("O4 CDEFGAB");
        let notes = note_commands(&cmds);
        let midi_notes: Vec<u32> = notes.iter().map(|(n, _)| *n).collect();
        assert_eq!(midi_notes, [60, 62, 64, 65, 67, 69, 71]);
    }

    #[test]
    fn test_sharp_and_flat() {
        let cmds = parse("C+ C# C-");
        let notes = note_commands(&cmds);
        assert_eq!(notes[0].0, 61); // C#
        assert_eq!(notes[1].0, 61); // C#
        assert_eq!(notes[2].0, 59); // Cb = B3
    }

    // Octave

    #[test]
    fn test_octave() {
        let cmds = parse("O3 C O5 C");
        let notes = note_commands(&cmds);
        assert_eq!(notes[0].0, 48); // C3
        assert_eq!(notes[1].0, 72); // C5
    }

    #[test]
    fn test_octave_shift() {
        let cmds = parse("O4 C > C < C");
        let notes = note_commands(&cmds);
        assert_eq!(notes[0].0, 60); // C4
        assert_eq!(notes[1].0, 72); // C5
        assert_eq!(notes[2].0, 60); // C4
    }

    // Note length

    #[test]
    fn test_note_length() {
        let cmds = parse("C1 C2 C4 C8 C16");
        let notes = note_commands(&cmds);
        let ticks: Vec<u32> = notes.iter().map(|(_, t)| *t).collect();
        // whole=192, half=96, quarter=48, eighth=24, sixteenth=12
        assert_eq!(ticks, [192, 96, 48, 24, 12]);
    }

    #[test]
    fn test_dotted_note() {
        let cmds = parse("C4.");
        let notes = note_commands(&cmds);
        // quarter dot = 48 + 24 = 72
        assert_eq!(notes[0].1, 72);
    }

    #[test]
    fn test_default_length() {
        let cmds = parse("L8 C D E");
        let notes = note_commands(&cmds);
        for (_, ticks) in &notes {
            assert_eq!(*ticks, 24); // eighth note
        }
    }

    // Rest

    #[test]
    fn test_rest() {
        let cmds = parse("C R C");
        let rests = rest_commands(&cmds);
        assert_eq!(rests, [48]); // default quarter note rest
    }

    #[test]
    fn test_rest_with_length() {
        let cmds = parse("R8 R16");
        let rests = rest_commands(&cmds);
        assert_eq!(rests, [24, 12]);
    }

    // Tempo

    #[test]
    fn test_tempo() {
        let cmds = parse("T120 C");
        let has_tempo = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Tempo { .. }));
        assert!(has_tempo);
    }

    // Volume

    #[test]
    fn test_volume() {
        let cmds = parse("V100 C");
        let has_volume = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Volume { .. }));
        assert!(has_volume);
    }

    // Repeat

    #[test]
    fn test_repeat() {
        let cmds = parse("[C D]3");
        let has_start = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::RepeatStart));
        let has_end = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::RepeatEnd { play_count: 3 }));
        assert!(has_start);
        assert!(has_end);
    }

    // Tie

    #[test]
    fn test_tie_extends_duration() {
        let cmds = parse("C4&4");
        let notes = note_commands(&cmds);
        // 48 + 48 = 96
        assert_eq!(notes[0].1, 96);
    }

    // Error cases

    #[test]
    fn test_invalid_character() {
        assert!(parse_mml("X").is_err());
    }

    #[test]
    fn test_octave_overflow() {
        // Starting at O9 and going up should error
        assert!(parse_mml("O9 > C").is_err());
    }

    #[test]
    fn test_octave_underflow() {
        assert!(parse_mml("O-1 < C").is_err());
    }

    // Case insensitive

    #[test]
    fn test_case_insensitive() {
        let upper = note_commands(&parse("C D E"));
        let lower = note_commands(&parse("c d e"));
        assert_eq!(upper, lower);
    }

    // Whitespace

    #[test]
    fn test_whitespace_ignored() {
        let no_space = note_commands(&parse("CDE"));
        let with_space = note_commands(&parse("C D E"));
        assert_eq!(no_space, with_space);
    }

    // calc_commands_sec

    #[test]
    fn test_calc_commands_sec_basic() {
        let cmds = parse("T120 C4");
        let sec = calc_commands_sec(&cmds);
        assert!(sec.is_some());
        assert!(sec.unwrap() > 0.0);
    }

    #[test]
    fn test_calc_commands_sec_infinite_loop() {
        let cmds = parse("[C]0"); // 0 = infinite repeat
        let sec = calc_commands_sec(&cmds);
        assert!(sec.is_none());
    }

    #[test]
    fn test_calc_commands_sec_finite_repeat() {
        let cmds = parse("T120 [C4]2");
        let sec = calc_commands_sec(&cmds);
        assert!(sec.is_some());
        // Should be roughly twice the duration of a single note
        let single = calc_commands_sec(&parse("T120 C4")).unwrap();
        assert!((sec.unwrap() - single * 2.0).abs() < 0.01);
    }

    // Envelope / Vibrato / Glide

    #[test]
    fn test_envelope_definition() {
        let cmds = parse("@ENV1{127, 10, 64} C");
        let has_env = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::EnvelopeSet { slot: 1, .. }));
        assert!(has_env);
    }

    #[test]
    fn test_envelope_slot_zero_reserved() {
        assert!(parse_mml("@ENV0{127}").is_err());
    }

    #[test]
    fn test_vibrato_definition() {
        let cmds = parse("@VIB1{10, 20, 50} C");
        let has_vib = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::VibratoSet { slot: 1, .. }));
        assert!(has_vib);
    }

    #[test]
    fn test_glide_definition() {
        let cmds = parse("@GLI1{100, 10} C");
        let has_glide = cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::GlideSet { slot: 1, .. }));
        assert!(has_glide);
    }

    #[test]
    fn test_glide_wildcard() {
        let cmds = parse("@GLI1{*, *} C");
        let has_glide = cmds.iter().any(|cmd| {
            matches!(
                cmd,
                MmlCommand::GlideSet {
                    slot: 1,
                    semitone_offset: None,
                    duration_ticks: None,
                }
            )
        });
        assert!(has_glide);
    }

    // Empty MML

    #[test]
    fn test_empty_mml() {
        let cmds = parse("");
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_rest_only() {
        let cmds = parse("R4 R8");
        assert_eq!(rest_commands(&cmds), [48, 24]);
        assert!(note_commands(&cmds).is_empty());
    }
}

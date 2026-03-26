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

struct CharStream<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> CharStream<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.bytes.get(self.pos).map(|&b| b as char)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.bytes.get(self.pos).map(|&b| b as char);
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
                        }
                        continue;
                    }
                }
            }

            // Insert default commands for any unset parameters
            macro_rules! ensure_set {
                ($flag:expr, $cmd:expr) => {
                    if !$flag {
                        $flag = true;
                        commands.push($cmd);
                    }
                };
            }
            ensure_set!(
                is_tempo_set,
                MmlCommand::Tempo {
                    clocks_per_tick: bpm_to_cpt(DEFAULT_TEMPO)
                }
            );
            ensure_set!(
                is_quantize_set,
                MmlCommand::Quantize {
                    gate_ratio: gate_time_to_gate_ratio(DEFAULT_QUANTIZE)
                }
            );
            ensure_set!(is_tone_set, MmlCommand::Tone { tone: 0 });
            ensure_set!(
                is_volume_set,
                MmlCommand::Volume {
                    level: volume_to_level(DEFAULT_VOLUME)
                }
            );
            ensure_set!(
                is_transpose_set,
                MmlCommand::Transpose {
                    semitone_offset: 0.0
                }
            );
            ensure_set!(
                is_detune_set,
                MmlCommand::Detune {
                    semitone_offset: 0.0
                }
            );
            ensure_set!(is_envelope_set, MmlCommand::Envelope { slot: 0 });
            ensure_set!(is_vibrato_set, MmlCommand::Vibrato { slot: 0 });
            ensure_set!(is_glide_set, MmlCommand::Glide { slot: 0 });

            if quantize != 100 {
                let gate_ratio = if is_connected {
                    1.0
                } else {
                    gate_time_to_gate_ratio(quantize)
                };
                commands.push(MmlCommand::Quantize { gate_ratio });
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
            let c = stream.peek().unwrap();
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
    while stream.peek().is_some_and(char::is_whitespace) {
        stream.next();
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

    if stream.peek() == Some('-') {
        parsed_str.push(stream.next().unwrap());
    }
    while let Some(c) = stream.peek() {
        if c.is_ascii_digit() {
            parsed_str.push(stream.next().unwrap());
        } else {
            break;
        }
    }

    if parsed_str.is_empty() {
        if let Some(c) = stream.peek() {
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
            Some(c) if c.eq_ignore_ascii_case(&expected) => {
                parsed_str.push(stream.next().unwrap());
            }
            Some(c) => {
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

    let semitone = match stream.peek().map(|c| c.to_ascii_uppercase()) {
        Some(c) => match c {
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
        if stream.peek().is_some_and(|c| c.is_ascii_digit()) {
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

    // Helpers

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

    fn quantize_values(commands: &[MmlCommand]) -> Vec<f32> {
        commands
            .iter()
            .filter_map(|cmd| match cmd {
                MmlCommand::Quantize { gate_ratio } => Some(*gate_ratio),
                _ => None,
            })
            .collect()
    }

    // ── Basic notes ──

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

    // ── Octave ──

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

    #[test]
    fn test_octave_boundaries() {
        let cmds = parse("O-1 C O9 B");
        let notes = note_commands(&cmds);
        assert_eq!(notes[0].0, 0); // C at O-1: (-1+1)*12+0 = 0
        assert_eq!(notes[1].0, 131); // B at O9: (9+1)*12+11 = 131
    }

    // ── Note length ──

    #[test]
    fn test_note_length() {
        let cmds = parse("C1 C2 C4 C8 C16");
        let notes = note_commands(&cmds);
        let ticks: Vec<u32> = notes.iter().map(|(_, t)| *t).collect();
        // whole=192, half=96, quarter=48, eighth=24, sixteenth=12
        assert_eq!(ticks, [192, 96, 48, 24, 12]);
    }

    #[test]
    fn test_note_length_boundaries() {
        let cmds = parse("C1 C192");
        let notes = note_commands(&cmds);
        assert_eq!(notes[0].1, 192); // whole note (longest)
        assert_eq!(notes[1].1, 1); // shortest possible
    }

    #[test]
    fn test_dotted_note() {
        let cmds = parse("C4.");
        let notes = note_commands(&cmds);
        // quarter dot = 48 + 24 = 72
        assert_eq!(notes[0].1, 72);
    }

    #[test]
    fn test_double_dotted_note() {
        let cmds = parse("C4..");
        let notes = note_commands(&cmds);
        // quarter=48, dot1=+24=72, dot2=+12=84
        assert_eq!(notes[0].1, 84);
    }

    #[test]
    fn test_default_length() {
        let cmds = parse("L8 C D E");
        let notes = note_commands(&cmds);
        for (_, ticks) in &notes {
            assert_eq!(*ticks, 24); // eighth note
        }
    }

    // ── Rest ──

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

    #[test]
    fn test_rest_with_tie() {
        let cmds = parse("R4&8");
        let rests = rest_commands(&cmds);
        // 48 + 24 = 72
        assert_eq!(rests, [72]);
    }

    #[test]
    fn test_rest_only() {
        let cmds = parse("R4 R8");
        assert_eq!(rest_commands(&cmds), [48, 24]);
        assert!(note_commands(&cmds).is_empty());
    }

    // ── Tie & connection ──

    #[test]
    fn test_tie_extends_duration() {
        let cmds = parse("C4&4");
        let notes = note_commands(&cmds);
        // 48 + 48 = 96
        assert_eq!(notes[0].1, 96);
    }

    #[test]
    fn test_tie_chain() {
        let cmds = parse("C4&4&4");
        let notes = note_commands(&cmds);
        // 48 + 48 + 48 = 144
        assert_eq!(notes, [(60, 144)]);
    }

    #[test]
    fn test_tie_different_notes_not_combined() {
        let cmds = parse("C4& D4");
        let notes = note_commands(&cmds);
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0], (60, 48)); // C4
        assert_eq!(notes[1], (62, 48)); // D4
    }

    // ── Repeat ──

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

    #[test]
    fn test_repeat_default_infinite() {
        let cmds = parse("[C]");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::RepeatEnd { play_count: 0 })));
    }

    // ── Parameter commands: Tempo ──

    #[test]
    fn test_tempo() {
        let cmds = parse("T120 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Tempo { .. })));
    }

    #[test]
    fn test_tempo_value() {
        let cmds = parse("T120 C");
        let cpt = cmds.iter().find_map(|cmd| match cmd {
            MmlCommand::Tempo { clocks_per_tick } => Some(*clocks_per_tick),
            _ => None,
        });
        // bpm_to_cpt(120) = (1_789_773.0 * 60.0 / (120.0 * 48.0)).round() = 18643
        assert_eq!(cpt, Some(18643));
    }

    // ── Parameter commands: Volume ──

    #[test]
    fn test_volume() {
        let cmds = parse("V100 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Volume { .. })));
    }

    #[test]
    fn test_volume_value() {
        // V127 → level = 1.0
        let cmds = parse("V127 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Volume { level } if (*level - 1.0).abs() < 1e-4)));

        // V0 → level = 0.0
        let cmds = parse("V0 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Volume { level } if *level == 0.0)));
    }

    // ── Parameter commands: Quantize ──

    #[test]
    fn test_quantize() {
        let cmds = parse("Q50 C");
        let qvals = quantize_values(&cmds);
        // Q50 → gate_ratio = 0.5
        assert!(
            qvals.iter().any(|&r| (r - 0.5).abs() < 1e-4),
            "expected gate_ratio 0.5, got {qvals:?}"
        );
    }

    #[test]
    fn test_quantize_full_gate() {
        // Q100 means no per-note quantize command is emitted
        let cmds = parse("Q100 C D");
        let qvals = quantize_values(&cmds);
        // Only the Q100 command itself (gate_ratio=1.0), no per-note quantize
        assert!(qvals.iter().all(|&r| (r - 1.0).abs() < 1e-4));
    }

    #[test]
    fn test_connected_note_quantize_full_gate() {
        // Connected note (C&) should get gate_ratio=1.0, normal note (D) gets 0.5
        let cmds = parse("Q50 C& D");
        let qvals = quantize_values(&cmds);
        assert_eq!(qvals.len(), 3);
        assert!((qvals[0] - 0.5).abs() < 1e-4, "Q50: {}", qvals[0]);
        assert!((qvals[1] - 1.0).abs() < 1e-4, "connected: {}", qvals[1]);
        assert!((qvals[2] - 0.5).abs() < 1e-4, "normal: {}", qvals[2]);
    }

    // ── Parameter commands: Tone, Transpose, Detune ──

    #[test]
    fn test_tone_command() {
        let cmds = parse("@3 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Tone { tone: 3 })));
    }

    #[test]
    fn test_transpose() {
        let cmds = parse("K5 C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::Transpose { semitone_offset } if (*semitone_offset - 5.0).abs() < 1e-4
        )));
    }

    #[test]
    fn test_transpose_negative() {
        let cmds = parse("K-5 C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::Transpose { semitone_offset } if (*semitone_offset + 5.0).abs() < 1e-4
        )));
    }

    #[test]
    fn test_detune() {
        let cmds = parse("Y50 C");
        // 50 cents = 0.5 semitones
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::Detune { semitone_offset } if (*semitone_offset - 0.5).abs() < 1e-4
        )));
    }

    // ── Effect definitions: Envelope ──

    #[test]
    fn test_envelope_definition() {
        let cmds = parse("@ENV1{127, 10, 64} C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::EnvelopeSet { slot: 1, .. })));
    }

    #[test]
    fn test_envelope_multiple_segments() {
        let cmds = parse("@ENV1{127, 10, 64, 10, 32} C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::EnvelopeSet { slot: 1, segments, .. } if segments.len() == 2
        )));
    }

    #[test]
    fn test_envelope_switch() {
        let cmds = parse("@ENV2 C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::Envelope { slot: 2 })));
    }

    // ── Effect definitions: Vibrato ──

    #[test]
    fn test_vibrato_definition() {
        let cmds = parse("@VIB1{10, 20, 50} C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::VibratoSet { slot: 1, .. })));
    }

    // ── Effect definitions: Glide ──

    #[test]
    fn test_glide_definition() {
        let cmds = parse("@GLI1{100, 10} C");
        assert!(cmds
            .iter()
            .any(|cmd| matches!(cmd, MmlCommand::GlideSet { slot: 1, .. })));
    }

    #[test]
    fn test_glide_wildcard() {
        let cmds = parse("@GLI1{*, *} C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::GlideSet {
                slot: 1,
                semitone_offset: None,
                duration_ticks: None,
            }
        )));
    }

    #[test]
    fn test_glide_partial_wildcard() {
        let cmds = parse("@GLI1{100, *} C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::GlideSet {
                slot: 1,
                semitone_offset: Some(_),
                duration_ticks: None,
            }
        )));

        let cmds = parse("@GLI1{*, 10} C");
        assert!(cmds.iter().any(|cmd| matches!(
            cmd,
            MmlCommand::GlideSet {
                slot: 1,
                semitone_offset: None,
                duration_ticks: Some(10),
            }
        )));
    }

    // ── Default insertion ──

    #[test]
    fn test_default_commands_inserted_on_first_note() {
        // First note triggers auto-insertion of all default parameter commands
        let cmds = parse("C");
        assert_eq!(cmds.len(), 11);
        assert!(matches!(cmds[0], MmlCommand::Tempo { .. }));
        assert!(matches!(cmds[1], MmlCommand::Quantize { .. }));
        assert!(matches!(cmds[2], MmlCommand::Tone { .. }));
        assert!(matches!(cmds[3], MmlCommand::Volume { .. }));
        assert!(matches!(cmds[4], MmlCommand::Transpose { .. }));
        assert!(matches!(cmds[5], MmlCommand::Detune { .. }));
        assert!(matches!(cmds[6], MmlCommand::Envelope { slot: 0 }));
        assert!(matches!(cmds[7], MmlCommand::Vibrato { slot: 0 }));
        assert!(matches!(cmds[8], MmlCommand::Glide { slot: 0 }));
        assert!(matches!(cmds[9], MmlCommand::Quantize { .. }));
        assert!(matches!(
            cmds[10],
            MmlCommand::Note {
                midi_note: 60,
                duration_ticks: 48
            }
        ));
    }

    // ── Case / whitespace / empty ──

    #[test]
    fn test_case_insensitive() {
        let upper = note_commands(&parse("C D E"));
        let lower = note_commands(&parse("c d e"));
        assert_eq!(upper, lower);
    }

    #[test]
    fn test_whitespace_ignored() {
        let no_space = note_commands(&parse("CDE"));
        let with_space = note_commands(&parse("C D E"));
        assert_eq!(no_space, with_space);
    }

    #[test]
    fn test_empty_mml() {
        let cmds = parse("");
        assert!(cmds.is_empty());
    }

    // ── Error cases ──

    #[test]
    fn test_err_invalid_character() {
        assert!(parse_mml("X").is_err());
    }

    #[test]
    fn test_err_octave_overflow() {
        assert!(parse_mml("O9 > C").is_err());
    }

    #[test]
    fn test_err_octave_underflow() {
        assert!(parse_mml("O-1 < C").is_err());
    }

    #[test]
    fn test_err_tempo_missing_number() {
        assert!(parse_mml("T C").is_err());
    }

    #[test]
    fn test_err_volume_missing_number() {
        assert!(parse_mml("V C").is_err());
    }

    #[test]
    fn test_err_envelope_slot_zero_reserved() {
        assert!(parse_mml("@ENV0{127}").is_err());
    }

    #[test]
    fn test_err_vibrato_slot_zero_reserved() {
        assert!(parse_mml("@VIB0{10, 20, 50}").is_err());
    }

    #[test]
    fn test_err_glide_slot_zero_reserved() {
        assert!(parse_mml("@GLI0{100, 10}").is_err());
    }

    #[test]
    fn test_err_note_length_out_of_range_falls_back() {
        // L0 and L193 are out of range — parser silently uses default length
        let cmds_l0 = parse("L0 C");
        let cmds_default = parse("C");
        assert_eq!(note_commands(&cmds_l0), note_commands(&cmds_default));

        let cmds_l193 = parse("L193 C");
        assert_eq!(note_commands(&cmds_l193), note_commands(&cmds_default));
    }

    #[test]
    fn test_err_note_length_not_divisible() {
        // 192 is not divisible by 7
        assert!(parse_mml("C7").is_err());
    }

    #[test]
    fn test_err_dot_on_odd_tick_length() {
        // L192 = 1 tick, cannot apply dot to odd value
        assert!(parse_mml("C192.").is_err());
    }

    // ── calc_commands_sec ──

    #[test]
    fn test_calc_commands_sec_quarter_note_at_120bpm() {
        let cmds = parse("T120 C4");
        let sec = calc_commands_sec(&cmds).unwrap();
        // Quarter note at 120 BPM ≈ 0.5 seconds
        assert!((sec - 0.5).abs() < 0.001, "expected ~0.5, got {sec}");
    }

    #[test]
    fn test_calc_commands_sec_tempo_change() {
        let cmds = parse("T120 C4 T60 C4");
        let sec = calc_commands_sec(&cmds).unwrap();
        // 0.5s (120bpm quarter) + 1.0s (60bpm quarter) ≈ 1.5s
        assert!((sec - 1.5).abs() < 0.01, "expected ~1.5, got {sec}");
    }

    #[test]
    fn test_calc_commands_sec_infinite_loop() {
        let cmds = parse("[C]0"); // 0 = infinite repeat
        assert!(calc_commands_sec(&cmds).is_none());
    }

    #[test]
    fn test_calc_commands_sec_finite_repeat() {
        let cmds = parse("T120 [C4]2");
        let sec = calc_commands_sec(&cmds).unwrap();
        let single = calc_commands_sec(&parse("T120 C4")).unwrap();
        assert!(
            (sec - single * 2.0).abs() < 0.01,
            "expected ~{}, got {sec}",
            single * 2.0
        );
    }

    #[test]
    fn test_calc_commands_sec_nested_repeat() {
        let cmds = parse("T120 [[C4]2]3");
        let sec = calc_commands_sec(&cmds).unwrap();
        let single = calc_commands_sec(&parse("T120 C4")).unwrap();
        // Inner loop plays 2 times, outer loop plays 3 times → 6 total notes
        assert!(
            (sec - single * 6.0).abs() < 0.01,
            "expected ~{}, got {sec}",
            single * 6.0
        );
    }

    #[test]
    fn test_calc_commands_sec_empty() {
        let cmds = parse("");
        assert_eq!(calc_commands_sec(&cmds), Some(0.0));
    }
}

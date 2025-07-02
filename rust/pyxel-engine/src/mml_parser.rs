use crate::mml_command::MmlCommand;
use crate::settings::{AUDIO_CLOCK_RATE, TICKS_PER_QUARTER_NOTE};

const RANGE_ALL: (i32, i32) = (i32::MIN, i32::MAX);
const RANGE_GE_0: (i32, i32) = (0, i32::MAX);
const RANGE_GE_1: (i32, i32) = (1, i32::MAX);

struct CharStream<'a> {
    chars: &'a [char],
    pos: usize,
}

impl<'a> CharStream<'a> {
    fn new(input: &'a str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let chars = Box::leak(chars.into_boxed_slice());
        Self { chars, pos: 0 }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.get(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn error(&self, message: &str) -> ! {
        panic!("MML:{}: {}", self.pos, message);
    }
}

macro_rules! parse_error {
    ($stream:expr, $fmt:literal $(, $arg:expr)*) => {
        $stream.error(&format!($fmt $(, $arg)*))
    };
}

struct ShouldInit {
    tempo: bool,
    quantize: bool,
    tone: bool,
    volume: bool,
    transpose: bool,
    detune: bool,
    envelope: bool,
    vibrato: bool,
    glide: bool,
}

impl ShouldInit {
    fn new() -> Self {
        Self {
            tempo: true,
            quantize: true,
            tone: true,
            volume: true,
            transpose: true,
            detune: true,
            envelope: true,
            vibrato: true,
            glide: true,
        }
    }

    fn ensure(&mut self, commands: &mut Vec<MmlCommand>) {
        if self.tempo {
            self.tempo = false;
            commands.push(MmlCommand::Tempo {
                clocks_per_tick: bpm_to_cpt(120),
            });
        }

        if self.quantize {
            self.quantize = false;
            commands.push(MmlCommand::Quantize {
                gate_ratio: 7.0 / 8.0,
            });
        }

        if self.tone {
            self.tone = false;
            commands.push(MmlCommand::Tone { tone_index: 0 });
        }

        if self.volume {
            self.volume = false;
            commands.push(MmlCommand::Volume { level: 1.0 });
        }

        if self.transpose {
            self.transpose = false;
            commands.push(MmlCommand::Transpose {
                semitone_offset: 0.0,
            });
        }

        if self.detune {
            self.detune = false;
            commands.push(MmlCommand::Detune {
                semitone_offset: 0.0,
            });
        }

        if self.envelope {
            self.envelope = false;
            commands.push(MmlCommand::Envelope { slot: 0 });
        }

        if self.vibrato {
            self.vibrato = false;
            commands.push(MmlCommand::Vibrato { slot: 0 });
        }

        if self.glide {
            self.glide = false;
            commands.push(MmlCommand::Glide { slot: 0 });
        }
    }
}

pub fn parse_mml(mml: &str) -> Vec<MmlCommand> {
    let mut stream = CharStream::new(mml);
    let mut commands = Vec::new();
    let mut octave: i32 = 4;
    let mut note_ticks: u32 = TICKS_PER_QUARTER_NOTE;
    let mut should_init = ShouldInit::new();

    // Parse MML commands
    while stream.peek().is_some() {
        if parse_string(&mut stream, "[").is_ok() {
            //
            // [ - Loop start marker
            //
            commands.push(MmlCommand::RepeatStart);
        } else if parse_string(&mut stream, "]").is_ok() {
            //
            // ] - Loop end (infinite repetition)
            // ]<count> - Loop end (repeat <count> times, count >= 1)
            //
            let count = parse_number(&mut stream, "Repeat count", RANGE_GE_1).unwrap_or(0);
            commands.push(MmlCommand::RepeatEnd {
                repeat_count: count,
            });
        } else if let Some(bpm) = parse_command(&mut stream, "T", RANGE_GE_1) {
            //
            // T<bpm> - Set tempo (bpm >= 1)
            //
            should_init.tempo = false;
            commands.push(MmlCommand::Tempo {
                clocks_per_tick: bpm_to_cpt(bpm),
            });
        } else if let Some(gate_time) = parse_command::<f64>(&mut stream, "Q", (1, 8)) {
            //
            // Q<gate_time> - Set quantize gate time (1 <= gate_time <= 8)
            //
            should_init.quantize = false;
            commands.push(MmlCommand::Quantize {
                gate_ratio: gate_time / 8.0,
            });
        } else if let Some(vol) = parse_command::<f64>(&mut stream, "V", (0, 15)) {
            //
            // V<vol> - Set volume level (0 <= vol <= 15)
            //
            should_init.volume = false;
            commands.push(MmlCommand::Volume { level: vol / 15.0 });
        } else if let Some(key_offset) = parse_command(&mut stream, "K", RANGE_ALL) {
            //
            // K<key_offset> - Transpose key in semitones
            //
            should_init.transpose = false;
            commands.push(MmlCommand::Transpose {
                semitone_offset: key_offset,
            });
        } else if let Some(offset_cents) = parse_command::<f64>(&mut stream, "Y", RANGE_ALL) {
            //
            // Y<offset_cents> - Set detune in cents
            //
            should_init.detune = false;
            commands.push(MmlCommand::Detune {
                semitone_offset: offset_cents / 100.0,
            });
        } else if let Some(command) = parse_envelope(&mut stream) {
            //
            // @ENV<slot> - Switch to envelope slot (slot >= 0, 0 = off)
            // @ENV<slot> { init_vol, dur_ticks1, vol1, ... } - Define envelope and switch to slot
            //
            should_init.envelope = false;
            commands.push(command);
        } else if let Some(command) = parse_vibrato(&mut stream) {
            //
            // @VIB<slot> - Switch to vibrato slot (slot >= 0, 0 = off)
            // @VIB<slot> { delay_ticks, period_ticks, depth_cents } - Define vibrato and switch to slot
            //
            should_init.vibrato = false;
            commands.push(command);
        } else if let Some(command) = parse_glide(&mut stream) {
            //
            // @GLI<slot> - Switch to glide slot (slot >= 0, 0 = off)
            // @GLI<slot> { offset_cents, dur_ticks } - Define glide and switch to slot
            //
            should_init.glide = false;
            commands.push(command);
        } else if let Some(tone_index) = parse_command(&mut stream, "@", RANGE_GE_0) {
            //
            // @<tone_index> - Set tone (tone_index >= 0)
            //
            should_init.tone = false;
            commands.push(MmlCommand::Tone { tone_index });
        } else if let Some(oct) = parse_command(&mut stream, "O", (-1, 9)) {
            //
            // O<oct> - Set octave (-1 <= oct <= 9)
            //
            octave = oct;
        } else if parse_string(&mut stream, ">").is_ok() {
            //
            // > - Octave up
            //
            if octave < 9 {
                octave += 1;
            } else {
                parse_error!(stream, "Octave exceeds maximum {octave}");
            }
        } else if parse_string(&mut stream, "<").is_ok() {
            //
            // < - Octave down
            //
            if octave > -1 {
                octave -= 1;
            } else {
                parse_error!(stream, "Octave is below minimum {octave}");
            }
        } else if parse_string(&mut stream, "L").is_ok() {
            //
            // L<len> - Set default note length
            //
            note_ticks = parse_length_as_ticks(&mut stream, note_ticks);
        } else if let Some(command) = parse_note(&mut stream, octave, note_ticks) {
            //
            // C/D/E/F/G/A/B[#+-][<len>][.] - Play note
            //
            should_init.ensure(&mut commands);
            commands.push(command);
        } else if let Some(command) = parse_rest(&mut stream, note_ticks) {
            //
            // R[<len>][.] - Rest
            //
            commands.push(command);
        } else {
            let c = *stream.peek().unwrap();
            parse_error!(stream, "Unexpected character '{c}'");
        }
    }

    commands
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

    if let Ok(value) = T::try_from(value) {
        Ok(value)
    } else {
        panic!();
    }
}

fn expect_number<T: TryFrom<i32>>(stream: &mut CharStream, name: &str, range: (i32, i32)) -> T {
    match parse_number(stream, name, range) {
        Ok(value) => value,
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

fn expect_string(stream: &mut CharStream, literal: &str) {
    if let Err(actual) = parse_string(stream, literal) {
        parse_error!(stream, "Expected '{literal}' but found '{actual}'");
    }
}

fn parse_command<T: TryFrom<i32>>(
    stream: &mut CharStream,
    name: &str,
    range: (i32, i32),
) -> Option<T> {
    if parse_string(stream, name).is_ok() {
        return Some(expect_number(stream, name, range));
    }

    None
}

fn parse_length_as_ticks(stream: &mut CharStream, note_ticks: u32) -> u32 {
    const WHOLE_NOTE_TICKS: u32 = TICKS_PER_QUARTER_NOTE * 4;

    let mut note_ticks = note_ticks;

    if let Ok(len) = parse_number::<u32>(stream, "Note length", RANGE_GE_1) {
        if WHOLE_NOTE_TICKS % len == 0 {
            note_ticks = WHOLE_NOTE_TICKS / len;
        } else {
            parse_error!(stream, "Invalid note length '{len}'");
        }
    }

    while parse_string(stream, ".").is_ok() {
        if note_ticks % 2 == 0 {
            note_ticks += note_ticks / 2;
        } else {
            parse_error!(stream, "Cannot apply dot to odd note length");
        }
    }

    note_ticks
}

fn parse_note(stream: &mut CharStream, octave: i32, note_ticks: u32) -> Option<MmlCommand> {
    let mut midi_note = (octave + 1) * 12
        + match stream.peek()?.to_ascii_uppercase() {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => return None,
        };
    stream.next();

    if parse_string(stream, "#").is_ok() || parse_string(stream, "+").is_ok() {
        midi_note += 1;
    } else if parse_string(stream, "-").is_ok() {
        midi_note -= 1;
    }

    Some(MmlCommand::Note {
        midi_note: midi_note as u32,
        duration_ticks: parse_length_as_ticks(stream, note_ticks),
    })
}

fn parse_rest(stream: &mut CharStream, note_ticks: u32) -> Option<MmlCommand> {
    if parse_string(stream, "R").is_err() {
        return None;
    }

    Some(MmlCommand::Rest {
        duration_ticks: parse_length_as_ticks(stream, note_ticks),
    })
}

fn parse_envelope(stream: &mut CharStream) -> Option<MmlCommand> {
    let slot = parse_command(stream, "@ENV", RANGE_GE_0)?;

    if parse_string(stream, "{").is_err() {
        return Some(MmlCommand::Envelope { slot });
    }

    if slot == 0 {
        parse_error!(stream, "Envelope slot 0 is reserved for disable");
    }

    let init_vol = expect_number::<f64>(stream, "init_vol", (0, 15));
    let mut segments = Vec::new();

    while parse_string(stream, "}").is_err() {
        expect_string(stream, ",");
        let dur_ticks = expect_number(stream, "dur_ticks", RANGE_GE_0);
        expect_string(stream, ",");
        let vol = expect_number::<f64>(stream, "vol", (0, 15));
        segments.push((dur_ticks, vol / 15.0));
    }

    Some(MmlCommand::EnvelopeSet {
        slot,
        initial_level: init_vol / 15.0,
        segments,
    })
}

fn parse_vibrato(stream: &mut CharStream) -> Option<MmlCommand> {
    let slot = parse_command(stream, "@VIB", RANGE_GE_0)?;

    if parse_string(stream, "{").is_err() {
        return Some(MmlCommand::Vibrato { slot });
    }

    if slot == 0 {
        parse_error!(stream, "Vibrato slot 0 is reserved for disable");
    }

    let delay_ticks = expect_number(stream, "delay_ticks", RANGE_GE_0);
    expect_string(stream, ",");
    let period_ticks = expect_number(stream, "period_ticks", RANGE_GE_1);
    expect_string(stream, ",");
    let depth_cents = expect_number::<f64>(stream, "depth_cents", RANGE_GE_0);
    expect_string(stream, "}");

    Some(MmlCommand::VibratoSet {
        slot,
        delay_ticks,
        period_ticks,
        semitone_depth: depth_cents / 100.0,
    })
}

fn parse_glide(stream: &mut CharStream) -> Option<MmlCommand> {
    let slot = parse_command(stream, "@GLI", RANGE_GE_0)?;

    if parse_string(stream, "{").is_err() {
        return Some(MmlCommand::Glide { slot });
    }

    if slot == 0 {
        parse_error!(stream, "Glide slot 0 is reserved for disable");
    }

    let offset_cents = expect_number::<f64>(stream, "offset_cents", RANGE_ALL);
    expect_string(stream, ",");
    let dur_ticks = expect_number(stream, "dur_ticks", RANGE_GE_1);
    expect_string(stream, "}");

    Some(MmlCommand::GlideSet {
        slot,
        semitone_offset: offset_cents / 100.0,
        duration_ticks: dur_ticks,
    })
}

fn bpm_to_cpt(bpm: u32) -> u32 {
    (AUDIO_CLOCK_RATE as f64 * 60.0 / (bpm as f64 * TICKS_PER_QUARTER_NOTE as f64)).round() as u32
}

/*
    Based on 8bit BGM generator by frenchbread
    https://github.com/shiromofufactory/8bit-bgm-generator
*/
use std::fmt::Write as _;

use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};

#[cfg(pyxel_core)]
use crate::pyxel::{self, Pyxel};
#[cfg(pyxel_core)]
use crate::sound::Sound;

// Generation parameters — field names match the original TS bgm-generator.ts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorParams {
    pub transpose: i32,        // -5..+5
    pub instrumentation: i32,  // 0-3
    pub speed: i32,            // internal speed unit (BPM = 28800 / speed)
    pub chord: i32,            // 0-7 preset, 8-9 custom
    pub base: i32,             // 0-7
    pub base_quantize: i32,    // 12-15
    pub drums: i32,            // 0-7
    pub melo_tone: i32,        // 0-5
    pub sub_tone: i32,         // 0-5
    pub melo_lowest_note: i32, // 28-33
    pub melo_density: i32,     // 0|2|4
    pub melo_use16: bool,
    #[serde(default)]
    pub custom_progression: Option<Vec<CustomChordEntryDef>>,
}

// Custom chord progression entry — sent from TS when chord >= PRESET_COUNT.
// Either `notes` (a 12-digit bits string) or `repeat` (a prior entry index) must be provided.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomChordEntryDef {
    pub loc: usize,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub repeat: Option<usize>,
}

const BARS: usize = 8;
const STEPS_PER_BAR: usize = 16;
const TOTAL_STEPS: usize = BARS * STEPS_PER_BAR;
const PRESET_COUNT: usize = 8;
const TONE_CANDIDATES: [usize; 6] = [11, 8, 2, 10, 6, 4];

const PRESETS: [GeneratorParams; PRESET_COUNT] = [
    GeneratorParams {
        transpose: 0,
        instrumentation: 3,
        speed: 216,
        chord: 0,
        base: 4,
        base_quantize: 14,
        drums: 4,
        melo_tone: 0,
        sub_tone: 0,
        melo_lowest_note: 28,
        melo_density: 2,
        melo_use16: true,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 3,
        speed: 216,
        chord: 1,
        base: 6,
        base_quantize: 12,
        drums: 5,
        melo_tone: 3,
        sub_tone: 3,
        melo_lowest_note: 28,
        melo_density: 4,
        melo_use16: true,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 0,
        speed: 312,
        chord: 2,
        base: 1,
        base_quantize: 15,
        drums: 0,
        melo_tone: 5,
        sub_tone: 5,
        melo_lowest_note: 30,
        melo_density: 2,
        melo_use16: false,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 3,
        speed: 276,
        chord: 3,
        base: 2,
        base_quantize: 15,
        drums: 3,
        melo_tone: 4,
        sub_tone: 4,
        melo_lowest_note: 28,
        melo_density: 0,
        melo_use16: false,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 3,
        speed: 240,
        chord: 4,
        base: 0,
        base_quantize: 14,
        drums: 2,
        melo_tone: 0,
        sub_tone: 1,
        melo_lowest_note: 29,
        melo_density: 2,
        melo_use16: false,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 2,
        speed: 216,
        chord: 5,
        base: 3,
        base_quantize: 14,
        drums: 4,
        melo_tone: 1,
        sub_tone: 1,
        melo_lowest_note: 30,
        melo_density: 2,
        melo_use16: true,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 1,
        speed: 192,
        chord: 6,
        base: 5,
        base_quantize: 13,
        drums: 6,
        melo_tone: 0,
        sub_tone: 0,
        melo_lowest_note: 28,
        melo_density: 4,
        melo_use16: true,
        custom_progression: None,
    },
    GeneratorParams {
        transpose: 0,
        instrumentation: 3,
        speed: 168,
        chord: 7,
        base: 7,
        base_quantize: 15,
        drums: 7,
        melo_tone: 3,
        sub_tone: 3,
        melo_lowest_note: 28,
        melo_density: 4,
        melo_use16: true,
        custom_progression: None,
    },
];

fn preset_params(preset: i32) -> GeneratorParams {
    let idx = preset.clamp(0, (PRESET_COUNT - 1) as i32) as usize;
    PRESETS[idx].clone()
}

// Instrument definition (maps to MML @, @ENV, @VIB, @GLI)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tone {
    pub wave: i32,            // 0=triangle, 1=square, 2=pulse, 3=noise
    pub attack: i32,          // ticks
    pub decay: i32,           // ticks
    pub sustain: i32,         // 0-100 (%)
    pub release: i32,         // ticks
    pub vibrato: i32,         // delay ticks (0=disabled)
    pub drum_notes: Vec<i32>, // pitch sweep sequence for drums (empty=normal tone)
}

// Per-channel note and control data (all sparse: None=continue previous)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub notes: Vec<Option<i32>>,   // None=sustain, -1=rest, 0+=pitch/drum key
    pub tones: Vec<Option<i32>>,   // tone index
    pub volumes: Vec<Option<i32>>, // 0-127
    pub quantizes: Vec<Option<i32>>, // 0-100 (gate percent)
}

// Complete BGM data — output of `generate_bgm()`, input for `compile_to_mml()`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgmData {
    pub tempo: i32,             // BPM (MML T command value)
    pub tones: Vec<Tone>,       // up to 16 tone definitions
    pub channels: Vec<Channel>, // up to 4 channels
}

#[cfg(not(pyxel_core))]
impl GeneratorParams {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("GeneratorParams serialization failed")
    }
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("GeneratorParams deserialization failed")
    }
}

#[cfg(not(pyxel_core))]
impl BgmData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("BgmData serialization failed")
    }
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("BgmData deserialization failed")
    }
}

// Wave, attack, decay, sustain, release, vibrato
const TONE_LIBRARY: [[i32; 6]; 16] = [
    [0, 0, 0, 100, 0, 0],
    [2, 0, 30, 50, 10, 60],
    [2, 20, 20, 70, 10, 60],
    [2, 40, 0, 100, 20, 90],
    [1, 15, 60, 50, 10, 90],
    [1, 0, 30, 30, 10, 0],
    [1, 0, 15, 10, 20, 0],
    [0, 0, 0, 100, 0, 60],
    [2, 0, 40, 20, 10, 0],
    [0, 15, 60, 60, 10, 0],
    [1, 0, 60, 80, 10, 0],
    [2, 0, 60, 80, 10, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [3, 0, 12, 0, 0, 0],
];

// Key, wave, notes, decay, sustain, velocity
const DRUM_KEYS: [i32; 6] = [1, 2, 3, 5, 6, 7];
const DRUM_WAVES: [i32; 6] = [3, 3, 3, 0, 0, 0];
const DRUM_DECAY: [i32; 6] = [8, 16, 10, 16, 16, 16];
const DRUM_SUSTAIN: [i32; 6] = [0, 0, 0, 0, 0, 0];
const DRUM_VELOCITY: [i32; 6] = [100, 100, 30, 100, 100, 100];
const DRUM_NOTES_1: [i32; 2] = [36, 24];
const DRUM_NOTES_2: [i32; 1] = [46];
const DRUM_NOTES_3: [i32; 1] = [58];
const DRUM_NOTES_5: [i32; 9] = [21, 19, 18, 17, 16, 15, 14, 13, 12];
const DRUM_NOTES_6: [i32; 9] = [27, 25, 24, 23, 22, 21, 20, 19, 18];
const DRUM_NOTES_7: [i32; 9] = [33, 31, 30, 29, 28, 27, 26, 25, 24];

// 16-step bass patterns:
// '.' = Hold previous note, '0' = Rest/stop, '1'..'4' = Degree selector
const BASS_PATTERNS: [(&str, &str); 8] = [
    ("4.....4.....0.44", "4.....4.....44.."),
    ("2.3.4.3.2.3.4.3.", "2.3.4.3.2...4..."),
    ("440...440...2...", "440...440...442."),
    ("0.2.4.2.0.2.4.2.", "0.2.4.2.2.4.2.4."),
    ("2.402.402.402.40", "2.402.402.40242."),
    ("2034203420342034", "2034203420444440"),
    ("2044204420442044", "2044204420442.4."),
    ("4.444.444.444.44", "4.444.442.442.22"),
];

// 16-step drum trigger patterns:
// '0' = Rest, other digits are drum keys (mapped in notes_to_mml)
const DRUM_PATTERNS: [(&str, &str); 8] = [
    ("1000000000001000", "1000000000001030"),
    ("1000001000001000", "1000001000007750"),
    ("3030003330300033", "3030003330303013"),
    ("1000001000003330", "1000001000003310"),
    ("1000301010003013", "1000301010003333"),
    ("3033303330333033", "3033303330336655"),
    ("3033203330332033", "3033203330332032"),
    ("1033203310332033", "1033203310332220"),
];

// Melody rhythm patterns:
// '0' = Onset, '.' = Hold, '-' = Rest onset
const RHYTHM_PATTERNS: &[&str] = &[
    "0....0000.......",
    "...0..0.0...0...",
    "0..0..0.0.......",
    "-.......-.......",
    "0....0000.......",
    "...0..0.0..0..0.",
    "0.....0...000...",
    "0...............",
    "0....0000.......",
    "....0...0...0...",
    "0......0...000..",
    "0.........0.00..",
    "0.........0.0.0.",
    "0.....0...000...",
    "0...............",
    "-.......-.0.0.0.",
    "0.....0.....0...",
    "0..0..0....00...",
    "0.....0.....0...",
    "........-.000000",
    "0..0..0.0...0...",
    "0..0..0.0..0..0.",
    "0...............",
    "........-0000000",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.........0.0.0.",
    "0...0...0.....0.",
    "0.......0...0...",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.0.0...........",
    "-.....0.0.0.0.0.",
    "0.........0.0.0.",
    "0...0...0.....0.",
    "0...............",
    "-...............",
    "0..0..0.0.......",
    "-.....000..0..0.",
    "0..0..0.0.......",
    "...0..0.0...0...",
    "0.....0.....0...",
    "0.0.0.0...000...",
    "0..0..0.0.......",
    "-.....000..0..0.",
    "0..0..0.0.......",
    "...0..0.0...0...",
    "0.....0.....0...",
    "0.0.0.0...000...",
    "0.....000.....00",
    "0.000.......0.0.",
    "0.....000.....00",
    "0.000...........",
    "0.....000.....00",
    "0.000.......0.0.",
    "0.....000.....00",
    "0.000...........",
    "000000000.......",
    "000000000.......",
    "00000...00000...",
    "0.....0.....0...",
];

#[derive(Clone, Copy)]
struct ChordEntry {
    // Step location in [0, 127]
    loc: usize,
    // 12-digit per-semitone weights (None means "use repeat source")
    notes: Option<&'static str>,
    // Index to another entry to reuse its notes
    repeat: Option<usize>,
}

macro_rules! ce {
    ($loc:expr, $notes:expr) => {
        ChordEntry {
            loc: $loc,
            notes: Some($notes),
            repeat: None,
        }
    };
    ($loc:expr, repeat $idx:expr) => {
        ChordEntry {
            loc: $loc,
            notes: None,
            repeat: Some($idx),
        }
    };
}

const CP0: [ChordEntry; 9] = [
    ce!(0, "209019030909"),
    ce!(16, "901093090920"),
    ce!(32, "209019030909"),
    ce!(48, "901093090920"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "209019030909"),
    ce!(112, "901093090920"),
    ce!(120, "901099010902"),
];
const CP1: [ChordEntry; 8] = [
    ce!(0, "309092090109"),
    ce!(16, "903092010901"),
    ce!(32, "309092090109"),
    ce!(48, "903092010901"),
    ce!(64, "909209030930"),
    ce!(80, "309201090190"),
    ce!(96, "909209030930"),
    ce!(112, "309201090190"),
];
const CP2: [ChordEntry; 10] = [
    ce!(0, "209019030909"),
    ce!(16, "903099020901"),
    ce!(32, "109309092090"),
    ce!(48, "901903099020"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "109309092090"),
    ce!(104, "901903099020"),
    ce!(112, "209091030909"),
    ce!(120, "209019030909"),
];
const CP3: [ChordEntry; 8] = [
    ce!(0, "309012090109"),
    ce!(16, "901020901903"),
    ce!(32, "109039010209"),
    ce!(48, "209019030909"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "901039090209"),
    ce!(112, "019039010209"),
];
const CP4: [ChordEntry; 9] = [
    ce!(0, "109039090209"),
    ce!(16, "903099020901"),
    ce!(32, "309092090109"),
    ce!(48, "903099020901"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, repeat 2),
    ce!(112, "903099020901"),
    ce!(120, "903090902901"),
];
const CP5: [ChordEntry; 11] = [
    ce!(0, "109039090209"),
    ce!(16, "309092090109"),
    ce!(32, "903099020901"),
    ce!(48, "209019030909"),
    ce!(56, "109019030902"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, repeat 2),
    ce!(112, "209019030909"),
    ce!(120, "903093090302"),
    ce!(124, "903029030903"),
];
const CP6: [ChordEntry; 9] = [
    ce!(0, "109030909209"),
    ce!(16, "902090109309"),
    ce!(32, "109030909209"),
    ce!(48, "902090109309"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "109030909209"),
    ce!(112, "902090109309"),
    ce!(120, "909020901903"),
];
const CP7: [ChordEntry; 8] = [
    ce!(0, "901029010903"),
    ce!(16, "309012090109"),
    ce!(32, "903019020901"),
    ce!(48, "109039030201"),
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "903019020901"),
    ce!(112, "019030909209"),
];

const CHORD_PROGRESSIONS: [&[ChordEntry]; PRESET_COUNT] =
    [&CP0, &CP1, &CP2, &CP3, &CP4, &CP5, &CP6, &CP7];

fn note_name(note: i32) -> (&'static str, i32) {
    let semitone = note.rem_euclid(12);
    let octave = note.div_euclid(12) + 2;
    let name = match semitone {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        _ => "B",
    };
    (name, octave)
}

fn push_octave(tokens: &mut Vec<String>, current_oct: &mut Option<i32>, target: i32) {
    match *current_oct {
        Some(cur) if cur == target => return,
        Some(cur) if target - cur == 1 => tokens.push(">".to_string()),
        Some(cur) if target - cur == -1 => tokens.push("<".to_string()),
        _ => tokens.push(format!("O{target}")),
    }
    *current_oct = Some(target);
}

fn length_units_to_tokens(units: usize) -> Vec<&'static str> {
    let table = [
        (16usize, "1"),
        (12usize, "2."),
        (8usize, "2"),
        (6usize, "4."),
        (4usize, "4"),
        (3usize, "8."),
        (2usize, "8"),
        (1usize, "16"),
    ];
    let mut out = Vec::new();
    let mut remaining = units;
    for (u, tok) in table {
        while remaining >= u {
            out.push(tok);
            remaining -= u;
        }
        if remaining == 0 {
            break;
        }
    }
    out
}

fn select_default_length(notes: &[Option<i32>]) -> &'static str {
    let mut c1 = 0usize;
    let mut c2 = 0usize;
    let mut c4 = 0usize;
    let mut c8 = 0usize;
    let mut c16 = 0usize;

    let mut i = 0usize;
    while i < notes.len() {
        let Some(_) = notes[i] else {
            i += 1;
            continue;
        };
        let mut len = 1usize;
        let mut j = i + 1;
        while j < notes.len() && notes[j].is_none() {
            len += 1;
            j += 1;
        }
        i = j;
        if let Some(head) = length_units_to_tokens(len).first().copied() {
            match head {
                "1" => c1 += 1,
                "2" => c2 += 1,
                "4" => c4 += 1,
                "8" => c8 += 1,
                "16" => c16 += 1,
                _ => {}
            }
        }
    }

    let mut best = ("16", c16, 16usize);
    for cand in [
        ("1", c1, 1usize),
        ("2", c2, 2usize),
        ("4", c4, 4usize),
        ("8", c8, 8usize),
    ] {
        if cand.1 > best.1 || (cand.1 == best.1 && cand.2 > best.2) {
            best = cand;
        }
    }
    best.0
}

fn note_token(note: &str, units: usize, default_len: &str, tie_out: bool) -> String {
    let lens = length_units_to_tokens(units);
    if lens.is_empty() {
        return note.to_string();
    }
    let mut out = String::new();
    out.push_str(note);
    if lens[0] != default_len {
        out.push_str(lens[0]);
    }
    for tok in lens.iter().skip(1) {
        out.push('&');
        out.push_str(tok);
    }
    if tie_out {
        out.push('&');
    }
    out
}

fn env_def_from_tone(tone_idx: usize, slot: i32) -> String {
    let tone = TONE_LIBRARY[tone_idx];
    let attack = tone[1].max(0);
    let decay = tone[2].max(0);
    let sustain = tone[3].clamp(0, 100);
    let release = tone[4].max(0);
    let sustain_level = (sustain * 127 + 50) / 100;

    if attack == 0 && decay == 0 && release == 0 && sustain_level == 127 {
        return format!("@ENV{slot}{{127}}");
    }

    let mut segs = vec![(attack, 127), (decay, sustain_level)];
    if release > 0 {
        segs.push((release, 0));
    }
    let mut out = format!("@ENV{slot}{{0");
    for (dur, vol) in segs {
        if dur >= 0 {
            let _ = write!(out, ",{dur},{vol}");
        }
    }
    out.push('}');
    out
}

fn vib_def_from_tone(tone_idx: usize, slot: i32) -> Option<String> {
    let vibrato = TONE_LIBRARY[tone_idx][5].max(0);
    if vibrato == 0 {
        return None;
    }
    Some(format!("@VIB{slot}{{{vibrato},20,25}}"))
}

fn drum_key_to_idx(key: i32) -> Option<usize> {
    DRUM_KEYS.iter().position(|&k| k == key)
}

fn drum_notes_for_key(key: i32) -> &'static [i32] {
    match key {
        2 => &DRUM_NOTES_2,
        3 => &DRUM_NOTES_3,
        5 => &DRUM_NOTES_5,
        6 => &DRUM_NOTES_6,
        7 => &DRUM_NOTES_7,
        _ => &DRUM_NOTES_1,
    }
}

fn used_drum_keys(notes: &[Option<i32>]) -> Vec<i32> {
    let mut keys: Vec<i32> = notes.iter().filter_map(|n| n.filter(|&v| v > 0)).collect();
    keys.sort_unstable();
    keys.dedup();
    keys
}

fn drum_env_slots(used_keys: &[i32]) -> [i32; 10] {
    let mut slots = [0i32; 10];
    for (idx, key) in used_keys.iter().enumerate() {
        let key_idx = (*key).clamp(0, 9) as usize;
        slots[key_idx] = (idx as i32) + 2;
    }
    slots
}

fn env_def_from_drum_key(key: i32, slot: i32) -> String {
    let idx = drum_key_to_idx(key).unwrap_or(0);
    let decay = DRUM_DECAY[idx].max(0);
    let sustain = DRUM_SUSTAIN[idx].clamp(0, 100);
    let velocity = DRUM_VELOCITY[idx].clamp(0, 100);
    let sustain_level = (sustain * 127 + 50) / 100;
    let init = (velocity * 127 + 50) / 100;
    if decay == 0 && sustain_level == 0 {
        return format!("@ENV{slot}{{{init}}}");
    }
    format!("@ENV{slot}{{{init},{decay},{sustain_level}}}")
}

fn compress_repeats(items: &[String], group: usize, skip_octave_shifts: bool) -> Vec<String> {
    if group <= 1 {
        // Compress single-element runs
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < items.len() {
            let mut j = i + 1;
            while j < items.len() && items[j] == items[i] {
                j += 1;
            }
            let count = j - i;
            if count > 1 && !(skip_octave_shifts && (items[i] == "<" || items[i] == ">")) {
                let expanded = items[i].repeat(count);
                let bracketed = format!("[{}]{}", items[i], count);
                out.push(if expanded.len() <= bracketed.len() {
                    expanded
                } else {
                    bracketed
                });
            } else {
                out.push(items[i].clone());
            }
            i = j;
        }
        return out;
    }

    // Compress multi-element chunk runs
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < items.len() {
        if i + group <= items.len() {
            let chunk = &items[i..i + group];
            let should_skip = skip_octave_shifts && chunk.iter().any(|t| t == "<" || t == ">");
            if !should_skip {
                let mut j = i + group;
                while j + group <= items.len() && &items[j..j + group] == chunk {
                    j += group;
                }
                let count = (j - i) / group;
                if count > 1 {
                    let joined = chunk.join("");
                    let expanded = joined.repeat(count);
                    let bracketed = format!("[{joined}]{count}");
                    out.push(if expanded.len() <= bracketed.len() {
                        expanded
                    } else {
                        bracketed
                    });
                    i = j;
                    continue;
                }
            }
        }
        out.push(items[i].clone());
        i += 1;
    }
    out
}

fn format_tokens(tokens: &[String]) -> String {
    let mut out = String::new();
    let mut last = "";
    for tok in tokens {
        let is_cmd = tok.starts_with('@') || tok.starts_with('O');
        let last_is_cmd = last.starts_with('@') || last.starts_with('O');
        if (is_cmd || ((tok == "<" || tok == ">") && last_is_cmd))
            && !out.is_empty()
            && !out.ends_with(' ')
        {
            out.push(' ');
        }
        out.push_str(tok);
        last = tok;
    }
    out.trim().to_string()
}

fn parse_notes_bits(s: &str) -> [i32; 12] {
    let mut out = [0; 12];
    for (i, ch) in s.bytes().take(12).enumerate() {
        out[i] = i32::from(ch.saturating_sub(b'0'));
    }
    out
}

fn root_from_bits(bits: &[i32; 12]) -> i32 {
    bits.iter().position(|v| *v == 2).unwrap_or(0) as i32
}

// Owned counterpart of ChordEntry — allows mixing static presets and runtime custom entries.
#[derive(Clone, Debug)]
struct OwnedChordEntry {
    loc: usize,
    notes: Option<String>,
    repeat: Option<usize>,
}

impl OwnedChordEntry {
    fn from_static(e: ChordEntry) -> Self {
        Self {
            loc: e.loc,
            notes: e.notes.map(String::from),
            repeat: e.repeat,
        }
    }
}

/// Resolve a chord selector (0..PRESET_COUNT = preset, >=PRESET_COUNT = custom slot) into
/// an owned progression that the downstream generators can consume uniformly.
fn resolve_progression(chord: i32, custom: Option<&[CustomChordEntryDef]>) -> Vec<OwnedChordEntry> {
    let chord_idx = chord as usize;
    if chord_idx >= PRESET_COUNT {
        if let Some(entries) = custom.filter(|e| !e.is_empty()) {
            let mut out: Vec<OwnedChordEntry> = entries
                .iter()
                .map(|e| OwnedChordEntry {
                    loc: e.loc.min(TOTAL_STEPS.saturating_sub(1)),
                    notes: e.notes.clone(),
                    repeat: e.repeat,
                })
                .collect();
            // `repeat` indices reference positions in the pre-sort TS-side array; after sorting
            // by loc we need to preserve those references. Since the TS encoder emits entries
            // in loc order, we skip sorting to keep `repeat` indices stable.
            // Guarantee there is an entry at loc 0 so every step has a defined chord.
            if out.first().is_none_or(|e| e.loc != 0) {
                out.insert(
                    0,
                    OwnedChordEntry {
                        loc: 0,
                        notes: Some("209019030909".to_string()),
                        repeat: None,
                    },
                );
                // Shift every repeat index up by 1 to account for the prepended entry.
                for entry in out.iter_mut().skip(1) {
                    if let Some(r) = entry.repeat.as_mut() {
                        *r += 1;
                    }
                }
            }
            return out;
        }
        // Fallback: hold I major for the full 8 bars.
        return vec![OwnedChordEntry {
            loc: 0,
            notes: Some("209019030909".to_string()),
            repeat: None,
        }];
    }
    CHORD_PROGRESSIONS[chord_idx.min(PRESET_COUNT - 1)]
        .iter()
        .copied()
        .map(OwnedChordEntry::from_static)
        .collect()
}

fn resolve_entry_notes(progressions: &[OwnedChordEntry], idx: usize) -> Option<&str> {
    progressions[idx].notes.as_deref().or_else(|| {
        progressions[idx]
            .repeat
            .and_then(|r| progressions.get(r))
            .and_then(|e| e.notes.as_deref())
    })
}

fn chord_bits_per_step(progression: &[OwnedChordEntry]) -> Vec<[i32; 12]> {
    let mut out = vec![[0; 12]; TOTAL_STEPS];

    for (loc, slot) in out.iter_mut().enumerate().take(TOTAL_STEPS) {
        let mut entry_idx = 0usize;
        for (i, e) in progression.iter().enumerate() {
            if e.loc <= loc {
                entry_idx = i;
            } else {
                break;
            }
        }
        *slot = resolve_entry_notes(progression, entry_idx).map_or([0; 12], parse_notes_bits);
    }
    out
}

fn rhythm_has_16th(line: &str) -> bool {
    line.as_bytes().windows(2).any(|w| w == b"00")
}

fn build_chord_note_pool(bits: &[i32; 12], key_shift: i32, lowest: i32) -> Vec<(i32, i32)> {
    let mut note_highest = None;
    let mut idx = 0i32;
    let mut results = Vec::new();
    loop {
        let note_type = bits[idx.rem_euclid(12) as usize];
        let note = 12 + idx + key_shift;
        if note >= lowest && matches!(note_type, 1 | 2 | 3 | 9) {
            results.push((note, note_type));
            if note_highest.is_none() {
                // Limit range to ~1 octave above the first valid note
                note_highest = Some(note + 15);
            }
        }
        if note_highest.is_some() && note >= note_highest.unwrap_or(note) {
            break;
        }
        idx += 1;
    }
    results
}

#[derive(Clone)]
struct MelodyChord {
    loc: usize,
    base: i32,
    no_root: bool,
    notes_bits: [i32; 12],
    notes: Vec<(i32, i32)>,
    repeat: Option<usize>,
}

#[derive(Clone)]
struct MelodyState {
    cur_chord_idx: i32,
    cur_chord_loc: usize,
    is_repeat: bool,
    chord_idx: usize,
    prev_note: i32,
    first_in_chord: bool,
}

const NOTE_UNSET: i32 = -2;
const NOTE_CONT: i32 = -3;

impl MelodyState {
    fn new() -> Self {
        Self {
            cur_chord_idx: -1,
            cur_chord_loc: 0,
            is_repeat: false,
            chord_idx: 0,
            prev_note: -1,
            first_in_chord: true,
        }
    }
}

fn build_melody_chord_plan(
    progression: &[OwnedChordEntry],
    key_shift: i32,
    lowest: i32,
) -> Vec<MelodyChord> {
    let mut out: Vec<MelodyChord> = Vec::with_capacity(progression.len());
    for p in progression {
        let mut base = 0;
        if let Some(repeat_idx) = p.repeat {
            base = out[repeat_idx].base;
        }
        let mut notes = Vec::new();
        let mut notes_bits = [0; 12];
        let mut no_root = false;
        if let Some(note_str) = p.notes.as_deref() {
            let notes_origin = parse_notes_bits(note_str);
            notes_bits = notes_origin;
            let mut note_chord_count = 0;
            for (i, v) in notes_origin.iter().enumerate() {
                if *v == 2 {
                    base = i as i32;
                }
                if matches!(*v, 1..=3) {
                    note_chord_count += 1;
                }
            }
            no_root = note_chord_count > 3;
            notes = build_chord_note_pool(&notes_origin, key_shift, lowest);
        }
        out.push(MelodyChord {
            loc: p.loc,
            base,
            no_root,
            notes_bits,
            notes,
            repeat: p.repeat,
        });
    }
    out
}

fn chord_at(plan: &[MelodyChord], loc: usize) -> (usize, usize) {
    let mut next_chord_loc = TOTAL_STEPS;
    for i in (0..plan.len()).rev() {
        if loc >= plan[i].loc {
            return (i, next_chord_loc);
        }
        next_chord_loc = plan[i].loc;
    }
    (0, next_chord_loc)
}

fn pick_rhythm_events(
    rng: &mut Xoshiro256StarStar,
    use_16th: bool,
    is_sub: bool,
) -> Vec<(usize, i32)> {
    loop {
        let mut results = Vec::new();
        let mut used16 = false;
        for bar in 0..BARS {
            let line = if is_sub {
                "0.0.0.0.0.0.0.0."
            } else {
                loop {
                    let idx = rng.random_range(0..RHYTHM_PATTERNS.len());
                    let line = RHYTHM_PATTERNS[idx];
                    if rhythm_has_16th(line) {
                        if !use_16th {
                            continue;
                        }
                        used16 = true;
                    }
                    if line.as_bytes().first().copied().unwrap_or(b'.') != b'.' {
                        break line;
                    }
                }
            };
            for (i, ch) in line.bytes().enumerate() {
                let val = match ch {
                    b'-' => -1,
                    b'0' => 0,
                    b'1'..=b'9' => i32::from(ch - b'0'),
                    _ => continue,
                };
                results.push((bar * STEPS_PER_BAR + i, val));
            }
        }
        if is_sub || !use_16th || used16 {
            results.push((TOTAL_STEPS, -1));
            results.push((TOTAL_STEPS, -1));
            return results;
        }
    }
}

fn place_melody(note_line: &mut [i32], loc: usize, note: i32, note_len: usize) {
    for i in 0..note_len {
        let pos = loc + i;
        if pos >= note_line.len() {
            break;
        }
        note_line[pos] = if i == 0 { note } else { NOTE_CONT };
    }
}

fn next_note_events(
    rhythm_set: &[(usize, i32)],
    loc: usize,
    is_sub: bool,
    state: &mut MelodyState,
    chord_plan: &[MelodyChord],
    melody: Option<&[Option<i32>]>,
    base: Option<&[Option<i32>]>,
    key_shift: i32,
    lowest: i32,
    rng: &mut Xoshiro256StarStar,
) -> Option<Vec<(usize, i32, usize)>> {
    let mut rhythm_kind = None;
    let mut rhythm_idx = 0usize;
    for (i, (rh_loc, rh_pat)) in rhythm_set.iter().enumerate() {
        if loc == *rh_loc {
            rhythm_idx = i;
            rhythm_kind = Some(*rh_pat);
            break;
        }
        if loc < *rh_loc {
            rhythm_idx = i;
            break;
        }
    }
    let note_len = rhythm_set[rhythm_idx + 1].0 - loc;

    let mut lookahead = false;
    let (mut next_chord_idx, mut next_chord_loc) = chord_at(chord_plan, loc);
    if next_chord_idx as i32 <= state.cur_chord_idx
        && !state.is_repeat
        && loc + note_len > next_chord_loc
    {
        (next_chord_idx, next_chord_loc) = chord_at(chord_plan, loc + note_len);
        lookahead = true;
    }

    if next_chord_idx as i32 > state.cur_chord_idx || lookahead {
        state.chord_idx = next_chord_idx;
        state.cur_chord_idx = next_chord_idx as i32;
        state.cur_chord_loc = loc;
        state.first_in_chord = true;
        state.is_repeat = chord_plan[state.chord_idx].repeat.is_some();
    }

    if state.is_repeat {
        return lookahead.then_some(Vec::new());
    }

    if rhythm_kind == Some(-1) {
        return Some(vec![(loc, -1, note_len)]);
    }

    let chord = &chord_plan[state.chord_idx];
    if chord.notes.is_empty() {
        return Some(vec![(loc, -1, note_len)]);
    }
    let no_root = state.first_in_chord || chord.no_root;
    let sub_pool = if is_sub {
        match (melody, base) {
            (Some(melody), Some(base)) => {
                let chord_bits = &chord.notes_bits;
                harmony_note_pool_at(loc, melody, base, chord_bits, key_shift, lowest)
            }
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };
    let note_pool: &[(i32, i32)] = if is_sub && !sub_pool.is_empty() {
        &sub_pool
    } else {
        &chord.notes
    };
    let mut next_idx = pick_target_note_idx(note_pool, state.prev_note, no_root, is_sub, rng);

    let mut following = Vec::new();
    let mut prev_loc = loc;
    loop {
        let next_rhythm_loc = rhythm_set[rhythm_idx + 1 + following.len()].0;
        let no_next =
            next_rhythm_loc >= next_chord_loc || next_rhythm_loc.saturating_sub(prev_loc) > 4;
        if following.is_empty() || !no_next {
            following.push((prev_loc, next_rhythm_loc.saturating_sub(prev_loc)));
        }
        if no_next {
            break;
        }
        prev_loc = next_rhythm_loc;
    }

    let (first_loc, first_len) = following[0];
    let mut cur_idx = None;
    if !lookahead {
        cur_idx = find_chord_note_index(note_pool, state.prev_note);
    }
    if state.prev_note < 0 || cur_idx.is_none() {
        return Some(vec![(first_loc, note_pool[next_idx].0, first_len)]);
    }
    let mut cur_idx = cur_idx.unwrap_or(0);
    let diff = (next_idx as i32 - cur_idx as i32).unsigned_abs() as usize;
    let direction = if next_idx > cur_idx { 1isize } else { -1isize };

    if diff == 0 {
        let cnt = following.len() / 2;
        if cnt == 0 || is_sub || rng.random_range(0..=1) == 0 {
            return Some(vec![(first_loc, state.prev_note, first_len)]);
        }
        let mut results = Vec::new();
        for i in 0..cnt {
            while next_idx == cur_idx {
                // Keep Python behavior: retry uses default pick_target_note()
                // Semantics (is_sub = false) even when current path is sub.
                next_idx = pick_target_note_idx(&chord.notes, state.prev_note, no_root, false, rng);
            }
            let dir = if next_idx > cur_idx { 1isize } else { -1isize };
            let next_pos = (cur_idx as isize + dir) as usize;
            let note = note_pool[next_pos].0;
            let prev_note = state.prev_note;
            let a = following[i * 2];
            results.push((a.0, note, a.1));
            let b = following[i * 2 + 1];
            results.push((b.0, prev_note, b.1));
        }
        return Some(results);
    }

    if diff > following.len() {
        return Some(vec![(first_loc, note_pool[next_idx].0, first_len)]);
    }

    let mut results = Vec::new();
    let mut i = 0usize;
    while next_idx != cur_idx {
        cur_idx = (cur_idx as isize + direction) as usize;
        let note = note_pool[cur_idx].0;
        let seg = following[i];
        results.push((seg.0, note, seg.1));
        i += 1;
    }
    Some(results)
}

fn melody_has_required_tones(
    notes: &[Option<i32>],
    sub_notes: Option<&[Option<i32>]>,
    chord_plan: &[MelodyChord],
) -> bool {
    let mut cur_chord_idx: i32 = -1;
    let mut need: Vec<i32> = Vec::new();

    for (loc, note) in notes.iter().enumerate().take(TOTAL_STEPS) {
        let (next_chord_idx, _) = chord_at(chord_plan, loc);
        if next_chord_idx as i32 > cur_chord_idx {
            if !need.is_empty() {
                return false;
            }
            cur_chord_idx = next_chord_idx as i32;
            need.clear();
            for (n, note_type) in &chord_plan[next_chord_idx].notes {
                if *note_type == 1 {
                    let semi = n.rem_euclid(12);
                    if !need.contains(&semi) {
                        need.push(semi);
                    }
                }
            }
        }

        if let Some(n) = note {
            if *n >= 0 {
                let semi = n.rem_euclid(12);
                need.retain(|x| *x != semi);
            }
        }
        if let Some(sub) = sub_notes {
            if let Some(n) = sub[loc] {
                if n >= 0 {
                    let semi = n.rem_euclid(12);
                    need.retain(|x| *x != semi);
                }
            }
        }
    }
    need.is_empty()
}

fn pick_target_note(
    chord_notes: &[(i32, i32)],
    prev_note: i32,
    no_root: bool,
    is_sub: bool,
    rng: &mut Xoshiro256StarStar,
) -> i32 {
    let allowed: &[i32] = if no_root { &[1, 3] } else { &[1, 2, 3] };
    let mut highest_note = 0;
    let mut highest_idx = 0usize;
    for (idx, (note, note_type)) in chord_notes.iter().enumerate() {
        if allowed.contains(note_type) && *note > highest_note {
            highest_note = *note;
            highest_idx = idx;
        }
    }
    if prev_note >= 0 && prev_note - highest_note > 12 {
        return chord_notes[highest_idx].0;
    }

    loop {
        let idx = rng.random_range(0..chord_notes.len());
        let (note, note_type) = chord_notes[idx];
        if !allowed.contains(&note_type) {
            continue;
        }
        if prev_note >= 0 {
            let diff = (prev_note - note).abs();
            if diff > 12 {
                continue;
            }
            let factor = if diff == 12 { 6 } else { diff };
            if rng.random_range(0..16) < factor as usize && !is_sub {
                continue;
            }
        }
        return note;
    }
}

fn find_chord_note_index(chord_notes: &[(i32, i32)], note: i32) -> Option<usize> {
    chord_notes.iter().position(|&(n, _)| n == note)
}

fn pick_target_note_idx(
    chord_notes: &[(i32, i32)],
    prev_note: i32,
    no_root: bool,
    is_sub: bool,
    rng: &mut Xoshiro256StarStar,
) -> usize {
    let target_note = pick_target_note(chord_notes, prev_note, no_root, is_sub, rng);
    find_chord_note_index(chord_notes, target_note).unwrap_or(0)
}

fn generate_melody(
    progression: &[OwnedChordEntry],
    density: i32,
    use_16th: bool,
    lowest: i32,
    key_shift: i32,
    base: &[Option<i32>],
    rng: &mut Xoshiro256StarStar,
    require_tones: bool,
) -> (Vec<Option<i32>>, Vec<Option<i32>>) {
    let density = density as usize;
    let chord_plan = build_melody_chord_plan(progression, key_shift, lowest);
    loop {
        let mut note_line = vec![NOTE_UNSET; TOTAL_STEPS];
        let mut melody_view = vec![None; TOTAL_STEPS];
        let mut sub_seed = vec![Some(NOTE_UNSET); TOTAL_STEPS];

        let mut rhythm_main_list: Vec<Vec<(usize, i32)>> = Vec::with_capacity(5);
        for _ in 0..5 {
            rhythm_main_list.push(pick_rhythm_events(rng, use_16th, false));
        }
        rhythm_main_list.sort_by_key(Vec::len);
        let rhythm_main = &rhythm_main_list[density.min(rhythm_main_list.len() - 1)];

        let mut state = MelodyState::new();

        for loc in 0..TOTAL_STEPS {
            if note_line[loc] != NOTE_UNSET {
                continue;
            }
            let note_events = next_note_events(
                rhythm_main,
                loc,
                false,
                &mut state,
                &chord_plan,
                None,
                None,
                key_shift,
                lowest,
                rng,
            );
            if note_events.is_none() {
                let repeat_idx = chord_plan[state.chord_idx].repeat.unwrap_or(0);
                let repeat_loc = chord_plan[repeat_idx].loc;
                let target_loc = repeat_loc + (loc - state.cur_chord_loc);
                let repeat_note = note_line[target_loc];
                place_melody(&mut note_line, loc, repeat_note, 1);
                melody_view[loc] = if repeat_note == NOTE_CONT {
                    None
                } else {
                    Some(repeat_note)
                };
                sub_seed[loc] = sub_seed[target_loc];
                if repeat_note != NOTE_UNSET && repeat_note != NOTE_CONT {
                    state.prev_note = repeat_note;
                    state.first_in_chord = false;
                }
                continue;
            }
            let mut total_event_len = 0usize;
            for (l, n, len) in note_events.unwrap_or_default() {
                place_melody(&mut note_line, l, n, len);
                let end = (l + len).min(TOTAL_STEPS);
                for pos in l..end {
                    melody_view[pos] = if note_line[pos] == NOTE_CONT {
                        None
                    } else {
                        Some(note_line[pos])
                    };
                }
                state.prev_note = n;
                state.first_in_chord = false;
                total_event_len += len;
            }
            if total_event_len > 0 {
                place_harmony(
                    &mut sub_seed,
                    &chord_plan[state.chord_idx].notes_bits,
                    &melody_view,
                    base,
                    key_shift,
                    lowest,
                    loc,
                    NOTE_UNSET,
                    total_event_len,
                );
            }
        }

        if note_line.contains(&NOTE_UNSET) {
            continue;
        }

        if !require_tones || melody_has_required_tones(&melody_view, None, &chord_plan) {
            let sub_seed: Vec<Option<i32>> = sub_seed
                .into_iter()
                .map(|v| match v {
                    Some(x) if x == NOTE_UNSET => None,
                    _ => v,
                })
                .collect();
            return (melody_view, sub_seed);
        }
    }
}

fn generate_bass(base: i32, bits_per_step: &[[i32; 12]], key_shift: i32) -> Vec<Option<i32>> {
    let mut notes = vec![Some(-1); TOTAL_STEPS];
    let bass_idx = base as usize;
    let (basic, final_pat) = BASS_PATTERNS[bass_idx];
    let adjust_list = [0, -1, 1, -2, 2, -3, 3];
    let base_highest_note = 26i32;
    for bar in 0..BARS {
        let pat = if bar < 7 {
            basic.as_bytes()
        } else {
            final_pat.as_bytes()
        };
        for (step, pat_cell) in pat.iter().enumerate().take(STEPS_PER_BAR) {
            let step_symbol = *pat_cell as char;
            let idx = bar * STEPS_PER_BAR + step;
            let bits = bits_per_step[idx];
            let root = root_from_bits(&bits) + key_shift;
            let mut base_root = 12 + root;
            while base_root + 24 > base_highest_note {
                base_root -= 12;
            }
            if step_symbol == '.' {
                notes[idx] = None;
                continue;
            }
            if step_symbol == '0' {
                notes[idx] = Some(-1);
                continue;
            }
            let base_add = match step_symbol {
                '1' => 7,
                '3' => 19,
                '4' => 24,
                _ => 12,
            };
            let mut chosen = base_root + base_add;
            for a in adjust_list {
                let n = base_root + base_add + a;
                if matches!(bits[((n - key_shift).rem_euclid(12)) as usize], 1..=3) {
                    chosen = n;
                    break;
                }
            }
            notes[idx] = Some(chosen);
        }
    }
    notes
}

fn harmony_note_pool_at(
    start_loc: usize,
    melody: &[Option<i32>],
    base: &[Option<i32>],
    chord_bits: &[i32; 12],
    key_shift: i32,
    lowest: i32,
) -> Vec<(i32, i32)> {
    let mut master_note: Option<i32> = None;
    let mut base_note: Option<i32> = None;
    let mut loc = start_loc % TOTAL_STEPS;
    for _ in 0..TOTAL_STEPS {
        if master_note.is_none() && melody[loc] != Some(-1) {
            if let Some(v) = melody[loc] {
                master_note = Some(v);
            }
        }
        if base_note.is_none() && base[loc] != Some(-1) {
            if let Some(v) = base[loc] {
                base_note = Some(v);
            }
        }
        if master_note.is_some() && base_note.is_some() {
            break;
        }
        loc = (loc + TOTAL_STEPS - 1) % TOTAL_STEPS;
    }

    let master = master_note.unwrap_or(-1);
    let base_min = base_note.unwrap_or(lowest - 3) + 3;
    let mut results = Vec::new();
    let mut has_important_tone = false;
    let mut idx = 0i32;
    loop {
        let note_type = chord_bits[idx.rem_euclid(12) as usize];
        if matches!(note_type, 1 | 2 | 3 | 9) {
            let note = 12 + idx + key_shift;
            if note > master - 3 && has_important_tone {
                break;
            }
            if note >= base_min {
                results.push((note, note_type));
                if matches!(note_type, 1 | 3) {
                    has_important_tone = true;
                }
            }
        }
        idx += 1;
    }
    results
}

fn find_lower_harmony_at(
    prev_note: i32,
    master_note: i32,
    loc: usize,
    melody: &[Option<i32>],
    base: &[Option<i32>],
    chord_bits: &[i32; 12],
    key_shift: i32,
    lowest: i32,
) -> i32 {
    let notes = harmony_note_pool_at(loc, melody, base, chord_bits, key_shift, lowest);
    if (prev_note - master_note).abs() >= 3 {
        return prev_note;
    }
    let mut cur = master_note - 3;
    while cur >= lowest {
        for (note, note_type) in &notes {
            if *note == cur && matches!(*note_type, 1..=3) {
                return cur;
            }
        }
        cur -= 1;
    }
    -1
}

fn place_harmony(
    sub: &mut [Option<i32>],
    chord_bits: &[i32; 12],
    melody: &[Option<i32>],
    base: &[Option<i32>],
    key_shift: i32,
    lowest: i32,
    loc: usize,
    note: i32,
    note_len: usize,
) {
    let mut master_note: Option<i32> = None;
    let mut sub_note = note;
    let mut master_loc = loc as i32;
    while master_loc >= 0 {
        if let Some(m) = melody[master_loc as usize] {
            if m >= 0 {
                master_note = Some(m);
                let prev = if note == NOTE_UNSET { m } else { note };
                sub_note = find_lower_harmony_at(
                    prev,
                    m,
                    master_loc as usize,
                    melody,
                    base,
                    chord_bits,
                    key_shift,
                    lowest,
                );
                break;
            }
        }
        master_loc -= 1;
    }

    let mut prev_sub: Option<i32> = None;
    for idx in 0..note_len {
        let pos = loc + idx;
        if pos >= TOTAL_STEPS {
            break;
        }
        if let Some(m) = melody[pos] {
            if m >= 0 {
                master_note = Some(m);
            }
        }
        if let Some(m) = master_note {
            sub_note = find_lower_harmony_at(
                sub_note, m, pos, melody, base, chord_bits, key_shift, lowest,
            );
        }
        let out = if prev_sub == Some(sub_note) {
            None
        } else {
            Some(sub_note)
        };
        sub[pos] = out;
        prev_sub = Some(sub_note);
    }
}

fn generate_submelody(
    progression: &[OwnedChordEntry],
    melody: &[Option<i32>],
    sub_seed: &[Option<i32>],
    base: &[Option<i32>],
    key_shift: i32,
    lowest: i32,
    rng: &mut Xoshiro256StarStar,
) -> Vec<Option<i32>> {
    let chord_plan = build_melody_chord_plan(progression, key_shift, lowest);
    let rhythm_sub = pick_rhythm_events(rng, true, true);
    let mut state = MelodyState::new();

    let mut sub = sub_seed.to_vec();
    let mut prev_note_loc: i32 = -1;
    for loc in 0..TOTAL_STEPS {
        if let Some(n) = sub[loc] {
            if n >= 0 {
                prev_note_loc = loc as i32;
                state.prev_note = n;
                continue;
            }
        }
        if (loc as i32) - prev_note_loc < 4 || loc % 4 != 0 {
            continue;
        }
        if let Some(note_events) = next_note_events(
            &rhythm_sub,
            loc,
            true,
            &mut state,
            &chord_plan,
            Some(melody),
            Some(base),
            key_shift,
            lowest,
            rng,
        ) {
            for (l, n, len) in note_events {
                place_harmony(
                    &mut sub,
                    &chord_plan[state.chord_idx].notes_bits,
                    melody,
                    base,
                    key_shift,
                    lowest,
                    l,
                    n,
                    len,
                );
            }
            prev_note_loc = loc as i32;
        }
    }
    sub
}

fn shifted_melody(melody: &[Option<i32>]) -> Vec<Option<i32>> {
    // Shift melody forward by 1 step (wrapping last to first)
    let mut notes = Vec::with_capacity(TOTAL_STEPS);
    notes.push(melody[TOTAL_STEPS - 1]);
    notes.extend_from_slice(&melody[..TOTAL_STEPS - 1]);
    notes
}

fn generate_drums(drums: i32) -> Vec<Option<i32>> {
    let drum_idx = drums as usize;
    let (basic, final_pat) = DRUM_PATTERNS[drum_idx];
    (0..TOTAL_STEPS)
        .map(|i| {
            let bar = i / STEPS_PER_BAR;
            let pat = if bar % 4 < 3 { basic } else { final_pat };
            let step_symbol = pat.as_bytes()[i % STEPS_PER_BAR];
            if step_symbol == b'0' {
                None
            } else {
                Some(i32::from(step_symbol - b'0'))
            }
        })
        .collect()
}

fn current_bar_mut(bar_tokens: &mut [Vec<String>]) -> &mut Vec<String> {
    bar_tokens.last_mut().expect("bar tokens")
}

fn notes_to_mml(
    notes: &[Option<i32>],
    tempo: i32,
    tone_idx: usize,
    volume: i32,
    quantize: i32,
    drums: bool,
) -> String {
    // Note event representation:
    // Some(note >= 0): Note onset
    // Some(-1): Rest onset
    // None: Continuation from previous step
    let default_len = select_default_length(notes);
    let wave = TONE_LIBRARY[tone_idx][0];
    let env_def = env_def_from_tone(tone_idx, 1);
    let vib_def = vib_def_from_tone(tone_idx, 1);
    let has_vib = vib_def.is_some();

    let mut tokens = vec![format!("T{tempo}"), format!("L{default_len}"), env_def];
    if let Some(vib) = vib_def {
        tokens.push(vib);
    }
    tokens.push(format!("Q{quantize}"));
    tokens.push(format!("V{volume}"));
    tokens.push(format!("@{wave}"));
    tokens.push("@ENV1".to_string());
    tokens.push(if has_vib {
        "@VIB1".to_string()
    } else {
        "@VIB0".to_string()
    });
    let used_keys = if drums {
        let used = used_drum_keys(notes);
        let insert_at = tokens
            .iter()
            .position(|t| t.starts_with('Q'))
            .unwrap_or(tokens.len());
        for (idx, key) in used.iter().enumerate() {
            tokens.insert(
                insert_at + idx,
                env_def_from_drum_key(*key, (idx as i32) + 2),
            );
        }
        used
    } else {
        Vec::new()
    };
    let mut cur_oct = None;
    let mut drum_note_idx = [0usize; 10];
    let drum_env_slot = if drums {
        drum_env_slots(&used_keys)
    } else {
        [0i32; 10]
    };
    let mut cur_wave = wave;
    let mut cur_env_slot = 1i32;
    let mut bar_tokens: Vec<Vec<String>> = vec![Vec::new()];
    let mut bar_units = 0usize;
    let mut i = 0usize;
    while i < notes.len() {
        let Some(event) = notes[i] else {
            i += 1;
            continue;
        };
        let mut len = 1usize;
        while i + len < notes.len() && notes[i + len].is_none() {
            len += 1;
        }
        i += len;

        if event == -1 {
            let mut remaining = len;
            while remaining > 0 {
                let space = 16 - bar_units;
                let seg = remaining.min(space);
                let token = note_token("R", seg, default_len, false);
                current_bar_mut(&mut bar_tokens[..]).push(token);
                bar_units += seg;
                remaining -= seg;
                if bar_units == 16 {
                    bar_tokens.push(Vec::new());
                    bar_units = 0;
                }
            }
            continue;
        }

        let pitch = if drums {
            let key = event.clamp(0, 9) as usize;
            if let Some(drum_idx) = drum_key_to_idx(key as i32) {
                let target_wave = DRUM_WAVES[drum_idx];
                if cur_wave != target_wave {
                    current_bar_mut(&mut bar_tokens[..]).push(format!("@{target_wave}"));
                    cur_wave = target_wave;
                }
                let target_env_slot = drum_env_slot[key];
                if target_env_slot > 0 && cur_env_slot != target_env_slot {
                    current_bar_mut(&mut bar_tokens[..]).push(format!("@ENV{target_env_slot}"));
                    cur_env_slot = target_env_slot;
                }
            }
            let note_list = drum_notes_for_key(key as i32);
            let idx = drum_note_idx[key] % note_list.len();
            drum_note_idx[key] += 1;
            note_list[idx]
        } else {
            event
        };
        let (name, oct) = note_name(pitch);
        let mut remaining = len;
        let mut first = true;
        while remaining > 0 {
            let space = 16 - bar_units;
            let seg = remaining.min(space);
            if first {
                push_octave(current_bar_mut(&mut bar_tokens[..]), &mut cur_oct, oct);
            }
            let tie_out = remaining > seg;
            let token = note_token(name, seg, default_len, tie_out);
            current_bar_mut(&mut bar_tokens[..]).push(token);
            bar_units += seg;
            remaining -= seg;
            first = false;
            if bar_units == 16 {
                bar_tokens.push(Vec::new());
                bar_units = 0;
            }
        }
    }

    if bar_tokens.last().is_some_and(Vec::is_empty) {
        bar_tokens.pop();
    }

    let mut bar_strings: Vec<String> = Vec::with_capacity(bar_tokens.len());
    for bar in &bar_tokens {
        let mut compressed = compress_repeats(bar, 4, true);
        if compressed == *bar {
            compressed = compress_repeats(bar, 2, true);
        }
        if compressed == *bar {
            compressed = compress_repeats(bar, 1, true);
        }
        bar_strings.push(format_tokens(&compressed));
    }

    let mut compressed = compress_repeats(&bar_strings, 2, false);
    if compressed == bar_strings {
        compressed = compress_repeats(&bar_strings, 1, false);
    }

    tokens.extend(compressed);
    tokens.join(" ")
}

fn silent_channel_mml(tempo: i32) -> String {
    format!("T{tempo} L16 @ENV1{{127}} Q100 V112 @0 @ENV1 @VIB0")
}

// Generate BGM as MML strings (one-shot: preset → MML)
pub fn generate_bgm_mml(
    preset: i32,
    transpose: i32,
    instrumentation: i32,
    seed: u64,
) -> Vec<String> {
    let mut params = preset_params(preset);
    params.transpose = transpose;
    params.instrumentation = instrumentation;
    let data = generate_bgm(&params, seed);
    compile_to_mml(&data)
}

// JSON interface for composer WASM
#[cfg(not(pyxel_core))]
pub fn preset_params_json(preset: i32) -> String {
    preset_params(preset).to_json()
}

#[cfg(not(pyxel_core))]
pub fn generate_bgm_json(params_json: &str, seed: u64) -> String {
    let params = GeneratorParams::from_json(params_json);
    generate_bgm(&params, seed).to_json()
}

#[cfg(not(pyxel_core))]
pub fn compile_to_mml_json(bgm_json: &str) -> String {
    let data = BgmData::from_json(bgm_json);
    serde_json::to_string(&compile_to_mml(&data)).expect("MML serialization failed")
}

/// Return the resolved progression for a chord slot as JSON. Presets (0..=7) return their
/// static `CHORD_PROGRESSIONS` entry. Custom slots (>= PRESET_COUNT) fall through to the
/// resolver's default (I major held for 8 bars) — the UI should not call this for custom
/// slots, those are stored client-side.
#[cfg(not(pyxel_core))]
pub fn preset_progression_json(preset: i32) -> String {
    let entries = resolve_progression(preset, None);
    let defs: Vec<CustomChordEntryDef> = entries
        .into_iter()
        .map(|e| CustomChordEntryDef {
            loc: e.loc,
            notes: e.notes,
            repeat: e.repeat,
        })
        .collect();
    serde_json::to_string(&defs).expect("preset progression serialization failed")
}

// --- New structured generation pipeline ---

const BASS_TONE_IDX: usize = 7;
const DRUM_TONE_IDX: usize = 15;

fn make_channel(notes: Vec<Option<i32>>, tone_idx: i32, volume: i32, quantize: i32) -> Channel {
    let len = notes.len();
    let mut tones = vec![None; len];
    let mut volumes = vec![None; len];
    let mut quantizes = vec![None; len];
    tones[0] = Some(tone_idx);
    volumes[0] = Some(volume);
    quantizes[0] = Some(quantize);
    Channel {
        notes,
        tones,
        volumes,
        quantizes,
    }
}

fn silent_channel() -> Channel {
    Channel {
        notes: vec![],
        tones: vec![],
        volumes: vec![],
        quantizes: vec![],
    }
}

fn build_tone(idx: usize) -> Tone {
    let t = TONE_LIBRARY[idx];
    Tone {
        wave: t[0],
        attack: t[1],
        decay: t[2],
        sustain: t[3],
        release: t[4],
        vibrato: t[5],
        drum_notes: if idx == DRUM_TONE_IDX {
            // Collect all drum note sequences
            let mut dn = Vec::new();
            dn.extend_from_slice(&DRUM_NOTES_1);
            dn.extend_from_slice(&DRUM_NOTES_2);
            dn.extend_from_slice(&DRUM_NOTES_3);
            dn.extend_from_slice(&DRUM_NOTES_5);
            dn.extend_from_slice(&DRUM_NOTES_6);
            dn.extend_from_slice(&DRUM_NOTES_7);
            dn
        } else {
            vec![]
        },
    }
}

fn generate_bgm(params: &GeneratorParams, seed: u64) -> BgmData {
    assert!((-5..=5).contains(&params.transpose), "invalid transpose");
    assert!(
        (0..=3).contains(&params.instrumentation),
        "invalid instrumentation"
    );
    assert!(params.speed >= 1, "invalid speed");
    assert!((0..10).contains(&params.chord), "invalid chord");
    assert!((0..8).contains(&params.base), "invalid base");
    assert!(
        (12..=15).contains(&params.base_quantize),
        "invalid base_quantize"
    );
    assert!((0..8).contains(&params.drums), "invalid drums");
    assert!((0..6).contains(&params.melo_tone), "invalid melo_tone");
    assert!((0..6).contains(&params.sub_tone), "invalid sub_tone");
    assert!(
        (28..=33).contains(&params.melo_lowest_note),
        "invalid melo_lowest_note"
    );
    assert!(
        (0..=4).contains(&params.melo_density),
        "invalid melo_density"
    );

    let instr = params.instrumentation as usize;
    let key_shift = params.transpose;
    let speed = params.speed.max(1);
    let tempo = (28800 / speed).max(1);
    let chord = params.chord;
    let density = params.melo_density;
    let use_16th = params.melo_use16;
    let lowest = params.melo_lowest_note;

    let mut rng = Xoshiro256StarStar::seed_from_u64(seed);

    let progression = resolve_progression(chord, params.custom_progression.as_deref());
    let bits_per_step = chord_bits_per_step(&progression);
    let bass = generate_bass(params.base, &bits_per_step, key_shift);
    let mut melody_and_seed = generate_melody(
        &progression,
        density,
        use_16th,
        lowest,
        key_shift,
        &bass,
        &mut rng,
        instr < 2,
    );
    let mut submelody = None;

    if instr >= 2 {
        let chord_plan = build_melody_chord_plan(&progression, key_shift, lowest);
        let mut candidate = generate_submelody(
            &progression,
            &melody_and_seed.0,
            &melody_and_seed.1,
            &bass,
            key_shift,
            lowest,
            &mut rng,
        );
        loop {
            if melody_has_required_tones(&melody_and_seed.0, Some(&candidate), &chord_plan) {
                break;
            }
            melody_and_seed = generate_melody(
                &progression,
                density,
                use_16th,
                lowest,
                key_shift,
                &bass,
                &mut rng,
                false,
            );
            candidate = generate_submelody(
                &progression,
                &melody_and_seed.0,
                &melody_and_seed.1,
                &bass,
                key_shift,
                lowest,
                &mut rng,
            );
        }
        submelody = Some(candidate);
    }

    let (melody, _) = melody_and_seed;
    let melo_tone_idx = TONE_CANDIDATES[params.melo_tone as usize] as i32;
    let sub_tone_idx = TONE_CANDIDATES[params.sub_tone as usize] as i32;
    let base_quantize = ((params.base_quantize * 100) + 8) / 16;

    // Build 4 channels based on instrumentation
    let ch0 = make_channel(melody.clone(), melo_tone_idx, 96, 88);
    let ch1 = make_channel(bass, BASS_TONE_IDX as i32, 112, base_quantize);

    let (ch2, ch3) = if instr == 0 {
        let shifted = shifted_melody(&melody);
        (
            make_channel(shifted, melo_tone_idx, 32, 88),
            silent_channel(),
        )
    } else {
        let mut c2 = silent_channel();
        let mut c3 = silent_channel();
        if instr == 1 || instr == 3 {
            let drum = generate_drums(params.drums);
            if instr == 1 {
                c2 = make_channel(drum, DRUM_TONE_IDX as i32, 80, 94);
            } else {
                c3 = make_channel(drum, DRUM_TONE_IDX as i32, 80, 94);
            }
        }
        if instr == 2 || instr == 3 {
            let sub = submelody.unwrap_or_else(|| vec![Some(-1); TOTAL_STEPS]);
            c2 = make_channel(sub, sub_tone_idx, 64, 94);
        }
        (c2, c3)
    };

    // Collect unique tone indices used
    let mut tone_indices: Vec<usize> = Vec::new();
    for ch in [&ch0, &ch1, &ch2, &ch3] {
        for idx in ch.tones.iter().flatten() {
            let idx = *idx as usize;
            if !tone_indices.contains(&idx) {
                tone_indices.push(idx);
            }
        }
    }
    tone_indices.sort_unstable();

    // Build full 16-slot tone table (sparse — only used slots populated)
    let mut tones = Vec::with_capacity(tone_indices.len().max(1));
    for &idx in &tone_indices {
        // Pad with default tones up to this index
        while tones.len() < idx {
            tones.push(build_tone(0));
        }
        tones.push(build_tone(idx));
    }
    // Ensure at least one tone
    if tones.is_empty() {
        tones.push(build_tone(0));
    }

    BgmData {
        tempo,
        tones,
        channels: vec![ch0, ch1, ch2, ch3],
    }
}

fn compile_to_mml(data: &BgmData) -> Vec<String> {
    data.channels
        .iter()
        .map(|ch| {
            if ch.notes.is_empty() {
                return silent_channel_mml(data.tempo);
            }
            let tone_idx = ch.tones.iter().find_map(|t| *t).unwrap_or(0) as usize;
            let volume = ch.volumes.iter().find_map(|v| *v).unwrap_or(96);
            let quantize = ch.quantizes.iter().find_map(|q| *q).unwrap_or(88);
            let is_drum = tone_idx == DRUM_TONE_IDX;
            notes_to_mml(&ch.notes, data.tempo, tone_idx, volume, quantize, is_drum)
        })
        .collect()
}

#[cfg(pyxel_core)]
impl Pyxel {
    pub fn gen_bgm(
        &mut self,
        preset: i32,
        transpose: i32,
        instrumentation: i32,
        seed: u64,
        play: Option<bool>,
    ) -> Vec<String> {
        let mml_list = generate_bgm_mml(preset, transpose, instrumentation, seed);

        if play.unwrap_or(false) {
            for (ch, mml) in mml_list.iter().enumerate() {
                let sound = Sound::new();
                if unsafe { &mut *sound }.set_mml(mml).is_ok() {
                    crate::platform::lock_audio();
                    unsafe { &mut *pyxel::channels()[ch] }.play_sound(sound, None, true, false);
                    crate::platform::unlock_audio();
                }
            }
        }

        mml_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_params_returns_valid_params() {
        let p = preset_params(0);
        assert_eq!(p.speed, 216);
        assert_eq!(p.chord, 0);
        assert_eq!(p.base, 4);
        assert_eq!(p.transpose, 0);
        assert_eq!(p.instrumentation, 3);
    }

    #[test]
    fn test_preset_params_clamps_out_of_range() {
        let p = preset_params(-1);
        assert_eq!(p.speed, 216); // preset 0
        let p = preset_params(99);
        assert_eq!(p.speed, 168); // preset 7
    }

    #[test]
    fn test_compile_generate_bgm_produces_valid_mml() {
        let params = preset_params(0);
        let data = generate_bgm(&params, 42);
        let mml = compile_to_mml(&data);
        assert_eq!(mml.len(), 4);
        for (i, s) in mml.iter().enumerate() {
            assert!(s.starts_with('T'), "channel {i} MML should start with T");
            assert!(s.len() > 10, "channel {i} MML should have content");
        }
    }

    #[test]
    fn test_generate_bgm_mml_all_presets() {
        // Verify generate_bgm_mml() produces valid MML for all presets
        for preset_idx in 0..PRESET_COUNT as i32 {
            let mml = generate_bgm_mml(preset_idx, 0, 0, 12345);
            assert_eq!(
                mml.len(),
                4,
                "preset {preset_idx} should produce 4 channels"
            );
            for (ch, s) in mml.iter().enumerate() {
                assert!(
                    s.starts_with('T'),
                    "preset {preset_idx} ch{ch} should start with T"
                );
            }
        }
    }

    #[test]
    fn test_generate_bgm_mml_with_overrides() {
        let mml_default = generate_bgm_mml(0, 0, 3, 42);
        let mml_transposed = generate_bgm_mml(0, 3, 3, 42);
        assert_ne!(mml_default, mml_transposed, "transpose should change MML");

        let mml_instr = generate_bgm_mml(0, 0, 0, 42);
        assert_ne!(
            mml_default, mml_instr,
            "instrumentation override should change MML"
        );
    }

    #[test]
    fn test_generate_bgm_mml_matches_pipeline() {
        // One-shot and pipeline should produce identical MML
        for preset_idx in 0..PRESET_COUNT as i32 {
            let params = preset_params(preset_idx);
            let one_shot =
                generate_bgm_mml(preset_idx, params.transpose, params.instrumentation, 12345);
            let data = generate_bgm(&params, 12345);
            let pipeline = compile_to_mml(&data);
            assert_eq!(one_shot, pipeline, "mismatch for preset {preset_idx}");
        }
    }

    #[cfg(not(pyxel_core))]
    #[test]
    fn test_bgm_data_json_roundtrip() {
        let data = BgmData {
            tempo: 133,
            tones: vec![Tone {
                wave: 0,
                attack: 0,
                decay: 0,
                sustain: 100,
                release: 0,
                vibrato: 0,
                drum_notes: vec![],
            }],
            channels: vec![Channel {
                notes: vec![Some(24), None, Some(-1), Some(36)],
                tones: vec![Some(0), None, None, None],
                volumes: vec![Some(96), None, None, None],
                quantizes: vec![Some(88), None, None, None],
            }],
        };
        let json = data.to_json();
        let restored = BgmData::from_json(&json);
        assert_eq!(data, restored);
    }

    #[cfg(not(pyxel_core))]
    #[test]
    fn test_generator_params_json_roundtrip() {
        let params = preset_params(0);
        let json = params.to_json();
        let restored = GeneratorParams::from_json(&json);
        assert_eq!(params, restored);
    }

    #[cfg(not(pyxel_core))]
    #[test]
    fn test_json_pipeline_matches_direct() {
        let params_json = preset_params_json(0);
        let bgm_json = generate_bgm_json(&params_json, 42);
        let mml_json = compile_to_mml_json(&bgm_json);
        let mml_from_json: Vec<String> = serde_json::from_str(&mml_json).unwrap();
        let mml_direct = generate_bgm_mml(0, 0, 3, 42);
        assert_eq!(mml_from_json, mml_direct);
    }
}

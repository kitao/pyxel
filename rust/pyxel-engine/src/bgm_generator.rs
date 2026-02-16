/*
    Based on 8bit BGM generator by frenchbread
    https://github.com/shiromofufactory/8bit-bgm-generator
*/
use std::fmt::Write as _;

use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;
use crate::sound::Sound;

const BARS: usize = 8;
const STEPS_PER_BAR: usize = 16;
const TOTAL_STEPS: usize = BARS * STEPS_PER_BAR;
const PRESET_COUNT: usize = 16;
const TONE_CANDIDATES: [usize; 6] = [11, 8, 2, 10, 6, 4];

const PRESET_SPEED: usize = 0;
const PRESET_CHORD: usize = 1;
const PRESET_BASE: usize = 2;
const PRESET_BASE_QUANTIZE: usize = 3;
const PRESET_DRUMS: usize = 4;
const PRESET_MELO_TONE: usize = 5;
const PRESET_SUB_TONE: usize = 6;
const PRESET_MELO_LOWEST_NOTE: usize = 7;
const PRESET_MELO_DENSITY: usize = 8;
const PRESET_MELO_USE16: usize = 9;

fn normalize_transp(transp: i32) -> i32 {
    transp.clamp(-5, 6)
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

// Preset fields:
// Speed, chord, base, base_quantize, drums
// Melo_tone, sub_tone, melo_lowest_note, melo_density, melo_use16
const PRESET_SETS: [[i32; 10]; PRESET_COUNT] = [
    //  Speed Chord Base BQ Drums MTone STone Lowest Density Use16
    [216, 0, 4, 14, 4, 0, 0, 28, 2, 1], // 0: Original
    [216, 1, 6, 12, 5, 3, 3, 28, 4, 1], // 1: Original
    [312, 2, 1, 15, 0, 5, 5, 30, 2, 0], // 2: Original
    [276, 3, 2, 15, 3, 4, 4, 28, 0, 0], // 3: Original
    [240, 4, 0, 14, 2, 0, 1, 29, 2, 0], // 4: Original
    [216, 5, 3, 14, 4, 1, 1, 30, 2, 1], // 5: Original
    [192, 6, 5, 13, 6, 0, 0, 28, 4, 1], // 6: Original
    [168, 7, 7, 15, 7, 3, 3, 28, 4, 1], // 7: Original
    // --- New presets ---
    [192, 8, 8, 13, 8, 0, 0, 28, 4, 1],    // 8:  Blues
    [192, 9, 9, 14, 9, 3, 1, 28, 4, 1],    // 9:  Dark Rock
    [276, 10, 10, 15, 10, 4, 4, 30, 2, 0], // 10: Bossa Nova
    [240, 11, 11, 15, 11, 5, 5, 29, 2, 0], // 11: Fantasy RPG
    [216, 12, 12, 14, 12, 1, 1, 28, 0, 0], // 12: Dungeon
    [276, 13, 14, 15, 0, 5, 5, 30, 2, 0],  // 13: Canon
    [216, 14, 13, 14, 13, 3, 3, 30, 2, 1], // 14: City Pop
    [168, 15, 15, 13, 15, 0, 0, 28, 4, 1], // 15: Boss Battle
];

// 16-step bass patterns:
// '.' = Hold previous note, '0' = Rest/stop, '1'..'4' = Degree selector
const BASS_PATTERNS: [(&str, &str); 16] = [
    ("4.....4.....0.44", "4.....4.....44.."),
    ("2.3.4.3.2.3.4.3.", "2.3.4.3.2...4..."),
    ("440...440...2...", "440...440...442."),
    ("0.2.4.2.0.2.4.2.", "0.2.4.2.2.4.2.4."),
    ("2.402.402.402.40", "2.402.402.40242."),
    ("2034203420342034", "2034203420444440"),
    ("2044204420442044", "2044204420442.4."),
    ("4.444.444.444.44", "4.444.444.442.22"),
    // --- New patterns ---
    ("2...2.4.2...2.4.", "2...2.4.24242.4."), // 8: Shuffle bass
    ("2.2.3.3.4.4.3.3.", "2.2.3.3.4.3.2.4."), // 9: Walking bass
    ("2..42..42..42..4", "2..42..424242424"), // 10: Syncopation
    ("2.4.2.4.2.4.2.4.", "2.4.2.4.24242424"), // 11: Octave bounce
    ("2222222222222222", "2222222244442222"), // 12: Minimal root pulse
    ("20402040204020.0", "204020402.4.2040"), // 13: Slap-style (rest-heavy)
    ("2.3.4.3.2.3.4.3.", "2.3.4.3.4.3.2.4."), // 14: Arpeggio climb
    ("4...4...2...2...", "4...4...2.4.2.4."), // 15: Power bass (low emphasis)
];

// 16-step drum trigger patterns:
// '0' = Rest, other digits are drum keys (mapped in notes_to_mml)
const DRUM_PATTERNS: [(&str, &str); 16] = [
    ("1000000000001000", "1000000000001030"),
    ("1000001000001000", "1000001000007750"),
    ("3030003330300033", "3030003330303013"),
    ("1000001000003330", "1000001000003310"),
    ("1000301010003013", "1000301010003333"),
    ("3033303330333033", "3033303330336655"),
    ("3033203330332033", "3033203330332032"),
    ("1033203310332033", "1033203310332220"),
    // --- New patterns ---
    ("1030003310300033", "1030003310303330"), // 8: Shuffle beat
    ("1030103010301030", "1030103013131313"), // 9: Fast rock
    ("1003001310030013", "1003001310031013"), // 10: Bossa nova
    ("1010301010103010", "1010301013303330"), // 11: March
    ("1000000030000000", "1000000030001030"), // 12: Half-time
    ("1033103310331033", "1033103310337750"), // 13: 16-beat
    ("1003100030031000", "1003100030037750"), // 14: Breakbeat
    ("1000300010301030", "1000300010307750"), // 15: Drum'n'bass
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

// CP8: Blues (I7 - IV7 - I7 - V7)
//   C=0 C#=1 D=2 D#=3 E=4 F=5 F#=6 G=7 G#=8 A=9 A#=10 B=11
//   2=root 1=harmony(3rd/7th) 3=chord(5th) 9=passing 0=unused
const CP8: [ChordEntry; 8] = [
    ce!(0, "209019030909"),   // C:  C(0)=2 E(4)=1 G(7)=3
    ce!(16, "209019030919"),  // C7: C(0)=2 E(4)=1 G(7)=3 A#(10)=1(b7)
    ce!(32, "309092090109"),  // F:  F(5)=2 A(9)=1 C(0)=3
    ce!(48, "309192090109"),  // F7: F(5)=2 A(9)=1 C(0)=3 D#(3)=1(b7)
    ce!(64, "209019030919"),  // C7
    ce!(80, "903099020901"),  // G:  G(7)=2 B(11)=1 D(2)=3
    ce!(96, "309192090109"),  // F7
    ce!(112, "209019030919"), // C7
];
// CP9: Dark Rock (Im - bVII - bVI - bVII)
const CP9: [ChordEntry; 8] = [
    ce!(0, "209100930909"),  // Cm: C(0)=2 D#(3)=1 G(7)=3
    ce!(16, "901093090920"), // Bb: A#(10)=2 D(2)=1 F(5)=3
    ce!(32, "109300909209"), // Ab: G#(8)=2 C(0)=1 D#(3)=3
    ce!(48, "901093090920"), // Bb
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, repeat 2),
    ce!(112, "901093090920"), // Bb
];
// CP10: Bossa Nova (IIm - V - I - VIm)
const CP10: [ChordEntry; 8] = [
    ce!(0, "002091090309"),  // Dm: D(2)=2 F(5)=1 A(9)=3
    ce!(16, "903099020901"), // G:  G(7)=2 B(11)=1 D(2)=3
    ce!(32, "209019030909"), // C:  C(0)=2 E(4)=1 G(7)=3
    ce!(48, "109039090209"), // Am: A(9)=2 C(0)=1 E(4)=3
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "903099020901"),  // G
    ce!(112, "209019030909"), // C
];
// CP11: Fantasy RPG (I - IIIm - IV - IVm)
const CP11: [ChordEntry; 9] = [
    ce!(0, "209019030909"),  // C:  C(0)=2 E(4)=1 G(7)=3
    ce!(16, "009020910003"), // Em: E(4)=2 G(7)=1 B(11)=3
    ce!(32, "309092090109"), // F:  F(5)=2 A(9)=1 C(0)=3
    ce!(48, "309092091009"), // Fm: F(5)=2 G#(8)=1 C(0)=3
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "309092090109"),  // F
    ce!(112, "903099020901"), // G
    ce!(120, "209019030909"), // C
];
// CP12: Dungeon Minor (Im - IVm - Vm - Im)
const CP12: [ChordEntry; 8] = [
    ce!(0, "209100930909"),  // Cm: C(0)=2 D#(3)=1 G(7)=3
    ce!(16, "309092091009"), // Fm: F(5)=2 G#(8)=1 C(0)=3
    ce!(32, "903090020919"), // Gm: G(7)=2 A#(10)=1 D(2)=3
    ce!(48, "209100930909"), // Cm
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "309092091009"),  // Fm
    ce!(112, "903099020901"), // G (V - dominant for tension)
];
// CP13: Canon (I - V - VIm - IIIm - IV - I - IV - V)
const CP13: [ChordEntry; 8] = [
    ce!(0, "209019030909"),   // C
    ce!(16, "903099020901"),  // G
    ce!(32, "109039090209"),  // Am
    ce!(48, "009020910003"),  // Em
    ce!(64, "309092090109"),  // F
    ce!(80, "209019030909"),  // C
    ce!(96, "309092090109"),  // F
    ce!(112, "903099020901"), // G
];
// CP14: City Pop (IVM7 - IIIm7 - VIm7 - IIm - V)
const CP14: [ChordEntry; 10] = [
    ce!(0, "309012090109"),  // FM7: F(5)=2 A(9)=1 C(0)=3 E(4)=1(M7)
    ce!(16, "001020910903"), // Em7: E(4)=2 G(7)=1 B(11)=3 D(2)=1(b7)
    ce!(32, "109039010209"), // Am7: A(9)=2 C(0)=1 E(4)=3 G(7)=1(b7)
    ce!(48, "002091090309"), // Dm:  D(2)=2 F(5)=1 A(9)=3
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "002091090309"),  // Dm
    ce!(104, "903099020901"), // G
    ce!(112, "209019030909"), // C
    ce!(120, "903099020901"), // G
];
// CP15: Boss Battle (Im - bVI - bVII - Im, with V7 tension ending)
const CP15: [ChordEntry; 10] = [
    ce!(0, "209100930909"),  // Cm: C(0)=2 D#(3)=1 G(7)=3
    ce!(16, "109300909209"), // Ab: G#(8)=2 C(0)=1 D#(3)=3
    ce!(32, "901093090920"), // Bb: A#(10)=2 D(2)=1 F(5)=3
    ce!(48, "209100930909"), // Cm
    ce!(64, repeat 0),
    ce!(80, repeat 1),
    ce!(96, "901093090920"),  // Bb
    ce!(104, "209100930909"), // Cm
    ce!(112, "903099020901"), // G (V - dominant tension)
    ce!(120, "209100930909"), // Cm (resolve)
];

const CHORD_PROGRESSIONS: [&[ChordEntry]; PRESET_COUNT] = [
    &CP0, &CP1, &CP2, &CP3, &CP4, &CP5, &CP6, &CP7, &CP8, &CP9, &CP10, &CP11, &CP12, &CP13, &CP14,
    &CP15,
];

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
    match current_oct {
        Some(cur) if *cur == target => {}
        Some(cur) if (target - *cur).abs() == 1 => {
            if target > *cur {
                tokens.push(">".to_string());
            } else {
                tokens.push("<".to_string());
            }
            *current_oct = Some(target);
        }
        None | Some(_) => {
            tokens.push(format!("O{target}"));
            *current_oct = Some(target);
        }
    }
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
    DRUM_KEYS.iter().position(|k| *k == key)
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
    let mut used_keys: Vec<i32> = Vec::new();
    for n in notes.iter().flatten() {
        if *n > 0 && !used_keys.contains(n) {
            used_keys.push(*n);
        }
    }
    used_keys.sort_unstable();
    used_keys
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

fn compress_token_runs(tokens: &[String], group: usize) -> Vec<String> {
    if group <= 1 {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < tokens.len() {
            let mut j = i + 1;
            while j < tokens.len() && tokens[j] == tokens[i] {
                j += 1;
            }
            let count = j - i;
            if count > 1 && tokens[i] != "<" && tokens[i] != ">" {
                let expanded = tokens[i].repeat(count);
                let bracketed = format!("[{}]{}", tokens[i], count);
                out.push(if expanded.len() <= bracketed.len() {
                    expanded
                } else {
                    bracketed
                });
            } else {
                out.push(tokens[i].clone());
            }
            i = j;
        }
        return out;
    }

    let mut out = Vec::new();
    let mut i = 0usize;
    while i < tokens.len() {
        if i + group <= tokens.len() {
            let chunk = &tokens[i..i + group];
            if !chunk.iter().any(|t| t == "<" || t == ">") {
                let mut j = i + group;
                while j + group <= tokens.len() && &tokens[j..j + group] == chunk {
                    j += group;
                }
                let count = (j - i) / group;
                if count > 1 {
                    let chunk_joined = chunk.join("");
                    let expanded = chunk_joined.repeat(count);
                    let bracketed = format!("[{chunk_joined}]{count}");
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
        out.push(tokens[i].clone());
        i += 1;
    }
    out
}

fn compress_chunks(lines: &[String], chunk_size: usize) -> Vec<String> {
    if chunk_size <= 1 {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < lines.len() {
            let mut j = i + 1;
            while j < lines.len() && lines[j] == lines[i] {
                j += 1;
            }
            let count = j - i;
            if count > 1 {
                let expanded = lines[i].repeat(count);
                let bracketed = format!("[{}]{}", lines[i], count);
                out.push(if expanded.len() <= bracketed.len() {
                    expanded
                } else {
                    bracketed
                });
            } else {
                out.push(lines[i].clone());
            }
            i = j;
        }
        return out;
    }

    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        if i + chunk_size <= lines.len() {
            let chunk = &lines[i..i + chunk_size];
            let mut j = i + chunk_size;
            while j + chunk_size <= lines.len() && &lines[j..j + chunk_size] == chunk {
                j += chunk_size;
            }
            let count = (j - i) / chunk_size;
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
        out.push(lines[i].clone());
        i += 1;
    }
    out
}

fn format_tokens(tokens: &[String]) -> String {
    let mut out = String::new();
    let mut last = String::new();
    for tok in tokens {
        let is_cmd = tok.starts_with('@') || tok.starts_with('O');
        if is_cmd && !out.is_empty() && !out.ends_with(' ') {
            out.push(' ');
        }
        if (tok == "<" || tok == ">")
            && !last.is_empty()
            && (last.starts_with('@') || last.starts_with('O'))
            && !out.ends_with(' ')
        {
            out.push(' ');
        }
        out.push_str(tok);
        last.clone_from(tok);
    }
    out.trim().to_string()
}

fn random_seed() -> u64 {
    rand::rng().random()
}

fn parse_notes_bits(s: &str) -> [i32; 12] {
    let mut out = [0; 12];
    for (i, ch) in s.bytes().take(12).enumerate() {
        out[i] = i32::from(ch.saturating_sub(b'0'));
    }
    out
}

fn root_from_bits(bits: &[i32; 12]) -> i32 {
    for (i, v) in bits.iter().enumerate() {
        if *v == 2 {
            return i as i32;
        }
    }
    0
}

fn resolve_entry_notes(progressions: &[ChordEntry], idx: usize) -> Option<&'static str> {
    if let Some(notes) = progressions[idx].notes {
        return Some(notes);
    }
    progressions[idx]
        .repeat
        .and_then(|r| progressions.get(r))
        .and_then(|e| e.notes)
}

fn chord_bits_per_step(preset: usize) -> Vec<[i32; 12]> {
    let chord_idx = PRESET_SETS[preset][PRESET_CHORD] as usize;
    let progression = CHORD_PROGRESSIONS[chord_idx];
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
    let bytes = line.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i - 1] == b'0' && bytes[i] == b'0' {
            return true;
        }
    }
    false
}

fn build_chord_note_pool(bits: &[i32; 12], key_shift: i32, lowest: i32) -> Vec<(i32, i32)> {
    let mut note_highest = None;
    let mut idx = 0i32;
    let mut results = Vec::new();
    loop {
        let note_type = bits[idx.rem_euclid(12) as usize];
        let note = 12 + idx + key_shift;
        if note >= lowest && (note_type == 1 || note_type == 2 || note_type == 3 || note_type == 9)
        {
            results.push((note, note_type));
            if note_highest.is_none() {
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
// Match original behavior: retry until a valid melody is produced.

fn default_melody_state() -> MelodyState {
    MelodyState {
        cur_chord_idx: -1,
        cur_chord_loc: 0,
        is_repeat: false,
        chord_idx: 0,
        prev_note: -1,
        first_in_chord: true,
    }
}

fn build_melody_chord_plan(preset: usize, key_shift: i32, lowest: i32) -> Vec<MelodyChord> {
    let chord_idx = PRESET_SETS[preset][PRESET_CHORD] as usize;
    let progression = CHORD_PROGRESSIONS[chord_idx];
    let mut out: Vec<MelodyChord> = Vec::with_capacity(progression.len());
    for p in progression {
        let mut base = 0;
        if let Some(repeat_idx) = p.repeat {
            base = out[repeat_idx].base;
        }
        let mut notes = Vec::new();
        let mut notes_bits = [0; 12];
        let mut no_root = false;
        if let Some(note_str) = p.notes {
            let notes_origin = parse_notes_bits(note_str);
            notes_bits = notes_origin;
            let mut note_chord_count = 0;
            for (i, v) in notes_origin.iter().enumerate() {
                if *v == 2 {
                    base = i as i32;
                }
                if *v == 1 || *v == 2 || *v == 3 {
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
    let mut idx = 0;
    for rev_idx in 0..plan.len() {
        let i = plan.len() - rev_idx - 1;
        if loc >= plan[i].loc {
            idx = i;
            break;
        }
        next_chord_loc = plan[i].loc;
    }
    (idx, next_chord_loc)
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
    chord_notes.iter().position(|(n, _)| *n == note)
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
    preset: usize,
    key_shift: i32,
    base: &[Option<i32>],
    rng: &mut Xoshiro256StarStar,
    require_tones: bool,
) -> (Vec<Option<i32>>, Vec<Option<i32>>) {
    let preset_def = PRESET_SETS[preset];
    let density = preset_def[PRESET_MELO_DENSITY].clamp(0, 4) as usize;
    let use_16th = preset_def[PRESET_MELO_USE16] != 0;
    let lowest = preset_def[PRESET_MELO_LOWEST_NOTE];
    let chord_plan = build_melody_chord_plan(preset, key_shift, lowest);
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

        let mut state = default_melody_state();

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

fn generate_bass(preset: usize, bits_per_step: &[[i32; 12]], key_shift: i32) -> Vec<Option<i32>> {
    let mut notes = vec![Some(-1); TOTAL_STEPS];
    let bass_idx = PRESET_SETS[preset][PRESET_BASE] as usize;
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
                if matches!(bits[((n + key_shift).rem_euclid(12)) as usize], 1 | 2 | 3) {
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
        if note_type == 1 || note_type == 2 || note_type == 3 || note_type == 9 {
            let note = 12 + idx + key_shift;
            if note > master - 3 && has_important_tone {
                break;
            }
            if note >= base_min {
                results.push((note, note_type));
                if note_type == 1 || note_type == 3 {
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
            if *note == cur && (*note_type == 1 || *note_type == 2 || *note_type == 3) {
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
    preset: usize,
    melody: &[Option<i32>],
    sub_seed: &[Option<i32>],
    base: &[Option<i32>],
    key_shift: i32,
    lowest: i32,
    rng: &mut Xoshiro256StarStar,
) -> Vec<Option<i32>> {
    let chord_plan = build_melody_chord_plan(preset, key_shift, lowest);
    let rhythm_sub = pick_rhythm_events(rng, true, true);
    let mut state = default_melody_state();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_transp_keeps_valid_range_values() {
        for transp in -5..=6 {
            assert_eq!(normalize_transp(transp), transp);
        }
        assert_eq!(normalize_transp(-999), -5);
        assert_eq!(normalize_transp(999), 6);
    }

    #[test]
    fn bass_notes_use_chord_tones_only() {
        for preset in 0..PRESET_COUNT {
            let bits_per_step = chord_bits_per_step(preset);
            for transp in -5..=6 {
                let bass = generate_bass(preset, &bits_per_step, transp);
                for (loc, note) in bass.iter().enumerate() {
                    let Some(note) = note else {
                        continue;
                    };
                    if *note < 0 {
                        continue;
                    }
                    let tone = bits_per_step[loc][((*note + transp).rem_euclid(12)) as usize];
                    assert!(
                        matches!(tone, 1 | 2 | 3),
                        "preset={preset} transp={transp} loc={loc} note={note} tone={tone}"
                    );
                }
            }
        }
    }

    #[test]
    fn seeded_generation_is_reproducible() {
        let cases = [
            (0, -5, 0, 1u64),
            (1, -2, 1, 2u64),
            (2, 0, 2, 3u64),
            (3, 4, 3, 4u64),
            (7, 6, 3, 123_456_789u64),
        ];
        for (preset, transp, instr, seed) in cases {
            let a = std::panic::catch_unwind(|| generate_bgm_mml(preset, instr, transp, Some(seed)))
                .unwrap_or_else(|_| {
                    panic!(
                        "seeded generation panicked (first run) preset={preset} transp={transp} instr={instr} seed={seed}"
                    )
                });
            let b = std::panic::catch_unwind(|| generate_bgm_mml(preset, instr, transp, Some(seed)))
                .unwrap_or_else(|_| {
                    panic!(
                        "seeded generation panicked (second run) preset={preset} transp={transp} instr={instr} seed={seed}"
                    )
                });
            assert_eq!(
                a, b,
                "seeded gen_bgm mismatch preset={preset} transp={transp} instr={instr} seed={seed}"
            );
        }
    }
}

fn shifted_melody(melody: &[Option<i32>]) -> Vec<Option<i32>> {
    let mut notes = vec![Some(-1); TOTAL_STEPS];
    for (i, out) in notes.iter_mut().enumerate().take(TOTAL_STEPS) {
        let prev = (i + TOTAL_STEPS - 1) % TOTAL_STEPS;
        *out = melody[prev];
    }
    notes
}

fn generate_drums(preset: usize) -> Vec<Option<i32>> {
    let mut notes = vec![None; TOTAL_STEPS];
    let drum_idx = PRESET_SETS[preset][PRESET_DRUMS] as usize;
    let (basic, final_pat) = DRUM_PATTERNS[drum_idx];
    for i in 0..TOTAL_STEPS {
        let bar = i / STEPS_PER_BAR;
        let pat = if bar % 4 < 3 {
            basic.as_bytes()
        } else {
            final_pat.as_bytes()
        };
        let step_symbol = pat[i % STEPS_PER_BAR];
        if step_symbol == b'0' {
            notes[i] = None;
        } else {
            notes[i] = Some(i32::from(step_symbol - b'0'));
        }
    }
    notes
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
        let mut compressed = compress_token_runs(bar, 4);
        if compressed == *bar {
            compressed = compress_token_runs(bar, 2);
        }
        if compressed == *bar {
            compressed = compress_token_runs(bar, 1);
        }
        bar_strings.push(format_tokens(&compressed));
    }

    let mut compressed = compress_chunks(&bar_strings, 2);
    if compressed == bar_strings {
        compressed = compress_chunks(&bar_strings, 1);
    }

    tokens.extend(compressed);
    tokens.join(" ")
}

fn silent_channel_mml(tempo: i32) -> String {
    format!("T{tempo} L16 @ENV1{{127}} Q100 V112 @0 @ENV1 @VIB0")
}

fn generate_bgm_mml(preset: i32, instr: i32, transp: i32, seed: Option<u64>) -> Vec<String> {
    let preset = preset.clamp(0, (PRESET_COUNT - 1) as i32) as usize;
    let instr = instr.clamp(0, 3) as usize;
    let transp = normalize_transp(transp);
    let key_shift = transp;

    let actual_seed = seed.unwrap_or_else(random_seed);
    let mut rng = Xoshiro256StarStar::seed_from_u64(actual_seed);
    let base_speed = PRESET_SETS[preset][PRESET_SPEED].max(1);
    let tempo = (28800 / base_speed).max(1);
    let bits_per_step = chord_bits_per_step(preset);
    let preset_def = PRESET_SETS[preset];
    let bass = generate_bass(preset, &bits_per_step, key_shift);
    let mut melody_and_seed = generate_melody(preset, key_shift, &bass, &mut rng, instr < 2);
    let mut submelody = None;

    if instr >= 2 {
        let chord_plan =
            build_melody_chord_plan(preset, key_shift, preset_def[PRESET_MELO_LOWEST_NOTE]);
        let mut candidate = generate_submelody(
            preset,
            &melody_and_seed.0,
            &melody_and_seed.1,
            &bass,
            key_shift,
            preset_def[PRESET_MELO_LOWEST_NOTE],
            &mut rng,
        );
        loop {
            if melody_has_required_tones(&melody_and_seed.0, Some(&candidate), &chord_plan) {
                break;
            }
            melody_and_seed = generate_melody(preset, key_shift, &bass, &mut rng, false);
            candidate = generate_submelody(
                preset,
                &melody_and_seed.0,
                &melody_and_seed.1,
                &bass,
                key_shift,
                preset_def[PRESET_MELO_LOWEST_NOTE],
                &mut rng,
            );
        }
        submelody = Some(candidate);
    }

    let (melody, _) = melody_and_seed;
    let melo_tone_idx = TONE_CANDIDATES[preset_def[PRESET_MELO_TONE] as usize];
    let sub_tone_idx = TONE_CANDIDATES[preset_def[PRESET_SUB_TONE] as usize];
    let base_quantize = ((preset_def[PRESET_BASE_QUANTIZE].clamp(0, 16) * 100) + 8) / 16;

    let mut mml_list = vec![
        notes_to_mml(&melody, tempo, melo_tone_idx, 96, 88, false),
        notes_to_mml(&bass, tempo, 7, 112, base_quantize, false),
        silent_channel_mml(tempo),
        silent_channel_mml(tempo),
    ];

    if instr == 0 {
        // No submelody, no drum: ch2 is shifted melody with melody tone settings
        let shifted = shifted_melody(&melody);
        mml_list[2] = notes_to_mml(&shifted, tempo, melo_tone_idx, 32, 88, false);
    } else {
        if instr == 1 || instr == 3 {
            let drum = generate_drums(preset);
            if instr == 1 {
                // Drum only: ch2 is drum track, ch3 silent
                mml_list[2] = notes_to_mml(&drum, tempo, 15, 80, 94, true);
            } else {
                // Submelody + drum
                mml_list[3] = notes_to_mml(&drum, tempo, 15, 80, 94, true);
            }
        }

        if instr == 2 || instr == 3 {
            // Submelody only: ch2 is submelody, ch3 silent
            let sub = submelody.unwrap_or_else(|| vec![Some(-1); TOTAL_STEPS]);
            mml_list[2] = notes_to_mml(&sub, tempo, sub_tone_idx, 64, 94, false);
        }
    }

    mml_list
}

impl Pyxel {
    pub fn gen_bgm(
        &mut self,
        preset: i32,
        transp: i32,
        instr: i32,
        seed: Option<u64>,
        play: Option<bool>,
    ) -> Vec<String> {
        let mml_list = generate_bgm_mml(preset, instr, transp, seed);

        if play.unwrap_or(false) {
            for (ch, mml) in mml_list.iter().enumerate() {
                let sound = Sound::new();
                if sound.lock().mml(mml).is_ok() {
                    self.channels.lock()[ch]
                        .lock()
                        .play1(sound, None, true, false);
                }
            }
        }

        mml_list
    }
}

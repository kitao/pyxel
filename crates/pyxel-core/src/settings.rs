use crate::key::KEY_ESCAPE;
use crate::types::{Color, Effect, Key, Note, Rgb8, Speed, Tone, Volume};

// System
pub const PYXEL_VERSION: &str = "1.8.2";
pub const PYXEL_WORKING_DIR: &str = ".pyxel";
pub const DEFAULT_TITLE: &str = "Pyxel";
pub const DEFAULT_FPS: u32 = 30;
pub const DEFAULT_QUIT_KEY: Key = KEY_ESCAPE;
pub const DEFAULT_CAPTURE_SCALE: u32 = 2;
pub const DEFAULT_CAPTURE_SEC: u32 = 10;
pub const DISPLAY_RATIO: f64 = 0.75;
pub const BACKGROUND_COLOR: Rgb8 = 0x101018;
pub const MAX_SKIP_FRAMES: u32 = 9;
pub const NUM_MEASURE_FRAMES: u32 = 10;
pub const ICON_SIZE: u32 = 16;
pub const ICON_SCALE: u32 = 4;
pub const ICON_DATA: [&str; ICON_SIZE as usize] = [
    "0000000110000000",
    "0000011F71100000",
    "00011FF11FF11000",
    "011FF111111FF110",
    "17E1111111111C71",
    "1E1EE111111CC1C1",
    "1E111EE11CC111C1",
    "1E11111E711111C1",
    "1E111111C11111C1",
    "1E111111C11111C1",
    "1E111111C11111C1",
    "17E11111C1111C71",
    "011EE111C11CC110",
    "00011EE1CCC11000",
    "0000011E71100000",
    "0000000110000000",
];

// Resource
pub const APP_FILE_EXTENSION: &str = ".pyxapp";
pub const APP_STARTUP_SCRIPT_FILE: &str = ".pyxapp_startup_script";
pub const RESOURCE_FILE_EXTENSION: &str = ".pyxres";
pub const RESOURCE_ARCHIVE_DIRNAME: &str = "pyxel_resource/";

// Graphics
pub const NUM_COLORS: u32 = 16;
pub const NUM_IMAGES: u32 = 3;
pub const IMAGE_SIZE: u32 = 256;
pub const NUM_TILEMAPS: u32 = 8;
pub const TILEMAP_SIZE: u32 = 256;
pub const TILE_SIZE: u32 = 8;
pub const DEFAULT_COLORS: [Rgb8; NUM_COLORS as usize] = [
    0x000000, 0x2b335f, 0x7e2072, 0x19959c, 0x8b4852, 0x395c98, 0xa9c1ff, 0xeeeeee, 0xd4186c,
    0xd38441, 0xe9c35b, 0x70c6a9, 0x7696de, 0xa3a3a3, 0xFF9798, 0xedc7b0,
];
pub const COLOR_BLACK: Color = 0;
pub const COLOR_NAVY: Color = 1;
pub const COLOR_PURPLE: Color = 2;
pub const COLOR_GREEN: Color = 3;
pub const COLOR_BROWN: Color = 4;
pub const COLOR_DARK_BLUE: Color = 5;
pub const COLOR_LIGHT_BLUE: Color = 6;
pub const COLOR_WHITE: Color = 7;
pub const COLOR_RED: Color = 8;
pub const COLOR_ORANGE: Color = 9;
pub const COLOR_YELLOW: Color = 10;
pub const COLOR_LIME: Color = 11;
pub const COLOR_CYAN: Color = 12;
pub const COLOR_GRAY: Color = 13;
pub const COLOR_PINK: Color = 14;
pub const COLOR_PEACH: Color = 15;
pub const CURSOR_WIDTH: u32 = 8;
pub const CURSOR_HEIGHT: u32 = 8;
pub const CURSOR_DATA: [&str; CURSOR_HEIGHT as usize] = [
    "11111100", "17776100", "17761000", "17676100", "16167610", "11016761", "00001610", "00000100",
];
pub const MIN_FONT_CODE: char = 32 as char;
pub const MAX_FONT_CODE: char = 127 as char;
pub const NUM_FONT_ROWS: u32 = 16;
pub const FONT_WIDTH: u32 = 4;
pub const FONT_HEIGHT: u32 = 6;
pub const FONT_DATA: [u32; MAX_FONT_CODE as usize - MIN_FONT_CODE as usize + 1] = [
    0x000000, 0x444040, 0xAA0000, 0xAEAEA0, 0x6C6C40, 0x824820, 0x4A4AC0, 0x440000, 0x244420,
    0x844480, 0xA4E4A0, 0x04E400, 0x000480, 0x00E000, 0x000040, 0x224880, 0x6AAAC0, 0x4C4440,
    0xC248E0, 0xC242C0, 0xAAE220, 0xE8C2C0, 0x68EAE0, 0xE24880, 0xEAEAE0, 0xEAE2C0, 0x040400,
    0x040480, 0x248420, 0x0E0E00, 0x842480, 0xE24040, 0x4AA860, 0x4AEAA0, 0xCACAC0, 0x688860,
    0xCAAAC0, 0xE8E8E0, 0xE8E880, 0x68EA60, 0xAAEAA0, 0xE444E0, 0x222A40, 0xAACAA0, 0x8888E0,
    0xAEEAA0, 0xCAAAA0, 0x4AAA40, 0xCAC880, 0x4AAE60, 0xCAECA0, 0x6842C0, 0xE44440, 0xAAAA60,
    0xAAAA40, 0xAAEEA0, 0xAA4AA0, 0xAA4440, 0xE248E0, 0x644460, 0x884220, 0xC444C0, 0x4A0000,
    0x0000E0, 0x840000, 0x06AA60, 0x8CAAC0, 0x068860, 0x26AA60, 0x06AC60, 0x24E440, 0x06AE24,
    0x8CAAA0, 0x404440, 0x2022A4, 0x8ACCA0, 0xC444E0, 0x0EEEA0, 0x0CAAA0, 0x04AA40, 0x0CAAC8,
    0x06AA62, 0x068880, 0x06C6C0, 0x4E4460, 0x0AAA60, 0x0AAA40, 0x0AAEE0, 0x0A44A0, 0x0AA624,
    0x0E24E0, 0x64C460, 0x444440, 0xC464C0, 0x6C0000, 0xEEEEE0,
];

// Audio
pub const CLOCK_RATE: u32 = 1789773; // 1.78 MHz clock rate
pub const SAMPLE_RATE: u32 = 44100; // 44.1 kHz sample rate
pub const NUM_SAMPLES: u32 = SAMPLE_RATE / 30;
pub const NUM_CLOCKS_PER_TICK: u32 = CLOCK_RATE / 120;
pub const OSCILLATOR_RESOLUTION: u32 = 32;
pub const VIBRATO_DEPTH: f64 = 0.015;
pub const VIBRATO_FREQUENCY: f64 = 6.0;
pub const NUM_CHANNELS: u32 = 4;
pub const NUM_SOUNDS: u32 = 64;
pub const NUM_MUSICS: u32 = 8;
pub const TRIANGLE_VOLUME_FACTOR: f64 = 1.0;
pub const SQUARE_VOLUME_FACTOR: f64 = 0.3;
pub const PULSE_VOLUME_FACTOR: f64 = 0.3;
pub const NOISE_VOLUME_FACTOR: f64 = 0.6;
pub const TONE_TRIANGLE: Tone = 0;
pub const TONE_SQUARE: Tone = 1;
pub const TONE_PULSE: Tone = 2;
pub const TONE_NOISE: Tone = 3;
pub const EFFECT_NONE: Effect = 0;
pub const EFFECT_SLIDE: Effect = 1;
pub const EFFECT_VIBRATO: Effect = 2;
pub const EFFECT_FADEOUT: Effect = 3;
pub const MAX_TONE: Tone = 3;
pub const MAX_NOTE: Note = 59; // 5 octaves
pub const MAX_VOLUME: Volume = 7;
pub const MAX_EFFECT: Effect = 3;
pub const INITIAL_SPEED: Speed = 30;

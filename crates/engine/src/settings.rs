use crate::palette::{Color, Rgb24};

//
// Common
//
pub const PYXEL_VERSION: &str = "2.0.0";
//const std::string WHITESPACE = " \t\v\r\n";

pub const DEFAULT_TITLE: &str = "Pyxel";
pub const DEFAULT_FPS: u32 = 30;

/*
const int32_t DEFAULT_SCALE = 0;
const int32_t DEFAULT_QUIT_KEY = KEY_ESCAPE;
*/

//
// System
//
pub const BACKGROUND_COLOR: Rgb24 = 0x101018;
pub const MAX_FRAME_SKIP_COUNT: u32 = 9;
pub const MEASURE_FRAME_COUNT: u32 = 10;

pub const ICON_SIZE: u32 = 16;
pub const ICON_SCALE: u32 = 4;

/*
const std::vector<std::string> ICON_DATA = {
    "0000000110000000", "0000011F71100000", "00011FF11FF11000",
    "011FF111111FF110", "17E1111111111C71", "1E1EE111111CC1C1",
    "1E111EE11CC111C1", "1E11111E711111C1", "1E111111C11111C1",
    "1E111111C11111C1", "1E111111C11111C1", "17E11111C1111C71",
    "011EE111C11CC110", "00011EE1CCC11000", "0000011E71100000",
    "0000000110000000",


const int32_t SCREEN_CAPTURE_COUNT = 900;
const int32_t SCREEN_CAPTURE_SCALE = 2;

const float MAX_WINDOW_SIZE_RATIO = 0.8f;
*/

//
// Resource
//
/*
const std::string RESOURCE_FILE_EXTENSION = ".pyxres";
const std::string RESOURCE_ARCHIVE_DIRNAME = "pyxel_resource/";
*/

//
// Graphics
//
pub const IMAGEBANK_COUNT: usize = 4;
pub const IMAGEBANK_SIZE: u32 = 256;

pub const TILEMAP_COUNT: usize = 8;
pub const TILEMAP_SIZE: u32 = 256;

pub const DISPLAY_COLORS: [Rgb24; 16] = [
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

/*
const int32_t TILEMAP_CHIP_WIDTH = 8;
const int32_t TILEMAP_CHIP_HEIGHT = 8;
const int32_t TILEMAP_CHIP_COUNT = (TILEMAP_BANK_WIDTH / TILEMAP_CHIP_WIDTH) *
                                   (TILEMAP_BANK_HEIGHT / TILEMAP_CHIP_HEIGHT);

const int32_t MOUSE_CURSOR_X = 2;
const int32_t MOUSE_CURSOR_Y = 2;
const int32_t MOUSE_CURSOR_WIDTH = 8;
const int32_t MOUSE_CURSOR_HEIGHT = 8;
const std::vector<std::string> MOUSE_CURSOR_DATA = {
    "00000011", "07776011", "07760111", "07676011",
    "06067601", "00106760", "11110601", "11111011",
};

const int32_t MIN_FONT_CODE = 32;
const int32_t MAX_FONT_CODE = 127;
const int32_t FONT_X = 12;
const int32_t FONT_Y = 0;
const int32_t FONT_WIDTH = 4;
const int32_t FONT_HEIGHT = 6;
const int32_t FONT_ROW_COUNT = 48;
const int32_t FONT_COLOR = 7;
const std::vector<uint32_t> FONT_DATA = {
    0x000000, 0x444040, 0xAA0000, 0xAEAEA0, 0x6C6C40, 0x824820, 0x4A4AC0,
    0x440000, 0x244420, 0x844480, 0xA4E4A0, 0x04E400, 0x000480, 0x00E000,
    0x000040, 0x224880, 0x6AAAC0, 0x4C4440, 0xC248E0, 0xC242C0, 0xAAE220,
    0xE8C2C0, 0x68EAE0, 0xE24880, 0xEAEAE0, 0xEAE2C0, 0x040400, 0x040480,
    0x248420, 0x0E0E00, 0x842480, 0xE24040, 0x4AA860, 0x4AEAA0, 0xCACAC0,
    0x688860, 0xCAAAC0, 0xE8E8E0, 0xE8E880, 0x68EA60, 0xAAEAA0, 0xE444E0,
    0x222A40, 0xAACAA0, 0x8888E0, 0xAEEAA0, 0xCAAAA0, 0x4AAA40, 0xCAC880,
    0x4AAE60, 0xCAECA0, 0x6842C0, 0xE44440, 0xAAAA60, 0xAAAA40, 0xAAEEA0,
    0xAA4AA0, 0xAA4440, 0xE248E0, 0x644460, 0x884220, 0xC444C0, 0x4A0000,
    0x0000E0, 0x840000, 0x06AA60, 0x8CAAC0, 0x068860, 0x26AA60, 0x06AC60,
    0x24E440, 0x06AE24, 0x8CAAA0, 0x404440, 0x2022A4, 0x8ACCA0, 0xC444E0,
    0x0EEEA0, 0x0CAAA0, 0x04AA40, 0x0CAAC8, 0x06AA62, 0x068880, 0x06C6C0,
    0x4E4460, 0x0AAA60, 0x0AAA40, 0x0AAEE0, 0x0A44A0, 0x0AA624, 0x0E24E0,
    0x64C460, 0x444440, 0xC464C0, 0x6C0000, 0xEEEEE0,
};
*/

//
// Audio
//
/*
const int32_t AUDIO_SAMPLE_RATE = 22050;
const int32_t AUDIO_BLOCK_SIZE = 2048;
const int32_t AUDIO_ONE_SPEED = AUDIO_SAMPLE_RATE / 120;
const int32_t AUDIO_ONE_VOLUME = 0x7FFF / (4  7);

const int32_t USER_SOUND_BANK_COUNT = 64;
const int32_t TOTAL_SOUND_BANK_COUNT = USER_SOUND_BANK_COUNT + 1;
const int32_t SOUND_BANK_FOR_SYSTEM = USER_SOUND_BANK_COUNT;

const int32_t MUSIC_BANK_COUNT = 8;
*/

//
// Sound class
//
/*
enum {
  TONE_TRIANGLE,
  TONE_SQUARE,
  TONE_PULSE,
  TONE_NOISE,
};

enum {
  EFFECT_NONE,
  EFFECT_SLIDE,
  EFFECT_VIBRATO,
  EFFECT_FADEOUT,
};

const int32_t INITIAL_SOUND_SPEED = 30;
*/

//
// Music class
//
/*
const int32_t MUSIC_CHANNEL_COUNT = 4;
*/

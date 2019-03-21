#ifndef PYXELCORE_CONSTANTS_H_
#define PYXELCORE_CONSTANTS_H_

#include <cstdint>
#include <map>
#include <string>

namespace pyxelcore {

const std::string VERSION = "1.1.0";
const std::string DEFAULT_CAPTION = "Pyxel";

const int32_t DEFAULT_SCALE = 0;
const int32_t DEFAULT_PALETTE_00 = 0x000000;
const int32_t DEFAULT_PALETTE_01 = 0x1D2B53;
const int32_t DEFAULT_PALETTE_02 = 0x7E2553;
const int32_t DEFAULT_PALETTE_03 = 0x008751;
const int32_t DEFAULT_PALETTE_04 = 0xAB5236;
const int32_t DEFAULT_PALETTE_05 = 0x5F574F;
const int32_t DEFAULT_PALETTE_06 = 0xC2C3C7;
const int32_t DEFAULT_PALETTE_07 = 0xFFF1E8;
const int32_t DEFAULT_PALETTE_08 = 0xFF004D;
const int32_t DEFAULT_PALETTE_09 = 0xFFA300;
const int32_t DEFAULT_PALETTE_10 = 0xFFEC27;
const int32_t DEFAULT_PALETTE_11 = 0x00E436;
const int32_t DEFAULT_PALETTE_12 = 0x29ADFF;
const int32_t DEFAULT_PALETTE_13 = 0x83769C;
const int32_t DEFAULT_PALETTE_14 = 0xFF77A8;
const int32_t DEFAULT_PALETTE_15 = 0xFFCCAA;
const int32_t DEFAULT_FPS = 30;
const int32_t DEFAULT_BORDER_WIDTH = 0;
const int32_t DEFAULT_BORDER_COLOR = 0x101018;

const int32_t COLOR_COUNT = 16;

const std::string ICON_DATA[] = {
    "0000000110000000", "0000011F71100000", "00011FF11FF11000",
    "011FF111111FF110", "1AE1111111111C71", "1E1EE111111CC1C1",
    "1E111EE11CC111C1", "1E11111E711111C1", "1E111111C11111C1",
    "1E111111C11111C1", "1E111111C11111C1", "1AE11111C1111C71",
    "011EE111C11CC110", "00011EE1CCC11000", "0000011E71100000",
    "0000000110000000",
};

const int32_t MOUSE_CURSOR_IMAGE_X = 0;
const int32_t MOUSE_CURSOR_IMAGE_Y = 16;
const int32_t MOUSE_CURSOR_WIDTH = 8;
const int32_t MOUSE_CURSOR_HEIGHT = 8;

const std::string MOUSE_CURSOR_DATA[] = {
    "00000011", "07776011", "07760111", "07676011",
    "06067601", "00106760", "11110601", "11111011",
};

const int32_t APP_SCREEN_MAX_SIZE = 255;
const int32_t APP_SCREEN_SCALE_CUTDOWN = 2;
const int32_t APP_SCREEN_SCALE_MINIMUM = 2;
const int32_t APP_GIF_CAPTURE_COUNT = 900;
const int32_t APP_GIF_CAPTURE_SCALE = 2;
const int32_t APP_MEASURE_FRAME_COUNT = 10;

const int32_t RENDERER_IMAGE_COUNT = 4;
const int32_t RENDERER_IMAGE_WIDTH = 256;
const int32_t RENDERER_IMAGE_HEIGHT = 256;
const int32_t RENDERER_TILEMAP_COUNT = 8;
const int32_t RENDERER_TILEMAP_WIDTH = 256;
const int32_t RENDERER_TILEMAP_HEIGHT = 256;
const int32_t RENDERER_MIN_TEXTURE_SIZE = 256;

const int32_t FONT_MIN_CODE = 32;
const int32_t FONT_MAX_CODE = 127;
const int32_t FONT_WIDTH = 4;
const int32_t FONT_HEIGHT = 6;
const int32_t FONT_ROW_COUNT = RENDERER_IMAGE_WIDTH / FONT_WIDTH;

const int32_t AUDIO_SAMPLE_RATE = 22050;
const int32_t AUDIO_BLOCK_SIZE = 2205;
const int32_t AUDIO_CHANNEL_COUNT = 4;
const int32_t AUDIO_SOUND_COUNT = 65;
const int32_t AUDIO_MUSIC_COUNT = 8;
const int32_t AUDIO_ONE_SPEED = AUDIO_SAMPLE_RATE / 120;
const int32_t AUDIO_ONE_VOLUME = 0x7FFF / (AUDIO_CHANNEL_COUNT * 7);

const int32_t SOUND_TONE_TRIANGLE = 0;
const int32_t SOUND_TONE_SQUARE = 1;
const int32_t SOUND_TONE_PULSE = 2;
const int32_t SOUND_TONE_NOISE = 3;
const int32_t SOUND_EFFECT_NONE = 0;
const int32_t SOUND_EFFECT_SLIDE = 1;
const int32_t SOUND_EFFECT_VIBRATO = 2;
const int32_t SOUND_EFFECT_FADEOUT = 3;

const int32_t SOUND_NOTE_C = 0;
const int32_t SOUND_NOTE_D = 2;
const int32_t SOUND_NOTE_E = 4;
const int32_t SOUND_NOTE_F = 5;
const int32_t SOUND_NOTE_G = 7;
const int32_t SOUND_NOTE_A = 9;
const int32_t SOUND_NOTE_B = 11;

class Constants {
 public:
  Constants();
  ~Constants();

  int32_t get_constant_number(const char* name);
  const char* get_constant_string(const char* name);

 private:
  std::map<std::string, int32_t> constant_number_map_;
  std::map<std::string, std::string> constant_string_map_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_

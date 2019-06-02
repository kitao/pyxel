#ifndef PYXELCORE_COMMON_H_
#define PYXELCORE_COMMON_H_

#include "pyxelcore/constants.h"

namespace pyxelcore {

class Sound;

typedef std::array<int32_t, COLOR_COUNT> PaletteColor;
typedef std::vector<std::string> ImageString;
typedef std::vector<std::string> TilemapString;
typedef std::vector<int32_t> SoundIndexList;
typedef std::vector<Sound*> SoundList;
typedef std::array<int32_t, MAX_SOUND_LENGTH> SoundData;
typedef std::array<Sound*, MAX_MUSIC_LENGTH> ChannelData;

template <typename T>
T Min(T a, T b) {
  return a < b ? a : b;
}

template <typename T>
T Min(T a, T b, T c) {
  return a < b ? (a < c ? a : c) : (b < c ? b : c);
}

template <typename T>
T Max(T a, T b) {
  return a > b ? a : b;
}

template <typename T>
T Max(T a, T b, T c) {
  return a > b ? (a > c ? a : c) : (b > c ? b : c);
}

template <typename T>
T Abs(T v) {
  return v < 0 ? -v : v;
}

template <typename T>
T Clamp(T v, T low, T high) {
  return v < low ? low : (v > high ? high : v);
}

inline void PrintError(const std::string& message,
                       const std::string& func_name) {
  std::cout << "pyxel error: " + message + " in '" + func_name + "'"
            << std::endl;
}

#define PRINT_ERROR(message) PrintError(message, __FUNCTION__)

}  // namespace pyxelcore

#endif  // PYXELCORE_COMMON_H_

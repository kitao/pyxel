#ifndef PYXELCORE_COMMON_H_
#define PYXELCORE_COMMON_H_

#include "pyxelcore/constants.h"

namespace pyxelcore {

class Sound;

typedef std::array<int32_t, COLOR_COUNT> PaletteColor;
typedef std::array<int32_t, COLOR_COUNT> PaletteTable;
typedef std::vector<std::string> ImageString;
typedef std::vector<std::string> TilemapString;
typedef std::vector<int32_t> SoundData;
typedef std::vector<int32_t> SoundIndexList;
typedef std::vector<Sound*> SoundList;

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

inline std::vector<std::string> Split(std::string str, char del) {
  std::istringstream iss(str);
  std::vector<std::string> res;

  for (std::string temp; std::getline(iss, temp, del); res.push_back(temp))
    ;

  return res;
}

inline std::string Trim(const std::string& str) {
  std::string res;
  std::string::size_type left = str.find_first_not_of(WHITESPACE);

  if (left != std::string::npos) {
    std::string::size_type right = str.find_last_not_of(WHITESPACE);
    res = str.substr(left, right - left + 1);
  }

  return res;
}

inline std::string GetTrimmedLine(std::istream& is) {
  std::string line;
  std::getline(is, line);
  return Trim(line);
}

inline std::string ReplaceAll(const std::string& str,
                              const std::string& from,
                              const std::string& to) {
  std::string res = str;
  std::string::size_type pos = res.find(from);

  while ((pos = res.find(from, pos)) != std::string::npos) {
    res.replace(pos, from.length(), to);
    pos += to.length();
  }

  return res;
}

inline void PyxelError(const std::string& message,
                       const std::string& func_name) {
  std::cout << "pyxel error: " + message + " in '" + func_name + "'"
            << std::endl;
  exit(1);
}

#define PYXEL_ERROR(message) pyxelcore::PyxelError(message, __FUNCTION__)

}  // namespace pyxelcore

#endif  // PYXELCORE_COMMON_H_

#ifndef PYXELCORE_COMMON_H_
#define PYXELCORE_COMMON_H_

#include "pyxelcore/constants.h"

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <cstdio>

#define PRINT_ERROR(message) PrintError(message, __FUNCTION__)

namespace pyxelcore {

template <typename T>
T Min(T a, T b) {
  return a < b ? a : b;
}

template <typename T>
T Max(T a, T b) {
  return a > b ? a : b;
}

template <typename T>
T Abs(T v) {
  return v < 0 ? -v : v;
}

template <typename T>
T Clamp(T v, T low, T high) {
  return v < low ? low : (v > high ? high : v);
}

inline void PrintError(const char* message, const char* func_name) {
  printf("pyxel: %s in %s\n", message, func_name);
}

}  // namespace pyxelcore

#endif  // PYXELCORE_COMMON_H_

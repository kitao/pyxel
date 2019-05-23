#ifndef PYXELCORE_COMMON_H_
#define PYXELCORE_COMMON_H_

#include "pyxelcore/constants.h"

namespace pyxelcore {

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

template <typename T, size_t N, size_t M>
T** NewPointerArrayFromArray2D(T (&array2d)[N][M]) {
  T** pointer_array = new T*[N];

  for (int32_t i = 0; i < N; i++) {
    pointer_array[i] = array2d[i];
  }

  return pointer_array;
}

inline void PrintError(const char* message, const char* func_name) {
  printf("pyxel error: %s in '%s'\n", message, func_name);
}

#define PRINT_ERROR(message) PrintError(message, __FUNCTION__)

}  // namespace pyxelcore

#endif  // PYXELCORE_COMMON_H_

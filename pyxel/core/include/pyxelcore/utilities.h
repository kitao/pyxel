#ifndef PYXELCORE_UTILITIES_H_
#define PYXELCORE_UTILITIES_H_

#include <cstdint>

namespace pyxelcore {

class Utilities {
 public:
  static int32_t GetConstantNumber(const char* name);
  static const char* GetConstantString(const char* name);
  static void RaiseError(const char* message);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_UTILITIES_H_

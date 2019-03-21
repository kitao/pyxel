#include <cstddef>

#include "pyxelcore/constants.h"

namespace pyxelcore {

Constants::Constants() {}

Constants::~Constants() {}

int32_t Constants::GetConstantNumber(const char* name) {
  return 0;
}

const char* Constants::GetConstantString(const char* name) {
  return "test";
}

}  // namespace pyxelcore

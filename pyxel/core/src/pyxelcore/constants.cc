#include <cstddef>

#include "pyxelcore/constants.h"

namespace pyxelcore {

Constants::Constants() {}

Constants::~Constants() {}

int32_t Constants::get_constant_number(const char* name) {
  return 0;
}

const char* Constants::get_constant_string(const char* name) {
  return "test";
}

}  // namespace pyxelcore

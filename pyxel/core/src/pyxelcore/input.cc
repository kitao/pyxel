#include "pyxelcore/input.h"

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

int32_t Input::btn(int32_t key) {
  return 0;
}

int32_t Input::btnp(int32_t key, int32_t hold, int32_t period) {
  return 0;
}

int32_t Input::btnr(int32_t key) {
  return 0;
}

void Input::mouse(int32_t visible) {}

}  // namespace pyxelcore

#include "pyxelcore/input.h"

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

int32_t Input::Btn(int32_t key) {
  return 0;
}

int32_t Input::Btnp(int32_t key, int32_t hold, int32_t period) {
  return 0;
}

int32_t Input::Btnr(int32_t key) {
  return 0;
}

void Input::Mouse(int32_t visible) {}

}  // namespace pyxelcore

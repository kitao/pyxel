#include "pyxelcore/input.h"

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

bool Input::Btn(int32_t key) {
  return 0;
}

bool Input::Btnp(int32_t key, int32_t hold, int32_t period) {
  return 0;
}

bool Input::Btnr(int32_t key) {
  return 0;
}

void Input::Mouse(int32_t visible) {}

}  // namespace pyxelcore
